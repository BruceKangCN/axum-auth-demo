use std::sync::Arc;

use axum::{
    Form, Json,
    extract::{Query, State},
    http::StatusCode,
    response::{IntoResponse, Redirect},
};
use axum_extra::extract::{
    CookieJar,
    cookie::{Cookie, SameSite},
};
use oauth2::{
    AccessToken, AuthorizationCode, CsrfToken, EmptyExtraTokenFields, RefreshToken,
    StandardTokenResponse, TokenResponse, basic::BasicTokenType,
};
use serde::{Deserialize, Serialize};
use tower_sessions::Session;
use utoipa::ToSchema;

use crate::{app::AppState, oauth2::Claims};

const CRSR_STATE_KEY: &str = "oauth2.csrf-state";
const NEXT_URL_KEY: &str = "oauth2.next-url";
const ACCESS_TOKEN_KEY: &str = "oauth2.access-token";
const REFRESH_TOKEN_KEY: &str = "oauth2.refresh-token";

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub(crate) struct LoginForm {
    next: Option<String>,
}

#[utoipa::path(post, path = "/login")]
pub async fn login(
    State(state): State<Arc<AppState>>,
    session: Session,
    Form(form): Form<LoginForm>,
) -> Result<impl IntoResponse, StatusCode> {
    let (auth_url, crsf_state) = state
        .oauth2_client
        .authorize_url(CsrfToken::new_random)
        .url();

    session
        .insert(CRSR_STATE_KEY, crsf_state.secret())
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    session
        .insert(NEXT_URL_KEY, form.next)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Redirect::to(auth_url.as_str()))
}

#[derive(Clone, Debug, Deserialize)]
pub(crate) struct CallbackQuery {
    code: String,
    state: CsrfToken,
}

#[utoipa::path(get, path = "/auth/callback")]
pub async fn callback(
    State(state): State<Arc<AppState>>,
    session: Session,
    jar: CookieJar,
    Query(query): Query<CallbackQuery>,
) -> Result<impl IntoResponse, StatusCode> {
    let Some(csrf_state) = session
        .get::<String>(CRSR_STATE_KEY)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    else {
        return Err(StatusCode::BAD_REQUEST);
    };

    if &csrf_state != query.state.secret() {
        return Err(StatusCode::BAD_REQUEST);
    }

    let auth_code = AuthorizationCode::new(query.code);
    let token_response = state
        .oauth2_client
        .exchange_code(auth_code)
        .request_async(&state.oauth2_request_client)
        .await
        .map_err(|_| StatusCode::BAD_GATEWAY)?;
    let access_token = token_response.access_token().secret().to_owned();

    // store access token in cookie because redirect response cannot return data
    let access_token_cookie = Cookie::build((ACCESS_TOKEN_KEY, access_token.clone()))
        .http_only(false)
        .secure(false)
        .same_site(SameSite::Lax)
        .build();
    let jar = update_cookie_jar(jar, &token_response).add(access_token_cookie);

    let profile_url = format!(
        "{}/application/o/userinfo/",
        state.settings.authentik_base_url.trim_end_matches('/')
    );
    let profile: Claims = state
        .http_client
        .get(profile_url)
        .bearer_auth(access_token)
        .send()
        .await
        .map_err(|_| StatusCode::BAD_GATEWAY)?
        .json()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // TODO: save or update user profile and access token
    dbg!(profile);

    let next_url = session
        .get(NEXT_URL_KEY)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .unwrap_or("/".to_string());

    Ok((jar, Redirect::to(&next_url)))
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
struct RefreshResponse {
    access_token: String,
}

#[utoipa::path(post, path = "/auth/refresh")]
pub async fn refresh(
    State(state): State<Arc<AppState>>,
    jar: CookieJar,
) -> Result<impl IntoResponse, StatusCode> {
    let Some(cookie) = jar.get(REFRESH_TOKEN_KEY) else {
        return Err(StatusCode::UNAUTHORIZED);
    };
    let token = RefreshToken::new(cookie.value().to_owned());

    let token_response = state
        .oauth2_client
        .exchange_refresh_token(&token)
        .request_async(&state.oauth2_request_client)
        .await
        .map_err(|_| StatusCode::BAD_GATEWAY)?;
    let access_token = token_response.access_token().secret().to_owned();
    let jar = update_cookie_jar(jar, &token_response);

    let refresh_response = RefreshResponse { access_token };

    Ok((jar, Json(refresh_response)))
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub(crate) struct LogoutForm {
    access_token: Option<String>,
}

#[utoipa::path(post, path = "/logout")]
pub async fn logout(
    State(state): State<Arc<AppState>>,
    jar: CookieJar,
    Form(form): Form<LogoutForm>,
) -> Result<impl IntoResponse, StatusCode> {
    if let Some(token) = form.access_token {
        let token = AccessToken::new(token);
        state
            .oauth2_client
            .revoke_token(token.into())
            .map_err(|_| StatusCode::BAD_REQUEST)?
            .request_async(&state.oauth2_request_client)
            .await
            .map_err(|_| StatusCode::BAD_GATEWAY)?;
    }

    if let Some(cookie) = jar.get(REFRESH_TOKEN_KEY) {
        let token = RefreshToken::new(cookie.value().to_owned());
        state
            .oauth2_client
            .revoke_token(token.into())
            .map_err(|_| StatusCode::BAD_REQUEST)?
            .request_async(&state.oauth2_request_client)
            .await
            .map_err(|_| StatusCode::BAD_GATEWAY)?;
    }

    Ok(jar.remove(REFRESH_TOKEN_KEY))
}

fn update_cookie_jar(
    jar: CookieJar,
    token_response: &StandardTokenResponse<EmptyExtraTokenFields, BasicTokenType>,
) -> CookieJar {
    let Some(refresh_token) = token_response.refresh_token() else {
        return jar;
    };

    let cookie = Cookie::build((REFRESH_TOKEN_KEY, refresh_token.secret().to_owned()))
        .http_only(true)
        .secure(false)
        .same_site(SameSite::Lax)
        .build();

    jar.add(cookie)
}

use std::sync::Arc;

use anyhow::Context;
use axum::Router;
use oauth2::{
    AuthUrl, ClientId, ClientSecret, DeviceAuthorizationUrl, EndpointSet, IntrospectionUrl,
    RedirectUrl, RevocationUrl, TokenUrl,
};
use tower_http::cors::{Any, CorsLayer};
use tower_sessions::{Expiry, MemoryStore, SessionManagerLayer, cookie::time::Duration};
use utoipa_axum::{router::OpenApiRouter, routes};
use utoipa_swagger_ui::SwaggerUi;

use crate::{
    handler::*,
    oauth2::{KeyCache, init_jwk_set_refresh},
    settings::ApplicationSettings,
};

type OAuth2Client =
    oauth2::basic::BasicClient<EndpointSet, EndpointSet, EndpointSet, EndpointSet, EndpointSet>;

#[derive(Debug)]
pub struct AppState {
    pub settings: ApplicationSettings,
    pub key_cache: KeyCache,
    pub http_client: reqwest::Client,
    pub oauth2_client: OAuth2Client,
    pub oauth2_request_client: oauth2::reqwest::Client,
}

impl AppState {
    pub async fn from_settings(settings: &ApplicationSettings) -> anyhow::Result<AppState> {
        let key_cache = init_jwk_set_refresh(settings).await?;

        let http_client = reqwest::Client::builder()
            .tls_danger_accept_invalid_certs(true)
            .user_agent(&settings.slug)
            .build()
            .context("failed to build HTTP client")?;

        // avoid SSRF
        let oauth2_request_client = oauth2::reqwest::Client::builder()
            .danger_accept_invalid_certs(true)
            .user_agent(&settings.slug)
            .redirect(oauth2::reqwest::redirect::Policy::none())
            .build()
            .context("failed to build request client of OAuth 2.0")?;

        let client_id = ClientId::new(settings.client_id.to_owned());
        let client_secret = ClientSecret::new(settings.client_secret.to_owned());
        let base_url = settings.authentik_base_url.trim_end_matches('/');
        let auth_url = format!("{}/application/o/authorize/", base_url);
        let token_url = format!("{}/application/o/token/", base_url);
        let revocation_url = format!("{}/application/o/revoke/", base_url);
        let dev_auth_url = format!("{}/application/o/device/", base_url);
        let introspection_url = format!("{}/application/o/introspect/", base_url);
        let oauth2_client = oauth2::basic::BasicClient::new(client_id)
            .set_client_secret(client_secret)
            .set_auth_uri(AuthUrl::new(auth_url)?)
            .set_token_uri(TokenUrl::new(token_url)?)
            .set_redirect_uri(RedirectUrl::new(settings.redirect_uri.to_owned())?)
            .set_revocation_url(RevocationUrl::new(revocation_url)?)
            .set_device_authorization_url(DeviceAuthorizationUrl::new(dev_auth_url)?)
            .set_introspection_url(IntrospectionUrl::new(introspection_url)?);

        Ok(AppState {
            settings: settings.clone(),
            key_cache,
            http_client,
            oauth2_client,
            oauth2_request_client,
        })
    }
}

pub fn create_app(state: AppState) -> Router {
    let (router, api) = OpenApiRouter::new()
        .routes(routes!(auth::login))
        .routes(routes!(auth::callback))
        .routes(routes!(auth::refresh))
        .routes(routes!(auth::logout))
        .routes(routes!(greet::handler))
        .routes(routes!(info::handler))
        .split_for_parts();
    let swagger = SwaggerUi::new("/swagger-ui").url("/openapi.json", api);
    let cors_layer = CorsLayer::new().allow_origin(Any);
    let session_layer = SessionManagerLayer::new(MemoryStore::default())
        .with_secure(false)
        .with_expiry(Expiry::OnInactivity(Duration::seconds(10)));

    router
        .merge(swagger)
        .with_state(Arc::new(state))
        .layer(cors_layer)
        .layer(session_layer)
}

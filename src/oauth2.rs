use std::{sync::Arc, time::Duration};

use anyhow::Context;
use axum::{extract::FromRequestParts, http::StatusCode};
use axum_extra::{
    TypedHeader,
    headers::{Authorization, authorization::Bearer},
};
use jsonwebtoken::{DecodingKey, decode_header, jwk::JwkSet};
use reqwest::Url;
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;
use tracing::{debug, info, warn};
use utoipa::ToSchema;

use crate::{app::AppState, settings::ApplicationSettings};

pub type KeyCache = Arc<RwLock<JwkSet>>;

#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
struct Claims {
    pub sub: String,
    pub preferred_username: String,
}

async fn fetch_jwk_set(url: &str) -> anyhow::Result<JwkSet> {
    let resp = reqwest::get(url).await.context("failed to fetch JWKS")?;
    let jwk_set: JwkSet = resp.json().await.context("failed to parse JWKS")?;

    Ok(jwk_set)
}

async fn jwk_set_refresh(cache: KeyCache, jwk_set_url: &str) {
    let interval = Duration::from_mins(15);
    let mut interval = tokio::time::interval(interval);
    loop {
        interval.tick().await;
        match fetch_jwk_set(jwk_set_url).await {
            Ok(keys) => *cache.write().await = keys,
            Err(e) => warn!(?e, "failed to refresh JWK set"),
        }
    }
}

pub async fn init_jwk_set_refresh(settings: &ApplicationSettings) -> anyhow::Result<KeyCache> {
    let base_url =
        Url::parse(&settings.authentik_base_url).context("failed to parse authentik base URL")?;
    let jwk_set_url = base_url
        .join(&format!("/application/o/{}/jwks/", &settings.slug))
        .context("failed to build JWKS URL")?;
    let initial_keys = fetch_jwk_set(jwk_set_url.as_str()).await?;
    let key_cache = Arc::new(RwLock::new(initial_keys));

    let jwk_set_url = jwk_set_url.to_owned();
    let key_cache_cloned = key_cache.clone();
    tokio::spawn(async move {
        jwk_set_refresh(key_cache_cloned, jwk_set_url.as_str()).await;
    });

    Ok(key_cache)
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, ToSchema)]
pub struct AuthenticatedUser {
    pub username: String,
    pub sub: String,
}

impl FromRequestParts<Arc<AppState>> for AuthenticatedUser {
    type Rejection = StatusCode;

    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        state: &Arc<AppState>,
    ) -> Result<Self, Self::Rejection> {
        let auth =
            match TypedHeader::<Authorization<Bearer>>::from_request_parts(parts, state).await {
                Ok(auth) => auth,
                Err(err) => {
                    debug!(?err, "failed to get authorization bearer from header");
                    return Err(StatusCode::UNAUTHORIZED);
                }
            };
        let token = auth.token();
        let jwk = state.key_cache.read().await.clone();

        decode_jwt(token, &jwk, &state.settings.client_id).map_err(|err| {
            info!(?err, "authentication failed");
            StatusCode::UNAUTHORIZED
        })
    }
}

fn decode_jwt(
    token: &str,
    key_cache: &JwkSet,
    client_id: &str,
) -> anyhow::Result<AuthenticatedUser> {
    let header = decode_header(token).context("failed to decode JWT")?;
    let kid = header
        .kid
        .as_deref()
        .ok_or(anyhow::anyhow!("missing key ID"))?;
    let jwk = key_cache
        .find(kid)
        .ok_or(anyhow::anyhow!("key not found from key set"))?
        .to_owned();
    let decoding_key =
        DecodingKey::from_jwk(&jwk).context("failed to get decoding key from JWK")?;
    debug!(
        ?header,
        kid,
        ?jwk,
        ?decoding_key,
        "get key for OAuth2 authentication"
    );

    let mut validation = jsonwebtoken::Validation::new(header.alg);
    validation.set_audience(&[client_id]);
    debug!(?validation, "create validation for OAuth2 authentication");

    let data = jsonwebtoken::decode::<Claims>(token, &decoding_key, &validation).map_err(|e| {
        info!(?e, "error during JWT decoding");
        anyhow::anyhow!("error during JWT decoding")
    })?;

    Ok(AuthenticatedUser {
        username: data.claims.preferred_username,
        sub: data.claims.sub,
    })
}

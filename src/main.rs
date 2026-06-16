mod auth;
mod common;
mod handler;
mod settings;

use std::sync::Arc;

use anyhow::Context;
use axum::Router;
use tower_http::cors::{Any, CorsLayer};

use crate::{
    auth::init_jwk_set_refresh,
    common::{AppState, init_listener, init_tracing},
    handler::*,
    settings::load_settings,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();
    let settings = load_settings().await.context("failed to load settings")?;
    init_tracing();
    let key_cache = init_jwk_set_refresh(&settings.app).await?;
    let listener = init_listener(&settings.server)
        .await
        .context("failed to create TCP listener")?;

    let state = AppState {
        settings: Arc::new(settings.app),
        key_cache: key_cache.clone(),
    };
    let app = Router::new()
        .merge(greet::router())
        .merge(info::router())
        .with_state(state)
        .layer(CorsLayer::new().allow_origin(Any));

    axum::serve(listener, app)
        .await
        .context("application encountered an error")
}

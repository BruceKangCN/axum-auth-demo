mod app;
mod common;
mod handler;
mod oauth2;
mod settings;

use anyhow::Context;

use crate::{
    app::{AppState, create_app},
    common::{init_listener, init_tracing},
    settings::load_settings,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();
    let settings = load_settings().await.context("failed to load settings")?;
    init_tracing();
    let listener = init_listener(&settings.server)
        .await
        .context("failed to create TCP listener")?;

    let state = AppState::from_settings(&settings.app).await?;
    let app = create_app(state);

    axum::serve(listener, app)
        .await
        .context("application encountered an error")
}

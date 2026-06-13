use std::sync::Arc;

use tokio::net::TcpListener;
use tracing_subscriber::{EnvFilter, layer::SubscriberExt, util::SubscriberInitExt};

use crate::{
    auth::KeyCache,
    settings::{ApplicationSettings, ServerSettings},
};

#[derive(Clone, Debug)]
pub struct AppState {
    pub settings: Arc<ApplicationSettings>,
    pub key_cache: KeyCache,
}

pub fn init_tracing() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer().pretty())
        .with(EnvFilter::from_default_env())
        .init();
}

pub async fn init_listener(settings: &ServerSettings) -> tokio::io::Result<TcpListener> {
    let host = settings.host.as_deref().unwrap_or("localhost");
    let port = settings.port.unwrap_or(3000);
    TcpListener::bind((host, port)).await
}

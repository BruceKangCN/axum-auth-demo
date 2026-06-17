use tokio::net::TcpListener;
use tracing_subscriber::{EnvFilter, layer::SubscriberExt, util::SubscriberInitExt};

use crate::settings::ServerSettings;

pub fn init_tracing() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer().pretty())
        .with(EnvFilter::from_default_env())
        .init();
}

pub async fn init_listener(settings: &ServerSettings) -> tokio::io::Result<TcpListener> {
    TcpListener::bind((settings.host.as_str(), settings.port)).await
}

pub async fn graceful_shutdown_handler() {
    tokio::signal::ctrl_c()
        .await
        .expect("failed to install Ctrl + C handler");
}

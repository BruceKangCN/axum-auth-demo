use std::sync::Arc;

use axum::{Router, routing::get};

use crate::{app::AppState, oauth2::AuthenticatedUser};

async fn handler(user: AuthenticatedUser) -> String {
    format!("Hello, {}!", &user.username)
}

pub fn router() -> Router<Arc<AppState>> {
    Router::new().route("/greet", get(handler))
}

use std::sync::Arc;

use axum::{Json, Router, routing::get};

use crate::{app::AppState, oauth2::AuthenticatedUser};

async fn handler(user: AuthenticatedUser) -> Json<AuthenticatedUser> {
    Json(user)
}

pub fn router() -> Router<Arc<AppState>> {
    Router::new().route("/info", get(handler))
}

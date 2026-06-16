use axum::{Json, Router, routing::get};

use crate::{auth::AuthenticatedUser, common::AppState};

async fn handler(user: AuthenticatedUser) -> Json<AuthenticatedUser> {
    Json(user)
}

pub fn router() -> Router<AppState> {
    Router::new().route("/info", get(handler))
}

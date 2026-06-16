use axum::{Router, routing::get};

use crate::{auth::AuthenticatedUser, common::AppState};

async fn handler(user: AuthenticatedUser) -> String {
    format!("Hello, {}!", &user.username)
}

pub fn router() -> Router<AppState> {
    Router::new().route("/greet", get(handler))
}

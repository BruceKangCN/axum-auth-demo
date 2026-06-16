use std::sync::Arc;

use axum::{
    Router,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
};
use serde::Serialize;

use crate::common::AppState;

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
struct TokenResponse {
    pub access_token: String,
    pub refresh_token: String,
}

async fn login() -> Result<impl IntoResponse, StatusCode> {
    todo!()
}

async fn callback() -> Result<impl IntoResponse, StatusCode> {
    todo!()
}

async fn refresh() -> Result<impl IntoResponse, StatusCode> {
    todo!()
}

async fn logout() -> Result<impl IntoResponse, StatusCode> {
    todo!()
}

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/login", post(login))
        .route("/auth/callback", get(callback))
        .route("/auth/refresh", post(refresh))
        .route("/logout", post(logout))
}

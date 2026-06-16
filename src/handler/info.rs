use axum::Json;

use crate::oauth2::AuthenticatedUser;

#[utoipa::path(get, path = "/info", responses((status = OK, body = AuthenticatedUser)))]
pub async fn handler(user: AuthenticatedUser) -> Json<AuthenticatedUser> {
    Json(user)
}

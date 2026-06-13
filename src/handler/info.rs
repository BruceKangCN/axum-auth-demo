use axum::Json;

use crate::auth::AuthenticatedUser;

pub async fn info_handler(user: AuthenticatedUser) -> Json<AuthenticatedUser> {
    Json(user)
}

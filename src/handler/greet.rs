use crate::oauth2::AuthenticatedUser;

#[utoipa::path(get, path = "/greet", responses((status = OK, body = String)))]
pub async fn handler(user: AuthenticatedUser) -> String {
    format!("Hello, {}!", &user.username)
}

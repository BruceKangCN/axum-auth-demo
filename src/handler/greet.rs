use crate::auth::AuthenticatedUser;

pub async fn greet_handler(user: AuthenticatedUser) -> String {
    format!("Hello, {}!", &user.username)
}

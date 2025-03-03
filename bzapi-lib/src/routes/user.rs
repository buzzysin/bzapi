use axum::Json;
use axum::Router;
use axum::routing::get;

use crate::models::User;
use crate::models::user::NewUser;

#[utoipa::path(get, path = "/")]
pub async fn all() -> Json<Vec<User>> {
    Json(vec![])
}

#[utoipa::path(get, path = "/new")]
pub async fn new(user: Json<NewUser>) -> Json<User> {
    Json(User {
        id: "0".to_string(),
        email: user.email.clone(),
        name: user.name.clone(),
    })
}

pub fn routes() -> Router {
    Router::new().route("/", get(all)).route("/new", get(new))
}

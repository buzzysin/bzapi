use axum::{Json, Router};

use crate::models::user::User;

pub fn routes() -> Router {
    Router::new().route("/", axum::routing::get(test))
}

#[axum::debug_handler]
async fn test() -> Json<Vec<User>> {
    let users = vec![
        User {
            name: Some("John Doe".to_string()),
            email: Some("john.doe@example.com".to_string()),
            ..Default::default()
        },
        User {
            name: Some("Jane Doe".to_string()),
            email: Some("jane.doe@example.com".to_string()),
            ..Default::default()
        },
    ];

    Json(users)
}

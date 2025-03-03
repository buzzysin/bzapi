use axum::Json;

use crate::models::User;
use crate::models::user::NewUser;

#[derive(Clone)]
pub struct UserHandler {}

impl Default for UserHandler {
    fn default() -> Self {
        Self {}
    }
}

impl UserHandler {
    pub async fn all() -> Json<Vec<User>> {
        Json(vec![
            // Some test data
            User {
                id: "1".to_string(),
                name: "John Doe".to_string(),
                email: "".to_string(),
            },
            User {
                id: "2".to_string(),
                name: "Jane Doe".to_string(),
                email: "".to_string(),
            },
        ])
    }

    pub async fn new(user: Json<NewUser>) -> Json<User> {
        Json(User {
            id: "3".to_string(),
            name: user.name.clone(),
            email: user.email.clone(),
        })
    }

    pub async fn get(id: String) -> Json<User> {
        Json(User {
            id: id.clone(),
            name: "John Doe".to_string(),
            email: "".to_string(),
        })
    }

    pub async fn update(id: String, user: Json<NewUser>) -> Json<User> {
        Json(User {
            id: id.clone(),
            name: user.name.clone(),
            email: user.email.clone(),
        })
    }

    pub async fn delete(id: String) -> Json<User> {
        Json(User {
            id: id.clone(),
            name: "John Doe".to_string(),
            email: "".to_string(),
        })
    }

    pub async fn search(name: String) -> Json<Vec<User>> {
        Json(vec![
            User {
                id: "1".to_string(),
                name: name.clone(),
                email: "".to_string(),
            },
            User {
                id: "2".to_string(),
                name: name.clone(),
                email: "".to_string(),
            },
        ])
    }
}

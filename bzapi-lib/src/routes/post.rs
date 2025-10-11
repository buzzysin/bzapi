use axum::{Json, Router};

use crate::models::post::Post;

pub fn routes() -> Router {
    Router::new().route("/", axum::routing::get(test))
}

#[axum::debug_handler]
async fn test() -> Json<Vec<Post>> {
    let posts = vec![];

    Json(posts)
}

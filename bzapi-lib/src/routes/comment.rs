use axum::{Json, Router};

use crate::models::comment::Comment;

pub fn routes() -> Router {
    Router::new().route("/", axum::routing::get(test))
}

#[axum::debug_handler]
async fn test() -> Json<Vec<Comment>> {
    let comments = vec![];

    Json(comments)
}

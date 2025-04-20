use crate::models::comment::Comment;
use axum::{Json, Router};

pub fn routes() -> Router {
    Router::new().route("/", axum::routing::get(test))
}

#[axum::debug_handler]
async fn test() -> Json<Vec<Comment>> {
    let comments = vec![];

    Json(comments)
}

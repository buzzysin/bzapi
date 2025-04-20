use crate::models::tag::Tag;
use axum::{Json, Router};

pub fn routes() -> Router {
    Router::new().route("/", axum::routing::get(test))
}

#[axum::debug_handler]
async fn test() -> Json<Vec<Tag>> {
    let tags = vec![];

    Json(tags)
}

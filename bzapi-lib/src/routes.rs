use std::sync::Arc;

use crate::api_context::ApiContext;
use axum::Router;
use errors::error404;

pub mod auth;
pub mod errors;
pub mod user;

pub fn router(api_context: Arc<ApiContext>) -> Router {
    let api_providers = Arc::new(api_context.providers.clone());

    Router::new()
        .nest("/user", user::routes())
        .nest("/auth", auth::routes(api_providers))
        .fallback(error404)
}

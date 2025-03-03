use axum::Json;
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Serialize, Deserialize)]
pub enum ApiError {
    UnknownEndpoint,
    ServerError,
    ProviderError(String),
}

#[axum::debug_handler]
pub async fn error404() -> Json<ApiError> {
    Json(ApiError::UnknownEndpoint)
}

#[axum::debug_handler]
pub async fn error500() -> Json<ApiError> {
    Json(ApiError::ServerError)
}

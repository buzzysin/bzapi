use std::sync::Arc;

use axum::RequestPartsExt;
use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use axum::response::IntoResponse;
use axum::{Extension, body::Body};
use serde::Deserialize;
use serde::Serialize;

use crate::routes::errors::ApiError;
use crate::{api_context::ApiContext, providers::oauth2_provider::Oauth2Provider};

#[derive(Clone)]
pub struct ExtractProvider(pub Box<dyn Oauth2Provider>);

#[derive(Debug, Serialize, Deserialize)]
pub enum ExtractProviderError {
    MissingProvider(String),
    UnknownProvider(String),
    ProviderUnavailable,
}

impl IntoResponse for ExtractProviderError {
    fn into_response(self) -> axum::response::Response<Body> {
        match self {
            ExtractProviderError::MissingProvider(provider) => {
                axum::response::Json(ApiError::ProviderError(provider)).into_response()
            }
            ExtractProviderError::UnknownProvider(provider) => {
                axum::response::Json(ApiError::ProviderError(provider)).into_response()
            }
            ExtractProviderError::ProviderUnavailable => {
                axum::response::Json(ApiError::ProviderError("Provider unavailable".to_string()))
                    .into_response()
            }
        }
    }
}

impl<S> FromRequestParts<S> for ExtractProvider
where
    S: Send + Sync,
{
    type Rejection = ExtractProviderError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        // Fetch the api context
        let api_context = parts
            .extract::<Extension<Arc<ApiContext>>>()
            .await
            .map_err(|err| {
                ExtractProviderError::MissingProvider(format!(
                    "Failed to extract api context: {}",
                    err
                ))
            })?;

        // Fetch the provider from the path (format is /auth/{action}/{provider})
        let path = parts.uri.path();
        let provider_name = match path.split('/').nth(2) {
            Some(provider) => provider,
            None => return Err(ExtractProviderError::ProviderUnavailable),
        };

        // Find the provider
        let provider = api_context
            .providers
            .iter()
            .find(|provider| provider.provider() == provider_name);

        match provider {
            Some(provider) => Ok(ExtractProvider(provider.clone())),
            None => Err(ExtractProviderError::UnknownProvider(
                provider_name.to_string(),
            )),
        }
    }
}

use std::sync::Arc;

use axum::routing::get;
use axum::{Json, Router};
use serde::{Deserialize, Serialize};

use crate::{
    api_context::Oauth2Providers, extractors::ExtractProvider,
    providers::oauth2_provider::Oauth2Provider,
};

use super::errors::error404;

async fn authorise(
    ExtractProvider(provider): ExtractProvider,
) -> Result<axum::http::Response<axum::body::Body>, axum::response::ErrorResponse> {
    provider.authorize().await
}

async fn callback(
    ExtractProvider(provider): ExtractProvider,
) -> Result<axum::http::Response<axum::body::Body>, axum::response::ErrorResponse> {
    provider.authorize().await
}

#[derive(Debug, Serialize, Deserialize)]
struct Oauth2ProviderInfo {
    name: String,
    display_name: String,
}

impl From<Box<dyn Oauth2Provider>> for Oauth2ProviderInfo {
    fn from(provider: Box<dyn Oauth2Provider>) -> Self {
        Oauth2ProviderInfo {
            name: provider.provider(),
            display_name: provider.display_name(),
        }
    }
}

pub fn routes(providers: Arc<Oauth2Providers>) -> Router {
    let mut router = Router::new();

    router = router
        .route("/authorise/{provider}", get(authorise))
        .route("/callback/{provider}", get(callback))
        .route("/logout/{provider}", get(error404));

    let providers_endpoint = move || async move {
        let providers = providers
            .iter()
            .map(|provider| Oauth2ProviderInfo::from(provider.clone()))
            .collect::<Vec<Oauth2ProviderInfo>>();

        Json(providers)
    };

    router = router.route("/providers", get(providers_endpoint));

    router
}

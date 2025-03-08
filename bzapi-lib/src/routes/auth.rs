use std::sync::Arc;

use axum::{Json, Router};
use axum::{extract::Request, routing::get};
use serde::{Deserialize, Serialize};

use crate::{
    api_context::Oauth2Providers, extractors::ExtractProvider,
    providers::oauth2_provider::Oauth2Provider,
};

use super::errors::error404;

async fn authorise(
    ExtractProvider(provider): ExtractProvider,
    request: Request,
) -> Result<axum::http::Response<axum::body::Body>, axum::response::ErrorResponse> {
    provider.authorise(request).await
}

async fn callback(
    ExtractProvider(provider): ExtractProvider,
    request: Request,
) -> Result<axum::http::Response<axum::body::Body>, axum::response::ErrorResponse> {
    provider.callback(request).await
}

#[derive(Debug, Serialize, Deserialize)]
struct Oauth2ProviderInfo {
    name: String,
    display_name: String,
}

impl From<Box<dyn Oauth2Provider>> for Oauth2ProviderInfo {
    fn from(provider: Box<dyn Oauth2Provider>) -> Self {
        Oauth2ProviderInfo {
            name: provider.get_id().unwrap(),
            display_name: provider.get_display_name().unwrap(),
        }
    }
}

pub fn routes(providers: Arc<Oauth2Providers>) -> Router {
    let mut router = Router::new();

    router = router
        .route("/authorise/{provider}", get(authorise))
        .route("/authorize/{provider}", get(authorise)) // for the americans
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

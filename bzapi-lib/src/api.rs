use axum::Extension;
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;
use std::sync::Arc;
use tokio::net::TcpListener;

use crate::api_context::ApiContext;
use crate::api_context::ApiContextParameters;
use crate::routes;

pub async fn serve<A: tokio::net::ToSocketAddrs>(
    addr: A,
    params: ApiContextParameters,
) -> Result<(), std::io::Error> {
    let api_context = Arc::new(ApiContext::new(params));
    let api_router = routes::router(api_context.clone())
        .layer(ServiceBuilder::new().layer(TraceLayer::new_for_http()))
        .layer(Extension(api_context.clone()));
    let api_listener = TcpListener::bind(addr).await.unwrap();

    axum::serve(api_listener, api_router.into_make_service()).await
}

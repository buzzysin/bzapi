use axum::Extension;
use axum::routing::Router;
use bzauth_rs::runtimes::axum::runtime::{AxumRuntime, AxumRuntimeOptions};

pub fn routes(axum_options: AxumRuntimeOptions) -> Router {
    // Create the Axum runtime
    let AxumRuntime {
        routes: auth_routes,
        auth,
    } = AxumRuntime::from_options(axum_options);

    // Create the sub-router
    Router::new().merge(auth_routes).layer(Extension(auth))
}

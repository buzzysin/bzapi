use tokio::net::TcpListener;
use tracing::debug;

#[tokio::main]
async fn main() {
    // Initialize the API
    let db_pool = bzapi_lib::init();
    debug!("🟢 Initialisation complete");

    // Create the API router
    debug!("🔵 Creating API router");
    let api = bzapi_lib::make_api(db_pool.clone());
    debug!("🟢 API router created");

    // Start a TCP listener and serve the API
    debug!("🔵 Starting TCP listener");
    let listener = TcpListener::bind("127.0.0.1:3001").await.unwrap();
    debug!("🚀 Serving API...");
    axum::serve(listener, api.into_make_service())
        .await
        .unwrap();
}

mod adaptor;
mod config;

use bzapi_lib::models::MyConnection;
use diesel::r2d2::{ConnectionManager, Pool};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    // Load environment variables from .env file
    tracing::debug!("🔵 Loading environment variables from .env file");
    dotenvy::dotenv().ok();
    tracing::debug!("🟢 Environment variables loaded");

    // Initialize the tracing subscriber
    tracing::debug!("🔵 Initializing tracing subscriber");
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .with_target(false)
        .without_time()
        .init();
    tracing::debug!("🔵 Tracing subscriber initialized");

    // Initialize the database connection pool
    tracing::debug!("🔵 Initializing database connection");
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db_pool = Pool::builder()
        .build(ConnectionManager::<MyConnection>::new(db_url.clone()))
        .expect("Failed to create database connection pool");
    tracing::debug!("🟢 Database connection pool created: {}", db_url);

    // Initialize the API
    bzapi_lib::run_migrations(db_pool.clone());
    tracing::debug!("🟢 Initialisation complete");

    // Configure the runtime
    tracing::debug!("🔵 Configuring runtime");
    let runtime = config::get_axum_runtime(db_pool.clone());

    // Create the API router
    tracing::debug!("🔵 Creating API router");
    let api = bzapi_lib::make_api(runtime);
    tracing::debug!("🟢 API router created");

    // Start a TCP listener and serve the API
    tracing::debug!("🔵 Starting TCP listener");
    let api_listener = TcpListener::bind("0.0.0.0:3001").await.unwrap();
    tracing::debug!(
        "🚀 Serving API at {}, port {}",
        api_listener.local_addr().unwrap().ip(),
        api_listener.local_addr().unwrap().port()
    );
    axum::serve(api_listener, api.into_make_service())
        .await
        .unwrap();
}

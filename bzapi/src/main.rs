use tokio::net::TcpListener;
use tracing::debug;

use diesel::SqliteConnection;
use diesel::r2d2::{ConnectionManager, Pool};

#[tokio::main]
async fn main() {
    // Load environment variables from .env file
    debug!("🔵 Loading environment variables from .env file");
    dotenvy::dotenv().ok();
    debug!("🟢 Environment variables loaded");

    // Initialize the tracing subscriber
    debug!("🔵 Initializing tracing subscriber");
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .with_target(false)
        .without_time()
        .init();
    debug!("🔵 Tracing subscriber initialized");

    // Initialize the database connection pool
    debug!("🔵 Initializing database connection");
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db_pool = Pool::builder()
        .max_size(5)
        .build(ConnectionManager::<SqliteConnection>::new(db_url))
        .expect("Failed to create database connection pool");
    debug!("🟢 Database connection pool created");

    // Initialize the API
    bzapi_lib::run_migrations(db_pool.clone());
    debug!("🟢 Initialisation complete");

    // Create the API router
    debug!("🔵 Creating API router");
    let api = bzapi_lib::make_api(db_pool.clone());
    debug!("🟢 API router created");

    // Start a TCP listener and serve the API
    debug!("🔵 Starting TCP listener");
    let api_listener = TcpListener::bind("127.0.0.1:3001").await.unwrap();
    debug!("🚀 Serving API...");
    axum::serve(api_listener, api.into_make_service())
        .await
        .unwrap();
}

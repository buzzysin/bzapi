pub mod models;
pub mod routes;
pub mod schema;

use axum::Router;
use diesel::{
    SqliteConnection,
    r2d2::{ConnectionManager, Pool},
};
use diesel_migrations::{EmbeddedMigrations, MigrationHarness, embed_migrations};
use routes::{auth, user};
use tracing::debug;
pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("../migrations/sqlite");

pub fn init() -> Pool<ConnectionManager<SqliteConnection>> {
    // Initialize the tracing subscriber
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .with_target(false)
        .without_time()
        .init();
    debug!("🔵 Tracing subscriber initialized");

    // Load environment variables from .env file
    debug!("🔵 Loading environment variables from .env file");
    dotenvy::dotenv().ok();
    debug!("🟢 Environment variables loaded");

    debug!("🔵 Initializing database connection");
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db_pool = Pool::builder()
        .max_size(5)
        .build(ConnectionManager::<SqliteConnection>::new(db_url))
        .expect("Failed to create database connection pool");
    debug!("🟢 Database connection pool created");

    debug!("🔵 Acquiring database connection");
    let mut connection = db_pool.get().expect("Failed to get database connection");
    debug!("🟢 Database connection acquired");

    debug!("🔵 Running migrations");
    connection
        .run_pending_migrations(MIGRATIONS)
        .expect("Migrations failed to run");
    debug!("🟢 Migrations completed");

    db_pool
}

pub fn make_api(conn_pool: Pool<ConnectionManager<SqliteConnection>>) -> Router {
    // Create the sub-router
    Router::new()
        .nest("/user", user::routes())
        .nest("/auth", auth::routes(conn_pool))
}

pub mod models;
pub mod openapi;
pub mod routes;
pub mod schema;

use axum::Router;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel_migrations::{EmbeddedMigrations, MigrationHarness, embed_migrations};
use tracing::debug;

use models::MyConnection;
use routes::{
    auth::{self},
    user,
};

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("../migrations/sqlite");

pub fn run_migrations(db_pool: Pool<ConnectionManager<MyConnection>>) {
    debug!("🔵 Acquiring database connection");
    let mut connection = db_pool.get().expect("Failed to get database connection");
    debug!("🟢 Database connection acquired");

    debug!("🔵 Running migrations");
    connection
        .run_pending_migrations(MIGRATIONS)
        .expect("Migrations failed to run");
    debug!("🟢 Migrations completed");
}

pub fn make_api(conn_pool: Pool<ConnectionManager<MyConnection>>) -> Router {
    // Create the sub-router
    Router::new()
        .nest("/user", user::routes())
        .nest("/auth", auth::routes(conn_pool))
}

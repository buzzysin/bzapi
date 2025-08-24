pub mod adaptor;
pub mod models;
pub mod openapi;
pub mod routes;
pub mod schema;

use std::time::Duration;

use axum::Router;
use bzauth_rs::runtimes::axum::AxumRuntimeOptions;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel_migrations::{EmbeddedMigrations, MigrationHarness, embed_migrations};
use models::MyConnection;
use routes::auth::{self};
use routes::user;
use tracing::debug;

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

fn cors_layer() -> tower_http::cors::CorsLayer {
    let layer = tower_http::cors::CorsLayer::new()
        .allow_methods(tower_http::cors::Any)
        .allow_headers(tower_http::cors::Any)
        .max_age(tower_http::cors::MaxAge::exact(Duration::from_secs(3600)))
        .pipe(|layer: tower_http::cors::CorsLayer| {
            #[cfg(debug_assertions)]
            {
                layer.allow_origin(tower_http::cors::Any)
            }
            #[cfg(not(debug_assertions))]
            {
                layer
            }
        });

    layer
}

pub fn make_api(axum_options: AxumRuntimeOptions) -> Router {
    // Create the sub-router
    Router::new()
        .nest("/user", user::routes())
        .nest("/auth", auth::routes(axum_options))
        .layer(cors_layer())
}

pub trait Pipe
where
    Self: Sized,
{
    fn pipe<F>(self, f: F) -> Self
    where
        F: FnOnce(Self) -> Self,
    {
        f(self)
    }
}

impl<T> Pipe for T {}

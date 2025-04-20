// Auth models
pub mod account;
pub mod session;
pub mod user;
pub mod verification_token;

// Content models
pub mod comment;
pub mod post;
pub mod tag;

#[cfg(feature = "sqlite")]
pub type MyConnection = diesel::SqliteConnection;

#[cfg(feature = "postgres")]
pub type MyConnection = diesel::PgConnection;
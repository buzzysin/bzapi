// Auth models
pub mod account;
pub mod session;
pub mod user;
pub mod verification_token;

// Content models
pub mod comment;
pub mod post;
pub mod tag;

#[cfg(all(
  // Ensure only one database feature is enabled
  feature = "sqlite", 
  feature = "postgres",
  // This will trigger a compile error if both features are enabled
))]
compile_error!(
    "Detected multiple database features enabled. Please enable only one, i.e. 'sqlite' or 'postgres'."
);

#[cfg(all(feature = "diesel", feature = "sqlite"))]
pub type MyConnection = diesel::SqliteConnection;

#[cfg(all(feature = "diesel", feature = "postgres"))]
pub type MyConnection = diesel::PgConnection;

use axum::{Extension, routing::Router};
use bzauth_rs::{
    adaptors::diesel::{DieselAdapterOptions, DieselAdaptor},
    auth::AuthOptions,
    providers::{DiscordProvider, GoogleProvider},
    runtimes::axum::runtime::{AxumRuntime, AxumRuntimeOptions},
};
use diesel::{
    SqliteConnection,
    r2d2::{ConnectionManager, Pool},
};

pub struct ApiAuthAdaptor;
bzauth_rs::adapt_diesel!(
    ApiAuthAdaptor,
    SqliteConnection,
    User = crate::models::user::User,
    UserTable = crate::schema::users,
    Account = crate::models::account::Account,
    AccountTable = crate::schema::accounts,
    Session = crate::models::session::Session,
    SessionTable = crate::schema::sessions,
    VerificationToken = crate::models::verification_token::VerificationToken,
    VerificationTokenTable = crate::schema::verification_tokens,
);

pub fn routes(conn_pool: Pool<ConnectionManager<SqliteConnection>>) -> Router {
    // Use the connection pool to create a new Diesel connection
    let diesel_options = DieselAdapterOptions {
        conn_pool,
        adaptor: ApiAuthAdaptor,
    };
    let diesel_adaptor = DieselAdaptor::from_options(diesel_options);

    // Create the auth options
    let auth_options = AuthOptions::new()
        .add_provider(DiscordProvider::new().into())
        .add_provider(GoogleProvider::new().into())
        .with_adaptor(diesel_adaptor.into());

    // Create the Axum runtime options
    let axum_options = AxumRuntimeOptions { auth_options };

    // Create the Axum runtime
    let AxumRuntime {
        routes: auth_routes,
        auth,
    } = AxumRuntime::from_options(axum_options);

    // Create the sub-router
    Router::new()
        .merge(auth_routes)
        .layer(Extension(auth))
}

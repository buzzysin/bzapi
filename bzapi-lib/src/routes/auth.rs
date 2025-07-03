use std::sync::Arc;

use axum::Extension;
use axum::routing::Router;
use bzauth_rs::adaptors::diesel::{DieselAdapterOptions, DieselAdaptor};
use bzauth_rs::auth::{AuthOptions, SignInOptions};
use bzauth_rs::providers::{DiscordProvider, GoogleProvider};
use bzauth_rs::runtimes::axum::runtime::{AxumRuntime, AxumRuntimeOptions};
use diesel::r2d2::{ConnectionManager, Pool};

use crate::models::MyConnection;

pub struct ApiAuthAdaptor;
bzauth_rs::adapt_diesel!(
    ApiAuthAdaptor,
    MyConnection,
    User = crate::models::user::User,
    UserTable = crate::schema::users,
    Account = crate::models::account::Account,
    AccountTable = crate::schema::accounts,
    Session = crate::models::session::Session,
    SessionTable = crate::schema::sessions,
    VerificationToken = crate::models::verification_token::VerificationToken,
    VerificationTokenTable = crate::schema::verification_tokens,
);

pub fn routes(conn_pool: Pool<ConnectionManager<MyConnection>>) -> Router {
    // Use the connection pool to create a new Diesel connection
    let diesel_options = DieselAdapterOptions {
        conn_pool,
        adaptor: ApiAuthAdaptor,
    };
    let diesel_adaptor = DieselAdaptor::from_options(diesel_options);

    // Create the auth options
    let auth_options = AuthOptions::new()
        .with_adaptor(diesel_adaptor.into())
        .add_provider(Box::new(DiscordProvider::new()))
        .add_provider(Box::new(GoogleProvider::new()))
        .with_callbacks(bzauth_rs::auth::AuthCallbackOptions {
            sign_in: Some(Arc::new(|sign_in_options: SignInOptions| {
                Box::pin(async move {
                    // Here you can implement custom sign-in logic
                    // For example, you could log the sign-in attempt
                    tracing::info!(
                        "[auth] User {:?} is attempting to sign in with account: {:?}",
                        sign_in_options.user.as_ref().map(|u| u.id.clone()),
                        sign_in_options.account
                    );

                    // Return a success result
                    bzauth_rs::auth::SignInResult::Success
                })
            })),
            ..Default::default()
        });

    // Create the Axum runtime options
    let axum_options = AxumRuntimeOptions { auth_options };

    // Create the Axum runtime
    let AxumRuntime {
        routes: auth_routes,
        auth,
    } = AxumRuntime::from_options(axum_options);

    // Create the sub-router
    Router::new().merge(auth_routes).layer(Extension(auth))
}

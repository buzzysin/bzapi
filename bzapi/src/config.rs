use bzapi_lib::adaptor::MyDieselAdaptor;
use bzapi_lib::models::MyConnection;
use bzauth_rs::adaptors::diesel::{DieselAdapterOptions, DieselAdaptor};
use bzauth_rs::auth::{AuthCallbackOptions, AuthOptions, SignInCallback};
use bzauth_rs::providers::{DiscordProvider, GoogleProvider};
use bzauth_rs::runtimes::axum::AxumRuntimeOptions;
use diesel::r2d2::{ConnectionManager, Pool};

pub fn get_axum_runtime(conn_pool: Pool<ConnectionManager<MyConnection>>) -> AxumRuntimeOptions {
    // Use the connection pool to create a new Diesel connection
    let diesel_options = DieselAdapterOptions {
        conn_pool,
        adaptor: MyDieselAdaptor,
    };
    let diesel_adaptor = DieselAdaptor::from_options(diesel_options);

    // Create the auth options
    let auth_options = AuthOptions::new()
        .with_adaptor(diesel_adaptor.into())
        .add_provider(Box::new(DiscordProvider::new()))
        .add_provider(Box::new(GoogleProvider::new()))
        .with_callbacks(AuthCallbackOptions {
            sign_in: SignInCallback::new(|sign_in_options| {
                Box::pin(async move {
                    tracing::info!(
                        "[auth] User {:?} is attempting to sign in with account: {:?}",
                        sign_in_options.user,
                        sign_in_options.account
                    );

                    // Return a success result
                    bzauth_rs::auth::SignInResult::Success
                })
            })
            .into(),
            ..Default::default()
        });

    // Create the Axum runtime options
    AxumRuntimeOptions { auth_options }
}

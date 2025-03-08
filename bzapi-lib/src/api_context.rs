use diesel::r2d2::ConnectionManager;
use diesel::r2d2::Pool as DieselPool;

use crate::handlers::*;
use crate::providers::oauth2_provider::Oauth2Provider;

#[cfg(feature = "postgres")]
pub type Conn = diesel::PgConnection;

#[cfg(feature = "sqlite")]
pub type Conn = diesel::SqliteConnection;

pub type DbManager = ConnectionManager<Conn>;
pub type DbPool = DieselPool<ConnectionManager<Conn>>;
pub type Oauth2Providers = Vec<Box<dyn Oauth2Provider>>;
pub struct ApiContext {
    pub db_pool: DbPool,
    pub providers: Oauth2Providers,
    pub user_handler: UserHandler,
    pub auth_handler: AuthHandler,
}

pub struct ApiContextParameters {
    pub db_pool: DbPool,
    pub providers: Oauth2Providers,
}

pub struct ApiAuthParameters {}

impl ApiContext {
    pub fn new(ApiContextParameters { db_pool, providers }: ApiContextParameters) -> Self {
        Self {
            db_pool,
            providers,
            user_handler: Default::default(),
            auth_handler: Default::default(),
        }
    }

    pub fn user_handler(&self) -> &UserHandler {
        &self.user_handler
    }
    pub fn user_handler_mut(&mut self) -> &mut UserHandler {
        &mut self.user_handler
    }

    ////////////////
}

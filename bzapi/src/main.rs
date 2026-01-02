use bzapi_lib::{
    api,
    api_context::{ApiContextParameters, DbManager, DbPool},
    logger,
    providers::google::GoogleProvider,
};
use std::env;
use tracing::{Level, debug, info};

#[tokio::main]
async fn main() {
    // Load environment variables from .env file
    debug!("🔵 Loading environment variables from .env file");
    dotenvy::dotenv().ok();

    // Initialise the logger
    logger::init(Level::DEBUG);

    // Setup database connection from environment variables
    debug!("🔵 Setting up database connection");
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db_manager = DbManager::new(db_url);
    let db_pool = DbPool::builder()
        .build(db_manager)
        .expect("Failed to create connection pool");

    debug!("🔵 Creating the api context");
    let api_context_parameters = ApiContextParameters {
        db_pool: db_pool.clone(),
        providers: vec![Box::new(GoogleProvider::new_from_env().unwrap())],
    };

    // Create the api context
    info!("🚀 Serving the api");
    api::serve("127.0.0.1:3001", api_context_parameters)
        .await
        .unwrap();
}

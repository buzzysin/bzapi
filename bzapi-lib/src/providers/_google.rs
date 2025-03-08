use oauth2::{EndpointNotSet, EndpointSet, basic::BasicClient};

use super::errors::ProviderError;

pub type GoogleClient =
    BasicClient<EndpointSet, EndpointNotSet, EndpointNotSet, EndpointNotSet, EndpointNotSet>;
pub struct GoogleProvider {
    client:
        BasicClient<EndpointSet, EndpointNotSet, EndpointNotSet, EndpointNotSet, EndpointNotSet>,
}

pub struct GoogleProviderConfig {
    pub client_id: String,
    pub client_secret: String,
}

struct GoogleProviderBuilder {}

impl GoogleProvider {
    
}

#[derive(Debug)]
pub enum ProviderError {
    Misconfiguration(String),
    InvalidUrl(oauth2::url::ParseError),
}

impl From<std::env::VarError> for ProviderError {
    fn from(e: std::env::VarError) -> Self {
        ProviderError::Misconfiguration(format!("EnvVarError: {}", e))
    }
}

impl From<oauth2::url::ParseError> for ProviderError {
    fn from(e: oauth2::url::ParseError) -> Self {
        ProviderError::InvalidUrl(e)
    }
}

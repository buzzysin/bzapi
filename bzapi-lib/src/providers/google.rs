use std::env;
use std::error::Error;
use std::fmt::Display;

use super::oauth2_provider::Oauth2Provider;
use super::oauth2_provider::util;
use super::oauth2_provider::util::RequestTokenErrorCompat;

use async_trait::async_trait;
use axum::body::Body;
use axum::response::{ErrorResponse, Response};
use oauth2::basic::BasicClient;
use oauth2::{
    AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, PkceCodeChallenge, RedirectUrl,
    Scope, TokenUrl,
};
use oauth2::{EndpointNotSet, EndpointSet};
use reqwest;

pub type GoogleClient =
    BasicClient<EndpointSet, EndpointNotSet, EndpointNotSet, EndpointNotSet, EndpointSet>;

#[derive(Debug, Clone)]
pub struct GoogleProvider {
    client: GoogleClient,
}

pub struct GoogleProviderParams {
    client_id: String,
    client_secret: String,
    redirect_uri: String,
}

impl GoogleProvider {
    pub fn new_from_env() -> Result<GoogleProvider, GoogleProviderError> {
        // Determine arguments from dotenvy
        let client_id = env::var("GOOGLE_CLIENT_ID")?;
        let client_secret = env::var("GOOGLE_CLIENT_SECRET")?;
        let redirect_uri = "http://localhost:3000/auth/google/callback".to_string();

        // Create the provider
        let provider = GoogleProvider::new(GoogleProviderParams {
            client_id,
            client_secret,
            redirect_uri,
        })?;

        Ok(provider)
    }

    pub fn new(
        GoogleProviderParams {
            client_id,
            client_secret,
            redirect_uri,
        }: GoogleProviderParams,
    ) -> Result<Self, GoogleProviderError> {
        let client_id = ClientId::new(client_id);
        let client_secret = ClientSecret::new(client_secret);
        let auth_url = AuthUrl::new("https://accounts.google.com/o/oauth2/auth".to_string())?;
        let token_url = TokenUrl::new("https://accounts.google.com/o/oauth2/token".to_string())?;
        let redirect_uri = RedirectUrl::new(redirect_uri)?;

        let client = BasicClient::new(client_id)
            .set_client_secret(client_secret)
            .set_auth_uri(auth_url)
            .set_token_uri(token_url)
            .set_redirect_uri(redirect_uri);

        Ok(Self { client })
    }

    pub fn client(&self) -> &GoogleClient {
        &self.client
    }

    pub fn client_mut(&mut self) -> &mut GoogleClient {
        &mut self.client
    }
}

#[async_trait]
impl Oauth2Provider for GoogleProvider {
    fn provider(&self) -> String {
        "google".to_string()
    }

    async fn authorize(&self) -> axum::response::Result<Response> {
        let (pkce_challenge, _pkce_verifier) = PkceCodeChallenge::new_random_sha256();
        let (auth_url, _csrf_token) = self
            .client
            .authorize_url(CsrfToken::new_random)
            .add_scope(Scope::new("email".to_string()))
            .set_pkce_challenge(pkce_challenge)
            .url();

        Ok(Response::builder()
            .status(302)
            .header("Location", auth_url.to_string())
            .body("".into())
            .unwrap())
    }

    async fn callback(&self, code: String) -> axum::response::Result<Response> {
        let code = AuthorizationCode::new(code);

        let token_result = self
            .client
            .exchange_code(code)
            .request_async(&util::async_http_client)
            .await
            .map_err(|e| RequestTokenErrorCompat(e))?;

        Ok(Response::builder()
            .status(200)
            .body(format!("{:?}", token_result).into())
            .unwrap())
    }

    async fn token(&self, code: String) -> axum::response::Result<Response> {
        let code = AuthorizationCode::new(code);

        let token_result = self
            .client
            .exchange_code(code)
            .request_async(&util::async_http_client)
            .await
            .map_err(|e| RequestTokenErrorCompat(e))?;

        Ok(Response::builder()
            .status(200)
            .body(format!("{:?}", token_result).into())
            .unwrap())
    }

    async fn validate(&self, token: String) -> axum::response::Result<Response> {
        let client = reqwest::Client::new();
        let response = client
            .get("https://www.googleapis.com/oauth2/v3/userinfo")
            .bearer_auth(token.clone())
            .send()
            .await
            .map_err(|e| ErrorResponse::from(format!("Error validating token: {}", e)))?;

        let status = response.status();
        if response.status().is_success() {
            let body = response
                .text()
                .await
                .map_err(|e| ErrorResponse::from(format!("Error reading response body: {}", e)))?
                .into_bytes();

            Ok(Response::builder()
                .status(status)
                .body(Body::from(body))
                .unwrap())
        } else {
            let error_body = response.text().await.map_err(|e| {
                ErrorResponse::from(format!("Error reading error response body: {}", e))
            })?;

            Err(ErrorResponse::from(format!(
                "Token validation failed: {}, Body: {}",
                status, error_body
            )))
        }
    }
}

#[derive(Debug)]
pub enum GoogleProviderError
where
    Self: std::error::Error + 'static,
{
    EnvVarError(env::VarError),
    ParseError(oauth2::url::ParseError),
}

impl Display for GoogleProviderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GoogleProviderError::EnvVarError(e) => write!(f, "EnvVarError: {}", e),
            GoogleProviderError::ParseError(e) => write!(f, "ParseError: {}", e),
        }
    }
}

impl Error for GoogleProviderError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            GoogleProviderError::EnvVarError(e) => Some(e),
            GoogleProviderError::ParseError(e) => Some(e),
        }
    }
}

impl From<env::VarError> for GoogleProviderError {
    fn from(e: env::VarError) -> Self {
        GoogleProviderError::EnvVarError(e)
    }
}

impl From<oauth2::url::ParseError> for GoogleProviderError {
    fn from(e: oauth2::url::ParseError) -> Self {
        GoogleProviderError::ParseError(e)
    }
}

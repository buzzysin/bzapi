use async_trait::async_trait;
use axum::response::Response;
use dyn_clone::DynClone;

use super::google::GoogleProvider;

/// ## OAuth2 Authorization Code Flow
///
/// I'm trying to abstract this to functions that can be used in any project.
///
/// 1. `authorize` - Redirects the user to the authorization server
/// 2. `callback` - Handles the callback from the authorization server
/// 3. `token` - Exchanges the authorization code for an access token
/// 4. `validate` - Validates the access token
///
/// TODO: determine the core functions needed for simple OAuth2 authorization code flow.
///
/// ### Providers and polymorphism
///
/// Each provider has a different emthod of handling the authorization code flow.
/// Some require scopes, others don't. Some require a client secret, others don't.
///
/// I'm trying to abstract this to a trait that can be implemented by each provider.

#[async_trait]
pub trait Oauth2Provider
where
    Self: DynClone,
    Self: Send + Sync,
{
    /// Identifier for the provider
    fn provider(&self) -> String;

    fn display_name(&self) -> String {
        self.provider()
    }

    /// Redirects the user to the authorization server
    async fn authorize(&self) -> axum::response::Result<Response>;

    /// Handles the callback from the authorization server
    async fn callback(&self, code: String) -> axum::response::Result<Response>;

    /// Exchanges the authorization code for an access token
    async fn token(&self, code: String) -> axum::response::Result<Response>;

    /// Validates the access token
    async fn validate(&self, token: String) -> axum::response::Result<Response>;

    // Refreshes the access token
    // async fn refresh(&self, token: String) -> axum::response::Result<Response>;

    // Casting
    fn as_google(&self) -> Option<&GoogleProvider> {
        None
    }
}

dyn_clone::clone_trait_object!(Oauth2Provider);

pub mod util {
    use axum::response::ErrorResponse;
    use oauth2::HttpRequest;
    use oauth2::HttpResponse;
    use oauth2::RequestTokenError;

    pub async fn async_http_client(request: HttpRequest) -> Result<HttpResponse, reqwest::Error> {
        let request_client = reqwest::Client::new();

        let req_uri = request.uri().to_string();
        let mut req_builder = request_client.request(request.method().clone(), req_uri);

        for (name, value) in request.headers().iter() {
            req_builder = req_builder.header(name.as_str(), value.as_bytes());
        }

        let res_body = request.body().to_vec();
        let response = req_builder.body(res_body).send().await?;

        let res_axum = HttpResponse::new(response.bytes().await?.to_vec());

        Ok(res_axum)
    }

    pub struct RequestTokenErrorCompat<
        T: std::error::Error + 'static,
        E: oauth2::ErrorResponse + 'static,
    >(pub RequestTokenError<T, E>);

    impl<T, E> From<RequestTokenErrorCompat<T, E>> for ErrorResponse
    where
        T: std::error::Error + 'static,
        E: oauth2::ErrorResponse + 'static,
    {
        fn from(RequestTokenErrorCompat(err): RequestTokenErrorCompat<T, E>) -> Self {
            match err {
                RequestTokenError::Request(e) => ErrorResponse::from(e.to_string()),
                RequestTokenError::Parse(e, _) => ErrorResponse::from(e.to_string()),
                RequestTokenError::ServerResponse(e) => ErrorResponse::from(e.to_string()),
                RequestTokenError::Other(e) => ErrorResponse::from(e.to_string()),
            }
        }
    }
}

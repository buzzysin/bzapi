use async_trait::async_trait;
use axum::response::Response;
use dyn_clone::DynClone;
use oauth2::{CsrfToken, EndpointMaybeSet, PkceCodeChallenge, Scope};

use super::provider::LikeOauth2Config;

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

// The endpoint types are not set by default, and will throw a configuration error if not set.
pub type CatchAllOauth2Client = oauth2::basic::BasicClient<
    EndpointMaybeSet,
    EndpointMaybeSet,
    EndpointMaybeSet,
    EndpointMaybeSet,
    EndpointMaybeSet,
>;

#[async_trait]
pub trait Oauth2Provider: LikeOauth2Config
where
    Self: DynClone,
    Self: Send + Sync,
{
    /// Identifier for the provider
    fn id() -> String
    where
        Self: Sized;

    fn display_name() -> String
    where
        Self: Sized,
    {
        // First letter is capitalised
        let mut display_name = Self::id().clone();
        display_name.replace_range(0..1, &display_name[0..1].to_uppercase());
        display_name
    }

    /// Redirects the user to the authorization server
    async fn authorise(&self, _: axum::extract::Request) -> axum::response::Result<Response> {
        // Check if authorisation is supported by this client
        if let Ok(authorisation) = self.client().authorize_url(CsrfToken::new_random) {
            let (pkce_challenge, _pkce_verifier) = PkceCodeChallenge::new_random_sha256();
            let (auth_url, _csrf_token) = authorisation
                .add_scopes(
                    self.get_options()
                        .get_scopes()
                        .iter()
                        .map(|s| Scope::new(s.clone()))
                        .collect::<Vec<Scope>>(),
                )
                .set_pkce_challenge(pkce_challenge)
                .url();

            Ok(Response::builder()
                .status(302)
                .header("Location", auth_url.to_string())
                .body("".into())
                .unwrap())
        }
        // If authorisation is not supported, return a 500 error
        else {
            Ok(Response::builder()
                .status(500)
                .body("Authorisation is not supported by this provider".into())
                .unwrap())
        }
    }

    /// Handles the callback from the authorization server
    async fn callback(&self, _: axum::extract::Request) -> axum::response::Result<Response> {
        todo!()
    }

    fn client(&self) -> &CatchAllOauth2Client;

    fn client_mut(&mut self) -> &mut CatchAllOauth2Client;
    // TODO: the rest
}

dyn_clone::clone_trait_object!(Oauth2Provider);

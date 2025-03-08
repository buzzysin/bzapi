use std::collections::HashMap;

use oauth2::{AuthUrl, ClientSecret, RedirectUrl, TokenUrl};

use super::{
    errors::ProviderError,
    oauth2_provider::{CatchAllOauth2Client, Oauth2Provider},
    provider::{
        AuthUser, AuthorisationByConfig, AuthorisationHandler, LikeOauth2Config,
        LikeOauth2MetaConfig, LikeSimpleProviderOptions, LikeUserData, TokenHandler,
        UserInfoHandler,
    },
};

#[derive(Clone)]
pub struct DiscordProvider {
    options: DiscordProviderOptions,
    client: CatchAllOauth2Client,
}

#[derive(Clone)]
pub struct DiscordProviderOptions {
    pub client_id: String,
    pub client_secret: String,
    pub redirect_uri: String,
    pub additional_scopes: Option<Vec<String>>,
}

#[derive(Clone)]
pub struct DiscordUserData {
    pub login: String,
    pub id: u64,
    pub node_id: String,
    pub avatar_url: String,
}

impl LikeUserData for DiscordUserData {
    fn get_id(&self) -> Option<String> {
        None
    }

    fn get_email(&self) -> Option<String> {
        todo!()
    }

    fn get_name(&self) -> Option<String> {
        todo!()
    }

    fn get_picture(&self) -> Option<String> {
        todo!()
    }
}

impl LikeSimpleProviderOptions for DiscordProvider {
    fn get_id(&self) -> Option<String> {
        Some(Self::id())
    }

    fn get_display_name(&self) -> Option<String> {
        Some(Self::display_name())
    }
}

#[async_trait::async_trait]
impl Oauth2Provider for DiscordProvider {
    fn id() -> String
    where
        Self: Sized,
    {
        "discord".to_string()
    }

    fn client(&self) -> &CatchAllOauth2Client {
        &self.client
    }

    fn client_mut(&mut self) -> &mut CatchAllOauth2Client {
        &mut self.client
    }
}

#[async_trait::async_trait]
impl LikeOauth2Config for DiscordProvider {
    fn get_auth_endpoint() -> Option<AuthorisationHandler> {
        let url = Some("https://discord.com/api/oauth2/authorize".to_string());

        let params = HashMap::new();

        let handler = AuthorisationByConfig {
            url,
            params,
            hook: None,
        };

        Some(handler.into())
    }

    fn get_token_endpoint() -> Option<TokenHandler> {
        Some("https://discord.com/api/oauth2/token".to_string().into())
    }

    fn get_user_info_endpoint() -> Option<UserInfoHandler> {
        Some("https://discord.com/api/users/@me".to_string().into())
    }

    async fn fetch_user_data(&self) -> Option<Box<dyn AuthUser>> {
        // it gets complicated here - we need to process the request by asking
        // the Discord api for the user info (internal requests)
        todo!()
    }

    fn get_options(&self) -> Box<dyn LikeOauth2MetaConfig> {
        Box::new(self.options.clone())
    }
}

impl LikeOauth2MetaConfig for DiscordProviderOptions {}

impl DiscordProvider {
    pub fn new(options: DiscordProviderOptions) -> Result<Self, ProviderError> {
        let client = Self::get_client(&options)?;
        Ok(Self { client, options })
    }

    pub fn new_from_env() -> Result<Self, ProviderError> {
        let provider_id = Self::id();
        let provider_caps = provider_id.to_uppercase();

        let client_id = std::env::var(format!("{}_CLIENT_ID", provider_caps))?;
        let client_secret = std::env::var(format!("{}_CLIENT_SECRET", provider_caps))?;

        let base_url = std::env::var("BASE_URL")?;
        let redirect_uri = format!("{}/auth/callback/{}", base_url, provider_id);

        let client = Self::get_client(&DiscordProviderOptions {
            client_id: client_id.clone(),
            client_secret: client_secret.clone(),
            redirect_uri: redirect_uri.clone(),
            additional_scopes: None,
        })?;

        let options = DiscordProviderOptions {
            client_id,
            client_secret,
            redirect_uri,
            additional_scopes: None,
        };

        Ok(Self { client, options })
    }

    fn get_client(options: &DiscordProviderOptions) -> Result<CatchAllOauth2Client, ProviderError> {
        let client_id = options.client_id.clone();
        let client_secret = options.client_secret.clone();
        let redirect_uri = options.redirect_uri.clone();

        let auth_url = Self::compute_auth_endpoint()?
            .map(|url| AuthUrl::new(url.to_string()))
            .transpose()?;
        let token_url = Self::compute_token_endpoint()?
            .map(|url| TokenUrl::new(url.to_string()))
            .transpose()?;
        let redirect_uri = RedirectUrl::new(redirect_uri)?;

        let client = oauth2::basic::BasicClient::new(oauth2::ClientId::new(client_id))
            .set_client_secret(ClientSecret::new(client_secret))
            .set_auth_uri_option(auth_url)
            .set_token_uri_option(token_url)
            .set_redirect_uri(redirect_uri)
            .set_device_authorization_url_option(None) // for now
            .set_revocation_url_option(None) // for now
            .set_introspection_url_option(None); // for now

        Ok(client)
    }
}

impl LikeOauth2MetaConfig for DiscordProvider {
    fn get_scopes(&self) -> Vec<String> {
        vec!["identify".to_string(), "email".to_string()]
    }
}

use std::{collections::HashMap, ops::Deref, time::Duration};

use dyn_clone::DynClone;
use oauth2::{AccessToken, RefreshToken, Scope, TokenResponse, TokenType};
use reqwest::Url;

use super::errors::ProviderError;

pub enum ProviderType {
    Oidc,
    Oauth,
    Email,
    Credentials,
    WebAuthn,
}

/// AKA CommonProviderOptions
pub trait LikeSimpleProviderOptions
where
    Self: DynClone,
{
    fn get_id(&self) -> Option<String> {
        None
    }
    fn get_display_name(&self) -> Option<String> {
        None
    }
    fn get_type(&self) -> Option<ProviderType> {
        None
    }
}
dyn_clone::clone_trait_object!(LikeSimpleProviderOptions);

/// AKA Profile
pub trait LikeUserData
where
    Self: DynClone,
    Self: Send + Sync,
{
    fn get_id(&self) -> Option<String> {
        None
    }
    fn get_email(&self) -> Option<String> {
        None
    }
    fn get_name(&self) -> Option<String> {
        None
    }
    fn get_picture(&self) -> Option<String> {
        None
    }
}
dyn_clone::clone_trait_object!(LikeUserData);

impl LikeUserData for Box<dyn LikeUserData> {
    fn get_id(&self) -> Option<String> {
        self.deref().get_id()
    }

    fn get_email(&self) -> Option<String> {
        self.deref().get_email()
    }

    fn get_name(&self) -> Option<String> {
        self.deref().get_name()
    }

    fn get_picture(&self) -> Option<String> {
        self.deref().get_picture()
    }
}

pub trait AuthUser: DynClone {}
dyn_clone::clone_trait_object!(AuthUser);

pub trait LikeOidcConfig<UD: LikeUserData> {}

#[async_trait::async_trait]
pub trait LikeOauth2Config: LikeSimpleProviderOptions
where
    Self: DynClone,
{
    // "overrides"
    fn get_type(&self) -> Option<ProviderType>
    where
        Self: Sized,
    {
        Some(ProviderType::Oauth)
    }

    // "extensions"
    fn get_well_known(&self) -> Option<String>
    where
        Self: Sized,
    {
        None
    }

    fn get_issuer(&self) -> Option<String>
    where
        Self: Sized,
    {
        None
    }

    fn get_auth_endpoint() -> Option<AuthorisationHandler>
    where
        Self: Sized,
    {
        None
    }

    fn get_token_endpoint() -> Option<TokenHandler>
    where
        Self: Sized,
    {
        None
    }

    fn get_user_info_endpoint() -> Option<UserInfoHandler>
    where
        Self: Sized,
    {
        None
    }

    fn get_options(&self) -> Box<dyn LikeOauth2MetaConfig>;

    fn compute_auth_endpoint() -> Result<Option<Url>, ProviderError>
    where
        Self: Sized,
    {
        match Self::get_auth_endpoint() {
            Some(AuthorisationHandler::Url(url)) => {
                let url = Url::parse(&url)?;
                Ok(Some(url))
            }
            Some(AuthorisationHandler::Config(config)) => {
                let url = Url::parse_with_params(&config.url.unwrap_or_default(), &config.params)?;
                // TODO: If a handler is provided with the config, we need to go to that URL and
                // extract extra parameters from the response
                Ok(Some(url))
            }
            None => Ok(None),
        }
    }

    fn compute_token_endpoint() -> Result<Option<Url>, ProviderError>
    where
        Self: Sized,
    {
        match Self::get_token_endpoint() {
            Some(TokenHandler::Url(url)) => {
                let url = Url::parse(&url)?;
                Ok(Some(url))
            }
            Some(TokenHandler::Config(config)) => {
                let url = Url::parse_with_params(&config.url.unwrap_or_default(), &config.params)?;
                // TODO: If a handler is provided with the config, we need to go to that URL and
                // extract extra parameters from the response
                Ok(Some(url))
            }
            None => Ok(None),
        }
    }

    /// i.e. profile
    async fn fetch_user_data(&self) -> Option<Box<dyn AuthUser>> {
        None
    }
}
dyn_clone::clone_trait_object!(LikeOauth2Config);

pub trait LikeOauth2MetaConfig
where
    Self: DynClone,
{
    fn get_well_known(&self) -> Option<String> {
        None
    }

    fn get_issuer(&self) -> Option<String> {
        None
    }

    fn get_auth_url(&self) -> Option<AuthorisationHandler> {
        None
    }

    fn get_scopes(&self) -> Vec<String> {
        vec![]
    }

    fn get_token_url(&self) -> Option<TokenHandler> {
        None
    }

    fn get_user_info_url(&self) -> Option<UserInfoHandler> {
        None
    }
}

dyn_clone::clone_trait_object!(LikeOauth2MetaConfig);

pub enum Provider {
    Oidc(Box<dyn LikeSimpleProviderOptions>),
    Oauth2(Box<dyn LikeSimpleProviderOptions>),
    Email(Box<dyn LikeSimpleProviderOptions>),
    Credentials(Box<dyn LikeSimpleProviderOptions>),
    WebAuthn(Box<dyn LikeSimpleProviderOptions>),
}

////////////////////////////////////////////////////////////////
/// Helper structs for handling authorisation and token requests
////////////////////////////////////////////////////////////////

#[async_trait::async_trait]
pub trait EndpointHook<P: LikeUserData>: DynClone {
    type Input;
    type Output;
    async fn call(&self, input: Self::Input) -> Self::Output;
}

pub struct AuthorisationHookInput {
    pub user_data: Box<dyn LikeUserData>,
}
pub struct AuthorisationHookOutput {
    pub user_data: Url,
}
pub trait AuthorisationEndpointHook<P: LikeUserData>:
    EndpointHook<P, Input = AuthorisationHookInput, Output = Url>
{
}
dyn_clone::clone_trait_object!(AuthorisationEndpointHook<dyn LikeUserData>);

#[derive(Clone)]
pub struct AuthorisationByConfig {
    pub url: Option<String>,
    pub params: HashMap<String, String>,
    pub hook: Option<Box<dyn AuthorisationEndpointHook<dyn LikeUserData>>>,
}

#[derive(Clone)]
pub enum AuthorisationHandler {
    Url(String),
    Config(AuthorisationByConfig),
}

impl From<String> for AuthorisationHandler {
    fn from(url: String) -> Self {
        AuthorisationHandler::Url(url.clone())
    }
}

impl From<AuthorisationByConfig> for AuthorisationHandler {
    fn from(config: AuthorisationByConfig) -> Self {
        AuthorisationHandler::Config(config.clone())
    }
}

#[derive(Clone)]
pub struct TokenByConfig {
    pub url: Option<String>,
    pub params: HashMap<String, String>,
}

#[derive(Clone)]
pub enum TokenHandler {
    Url(String),
    Config(TokenByConfig),
}

impl From<String> for TokenHandler {
    fn from(url: String) -> Self {
        TokenHandler::Url(url.clone())
    }
}

impl From<TokenByConfig> for TokenHandler {
    fn from(config: TokenByConfig) -> Self {
        TokenHandler::Config(config.clone())
    }
}

pub trait LikePartialTokenResponse {
    type TokenType: TokenType;
    /// REQUIRED. The access token issued by the authorization server.
    fn access_token(&self) -> Option<&AccessToken> {
        None
    }
    fn token_type(&self) -> Option<&Self::TokenType> {
        None
    }
    fn expires_in(&self) -> Option<Duration> {
        None
    }
    fn refresh_token(&self) -> Option<&RefreshToken> {
        None
    }
    fn scopes(&self) -> Vec<Scope> {
        vec![]
    }
}

pub trait LikeTokenResponse: TokenResponse {
    fn expires_at(&self) -> Option<i64> {
        None
    }
}

// AKA ProfileHook
// pub trait UserDataHook<P: LikeUserData>: DynClone {
// async fn call(&self, user_info: P, token_set: HashMap<String, String>) -> Box<dyn AuthUser>;
// }
pub struct UserInfoHookInput {
    pub user_info: Box<dyn LikeUserData>,
    pub token_set: HashMap<String, String>,
}
pub struct UserInfoHookOutput {
    pub user: Box<dyn AuthUser>,
}
pub trait UserInfoHook<P: LikeUserData>:
    EndpointHook<P, Input = UserInfoHookInput, Output = Box<dyn AuthUser>>
{
}
dyn_clone::clone_trait_object!(UserInfoHook<dyn LikeUserData>);

#[derive(Clone)]
pub struct UserInfoByConfig {
    pub url: Option<String>,
    pub params: HashMap<String, String>,
    pub hook: Option<Box<dyn UserInfoHook<dyn LikeUserData>>>,
}

#[derive(Clone)]
pub enum UserInfoHandler {
    Url(String),
    Config(UserInfoByConfig),
}

impl From<String> for UserInfoHandler {
    fn from(url: String) -> Self {
        UserInfoHandler::Url(url.clone())
    }
}

impl From<UserInfoByConfig> for UserInfoHandler {
    fn from(config: UserInfoByConfig) -> Self {
        UserInfoHandler::Config(config.clone())
    }
}

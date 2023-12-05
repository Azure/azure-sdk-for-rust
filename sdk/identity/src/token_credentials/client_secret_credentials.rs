use crate::oauth2_http_client::Oauth2HttpClient;
use crate::token_credentials::cache::TokenCache;
use azure_core::{
    auth::{AccessToken, Secret, TokenCredential},
    authority_hosts::AZURE_PUBLIC_CLOUD,
    error::{ErrorKind, ResultExt},
    HttpClient, Url,
};
use oauth2::{basic::BasicClient, AuthType, AuthUrl, Scope, TokenUrl};
use std::{str, sync::Arc};
use time::OffsetDateTime;

/// Provides options to configure how the Identity library makes authentication
/// requests to Azure Active Directory.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TokenCredentialOptions {
    authority_host: Url,
}

impl Default for TokenCredentialOptions {
    fn default() -> Self {
        Self {
            authority_host: AZURE_PUBLIC_CLOUD.to_owned(),
        }
    }
}

impl TokenCredentialOptions {
    /// Create a new `TokenCredentialsOptions`. `default()` may also be used.
    pub fn new(authority_host: Url) -> Self {
        Self { authority_host }
    }
    /// Set the authority host for authentication requests.
    pub fn set_authority_host(&mut self, authority_host: Url) {
        self.authority_host = authority_host;
    }

    /// The authority host to use for authentication requests.  The default is
    /// `https://login.microsoftonline.com`.
    pub fn authority_host(&self) -> &Url {
        &self.authority_host
    }
}

/// A list of tenant IDs
pub mod tenant_ids {
    /// The tenant ID for multi-tenant apps
    ///
    /// <https://docs.microsoft.com/azure/active-directory/develop/howto-convert-app-to-be-multi-tenant>
    pub const TENANT_ID_COMMON: &str = "common";
    /// The tenant ID for Active Directory Federated Services
    pub const TENANT_ID_ADFS: &str = "adfs";
}

/// Enables authentication to Azure Active Directory using a client secret that was generated for an App Registration.
///
/// More information on how to configure a client secret can be found here:
/// <https://docs.microsoft.com/azure/active-directory/develop/quickstart-configure-app-access-web-apis#add-credentials-to-your-web-application>
#[derive(Debug)]
pub struct ClientSecretCredential {
    http_client: Arc<dyn HttpClient>,
    tenant_id: String,
    client_id: oauth2::ClientId,
    client_secret: Option<oauth2::ClientSecret>,
    options: TokenCredentialOptions,
    cache: TokenCache,
}

impl ClientSecretCredential {
    /// Create a new `ClientSecretCredential`
    pub fn new(
        http_client: Arc<dyn HttpClient>,
        tenant_id: String,
        client_id: String,
        client_secret: String,
        options: TokenCredentialOptions,
    ) -> ClientSecretCredential {
        ClientSecretCredential {
            http_client,
            tenant_id,
            client_id: oauth2::ClientId::new(client_id),
            client_secret: Some(oauth2::ClientSecret::new(client_secret)),
            options,
            cache: TokenCache::new(),
        }
    }

    fn options(&self) -> &TokenCredentialOptions {
        &self.options
    }

    async fn get_token(&self, scopes: &[&str]) -> azure_core::Result<AccessToken> {
        let options = self.options();
        let authority_host = options.authority_host();

        let token_url = TokenUrl::from_url(
            Url::parse(&format!(
                "{}/{}/oauth2/v2.0/token",
                authority_host, self.tenant_id
            ))
            .with_context(ErrorKind::Credential, || {
                format!(
                    "failed to construct token endpoint with tenant id {}",
                    self.tenant_id
                )
            })?,
        );

        let auth_url = AuthUrl::from_url(
            Url::parse(&format!(
                "{}/{}/oauth2/v2.0/authorize",
                authority_host, self.tenant_id
            ))
            .with_context(ErrorKind::Credential, || {
                format!(
                    "failed to construct authorize endpoint with tenant id {}",
                    self.tenant_id
                )
            })?,
        );

        let client = BasicClient::new(
            self.client_id.clone(),
            self.client_secret.clone(),
            auth_url,
            Some(token_url),
        )
        .set_auth_type(AuthType::RequestBody);

        let scopes = scopes.iter().map(|x| Scope::new(x.to_string()));
        let oauth_http_client = Oauth2HttpClient::new(self.http_client.clone());
        let token_result = client
            .exchange_client_credentials()
            .add_scopes(scopes)
            .request_async(|request| oauth_http_client.request(request))
            .await
            .map(|r| {
                use oauth2::TokenResponse as _;
                AccessToken::new(
                    Secret::new(r.access_token().secret().to_owned()),
                    OffsetDateTime::now_utc() + r.expires_in().unwrap_or_default(),
                )
            })
            .context(ErrorKind::Credential, "request token error")?;

        Ok(token_result)
    }
}

#[cfg_attr(target_arch = "wasm32", async_trait::async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait::async_trait)]
impl TokenCredential for ClientSecretCredential {
    async fn get_token(&self, scopes: &[&str]) -> azure_core::Result<AccessToken> {
        self.cache.get_token(scopes, self.get_token(scopes)).await
    }
    /// Clear the credential's cache.
    async fn clear_cache(&self) -> azure_core::Result<()> {
        self.cache.clear().await
    }
}

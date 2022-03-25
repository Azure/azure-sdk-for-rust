use super::TokenCredential;
use azure_core::auth::TokenResponse;
use chrono::Utc;
use oauth2::{
    basic::{BasicClient, BasicErrorResponseType},
    reqwest::async_http_client,
    AccessToken, AuthType, AuthUrl, Scope, StandardErrorResponse, TokenUrl,
};
use std::{str, time::Duration};
use url::Url;

/// Provides options to configure how the Identity library makes authentication
/// requests to Azure Active Directory.
#[derive(Clone, Debug, PartialEq)]
pub struct TokenCredentialOptions {
    authority_host: String,
}

impl Default for TokenCredentialOptions {
    fn default() -> Self {
        Self {
            authority_host: authority_hosts::AZURE_PUBLIC_CLOUD.to_owned(),
        }
    }
}

impl TokenCredentialOptions {
    /// Create a new TokenCredentialsOptions. `default()` may also be used.
    pub fn new(authority_host: String) -> Self {
        Self { authority_host }
    }
    /// Set the authority host for authentication requests.
    pub fn set_authority_host(&mut self, authority_host: String) {
        self.authority_host = authority_host
    }

    /// The authority host to use for authentication requests.  The default is
    /// `https://login.microsoftonline.com`.
    pub fn authority_host(&self) -> &str {
        &self.authority_host
    }
}

/// A list of known Azure authority hosts
pub mod authority_hosts {
    /// China-based Azure Authority Host
    pub const AZURE_CHINA: &str = "https://login.chinacloudapi.cn";
    /// Germany-based Azure Authority Host
    pub const AZURE_GERMANY: &str = "https://login.microsoftonline.de";
    /// US Government Azure Authority Host
    pub const AZURE_GOVERNMENT: &str = "https://login.microsoftonline.us";
    /// Public Cloud Azure Authority Host
    pub const AZURE_PUBLIC_CLOUD: &str = "https://login.microsoftonline.com";
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
pub struct ClientSecretCredential {
    tenant_id: String,
    client_id: oauth2::ClientId,
    client_secret: Option<oauth2::ClientSecret>,
    options: TokenCredentialOptions,
}

impl ClientSecretCredential {
    /// Create a new ClientSecretCredential
    pub fn new(
        tenant_id: String,
        client_id: String,
        client_secret: String,
        options: TokenCredentialOptions,
    ) -> ClientSecretCredential {
        ClientSecretCredential {
            tenant_id,
            client_id: oauth2::ClientId::new(client_id),
            client_secret: Some(oauth2::ClientSecret::new(client_secret)),
            options,
        }
    }

    fn options(&self) -> &TokenCredentialOptions {
        &self.options
    }
}

#[allow(missing_docs)]
#[non_exhaustive]
#[derive(Debug, thiserror::Error)]
pub enum ClientSecretCredentialError {
    #[error("Failed to construct token endpoint with tenant id {1}: {0}")]
    FailedConstructTokenEndpoint(url::ParseError, String),
    #[error("Failed to construct authorize endpoint with tenant id {1}: {0}")]
    FailedConstructAuthorizeEndpoint(url::ParseError, String),
    #[error("Request token error: {0}")]
    RequestTokenError(
        oauth2::RequestTokenError<
            oauth2::reqwest::Error<reqwest::Error>,
            StandardErrorResponse<BasicErrorResponseType>,
        >,
    ),
}

#[async_trait::async_trait]
impl TokenCredential for ClientSecretCredential {
    type Error = ClientSecretCredentialError;

    async fn get_token(&self, resource: &str) -> Result<TokenResponse, Self::Error> {
        let options = self.options();
        let authority_host = options.authority_host();

        let token_url = TokenUrl::from_url(
            Url::parse(&format!(
                "{}/{}/oauth2/v2.0/token",
                authority_host, self.tenant_id
            ))
            .map_err(|error| {
                ClientSecretCredentialError::FailedConstructTokenEndpoint(
                    error,
                    self.tenant_id.clone(),
                )
            })?,
        );

        let auth_url = AuthUrl::from_url(
            Url::parse(&format!(
                "{}/{}/oauth2/v2.0/authorize",
                authority_host, self.tenant_id
            ))
            .map_err(|error| {
                ClientSecretCredentialError::FailedConstructAuthorizeEndpoint(
                    error,
                    self.tenant_id.clone(),
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

        let token_result = client
            .exchange_client_credentials()
            .add_scope(Scope::new(format!("{}/.default", resource)))
            .request_async(async_http_client)
            .await
            .map(|r| {
                use oauth2::TokenResponse as _;
                TokenResponse::new(
                    AccessToken::new(r.access_token().secret().to_owned()),
                    Utc::now()
                        + chrono::Duration::from_std(
                            r.expires_in().unwrap_or_else(|| Duration::from_secs(0)),
                        )
                        .unwrap(),
                )
            })
            .map_err(ClientSecretCredentialError::RequestTokenError)?;

        Ok(token_result)
    }
}

#[async_trait::async_trait]
impl azure_core::auth::TokenCredential for ClientSecretCredential {
    async fn get_token(
        &self,
        resource: &str,
    ) -> Result<azure_core::auth::TokenResponse, azure_core::Error> {
        TokenCredential::get_token(self, resource)
            .await
            .map_err(|error| azure_core::Error::GetToken(Box::new(error)))
    }
}

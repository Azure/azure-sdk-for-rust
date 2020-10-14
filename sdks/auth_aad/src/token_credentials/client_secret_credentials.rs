use crate::token_credentials::TokenCredential;

use azure_sdk_core::errors::AzureError;
use chrono::Utc;
use oauth2::{
    basic::BasicClient, reqwest::async_http_client, AccessToken, AuthType, AuthUrl, Scope,
    TokenResponse, TokenUrl,
};
use std::{str, time::Duration};
use url::Url;

/// Enables authentication to Azure Active Directory using a client secret that was generated for an App Registration. More information on how
/// to configure a client secret can be found here:
/// https://docs.microsoft.com/en-us/azure/active-directory/develop/quickstart-configure-app-access-web-apis#add-credentials-to-your-web-application
pub struct ClientSecretCredential {
    tenant_id: String,
    client_id: oauth2::ClientId,
    client_secret: Option<oauth2::ClientSecret>,
}

impl ClientSecretCredential {
    pub fn new(
        tenant_id: String,
        client_id: String,
        client_secret: String,
    ) -> ClientSecretCredential {
        ClientSecretCredential {
            tenant_id,
            client_id: oauth2::ClientId::new(client_id),
            client_secret: Some(oauth2::ClientSecret::new(client_secret)),
        }
    }
}

#[async_trait::async_trait]
impl TokenCredential for ClientSecretCredential {
    async fn get_token(&self, resource: &str) -> Result<crate::TokenResponse, AzureError> {
        let token_url = TokenUrl::from_url(
            Url::parse(&format!(
                "https://login.microsoftonline.com/{}/oauth2/v2.0/token",
                self.tenant_id
            ))
            .map_err(|_| {
                AzureError::GenericErrorWithText(format!(
                    "Failed to construct token endpoint with tenant id {}",
                    self.tenant_id,
                ))
            })?,
        );

        let auth_url = AuthUrl::from_url(
            Url::parse(&format!(
                "https://login.microsoftonline.com/{}/oauth2/v2.0/authorize",
                self.tenant_id
            ))
            .map_err(|_| {
                AzureError::GenericErrorWithText(format!(
                    "Failed to construct authorize endpoint with tenant id {}",
                    self.tenant_id,
                ))
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
            .add_scope(Scope::new(format!("{}.default", resource)))
            .request_async(async_http_client)
            .await
            .map(|r| {
                crate::TokenResponse::new(
                    AccessToken::new(r.access_token().secret().to_owned()),
                    Utc::now()
                        + chrono::Duration::from_std(
                            r.expires_in().unwrap_or(Duration::from_secs(0)),
                        )
                        .unwrap(),
                )
            })
            .map_err(|e| match e {
                oauth2::RequestTokenError::ServerResponse(s) => AzureError::GenericErrorWithText(
                    s.error_description()
                        .unwrap_or(&"Server error without description".to_string())
                        .to_owned(),
                ),
                _ => AzureError::GenericErrorWithText("OAuth2 error".to_string()),
            })?;

        Ok(token_result)
    }
}

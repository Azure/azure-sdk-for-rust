use crate::{token_credentials::TokenCredential, TokenResponse};

use azure_sdk_core::errors::AzureError;
use chrono::{DateTime, Utc};
use oauth2::AccessToken;
use std::str;
use url::Url;

#[derive(Debug, Clone, Deserialize)]
struct MsiTokenResponse {
    pub access_token: AccessToken,
    pub expires_on: DateTime<Utc>,
    pub token_type: String,
    pub resource: String,
}

const MSI_ENDPOINT_ENV_KEY: &str = "IDENTITY_ENDPOINT";
const MSI_SECRET_ENV_KEY: &str = "IDENTITY_HEADER";
const MSI_API_VERSION: &str = "2019-08-01";

/// Attempts authentication using a managed identity that has been assigned to the deployment environment. This authentication type works in Azure VMs,
/// App Service and Azure Functions applications, as well as the Azure Cloud Shell
///
/// Built up from docs at [https://docs.microsoft.com/en-us/azure/app-service/overview-managed-identity#using-the-rest-protocol](https://docs.microsoft.com/en-us/azure/app-service/overview-managed-identity#using-the-rest-protocol)
pub struct ManagedIdentityCredential;

#[async_trait::async_trait]
impl TokenCredential for ManagedIdentityCredential {
    async fn get_token(&self, resource: &str) -> Result<TokenResponse, AzureError> {
        let msi_endpoint = std::env::var(MSI_ENDPOINT_ENV_KEY)
            .unwrap_or("http://169.254.169.254/metadata/identity/oauth2/token".to_owned());

        let query_items = vec![("api-version", MSI_API_VERSION), ("resource", resource)];

        let msi_endpoint_url = Url::parse_with_params(&msi_endpoint, &query_items)
            .map_err(|error| AzureError::GenericErrorWithText(error.to_string()))?;

        let msi_secret = std::env::var(MSI_SECRET_ENV_KEY).map_err(|_| {
            AzureError::GenericErrorWithText(format!(
                "Missing environment variable {}",
                MSI_SECRET_ENV_KEY
            ))
        })?;

        let client = reqwest::Client::new();
        let res_body = client
            .get(msi_endpoint_url)
            .header("Metadata", "true")
            .header("X-IDENTITY-HEADER", msi_secret)
            .send()
            .await
            .map_err(|e| AzureError::GenericErrorWithText(e.to_string()))?
            .text()
            .await
            .map_err(|e| AzureError::GenericErrorWithText(e.to_string()))?;

        let token_response = serde_json::from_str::<MsiTokenResponse>(&res_body)
            .map_err(|_| AzureError::GenericError)?;

        Ok(TokenResponse::new(
            token_response.access_token,
            token_response.expires_on,
        ))
    }
}

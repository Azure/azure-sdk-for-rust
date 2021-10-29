use super::TokenCredential;
use azure_core::TokenResponse;
use chrono::{DateTime, TimeZone, Utc};
use oauth2::AccessToken;
use serde::{
    de::{self, Deserializer},
    Deserialize,
};
use std::str;
use url::Url;

const MSI_ENDPOINT_ENV_KEY: &str = "IDENTITY_ENDPOINT";
const MSI_SECRET_ENV_KEY: &str = "IDENTITY_HEADER";
const MSI_API_VERSION: &str = "2019-08-01";

/// Attempts authentication using a managed identity that has been assigned to the deployment environment.
///
/// This authentication type works in Azure VMs, App Service and Azure Functions applications, as well as the Azure Cloud Shell
///
/// Built up from docs at [https://docs.microsoft.com/azure/app-service/overview-managed-identity#using-the-rest-protocol](https://docs.microsoft.com/azure/app-service/overview-managed-identity#using-the-rest-protocol)
pub struct ManagedIdentityCredential;

#[non_exhaustive]
#[derive(Debug, thiserror::Error)]
pub enum ManagedIdentityCredentialError {
    #[error("Error parsing url for MSI endpoint: {0}")]
    MsiEndpointParseUrlError(url::ParseError),
    #[error(
        "Missing MSI secret set in {} environment variable",
        MSI_SECRET_ENV_KEY
    )]
    MissingMsiSecret(std::env::VarError),
    #[error("Refresh token send error: {0}")]
    SendError(reqwest::Error),
    #[error("Error getting text for refresh token: {0}")]
    TextError(reqwest::Error),
    #[error("Error deserializing refresh token: {0}")]
    DeserializeError(serde_json::Error),
}

#[async_trait::async_trait]
impl TokenCredential for ManagedIdentityCredential {
    type Error = ManagedIdentityCredentialError;

    async fn get_token(&self, resource: &str) -> Result<TokenResponse, Self::Error> {
        let msi_endpoint = std::env::var(MSI_ENDPOINT_ENV_KEY)
            .unwrap_or_else(|_| "http://169.254.169.254/metadata/identity/oauth2/token".to_owned());

        let query_items = vec![("api-version", MSI_API_VERSION), ("resource", resource)];

        let msi_endpoint_url = Url::parse_with_params(&msi_endpoint, &query_items)
            .map_err(ManagedIdentityCredentialError::MsiEndpointParseUrlError)?;

        let msi_secret = std::env::var(MSI_SECRET_ENV_KEY)
            .map_err(ManagedIdentityCredentialError::MissingMsiSecret)?;

        let client = reqwest::Client::new();
        let res_body = client
            .get(msi_endpoint_url)
            .header("Metadata", "true")
            .header("X-IDENTITY-HEADER", msi_secret)
            .send()
            .await
            .map_err(ManagedIdentityCredentialError::SendError)?
            .text()
            .await
            .map_err(ManagedIdentityCredentialError::TextError)?;

        let token_response = serde_json::from_str::<MsiTokenResponse>(&res_body)
            .map_err(ManagedIdentityCredentialError::DeserializeError)?;

        Ok(TokenResponse::new(
            token_response.access_token,
            token_response.expires_on,
        ))
    }
}

#[async_trait::async_trait]
impl azure_core::TokenCredential for ManagedIdentityCredential {
    async fn get_token(
        &self,
        resource: &str,
    ) -> Result<azure_core::TokenResponse, azure_core::Error> {
        TokenCredential::get_token(self, resource)
            .await
            .map_err(|error| azure_core::Error::GetTokenError(Box::new(error)))
    }
}

pub fn expires_on_string<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
where
    D: Deserializer<'de>,
{
    let v = String::deserialize(deserializer)?;
    let as_i64 = v.parse::<i64>().map_err(de::Error::custom)?;
    Ok(Utc.timestamp(as_i64, 0))
}

// NOTE: expires_on is a String version of unix epoch time, not an integer.
// https://docs.microsoft.com/en-us/azure/app-service/overview-managed-identity?tabs=dotnet#rest-protocol-examples
#[derive(Debug, Clone, Deserialize)]
struct MsiTokenResponse {
    pub access_token: AccessToken,
    #[serde(deserialize_with = "expires_on_string")]
    pub expires_on: DateTime<Utc>,
    pub token_type: String,
    pub resource: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Deserialize)]
    struct TestExpires {
        #[serde(deserialize_with = "expires_on_string")]
        date: DateTime<Utc>,
    }

    #[test]
    fn check_expires_on_string() {
        let as_string = r#"{"date": "1586984735"}"#;
        let expected = Utc.ymd(2020, 4, 15).and_hms(21, 5, 35);
        let parsed: TestExpires =
            serde_json::from_str(as_string).expect("deserialize should succeed");
        assert_eq!(expected, parsed.date);
    }
}

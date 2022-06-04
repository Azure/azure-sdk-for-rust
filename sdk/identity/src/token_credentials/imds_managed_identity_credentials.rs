use azure_core::auth::{TokenCredential, TokenResponse};
use azure_core::error::{Error, ErrorKind, Result, ResultExt};
use chrono::{DateTime, TimeZone, Utc};
use oauth2::AccessToken;
use reqwest::header::HeaderMap;
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
#[derive(Default)]
pub struct ImdsManagedIdentityCredential {
    object_id: Option<String>,
    client_id: Option<String>,
    msi_res_id: Option<String>,
}

impl ImdsManagedIdentityCredential {
    /// Specifies the object id associated with a user assigned managed service identity resource that should be used to retrieve the access token.
    ///
    /// The values of client_id and msi_res_id are discarded, as only one id parameter may be set when getting a token.
    pub fn with_object_id<A>(mut self, object_id: A) -> Self
    where
        A: Into<String>,
    {
        self.object_id = Some(object_id.into());
        self.client_id = None;
        self.msi_res_id = None;
        self
    }

    /// Specifies the application id (client id) associated with a user assigned managed service identity resource that should be used to retrieve the access token.
    ///
    /// The values of object_id and msi_res_id are discarded, as only one id parameter may be set when getting a token.
    pub fn with_client_id<A>(mut self, client_id: A) -> Self
    where
        A: Into<String>,
    {
        self.client_id = Some(client_id.into());
        self.object_id = None;
        self.msi_res_id = None;
        self
    }

    /// Specifies the ARM resource id of the user assigned managed service identity resource that should be used to retrieve the access token.
    ///
    /// The values of object_id and client_id are discarded, as only one id parameter may be set when getting a token.
    pub fn with_identity<A>(mut self, msi_res_id: A) -> Self
    where
        A: Into<String>,
    {
        self.msi_res_id = Some(msi_res_id.into());
        self.object_id = None;
        self.client_id = None;
        self
    }
}

#[async_trait::async_trait]
impl TokenCredential for ImdsManagedIdentityCredential {
    async fn get_token(&self, resource: &str) -> Result<TokenResponse> {
        let msi_endpoint = std::env::var(MSI_ENDPOINT_ENV_KEY)
            .unwrap_or_else(|_| "http://169.254.169.254/metadata/identity/oauth2/token".to_owned());

        let mut query_items = vec![("api-version", MSI_API_VERSION), ("resource", resource)];

        let mut headers = HeaderMap::new();
        headers.insert("Metadata", "true".parse().unwrap());

        match (
            self.object_id.as_ref(),
            self.client_id.as_ref(),
            self.msi_res_id.as_ref(),
        ) {
            (Some(object_id), None, None) => query_items.push(("object_id", object_id)),
            (None, Some(client_id), None) => query_items.push(("client_id", client_id)),
            (None, None, Some(msi_res_id)) => query_items.push(("msi_res_id", msi_res_id)),
            _ => (),
        }

        let msi_endpoint_url = Url::parse_with_params(&msi_endpoint, &query_items)
            .context(ErrorKind::Credential, "error parsing url for MSI endpoint")?;

        let msi_secret = std::env::var(MSI_SECRET_ENV_KEY);
        if let Ok(val) = msi_secret {
            headers.insert("X-IDENTITY-HEADER", val.parse().unwrap());
        };

        let client = reqwest::Client::new();
        let response = client
            .get(msi_endpoint_url)
            .headers(headers)
            .send()
            .await
            .map_kind(ErrorKind::Credential)?;

        match response.status().as_u16() {
            400 => Err(Error::new(
                ErrorKind::Credential,
                "the requested identity has not been assigned to this resource",
            )),
            502 | 504 => Err(Error::new(
                ErrorKind::Credential,
                "the request failed due to a gateway error",
            )),
            _ => {
                let token_response = response
                    .json::<MsiTokenResponse>()
                    .await
                    .map_kind(ErrorKind::Credential)?;
                Ok(TokenResponse::new(
                    token_response.access_token,
                    token_response.expires_on,
                ))
            }
        }
    }
}

fn expires_on_string<'de, D>(deserializer: D) -> std::result::Result<DateTime<Utc>, D::Error>
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
#[allow(unused)]
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

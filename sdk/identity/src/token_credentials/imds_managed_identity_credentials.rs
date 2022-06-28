use azure_core::auth::{AccessToken, TokenCredential, TokenResponse};
use azure_core::error::{Error, ErrorKind, ResultExt};
use azure_core::Method;
use azure_core::{HttpClient, Request};
use chrono::{DateTime, TimeZone, Utc};
use serde::{
    de::{self, Deserializer},
    Deserialize,
};
use std::str;
use std::sync::Arc;
use url::Url;

const MSI_ENDPOINT_ENV_KEY: &str = "IDENTITY_ENDPOINT";
const MSI_SECRET_ENV_KEY: &str = "IDENTITY_HEADER";
const MSI_API_VERSION: &str = "2019-08-01";

/// Attempts authentication using a managed identity that has been assigned to the deployment environment.
///
/// This authentication type works in Azure VMs, App Service and Azure Functions applications, as well as the Azure Cloud Shell
///
/// Built up from docs at [https://docs.microsoft.com/azure/app-service/overview-managed-identity#using-the-rest-protocol](https://docs.microsoft.com/azure/app-service/overview-managed-identity#using-the-rest-protocol)
pub struct ImdsManagedIdentityCredential {
    http_client: Arc<dyn HttpClient>,
    object_id: Option<String>,
    client_id: Option<String>,
    msi_res_id: Option<String>,
}

#[cfg(any(feature = "enable_reqwest", feature = "enable_reqwest_rustls"))]
#[cfg(not(target_arch = "wasm32"))]
impl Default for ImdsManagedIdentityCredential {
    /// Creates an instance of the `TransportOptions` using the default `HttpClient`.
    fn default() -> Self {
        Self::new(azure_core::new_http_client())
    }
}

impl ImdsManagedIdentityCredential {
    /// Creates a new `ImdsManagedIdentityCredential` using the given `HttpClient`.
    pub fn new(http_client: Arc<dyn HttpClient>) -> Self {
        Self {
            http_client,
            object_id: None,
            client_id: None,
            msi_res_id: None,
        }
    }

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
    async fn get_token(&self, resource: &str) -> azure_core::Result<TokenResponse> {
        let msi_endpoint = std::env::var(MSI_ENDPOINT_ENV_KEY)
            .unwrap_or_else(|_| "http://169.254.169.254/metadata/identity/oauth2/token".to_owned());

        let mut query_items = vec![("api-version", MSI_API_VERSION), ("resource", resource)];

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

        let url = Url::parse_with_params(&msi_endpoint, &query_items).context(
            ErrorKind::DataConversion,
            "error parsing url for MSI endpoint",
        )?;

        let mut req = Request::new(url, Method::Get);

        req.insert_header("Metadata", "true");

        let msi_secret = std::env::var(MSI_SECRET_ENV_KEY);
        if let Ok(val) = msi_secret {
            req.insert_header("X-IDENTITY-HEADER", val);
        };

        let rsp = self.http_client.execute_request(&req).await?;
        let rsp_status = rsp.status();
        let rsp_body = rsp.into_body().await;

        if !rsp_status.is_success() {
            match rsp_status as u16 {
                400 => {
                    return Err(Error::message(
                        ErrorKind::Credential,
                        "the requested identity has not been assigned to this resource",
                    ))
                }
                502 | 504 => {
                    return Err(Error::message(
                        ErrorKind::Credential,
                        "the request failed due to a gateway error",
                    ))
                }
                rsp_status => {
                    return Err(
                        ErrorKind::http_response_from_body(rsp_status, &rsp_body).into_error()
                    )
                }
            }
        }

        let token_response: MsiTokenResponse = serde_json::from_slice(&rsp_body)?;
        Ok(TokenResponse::new(
            token_response.access_token,
            token_response.expires_on,
        ))
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

// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use crate::{credentials::cache::TokenCache, TokenCredentialOptions};
use azure_core::{
    credentials::{AccessToken, Secret, TokenCredential},
    error::{http_response_from_body, Error, ErrorKind},
    http::{headers::HeaderName, request::Request, HttpClient, Method, StatusCode, Url},
    json::from_json,
};
use serde::{
    de::{self, Deserializer},
    Deserialize,
};
use std::{str, sync::Arc};
use time::OffsetDateTime;

/// An identifier for the Azure Instance Metadata Service (IMDS).
///
/// IMDS provides information about currently running virtual machine instances. For more information, see:
/// * <https://learn.microsoft.com/azure/virtual-machines/instance-metadata-service>
/// * <https://learn.microsoft.com/azure/app-service/overview-managed-identity#rest-endpoint-reference>
#[derive(Debug)]
pub enum ImdsId {
    SystemAssigned,
    /// The client ID of the user-assigned identity to be used.
    ClientId(String),
    /// The principal ID of the user-assigned identity to be used.
    ObjectId(String),
    /// The Azure resource ID of the user-assigned identity to be used.
    MsiResId(String),
}

/// Attempts authentication using a managed identity that has been assigned to the deployment environment.
///
/// This authentication type works in Azure VMs, App Service and Azure Functions applications, as well as the Azure Cloud Shell
///
/// Built up from docs at [https://learn.microsoft.com/azure/app-service/overview-managed-identity#using-the-rest-protocol](https://learn.microsoft.com/azure/app-service/overview-managed-identity#using-the-rest-protocol)
#[derive(Debug)]
pub(crate) struct ImdsManagedIdentityCredential {
    http_client: Arc<dyn HttpClient>,
    endpoint: Url,
    api_version: String,
    secret_header: HeaderName,
    secret_env: String,
    id: ImdsId,
    cache: TokenCache,
}

impl ImdsManagedIdentityCredential {
    pub fn new(
        options: impl Into<TokenCredentialOptions>,
        endpoint: Url,
        api_version: &str,
        secret_header: HeaderName,
        secret_env: &str,
        id: ImdsId,
    ) -> Self {
        let options = options.into();
        Self {
            http_client: options.http_client(),
            endpoint,
            api_version: api_version.to_owned(),
            secret_header: secret_header.to_owned(),
            secret_env: secret_env.to_owned(),
            id,
            cache: TokenCache::new(),
        }
    }

    async fn get_token(&self, scopes: &[&str]) -> azure_core::Result<AccessToken> {
        let resource = scopes_to_resource(scopes)?;

        let mut query_items = vec![
            ("api-version", self.api_version.as_str()),
            ("resource", resource),
        ];

        match self.id {
            ImdsId::SystemAssigned => (),
            ImdsId::ClientId(ref client_id) => query_items.push(("client_id", client_id)),
            ImdsId::ObjectId(ref object_id) => query_items.push(("object_id", object_id)),
            ImdsId::MsiResId(ref msi_res_id) => query_items.push(("msi_res_id", msi_res_id)),
        }

        let mut url = self.endpoint.clone();
        url.query_pairs_mut().extend_pairs(query_items);

        let mut req = Request::new(url, Method::Get);

        req.insert_header("metadata", "true");

        let msi_secret = std::env::var(&self.secret_env);
        if let Ok(val) = msi_secret {
            req.insert_header(self.secret_header.clone(), val);
        };

        let rsp = self.http_client.execute_request(&req).await?;

        let (rsp_status, _, rsp_body) = rsp.deconstruct();
        let rsp_body = rsp_body.collect().await?;

        if !rsp_status.is_success() {
            match rsp_status {
                StatusCode::BadRequest => {
                    return Err(Error::message(
                        ErrorKind::Credential,
                        "the requested identity has not been assigned to this resource",
                    ))
                }
                StatusCode::BadGateway | StatusCode::GatewayTimeout => {
                    return Err(Error::message(
                        ErrorKind::Credential,
                        "the request failed due to a gateway error",
                    ))
                }
                rsp_status => {
                    return Err(http_response_from_body(rsp_status, &rsp_body).into_error())
                }
            }
        }

        let token_response: MsiTokenResponse = from_json(&rsp_body)?;
        Ok(AccessToken::new(
            token_response.access_token,
            token_response.expires_on,
        ))
    }
}

#[cfg_attr(target_arch = "wasm32", async_trait::async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait::async_trait)]
impl TokenCredential for ImdsManagedIdentityCredential {
    async fn get_token(&self, scopes: &[&str]) -> azure_core::Result<AccessToken> {
        self.cache.get_token(scopes, self.get_token(scopes)).await
    }

    async fn clear_cache(&self) -> azure_core::Result<()> {
        self.cache.clear().await
    }
}

// `expires_on` varies between a number and a date string depending on token server implementation
// https://github.com/Azure/azure-sdk-for-go/blob/66eca06a3fb1a931ddd3c7e61462967f6e5b9c2e/sdk/azidentity/managed_identity_client.go#L310
fn expires_on_string<'de, D>(deserializer: D) -> std::result::Result<OffsetDateTime, D::Error>
where
    D: Deserializer<'de>,
{
    struct ExpiresOnVisitor;

    impl<'de> de::Visitor<'de> for ExpiresOnVisitor {
        type Value = OffsetDateTime;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a string or integer representing a Unix timestamp")
        }

        fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
        where
            E: de::Error,
        {
            let as_i64 = value.parse::<i64>().map_err(de::Error::custom)?;
            OffsetDateTime::from_unix_timestamp(as_i64).map_err(de::Error::custom)
        }

        fn visit_i64<E>(self, value: i64) -> std::result::Result<Self::Value, E>
        where
            E: de::Error,
        {
            OffsetDateTime::from_unix_timestamp(value).map_err(de::Error::custom)
        }

        fn visit_u64<E>(self, value: u64) -> std::result::Result<Self::Value, E>
        where
            E: de::Error,
        {
            OffsetDateTime::from_unix_timestamp(value as i64).map_err(de::Error::custom)
        }
    }

    deserializer.deserialize_any(ExpiresOnVisitor)
}

/// Convert a `AADv2` scope to an `AADv1` resource
///
/// Directly based on the `azure-sdk-for-python` implementation:
/// ref: <https://github.com/Azure/azure-sdk-for-python/blob/d6aeefef46c94b056419613f1a5cc9eaa3af0d22/sdk/identity/azure-identity/azure/identity/_internal/__init__.py#L22>
fn scopes_to_resource<'a>(scopes: &'a [&'a str]) -> azure_core::Result<&'a str> {
    if scopes.len() != 1 {
        return Err(Error::message(
            ErrorKind::Credential,
            "only one scope is supported for IMDS authentication",
        ));
    }

    let Some(scope) = scopes.first() else {
        return Err(Error::message(
            ErrorKind::Credential,
            "no scopes were provided",
        ));
    };

    Ok(scope.strip_suffix("/.default").unwrap_or(*scope))
}

// NOTE: expires_on is _meant_ to be a String version of unix epoch time, not an integer, though it varies between implementations.
// https://learn.microsoft.com/azure/app-service/overview-managed-identity?tabs=dotnet#rest-protocol-examples
#[derive(Debug, Clone, Deserialize)]
#[allow(unused)]
struct MsiTokenResponse {
    pub access_token: Secret,
    #[serde(deserialize_with = "expires_on_string")]
    pub expires_on: OffsetDateTime,
    pub token_type: String,
    pub resource: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use time::macros::datetime;

    #[derive(Debug, Deserialize)]
    struct TestExpires {
        #[serde(deserialize_with = "expires_on_string")]
        date: OffsetDateTime,
    }

    #[test]
    fn check_expires_on_string() -> azure_core::Result<()> {
        let as_string = r#"{"date": "1586984735"}"#;
        let expected = datetime!(2020-4-15 21:5:35 UTC);
        let parsed: TestExpires = from_json(as_string)?;
        assert_eq!(expected, parsed.date);
        Ok(())
    }

    #[test]
    fn check_expires_on_int() -> azure_core::Result<()> {
        let as_string = r#"{"date": 1586984735}"#;
        let expected = datetime!(2020-4-15 21:5:35 UTC);
        let parsed: TestExpires = from_json(as_string)?;
        assert_eq!(expected, parsed.date);
        Ok(())
    }

    #[test]
    fn check_expires_on_invalid() {
        let as_string = r#"{"date": "invalid"}"#;
        let parsed: Result<TestExpires, Error> = from_json(as_string);
        assert!(parsed.is_err());
    }
}

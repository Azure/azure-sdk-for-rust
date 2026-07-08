// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use crate::{env::Env, TokenCache, UserAssignedId};
use azure_core::{
    credentials::{AccessToken, Secret, TokenCredential, TokenRequestOptions},
    error::{Error, ErrorKind},
    http::{
        headers::HeaderName, request::Request, ClientOptions, Method, Pipeline, PipelineOptions,
        PipelineSendOptions, StatusCode, Url,
    },
    json::from_json,
    time::OffsetDateTime,
};
use serde::{
    de::{self, Deserializer},
    Deserialize,
};
use std::{any::type_name, fmt, fs::File, io::Read, path::Path, str};

const AUTHENTICATE_HEADER: HeaderName = HeaderName::from_static("www-authenticate");
const AUTHORIZATION_HEADER: HeaderName = HeaderName::from_static("authorization");

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

impl From<UserAssignedId> for ImdsId {
    fn from(user_assigned_id: UserAssignedId) -> Self {
        match user_assigned_id {
            UserAssignedId::ClientId(client_id) => ImdsId::ClientId(client_id),
            UserAssignedId::ObjectId(object_id) => ImdsId::ObjectId(object_id),
            UserAssignedId::ResourceId(resource_id) => ImdsId::MsiResId(resource_id),
        }
    }
}

/// Attempts authentication using a managed identity that has been assigned to the deployment environment.
///
/// This authentication type works in Azure VMs, App Service and Azure Functions applications, as well as the Azure Cloud Shell
///
/// Built up from docs at [https://learn.microsoft.com/azure/app-service/overview-managed-identity#using-the-rest-protocol](https://learn.microsoft.com/azure/app-service/overview-managed-identity#using-the-rest-protocol)
pub(crate) struct ImdsManagedIdentityCredential {
    pipeline: Pipeline,
    endpoint: Url,
    api_version: String,
    secret_header: Option<HeaderName>,
    secret_env: Option<String>,
    id: ImdsId,
    cache: TokenCache,
    env: Env,
}

impl fmt::Debug for ImdsManagedIdentityCredential {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct(type_name::<Self>())
            .field("endpoint", &self.endpoint)
            .finish_non_exhaustive()
    }
}

impl ImdsManagedIdentityCredential {
    #[allow(clippy::too_many_arguments, reason = "private API")]
    pub fn new(
        endpoint: Url,
        api_version: &str,
        secret_header: Option<HeaderName>,
        secret_env: Option<&str>,
        id: ImdsId,
        client_options: ClientOptions,
        pipeline_options: Option<PipelineOptions>,
        env: Env,
    ) -> Self {
        let pipeline = Pipeline::new(
            option_env!("CARGO_PKG_NAME"),
            option_env!("CARGO_PKG_VERSION"),
            client_options,
            Vec::default(),
            Vec::default(),
            pipeline_options,
        );
        Self {
            pipeline,
            endpoint,
            api_version: api_version.to_owned(),
            secret_header: secret_header.to_owned(),
            secret_env: secret_env.map(|env| env.to_owned()),
            id,
            cache: TokenCache::new(),
            env,
        }
    }

    async fn get_token(
        &self,
        scopes: &[&str],
        options: Option<TokenRequestOptions<'_>>,
    ) -> azure_core::Result<AccessToken> {
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

        if let Some(ref secret_env) = self.secret_env {
            if let Some(ref secret_header) = self.secret_header {
                let msi_secret = self.env.var(secret_env);
                if let Ok(val) = msi_secret {
                    req.insert_header(secret_header.clone(), val);
                };
            }
        }

        let options = options.unwrap_or_default();
        let ctx = options.method_options.context.to_borrowed();
        let mut rsp = self
            .pipeline
            .send(
                &ctx,
                &mut req,
                Some(PipelineSendOptions {
                    skip_checks: true,
                    ..Default::default()
                }),
            )
            .await?;

        let mut status = rsp.status();

        if status == StatusCode::Unauthorized {
            if let Ok(challenge) = rsp.headers().get_str(&AUTHENTICATE_HEADER) {
                if let Some(challenge_location) = challenge
                    .split_once('=')
                    .map(|(_, location)| location.trim())
                {
                    let challenge_response =
                        self.retrieve_challenge_response(challenge_location)?;
                    req.insert_header(AUTHORIZATION_HEADER, format!("Basic {challenge_response}"));

                    // try the request again with the challenge response header. Then, drop through to the usual error handling and token extraction
                    rsp = self
                        .pipeline
                        .send(
                            &ctx,
                            &mut req,
                            Some(PipelineSendOptions {
                                skip_checks: true,
                                ..Default::default()
                            }),
                        )
                        .await?;
                    status = rsp.status();
                }
            }
        }

        if !status.is_success() {
            let message = match status {
                StatusCode::BadRequest => {
                    "The requested identity has not been assigned to this resource".to_string()
                }
                StatusCode::BadGateway | StatusCode::GatewayTimeout => {
                    "The request failed due to a gateway error".to_string()
                }
                _ => {
                    let body = String::from_utf8_lossy(rsp.body());
                    format!("The request failed: {body}")
                }
            };
            return Err(Error::new(
                ErrorKind::HttpResponse {
                    error_code: None,
                    raw_response: Some(Box::new(rsp)),
                    status,
                },
                message,
            ));
        }

        let token_response: MsiTokenResponse = from_json(rsp.into_body())?;
        Ok(AccessToken::new(
            token_response.access_token,
            token_response.expires_on,
        ))
    }

    // This is used for Arc for server's flavour of IMDS, where a challenge-response protocol is implemented.
    fn retrieve_challenge_response(&self, challenge: &str) -> Result<String, Error> {
        let challenge_path = Path::new(challenge).canonicalize()?;
        let expected_challenge_base = if cfg!(windows) {
            let program_data_dir = self.env.var("PROGRAMDATA")?;
            Path::new(&program_data_dir).join("AzureConnectedMachineAgent\\Tokens\\")
        } else {
            Path::new("/var/opt/azcmagent/tokens/").to_path_buf()
        };

        if !(challenge_path.starts_with(expected_challenge_base)
            && challenge_path.extension().is_some_and(|ext| ext == "key"))
        {
            if !cfg!(test) {
                return Err(Error::with_message(
                    ErrorKind::Credential,
                    format!("Challenge received was invalid: {challenge}"),
                ));
            } else {
                // for tests, it's okay if the challenge file is not in the expected location
            }
        }

        let challenge_file = File::open(challenge_path)?;
        let mut challenge_response = String::new();
        challenge_file
            .take(4096) // avoid slurping a huge file - the challenge response will not be > 4KiB
            .read_to_string(&mut challenge_response)?;

        Ok(challenge_response)
    }
}

#[async_trait::async_trait]
impl TokenCredential for ImdsManagedIdentityCredential {
    async fn get_token(
        &self,
        scopes: &[&str],
        options: Option<TokenRequestOptions<'_>>,
    ) -> azure_core::Result<AccessToken> {
        self.cache
            .get_token(scopes, options, |s, o| self.get_token(s, o))
            .await
    }
}

fn expires_on_string<'de, D>(deserializer: D) -> std::result::Result<OffsetDateTime, D::Error>
where
    D: Deserializer<'de>,
{
    let v = String::deserialize(deserializer)?;
    let as_i64 = v.parse::<i64>().map_err(de::Error::custom)?;
    OffsetDateTime::from_unix_timestamp(as_i64).map_err(de::Error::custom)
}

/// Convert a `AADv2` scope to an `AADv1` resource
///
/// Directly based on the `azure-sdk-for-python` implementation:
/// ref: <https://github.com/Azure/azure-sdk-for-python/blob/d6aeefef46c94b056419613f1a5cc9eaa3af0d22/sdk/identity/azure-identity/azure/identity/_internal/__init__.py#L22>
fn scopes_to_resource<'a>(scopes: &'a [&'a str]) -> azure_core::Result<&'a str> {
    if scopes.len() != 1 {
        return Err(Error::with_message(
            ErrorKind::Credential,
            "only one scope is supported for IMDS authentication",
        ));
    }

    let Some(scope) = scopes.first() else {
        return Err(Error::with_message(
            ErrorKind::Credential,
            "no scopes were provided",
        ));
    };

    Ok(scope.strip_suffix("/.default").unwrap_or(*scope))
}

// NOTE: expires_on is a String version of unix epoch time, not an integer.
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
}

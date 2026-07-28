// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use crate::{env::Env, ImdsId, TokenCache};
use azure_core::{
    credentials::{AccessToken, Secret, TokenCredential, TokenRequestOptions},
    error::{Error, ErrorKind},
    http::{
        headers::{AUTHORIZATION, WWW_AUTHENTICATE},
        request::Request,
        ClientOptions, ExponentialRetryOptions, Method, Pipeline, PipelineOptions,
        PipelineSendOptions, RetryOptions, StatusCode, Url,
    },
    json::from_json,
    time::OffsetDateTime,
};
use serde::{
    de::{self, Deserializer},
    Deserialize,
};
use std::{
    any::type_name,
    fmt,
    fs::{self, File},
    io::Read,
    path::{Path, PathBuf},
    str,
    sync::Arc,
};
use time::Duration;

const DEFAULT_ENDPOINT: &str = "http://localhost:40342/metadata/identity/oauth2/token";
const API_VERSION: &str = "2021-02-01";

/// Attempts authentication using a managed identity that has been assigned to the deployment environment on Azure Arc-connected servers.
pub(crate) struct AzureArcCredential {
    endpoint: Url,
    pipeline: Pipeline,
    id: ImdsId,
    cache: TokenCache,
    #[allow(dead_code)] // this is used in windows through a #[cfg(windows)] directive
    env: Env,
}

impl fmt::Debug for AzureArcCredential {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct(type_name::<Self>())
            .field("endpoint", &self.endpoint)
            .finish_non_exhaustive()
    }
}

impl AzureArcCredential {
    #[allow(clippy::too_many_arguments, reason = "private API")]
    pub fn new(
        id: ImdsId,
        client_options: ClientOptions,
        env: Env,
    ) -> azure_core::Result<Arc<Self>> {
        let identity_endpoint = match (
            env.var("IDENTITY_ENDPOINT").ok(),
            env.var("IMDS_ENDPOINT").ok(),
        ) {
            (Some(identity_endpoint), Some(_)) => identity_endpoint,
            _ => DEFAULT_ENDPOINT.to_owned(),
        };

        let token_url = Url::parse(&identity_endpoint)?;
        let pipeline_options = Some(PipelineOptions {
            // https://learn.microsoft.com/entra/identity/managed-identities-azure-resources/how-to-use-vm-token#error-handling
            retry_status_codes: Vec::from([
                StatusCode::NotFound,
                StatusCode::Gone,
                StatusCode::TooManyRequests,
                StatusCode::InternalServerError,
                StatusCode::NotImplemented,
                StatusCode::BadGateway,
                StatusCode::ServiceUnavailable,
                StatusCode::GatewayTimeout,
                StatusCode::HttpVersionNotSupported,
                StatusCode::VariantAlsoNegotiates,
                StatusCode::InsufficientStorage,
                StatusCode::LoopDetected,
                StatusCode::NotExtended,
                StatusCode::NetworkAuthenticationRequired,
            ]),
            ..Default::default()
        });
        // these settings approximate the recommendations at
        // https://learn.microsoft.com/entra/identity/managed-identities-azure-resources/how-to-use-vm-token#retry-guidance
        let client_options = ClientOptions {
            retry: RetryOptions::exponential(ExponentialRetryOptions {
                initial_delay: Duration::milliseconds(1340),
                max_retries: 6,
                max_total_elapsed: Duration::seconds(72),
                ..Default::default()
            }),
            ..client_options
        };

        let pipeline = Pipeline::new(
            option_env!("CARGO_PKG_NAME"),
            option_env!("CARGO_PKG_VERSION"),
            client_options,
            Vec::default(),
            Vec::default(),
            pipeline_options,
        );
        Ok(Arc::new(Self {
            endpoint: token_url,
            pipeline,
            id,
            cache: TokenCache::new(),
            env,
        }))
    }

    async fn get_token(
        &self,
        scopes: &[&str],
        options: Option<TokenRequestOptions<'_>>,
    ) -> azure_core::Result<AccessToken> {
        let resource = scopes_to_resource(scopes)?;

        let mut query_items = vec![("api-version", API_VERSION), ("resource", resource)];

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
            if let Ok(challenge) = rsp.headers().get_str(&WWW_AUTHENTICATE) {
                if let Some(challenge_location) = challenge
                    .split_once('=')
                    .map(|(_, location)| location.trim())
                {
                    let challenge_response =
                        self.retrieve_challenge_response(challenge_location)?;
                    req.insert_header(AUTHORIZATION, format!("Basic {challenge_response}"));

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

        #[cfg(all(windows, not(test)))]
        fn get_expected_challenge_base(e: &Env) -> Result<PathBuf, Error> {
            let program_data_dir = e.var("PROGRAMDATA").map_err(|err| {
                Error::with_error(ErrorKind::Io, err, "Could not find program data directory")
            })?;
            Path::new(&program_data_dir)
                .join("AzureConnectedMachineAgent\\Tokens\\")
                .canonicalize()
                .map_err(|err| {
                    Error::with_error(
                        ErrorKind::Io,
                        err,
                        "Could not find Azure Arc token directory",
                    )
                })
        }

        #[cfg(all(not(windows), not(test)))]
        fn get_expected_challenge_base(_: &Env) -> Result<PathBuf, Error> {
            Path::new("/var/opt/azcmagent/tokens/")
                .to_path_buf()
                .canonicalize()
                .map_err(|err| {
                    Error::with_error(
                        ErrorKind::Io,
                        err,
                        "Could not find Azure Arc token directory",
                    )
                })
        }

        #[cfg(test)]
        fn get_expected_challenge_base(_: &Env) -> Result<PathBuf, Error> {
            // the tests will be using temp for storing test token files
            std::env::temp_dir().canonicalize().map_err(|err| {
                Error::with_error(ErrorKind::Io, err, "Could not find temp directory")
            })
        }

        let expected_challenge_base = get_expected_challenge_base(&self.env)?;

        if !(challenge_path
            .parent()
            .is_some_and(|challenge_base| challenge_base == expected_challenge_base)
            && challenge_path.extension().is_some_and(|ext| ext == "key"))
        {
            return Err(Error::with_message(
                ErrorKind::Credential,
                format!("Challenge received was invalid: {challenge}"),
            ));
        }

        let mut challenge_file = File::open(challenge_path)?;

        let size = challenge_file
            .metadata()
            .map_or(u64::MAX, |metadata| metadata.len());
        if size > 4096 {
            return Err(Error::with_message(
                ErrorKind::Io,
                "Arc challenge token file was larger than expected",
            ));
        }

        let mut challenge_response = String::new();
        challenge_file.read_to_string(&mut challenge_response)?;

        Ok(challenge_response)
    }
}

#[async_trait::async_trait]
impl TokenCredential for AzureArcCredential {
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

pub(crate) fn is_arc_agent_present(env: &Env) -> bool {
    if cfg!(windows) {
        if let Ok(program_files_path) = env.var("PROGRAMFILES") {
            !program_files_path.is_empty()
                && fs::exists(format!(
                    "{program_files_path}\\AzureConnectedMachineAgent\\himds.exe"
                ))
                .unwrap_or(false)
        } else {
            // %PROGRAMFILES% should exist on Windows, but if it's not there,
            // we can't tell if we're on Arc or not, so just assume we aren't
            false
        }
    } else {
        fs::exists("/opt/azcmagent/bin/himds").unwrap_or(false)
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

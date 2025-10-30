// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![warn(missing_docs)]

mod app_service_managed_identity_credential;
#[cfg(not(target_arch = "wasm32"))]
mod azure_cli_credential;
#[cfg(not(target_arch = "wasm32"))]
mod azure_developer_cli_credential;
mod azure_pipelines_credential;
mod cache;
mod client_assertion_credential;
#[cfg(feature = "client_certificate")]
mod client_certificate_credential;
mod client_secret_credential;
#[cfg(not(target_arch = "wasm32"))]
mod developer_tools_credential;
mod env;
mod imds_managed_identity_credential;
mod managed_identity_credential;
#[cfg(not(target_arch = "wasm32"))]
mod process;
mod virtual_machine_managed_identity_credential;
mod workload_identity_credential;

#[cfg(not(target_arch = "wasm32"))]
pub use azure_cli_credential::*;
#[cfg(not(target_arch = "wasm32"))]
pub use azure_developer_cli_credential::*;
pub use azure_pipelines_credential::*;
pub use client_assertion_credential::*;
#[cfg(feature = "client_certificate")]
pub use client_certificate_credential::*;
pub use client_secret_credential::*;
#[cfg(not(target_arch = "wasm32"))]
pub use developer_tools_credential::*;
pub use managed_identity_credential::*;
#[cfg(not(target_arch = "wasm32"))]
pub use process::{new_executor, Executor};
pub use workload_identity_credential::*;

pub(crate) use app_service_managed_identity_credential::*;
pub(crate) use cache::TokenCache;
pub(crate) use imds_managed_identity_credential::*;
pub(crate) use virtual_machine_managed_identity_credential::*;

use crate::env::Env;
use azure_core::{
    cloud::CloudConfiguration,
    error::{ErrorKind, ResultExt},
    http::{RawResponse, Url},
    Error, Result,
};
use serde::Deserialize;
use std::borrow::Cow;

#[derive(Debug, Default, Deserialize)]
#[serde(default)]
struct EntraIdErrorResponse {
    error_description: String,
}

#[derive(Debug, Default, Deserialize)]
#[serde(default)]
struct EntraIdTokenResponse {
    token_type: String,
    // these are i64 to avoid conversion when calling Duration::seconds
    // (real values are unsigned)
    expires_in: i64,
    ext_expires_in: i64,
    access_token: String,
}

fn deserialize<T>(credential_name: &str, res: RawResponse) -> Result<T>
where
    T: serde::de::DeserializeOwned,
{
    let t: T = res
        .into_body()
        .json()
        .with_context_fn(ErrorKind::Credential, || {
            format!(
                "{} authentication failed: invalid response",
                credential_name
            )
        })?;
    Ok(t)
}

fn validate_not_empty<C>(value: &str, message: C) -> Result<()>
where
    C: Into<Cow<'static, str>>,
{
    if value.is_empty() {
        return Err(Error::with_message(ErrorKind::Credential, message));
    }

    Ok(())
}

const AZURE_AUTHORITY_HOST_ENV_KEY: &str = "AZURE_AUTHORITY_HOST";
const AZURE_PUBLIC_CLOUD: &str = "https://login.microsoftonline.com";

fn get_authority_host(env: Option<Env>, cloud: Option<&CloudConfiguration>) -> Result<Url> {
    let authority_host = match cloud {
        None => env
            .unwrap_or_default()
            .var(AZURE_AUTHORITY_HOST_ENV_KEY)
            .unwrap_or_else(|_| AZURE_PUBLIC_CLOUD.to_string()),
        Some(CloudConfiguration::Custom(config)) => config.authority_host.clone(),
        Some(CloudConfiguration::AzureGovernment) => "https://login.microsoftonline.us".to_string(),
        Some(CloudConfiguration::AzureChina) => "https://login.chinacloudapi.cn".to_string(),
        Some(CloudConfiguration::AzurePublic) => AZURE_PUBLIC_CLOUD.to_string(),
        // need this arm because CloudConfiguration is non-exhaustive
        _ => {
            return Err(Error::with_message(
                ErrorKind::Other,
                format!("unexpected cloud configuration: {:?}", cloud),
            ))
        }
    };

    let url = Url::parse(&authority_host)?;
    if url.scheme() != "https" {
        return Err(Error::with_message(
            ErrorKind::Other,
            format!("authority host doesn't use HTTPS scheme: {authority_host}"),
        ));
    }
    Ok(url)
}

const TSG_LINK_ERROR_TEXT: &str =
    ". To troubleshoot, visit https://aka.ms/azsdk/rust/identity/troubleshoot";

/// Map an error from a credential's get_token() method to an ErrorKind::Credential error, appending
/// a link to the troubleshooting guide entry for that credential, if it has one.
///
/// TODO: decide whether to map to ErrorKind::Credential here (https://github.com/Azure/azure-sdk-for-rust/issues/3127)
fn authentication_error<T: 'static>(e: azure_core::Error) -> azure_core::Error {
    azure_core::Error::with_message_fn(e.kind().clone(), || {
        let type_name = std::any::type_name::<T>();
        let short_name = type_name.rsplit("::").next().unwrap_or(type_name); // cspell:ignore rsplit
        let link = match short_name {
            "AzureCliCredential" => format!("{TSG_LINK_ERROR_TEXT}#azure-cli"),
            "AzureDeveloperCliCredential" => format!("{TSG_LINK_ERROR_TEXT}#azd"),
            "AzurePipelinesCredential" => format!("{TSG_LINK_ERROR_TEXT}#apc"),
            #[cfg(feature = "client_certificate")]
            "ClientCertificateCredential" => format!("{TSG_LINK_ERROR_TEXT}#client-cert"),
            "ClientSecretCredential" => format!("{TSG_LINK_ERROR_TEXT}#client-secret"),
            "ManagedIdentityCredential" => format!("{TSG_LINK_ERROR_TEXT}#managed-id"),
            "WorkloadIdentityCredential" => format!("{TSG_LINK_ERROR_TEXT}#workload"),
            _ => "".to_string(),
        };

        format!("{short_name} authentication failed: {e}{link}")
    })
}

#[test]
fn test_validate_not_empty() {
    assert!(validate_not_empty("", "it's empty").is_err());
    assert!(validate_not_empty(" ", "it's not empty").is_ok());
    assert!(validate_not_empty("not empty", "it's not empty").is_ok());
}

#[cfg_attr(target_arch = "wasm32", allow(dead_code))]
fn validate_scope(scope: &str) -> Result<()> {
    if scope.is_empty()
        || !scope.chars().all(|c| {
            c.is_alphanumeric() || c == '.' || c == '-' || c == '_' || c == ':' || c == '/'
        })
    {
        return Err(Error::with_message(
            ErrorKind::Credential,
            format!("invalid scope {scope}"),
        ));
    }

    Ok(())
}

#[test]
fn test_validate_scope() {
    assert!(validate_scope("").is_err());
    assert!(validate_scope("invalid_scope@id").is_err());
    assert!(validate_scope("A-1b_2c:3d/4.z").is_ok());
    assert!(validate_scope("http://vault.azure.net").is_ok());
}

#[cfg_attr(target_arch = "wasm32", allow(dead_code))]
fn validate_subscription(subscription: &str) -> Result<()> {
    if subscription.is_empty()
        || !subscription
            .chars()
            .all(|c| c.is_alphanumeric() || c == '.' || c == '-' || c == '_' || c == ' ')
    {
        return Err(Error::with_message(
            ErrorKind::Credential,
            format!("invalid subscription {subscription}. If this is the name of a subscription, use its ID instead"),
        ));
    }

    Ok(())
}

#[test]
fn test_validate_subscription() {
    assert!(validate_subscription("").is_err());
    assert!(validate_subscription("invalid_subscription@id").is_err());
    assert!(validate_subscription("A-1b_2c 3.z").is_ok());
    assert!(validate_subscription("7b795fb9-09d3-42f4-a494-38864f99ba3c").is_ok());
}

fn validate_tenant_id(tenant_id: &str) -> Result<()> {
    if tenant_id.is_empty()
        || !tenant_id
            .chars()
            .all(|c| c.is_alphanumeric() || c == '.' || c == '-')
    {
        return Err(Error::with_message(
            ErrorKind::Credential,
            format!("invalid tenant ID {tenant_id}. You can locate your tenantID by following the instructions listed here: https://learn.microsoft.com/partner-center/find-ids-and-domain-names"),
        ));
    }

    Ok(())
}

#[test]
fn test_validate_tenant_id() {
    assert!(validate_tenant_id("").is_err());
    assert!(validate_tenant_id("invalid_tenant@id").is_err());
    assert!(validate_tenant_id("A-1.z").is_ok());
    assert!(validate_tenant_id("7b795fb9-09d3-42f4-a494-38864f99ba3c").is_ok());
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{env::Env, process::Executor};
    use async_trait::async_trait;
    use azure_core::{
        cloud::{CloudConfiguration, CustomConfiguration},
        error::ErrorKind,
        http::{headers::Headers, BufResponse, Request, StatusCode},
        Bytes, Error, Result,
    };
    use std::{
        ffi::OsStr,
        process::Output,
        sync::{
            atomic::{AtomicUsize, Ordering},
            Arc, Mutex,
        },
    };

    pub const FAKE_CLIENT_ID: &str = "fake-client";
    pub const FAKE_PUBLIC_CLOUD_AUTHORITY: &str = "https://login.microsoftonline.com/fake-tenant";
    pub const FAKE_TENANT_ID: &str = "fake-tenant";
    pub const FAKE_TOKEN: &str = "***";
    pub const LIVE_TEST_RESOURCE: &str = "https://management.azure.com";
    pub const LIVE_TEST_SCOPES: &[&str] = &["https://management.azure.com/.default"];

    pub type RunCallback = Arc<dyn Fn(&OsStr, &[&OsStr]) + Send + Sync>;

    #[derive(Default)]
    pub struct MockExecutor {
        call_count: AtomicUsize,
        error: Option<std::io::Error>,
        on_run: Option<RunCallback>,
        output: Mutex<Option<Output>>,
    }

    impl std::fmt::Debug for MockExecutor {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("MockExecutor").finish()
        }
    }

    impl MockExecutor {
        pub fn with_error(err: std::io::Error) -> Arc<Self> {
            Arc::new(Self {
                error: Some(err),
                ..Default::default()
            })
        }

        pub fn with_output(
            exit_code: i32,
            stdout: &str,
            stderr: &str,
            on_run: Option<RunCallback>,
        ) -> Arc<Self> {
            let output = Output {
                status: {
                    #[cfg(windows)]
                    {
                        std::os::windows::process::ExitStatusExt::from_raw(
                            exit_code.try_into().unwrap(),
                        )
                    }
                    #[cfg(unix)]
                    {
                        std::os::unix::process::ExitStatusExt::from_raw(exit_code)
                    }
                },
                stdout: stdout.as_bytes().to_vec(),
                stderr: stderr.as_bytes().to_vec(),
            };
            Arc::new(Self {
                on_run,
                output: Mutex::new(Some(output)),
                call_count: AtomicUsize::new(0),
                ..Default::default()
            })
        }

        pub fn call_count(&self) -> usize {
            self.call_count.load(Ordering::SeqCst)
        }
    }

    #[async_trait]
    impl Executor for MockExecutor {
        async fn run(&self, program: &OsStr, args: &[&OsStr]) -> std::io::Result<Output> {
            self.call_count.fetch_add(1, Ordering::SeqCst);

            if let Some(on_run) = &self.on_run {
                on_run(program, args);
            }
            if let Some(err) = &self.error {
                return Err(std::io::Error::new(err.kind(), err.to_string()));
            }
            let output = self.output.lock().unwrap();
            match output.as_ref() {
                Some(output) => Ok(output.clone()),
                None => panic!("MockExecutor output not configured"),
            }
        }
    }

    pub fn token_response() -> BufResponse {
        BufResponse::from_bytes(
            StatusCode::Ok,
            Headers::default(),
            Bytes::from(format!(
                r#"{{"access_token":"{FAKE_TOKEN}","expires_in":3600,"token_type":"Bearer"}}"#,
            )),
        )
    }

    pub type RequestCallback = Arc<dyn Fn(&Request) -> Result<()> + Send + Sync>;

    pub struct MockSts {
        responses: Mutex<Vec<BufResponse>>,
        on_request: Option<RequestCallback>,
    }

    impl std::fmt::Debug for MockSts {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("MockSts").finish()
        }
    }

    impl MockSts {
        pub fn new(responses: Vec<BufResponse>, on_request: Option<RequestCallback>) -> Self {
            Self {
                responses: Mutex::new(responses),
                on_request,
            }
        }
    }

    #[async_trait::async_trait]
    impl azure_core::http::HttpClient for MockSts {
        async fn execute_request(&self, request: &Request) -> Result<BufResponse> {
            self.on_request.as_ref().map_or(Ok(()), |f| f(request))?;
            let mut responses = self.responses.lock().unwrap();
            if responses.is_empty() {
                Err(Error::with_message(
                    ErrorKind::Other,
                    "No more mock responses",
                ))
            } else {
                Ok(responses.remove(0)) // Use remove(0) to return responses in the correct order
            }
        }
    }

    pub fn cloud_configuration_cases() -> Vec<(CloudConfiguration, String)> {
        let custom_host = "https://login.contoso.local/".to_string();

        let mut custom_no_trailing_slash = CustomConfiguration::default();
        custom_no_trailing_slash.authority_host = custom_host.trim_end_matches('/').to_string();

        let mut custom_trailing_slash = CustomConfiguration::default();
        custom_trailing_slash.authority_host = custom_host;

        vec![
            (
                CloudConfiguration::AzurePublic,
                FAKE_PUBLIC_CLOUD_AUTHORITY.to_string(),
            ),
            (
                CloudConfiguration::AzureGovernment,
                format!("https://login.microsoftonline.us/{FAKE_TENANT_ID}"),
            ),
            (
                CloudConfiguration::AzureChina,
                format!("https://login.chinacloudapi.cn/{FAKE_TENANT_ID}"),
            ),
            (
                CloudConfiguration::Custom(custom_trailing_slash),
                format!("https://login.contoso.local/{FAKE_TENANT_ID}"),
            ),
            (
                CloudConfiguration::Custom(custom_no_trailing_slash),
                format!("https://login.contoso.local/{FAKE_TENANT_ID}"),
            ),
        ]
    }

    #[test]
    fn cloud_configuration_overrides_env() {
        let mut config = CustomConfiguration::default();
        config.authority_host = "https://custom".to_string();
        let cloud = CloudConfiguration::Custom(config);

        let env = Env::from(&[(crate::AZURE_AUTHORITY_HOST_ENV_KEY, "https://env")][..]);

        let authority = get_authority_host(Some(env), Some(&cloud)).unwrap();
        assert_eq!(authority.as_str(), "https://custom/"); // Url::parse adds the trailing slash
    }

    #[test]
    fn insecure_authority_host() {
        let authority_host = "http://insecure";
        let env = Env::from(&[(crate::AZURE_AUTHORITY_HOST_ENV_KEY, authority_host)][..]);
        let err = get_authority_host(Some(env), None).unwrap_err();
        assert!(err.to_string().contains("HTTPS"));

        let mut config = CustomConfiguration::default();
        config.authority_host = authority_host.to_string();
        let cloud = CloudConfiguration::Custom(config);
        let err = get_authority_host(None, Some(&cloud)).unwrap_err();
        assert!(err.to_string().contains("HTTPS"));
    }
}

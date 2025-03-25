// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use crate::{env::Env, validate_scope, validate_tenant_id};
use azure_core::{
    credentials::{AccessToken, Secret, TokenCredential},
    error::{Error, ErrorKind},
    json::from_json,
    process::{new_executor, Executor},
};
use serde::de::{self, Deserializer};
use serde::Deserialize;
use std::{ffi::OsStr, fmt::Debug, str, sync::Arc};
use time::format_description::well_known::Rfc3339;
use time::OffsetDateTime;

const AZURE_DEVELOPER_CLI_CREDENTIAL: &str = "AzureDeveloperCliCredential";

#[derive(Clone, Debug, Deserialize)]
struct AzdTokenResponse {
    #[serde(rename = "token")]
    pub access_token: Secret,
    #[serde(rename = "expiresOn", deserialize_with = "parse_expires_on")]
    pub expires_on: OffsetDateTime,
}

fn parse_expires_on<'de, D>(deserializer: D) -> std::result::Result<OffsetDateTime, D::Error>
where
    D: Deserializer<'de>,
{
    let s: &str = Deserialize::deserialize(deserializer)?;
    OffsetDateTime::parse(s, &Rfc3339).map_err(de::Error::custom)
}

/// Authenticates the identity logged in to the [Azure Developer CLI](https://learn.microsoft.com/azure/developer/azure-developer-cli/overview).
#[derive(Debug)]
pub struct AzureDeveloperCliCredential {
    env: Env,
    executor: Arc<dyn Executor>,
    tenant_id: Option<String>,
}

/// Options for constructing an [`AzureDeveloperCliCredential`].
#[derive(Clone, Debug, Default)]
pub struct AzureDeveloperCliCredentialOptions {
    /// An implementation of [`Executor`] to run commands asynchronously.
    ///
    /// If `None`, one is created using [`new_executor`]; alternatively,
    /// you can supply your own implementation using a different asynchronous runtime.
    pub executor: Option<Arc<dyn Executor>>,

    /// Identifies the tenant the credential should authenticate in.
    ///
    /// Defaults to the azd environment, which is the tenant of the selected Azure subscription.
    pub tenant_id: Option<String>,

    env: Option<Env>,
}

impl AzureDeveloperCliCredential {
    /// Create a new [`AzureDeveloperCliCredential`].
    pub fn new(
        options: Option<AzureDeveloperCliCredentialOptions>,
    ) -> azure_core::Result<Arc<Self>> {
        let options = options.unwrap_or_default();
        if let Some(ref tenant_id) = options.tenant_id {
            validate_tenant_id(tenant_id)?;
        }
        let env = options.env.unwrap_or_default();
        let executor = options.executor.unwrap_or(new_executor());
        Ok(Arc::new(Self {
            env,
            executor,
            tenant_id: options.tenant_id,
        }))
    }
}

#[cfg_attr(target_arch = "wasm32", async_trait::async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait::async_trait)]
impl TokenCredential for AzureDeveloperCliCredential {
    async fn get_token(&self, scopes: &[&str]) -> azure_core::Result<AccessToken> {
        if scopes.is_empty() {
            return Err(Error::new(
                ErrorKind::Credential,
                "at least one scope required",
            ));
        }
        let mut command = "azd auth token -o json".to_string();
        for scope in scopes {
            validate_scope(scope)?;
            command.push_str(" --scope ");
            command.push_str(scope);
        }
        if let Some(ref tenant_id) = self.tenant_id {
            command.push_str(" --tenant-id ");
            command.push_str(tenant_id);
        }
        let (workdir, program, c_switch) = if cfg!(target_os = "windows") {
            let system_root = self.env.var("SYSTEMROOT").map_err(|_| {
                Error::message(
                    ErrorKind::Credential,
                    "SYSTEMROOT environment variable not set",
                )
            })?;
            (system_root, "cmd", "/C")
        } else {
            ("/bin".to_string(), "/bin/sh", "-c")
        };
        let command_string = format!("cd {workdir} && {command}");
        let args = vec![OsStr::new(c_switch), OsStr::new(command_string.as_str())];

        let status = self.executor.run(OsStr::new(program), &args).await;

        match status {
            Ok(azd_output) if azd_output.status.success() => {
                let output = str::from_utf8(&azd_output.stdout)?;
                let response: AzdTokenResponse = from_json(output)?;
                Ok(AccessToken::new(response.access_token, response.expires_on))
            }
            Ok(azd_output) => {
                let stderr = String::from_utf8_lossy(&azd_output.stderr);
                let message = if stderr.contains("azd auth login") {
                    "please run 'azd auth login' from a command prompt before using this credential"
                } else if azd_output.status.code() == Some(127)
                    || stderr.contains("'azd' is not recognized")
                {
                    "Azure Developer CLI not found on path"
                } else {
                    &stderr
                };
                Err(Error::with_message(ErrorKind::Credential, || {
                    format!("{AZURE_DEVELOPER_CLI_CREDENTIAL} authentication failed: {message}")
                }))
            }
            Err(e) => {
                let message = format!(
                    "{AZURE_DEVELOPER_CLI_CREDENTIAL} authentication failed due to {} error: {e}",
                    e.kind()
                );
                Err(Error::with_message(ErrorKind::Credential, || message))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::{MockExecutor, FAKE_TENANT_ID, FAKE_TOKEN, LIVE_TEST_SCOPES};
    use time::UtcOffset;

    async fn run_test(
        exit_code: i32,
        stdout: &str,
        stderr: &str,
        tenant_id: Option<String>,
    ) -> azure_core::Result<AccessToken> {
        let tenant_id_for_on_run = tenant_id.clone();
        let options = AzureDeveloperCliCredentialOptions {
            executor: Some(MockExecutor::with_output(
                exit_code,
                stdout,
                stderr,
                Some(Arc::new(move |program: &OsStr, args: &[&OsStr]| {
                    let args: Vec<String> = args
                        .iter()
                        .map(|arg| arg.to_string_lossy().to_string())
                        .collect();
                    if cfg!(target_os = "windows") {
                        assert_eq!(program.to_string_lossy(), "cmd");
                        assert_eq!(args[0], "/C");
                    } else {
                        assert_eq!(program, "/bin/sh");
                        assert_eq!(args[0], "-c");
                    }
                    for scope in LIVE_TEST_SCOPES {
                        assert!(args[1].contains(&format!(" --scope {scope}")));
                    }
                    if let Some(ref tenant_id) = tenant_id_for_on_run {
                        assert!(args[1].ends_with(&format!(" --tenant-id {tenant_id}")));
                    } else {
                        assert!(!args[1].contains("--tenant-id"));
                    }
                })),
            )),
            tenant_id,
            ..Default::default()
        };
        let cred = AzureDeveloperCliCredential::new(Some(options))?;
        return cred.get_token(LIVE_TEST_SCOPES).await;
    }

    #[tokio::test]
    async fn error_includes_stderr() {
        let stderr = "something went wrong";
        let err = run_test(1, "stdout", stderr, None)
            .await
            .expect_err("expected error");
        assert!(matches!(err.kind(), ErrorKind::Credential));
        assert!(err.to_string().contains(stderr));
    }

    #[tokio::test]
    async fn get_token_success() {
        let expires_on = "2038-01-18T00:00:00Z";
        let stdout = format!(r#"{{"token":"{FAKE_TOKEN}","expiresOn":"{expires_on}"}}"#);
        let token = run_test(0, &stdout, "", None).await.expect("token");
        assert_eq!(FAKE_TOKEN, token.token.secret());
        assert_eq!(
            OffsetDateTime::parse(expires_on, &Rfc3339).unwrap(),
            token.expires_on
        );
        assert_eq!(UtcOffset::UTC, token.expires_on.offset());
    }

    #[tokio::test]
    async fn not_logged_in() {
        let stderr = r#"{{"type":"consoleMessage","timestamp":"2038-01-18T00:00:00Z","data":{"message":"\nERROR: not logged in, run `azd auth login` to login\n"}}"#;
        let err = run_test(1, "", stderr, None).await.expect_err("error");
        assert!(matches!(err.kind(), ErrorKind::Credential));
        assert!(err.to_string().contains("azd auth login"));
    }

    #[tokio::test]
    async fn program_not_found() {
        let executor = MockExecutor::with_error(std::io::Error::from_raw_os_error(127));
        let options = AzureDeveloperCliCredentialOptions {
            executor: Some(executor),
            ..Default::default()
        };
        let cred = AzureDeveloperCliCredential::new(Some(options)).expect("valid credential");
        let err = cred
            .get_token(LIVE_TEST_SCOPES)
            .await
            .expect_err("expected error");
        assert!(matches!(err.kind(), ErrorKind::Credential));
    }

    #[tokio::test]
    async fn tenant_id() {
        let stdout = format!(r#"{{"token":"{FAKE_TOKEN}","expiresOn":"2038-01-18T00:00:00Z"}}"#);
        let token = run_test(0, &stdout, "", Some(FAKE_TENANT_ID.to_string()))
            .await
            .expect("token");
        assert_eq!(FAKE_TOKEN, token.token.secret());
        assert_eq!(UtcOffset::UTC, token.expires_on.offset());
    }
}

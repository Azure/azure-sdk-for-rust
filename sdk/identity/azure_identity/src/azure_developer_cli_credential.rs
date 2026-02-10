// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use crate::{
    authentication_error,
    env::Env,
    process::{new_executor, shell_exec, Executor, OutputProcessor},
    validate_scope, validate_tenant_id,
};
use azure_core::{
    credentials::{AccessToken, Secret, TokenCredential, TokenRequestOptions},
    error::{Error, ErrorKind},
    json::from_json,
    time::OffsetDateTime,
};
use serde::de::{self, Deserializer};
use serde::Deserialize;
use std::{ffi::OsString, sync::Arc};
use time::format_description::well_known::Rfc3339;

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

impl OutputProcessor for AzdTokenResponse {
    fn deserialize_token(stdout: &str) -> azure_core::Result<AccessToken> {
        let response: Self = from_json(stdout)?;
        Ok(AccessToken::new(response.access_token, response.expires_on))
    }

    fn get_error_message(stderr: &str) -> Option<&str> {
        // azd embeds its "you need to log in" error message in JSON, so in that case we can provide a clearer one
        if stderr.contains("azd auth login") {
            Some("please run `azd auth login` from a command prompt before using this credential")
        } else {
            None
        }
    }

    fn tool_name() -> &'static str {
        "azd"
    }
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

    #[cfg(test)]
    pub(crate) env: Option<Env>,
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
        #[cfg(test)]
        let env = options.env.unwrap_or_default();
        #[cfg(not(test))]
        let env = Env::default();
        let executor = options.executor.unwrap_or(new_executor());
        Ok(Arc::new(Self {
            env,
            executor,
            tenant_id: options.tenant_id,
        }))
    }
}

#[async_trait::async_trait]
impl TokenCredential for AzureDeveloperCliCredential {
    async fn get_token(
        &self,
        scopes: &[&str],
        _: Option<TokenRequestOptions<'_>>,
    ) -> azure_core::Result<AccessToken> {
        if scopes.is_empty() {
            return Err(Error::new(
                ErrorKind::Credential,
                "at least one scope required",
            ));
        }
        let mut command = OsString::from("azd auth token -o json --no-prompt");
        for scope in scopes {
            validate_scope(scope)?;
            command.push(" --scope ");
            command.push(scope);
        }
        if let Some(ref tenant_id) = self.tenant_id {
            command.push(" --tenant-id ");
            command.push(tenant_id);
        }
        shell_exec::<AzdTokenResponse>(self.executor.clone(), &self.env, &command)
            .await
            .map_err(|err| authentication_error(stringify!(AzureDeveloperCliCredential), err))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::{MockExecutor, FAKE_TENANT_ID, FAKE_TOKEN, LIVE_TEST_SCOPES};
    use std::ffi::OsStr;
    use time::UtcOffset;

    async fn run_test(
        exit_code: i32,
        stdout: &str,
        stderr: &str,
        tenant_id: Option<String>,
    ) -> azure_core::Result<AccessToken> {
        let tenant_id_for_on_run = tenant_id.clone();
        let system_root = "/dev/null";
        let options = AzureDeveloperCliCredentialOptions {
            env: Some(Env::from(&[("SYSTEMROOT", system_root)][..])),
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
                        assert!(args[1].starts_with(&format!(
                            "cd {system_root} && azd auth token -o json --no-prompt"
                        )));
                    } else {
                        assert_eq!(program, "/bin/sh");
                        assert_eq!(args[0], "-c");
                        assert!(
                            args[1].starts_with("cd /bin && azd auth token -o json --no-prompt")
                        );
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
        };
        let cred = AzureDeveloperCliCredential::new(Some(options))?;
        return cred.get_token(LIVE_TEST_SCOPES, None).await;
    }

    #[tokio::test]
    async fn error_includes_stderr() {
        let err = run_test(1, "stdout", "something went wrong", None)
            .await
            .expect_err("expected error");
        assert!(matches!(err.kind(), ErrorKind::Credential));
        assert_eq!(
            "AzureDeveloperCliCredential authentication failed. something went wrong\nTo troubleshoot, visit https://aka.ms/azsdk/rust/identity/troubleshoot#azd",
            err.to_string()
        );
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
        assert_eq!(
            "AzureDeveloperCliCredential authentication failed. please run `azd auth login` from a command prompt before using this credential\nTo troubleshoot, visit https://aka.ms/azsdk/rust/identity/troubleshoot#azd",
            err.to_string()
        );
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
            .get_token(LIVE_TEST_SCOPES, None)
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

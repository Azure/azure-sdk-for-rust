// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use crate::{
    credentials::{cache::TokenCache, TokenCredentialOptions},
    env::Env,
    process::{shell_exec, OutputProcessor},
    validate_scope, validate_subscription, validate_tenant_id,
};
use azure_core::{
    credentials::{AccessToken, Secret, TokenCredential},
    error::{Error, ErrorKind, ResultExt},
    json::from_json,
    process::{new_executor, Executor},
};
use serde::Deserialize;
use std::sync::Arc;
use time::OffsetDateTime;
use tracing::trace;

/// The response from `az account get-access-token --output json`.
#[derive(Debug, Clone, Deserialize)]
struct CliTokenResponse {
    #[serde(rename = "accessToken")]
    pub access_token: Secret,
    #[serde(rename = "expires_on")]
    /// The token's expiry time in seconds since the epoch, a unix timestamp.
    /// Available in Azure CLI 2.54.0 or newer.
    pub expires_on: Option<i64>,
    #[allow(unused)]
    #[serde(rename = "tokenType")]
    pub token_type: String,
}

impl CliTokenResponse {
    pub fn expires_on(&self) -> azure_core::Result<OffsetDateTime> {
        match self.expires_on {
            Some(timestamp) => Ok(OffsetDateTime::from_unix_timestamp(timestamp)
                .with_context(ErrorKind::DataConversion, || {
                    format!("unable to parse expires_on '{timestamp}'")
                })?),
            None => Err(Error::message(
                ErrorKind::DataConversion,
                "expires_on field not found. Please use Azure CLI 2.54.0 or newer.",
            )),
        }
    }
}

impl OutputProcessor for CliTokenResponse {
    fn credential_name() -> &'static str {
        "AzureCliCredential"
    }

    fn get_error_message(_stderr: &str) -> Option<&str> {
        // Azure CLI's errors are generally clear and more helpful than anything we'd write here
        None
    }

    fn deserialize_token(stdout: &str) -> azure_core::Result<AccessToken> {
        let response: Self = from_json(stdout)?;
        let expires_on = response.expires_on()?;
        Ok(AccessToken::new(response.access_token, expires_on))
    }

    fn tool_name() -> &'static str {
        "Azure CLI"
    }
}

/// Enables authentication to Azure Active Directory using Azure CLI to obtain an access token.
#[derive(Debug)]
pub struct AzureCliCredential {
    cache: TokenCache,
    options: AzureCliCredentialOptions,
}

/// Options for constructing an [`AzureCliCredential`].
#[derive(Clone, Debug, Default)]
pub struct AzureCliCredentialOptions {
    /// Specifies tenants to which the credential may authenticate, in addition to [`Self::tenant_id`].
    ///
    /// When `tenant_id` is `None` this option has no effect and the credential will authenticate to any requested tenant.
    /// Add the wildcard value "*" to allow the credential to authenticate to any tenant.
    pub additionally_allowed_tenants: Vec<String>,

    /// The name or ID of a subscription
    ///
    /// Set this to acquire tokens for an account other than the Azure CLI's current account.
    pub subscription: Option<String>,

    /// Identifies the tenant the credential should authenticate in.
    ///
    /// Defaults to the CLI's default tenant, which is typically the home tenant of the logged in user.
    pub tenant_id: Option<String>,

    /// An implementation of [`Executor`] to run commands asynchronously.
    ///
    /// If `None`, one is created using [`new_executor`]; alternatively,
    /// you can supply your own implementation using a different asynchronous runtime.
    pub executor: Option<Arc<dyn Executor>>,

    env: Option<Env>,
}

impl AzureCliCredential {
    /// Create a new `AzureCliCredential`.
    pub fn new(options: Option<AzureCliCredentialOptions>) -> azure_core::Result<Arc<Self>> {
        let mut options = options.unwrap_or_default();
        if let Some(ref tenant_id) = options.tenant_id {
            validate_tenant_id(tenant_id)?;
        }
        if let Some(ref subscription) = options.subscription {
            validate_subscription(subscription)?;
        }
        if options.env.is_none() {
            options.env = Some(Env::default());
        }
        if options.executor.is_none() {
            options.executor = Some(new_executor());
        }

        Ok(Arc::new(Self {
            cache: TokenCache::new(),
            options,
        }))
    }

    async fn get_token(&self, scopes: &[&str]) -> azure_core::Result<AccessToken> {
        if scopes.is_empty() {
            return Err(Error::new(
                ErrorKind::Credential,
                "exactly one scope required",
            ));
        }
        validate_scope(scopes[0])?;

        let mut command = "az account get-access-token -o json --scope ".to_string();
        command.push_str(scopes[0]);
        if let Some(ref tenant_id) = self.options.tenant_id {
            command.push_str(" --tenant ");
            command.push_str(tenant_id);
        }
        if let Some(ref subscription) = self.options.subscription {
            command.push_str(r#" --subscription ""#);
            command.push_str(subscription);
            command.push('"');
        }

        trace!("running Azure CLI command: {command}");

        shell_exec::<CliTokenResponse>(
            // unwrap() is safe because new() ensured the values are Some
            self.options.executor.clone().unwrap(),
            self.options.env.as_ref().unwrap(),
            &command,
        )
        .await
    }
}

#[cfg_attr(target_arch = "wasm32", async_trait::async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait::async_trait)]
impl TokenCredential for AzureCliCredential {
    async fn get_token(&self, scopes: &[&str]) -> azure_core::Result<AccessToken> {
        self.cache.get_token(scopes, self.get_token(scopes)).await
    }
}

impl From<TokenCredentialOptions> for AzureCliCredentialOptions {
    fn from(options: TokenCredentialOptions) -> Self {
        Self {
            executor: Some(options.executor.clone()),
            ..Default::default()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::{MockExecutor, FAKE_TENANT_ID, FAKE_TOKEN, LIVE_TEST_SCOPES};
    use std::ffi::OsStr;
    use time::{format_description::well_known::Rfc3339, UtcOffset};

    async fn run_test(
        exit_code: i32,
        stdout: &str,
        stderr: &str,
        subscription: Option<String>,
        tenant_id: Option<String>,
    ) -> azure_core::Result<AccessToken> {
        let subscription_for_on_run = subscription.clone();
        let tenant_for_on_run = tenant_id.clone();
        let system_root = "/dev/null";
        let options =
            AzureCliCredentialOptions {
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
                                "cd {system_root} && az account get-access-token -o json"
                            )));
                        } else {
                            assert_eq!(program, "/bin/sh");
                            assert_eq!(args[0], "-c");
                            assert!(args[1]
                                .starts_with("cd /bin && az account get-access-token -o json"));
                        }
                        for scope in LIVE_TEST_SCOPES {
                            assert!(args[1].contains(&format!(" --scope {scope}")));
                        }
                        if let Some(ref subscription_id) = subscription_for_on_run {
                            assert!(args[1]
                                .contains(&format!(r#" --subscription "{subscription_id}""#)));
                        } else {
                            assert!(!args[1].contains("--subscription"));
                        }
                        if let Some(ref tenant_id) = tenant_for_on_run {
                            assert!(args[1].contains(&format!(" --tenant {tenant_id}")));
                        } else {
                            assert!(!args[1].contains("--tenant"));
                        }
                    })),
                )),
                tenant_id,
                subscription,
                ..Default::default()
            };
        let cred = AzureCliCredential::new(Some(options))?;
        return cred.get_token(LIVE_TEST_SCOPES).await;
    }

    #[tokio::test]
    async fn error_includes_stderr() {
        let stderr = "something went wrong";
        let err = run_test(1, "stdout", stderr, None, None)
            .await
            .expect_err("expected error");
        assert!(matches!(err.kind(), ErrorKind::Credential));
        assert!(err.to_string().contains(stderr));
    }

    #[tokio::test]
    async fn get_token_success() {
        let expires_on = OffsetDateTime::parse("2038-01-18T00:00:00Z", &Rfc3339).unwrap();
        let stdout = format!(
            r#"{{"accessToken":"{FAKE_TOKEN}",
            "expiresOn":"2030-01-02 03:04:05.000000",
            "expires_on":{},
            "subscription":"...",
            "tenant":"{FAKE_TENANT_ID}",
            "tokenType":"Bearer"}}"#,
            expires_on.unix_timestamp()
        );

        let token = run_test(0, &stdout, "", None, None).await.expect("token");

        assert_eq!(FAKE_TOKEN, token.token.secret());
        assert_eq!(UtcOffset::UTC, token.expires_on.offset());
        assert_eq!(expires_on, token.expires_on);
    }

    #[tokio::test]
    async fn not_logged_in() {
        let err = run_test(1, "", "Please run 'az login' to setup account.", None, None)
            .await
            .expect_err("error");
        assert!(matches!(err.kind(), ErrorKind::Credential));
        assert!(err.to_string().contains("az login"));
    }

    #[tokio::test]
    async fn program_not_found() {
        let executor = MockExecutor::with_error(std::io::Error::from_raw_os_error(127));
        let options = AzureCliCredentialOptions {
            executor: Some(executor),
            ..Default::default()
        };

        let err = AzureCliCredential::new(Some(options))
            .expect("valid credential")
            .get_token(LIVE_TEST_SCOPES)
            .await
            .expect_err("expected error");

        assert!(matches!(err.kind(), ErrorKind::Credential));
    }

    #[tokio::test]
    async fn subscription() {
        let expires_on = OffsetDateTime::parse("2038-01-18T00:00:00Z", &Rfc3339).unwrap();
        let subscription = "subscription name";
        let stdout = format!(
            r#"{{"accessToken":"{FAKE_TOKEN}",
            "expiresOn":"2030-01-02 03:04:05.000000",
            "expires_on":{},
            "subscription":"{subscription}",
            "tenant":"{FAKE_TENANT_ID}",
            "tokenType":"Bearer"}}"#,
            expires_on.unix_timestamp()
        );

        let token = run_test(0, &stdout, "", Some(subscription.to_string()), None)
            .await
            .expect("token");

        assert_eq!(FAKE_TOKEN, token.token.secret());
        assert_eq!(UtcOffset::UTC, token.expires_on.offset());
        assert_eq!(expires_on, token.expires_on);
    }

    #[tokio::test]
    async fn tenant_id() {
        let expires_on = OffsetDateTime::parse("2038-01-18T00:00:00Z", &Rfc3339).unwrap();
        let stdout = format!(
            r#"{{"accessToken":"{FAKE_TOKEN}",
            "expiresOn":"2030-01-02 03:04:05.000000",
            "expires_on":{},
            "subscription":"...",
            "tenant":"{FAKE_TENANT_ID}",
            "tokenType":"Bearer"}}"#,
            expires_on.unix_timestamp()
        );

        let token = run_test(0, &stdout, "", None, Some(FAKE_TENANT_ID.to_string()))
            .await
            .expect("token");

        assert_eq!(FAKE_TOKEN, token.token.secret());
        assert_eq!(UtcOffset::UTC, token.expires_on.offset());
        assert_eq!(expires_on, token.expires_on);
    }
}

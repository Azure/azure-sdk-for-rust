// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use crate::{
    credentials::{cache::TokenCache, TokenCredentialOptions},
    validate_scope, validate_subscription, validate_tenant_id,
};
use azure_core::{
    credentials::{AccessToken, Secret, TokenCredential},
    error::{Error, ErrorKind, ResultExt},
    json::from_json,
    process::{new_executor, Executor},
};
use serde::Deserialize;
use std::{ffi::OsStr, fmt::Debug, str, sync::Arc};
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
        if options.executor.is_none() {
            options.executor = Some(new_executor());
        }

        Ok(Arc::new(Self {
            cache: TokenCache::new(),
            options,
        }))
    }

    /// Get an access token for an optional resource
    async fn get_access_token(&self, scopes: &[&str]) -> azure_core::Result<CliTokenResponse> {
        if scopes.is_empty() {
            return Err(Error::new(
                ErrorKind::Credential,
                "exactly one scope required",
            ));
        }
        // Pass the CLI a Microsoft Entra ID v1 resource because we don't know which CLI version is installed and older ones don't support v2 scopes.
        let resource = scopes[0].trim_end_matches("/.default");
        validate_scope(resource)?;

        // On Windows az is a cmd and it should be called like this.
        // See https://doc.rust-lang.org/nightly/std/process/struct.Command.html
        let program = if cfg!(target_os = "windows") {
            "cmd"
        } else {
            "az"
        };
        let mut args = Vec::new();
        if cfg!(target_os = "windows") {
            args.push("/C");
            args.push("az");
        }
        args.push("account");
        args.push("get-access-token");
        args.push("--output");
        args.push("json");
        args.push("--resource");
        args.push(resource);

        if let Some(ref tenant_id) = self.options.tenant_id {
            args.push("--tenant");
            args.push(tenant_id);
        }
        if let Some(ref subscription) = self.options.subscription {
            args.push("--subscription");
            args.push(subscription);
        }

        trace!(
            "fetching credential via Azure CLI: {program} {}",
            args.join(" "),
        );

        let args = args.iter().map(|arg| arg.as_ref()).collect::<Vec<&OsStr>>();

        let status = self
            .options
            .executor
            .as_ref()
            // It's okay to call unwrap() here because new() ensures it's initialized.
            .unwrap()
            .run(OsStr::new(program), &args)
            .await;
        match status {
            Ok(az_output) if az_output.status.success() => {
                let output = str::from_utf8(&az_output.stdout)?;

                let access_token = from_json(output)?;
                Ok(access_token)
            }
            Ok(az_output) => {
                let output = String::from_utf8_lossy(&az_output.stderr);
                Err(Error::with_message(ErrorKind::Credential, || {
                    format!("'az account get-access-token' command failed: {output}")
                }))
            }
            Err(e) => match e.kind() {
                std::io::ErrorKind::NotFound => {
                    Err(Error::message(ErrorKind::Other, "Azure CLI not installed"))
                }
                error_kind => Err(Error::with_message(ErrorKind::Other, || {
                    format!("Unknown error of kind: {error_kind:?}")
                })),
            },
        }
    }

    async fn get_token(&self, scopes: &[&str]) -> azure_core::Result<AccessToken> {
        let tr = self.get_access_token(scopes).await?;
        let expires_on = tr.expires_on()?;
        Ok(AccessToken::new(tr.access_token, expires_on))
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

    #[test]
    fn read_cli_token_response() -> azure_core::Result<()> {
        let json = br#"
        {
            "accessToken": "MuchLonger_NotTheRealOne_Sv8Orn0Wq0OaXuQEg",
            "expiresOn": "2024-01-01 19:23:16.000000",
            "expires_on": 1704158596,
            "subscription": "33b83be5-faf7-42ea-a712-320a5f9dd111",
            "tenant": "065e9f5e-870d-4ed1-af2b-1b58092353f3",
            "tokenType": "Bearer"
        }
        "#;
        let token_response: CliTokenResponse = from_json(json)?;
        assert_eq!(token_response.expires_on, Some(1704158596));
        assert_eq!(token_response.expires_on()?.unix_timestamp(), 1704158596);
        Ok(())
    }
}

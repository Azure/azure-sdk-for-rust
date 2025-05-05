// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use crate::{
    credentials::{cache::TokenCache, TokenCredentialOptions},
    env::Env,
    process_ext::{ExecutorExt, OutputProcessor},
    validate_scope, validate_subscription, validate_tenant_id,
};
use azure_core::{
    credentials::{AccessToken, Secret, TokenCredential},
    error::{Error, ErrorKind, ResultExt},
    json::from_json,
    process::{new_executor, Executor},
};
use serde::Deserialize;
use std::{fmt::Debug, marker::PhantomData, str, sync::Arc};
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

        // unwrap() is safe because new() ensures the values are Some
        self.options
            .executor
            .as_ref()
            .unwrap()
            .shell_exec(
                self.options.env.as_ref().unwrap(),
                &command,
                PhantomData::<CliTokenResponse>,
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

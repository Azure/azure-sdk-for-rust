// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use crate::{credentials::cache::TokenCache, federated_credentials_flow, TokenCredentialOptions};
use azure_core::{
    credentials::{AccessToken, TokenCredential},
    error::{ErrorKind, ResultExt},
};
use std::{fmt::Debug, str, sync::Arc, time::Duration};
use time::OffsetDateTime;

const AZURE_TENANT_ID_ENV_KEY: &str = "AZURE_TENANT_ID";
const AZURE_CLIENT_ID_ENV_KEY: &str = "AZURE_CLIENT_ID";

/// Enables authentication of a Microsoft Entra service principal using a signed client assertion.
#[derive(Debug)]
pub struct ClientAssertionCredential<C> {
    tenant_id: String,
    client_id: String,
    assertion: C,
    cache: TokenCache,
    options: ClientAssertionCredentialOptions,
}

/// Options for constructing a new [`ClientAssertionCredential`].
#[derive(Debug, Default)]
pub struct ClientAssertionCredentialOptions {
    /// Additional tenants for which the credential may acquire tokens.
    ///
    /// Add the wildcard value "*" to allow the credential to acquire tokens for any tenant in which the application is registered.
    pub additionally_allowed_tenants: Vec<String>,

    /// Should be set true only by applications authenticating in disconnected clouds, or private clouds such as Azure Stack.
    ///
    /// It determines whether the credential requests Microsoft Entra instance metadata
    /// from <https://login.microsoft.com> before authenticating. Setting this to true will skip this request, making
    /// the application responsible for ensuring the configured authority is valid and trustworthy.
    pub disable_instance_discovery: bool,

    /// Options for constructing credentials.
    pub credential_options: TokenCredentialOptions,
}

#[cfg_attr(target_arch = "wasm32", async_trait::async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait::async_trait)]
/// Represents an entity capable of supplying a client assertion.
pub trait ClientAssertion: Send + Sync + Debug {
    /// Supply the client assertion secret.
    async fn secret(&self) -> azure_core::Result<String>;
}

impl<C: ClientAssertion> ClientAssertionCredential<C> {
    /// Create a new `ClientAssertionCredential`.
    pub fn new(
        tenant_id: String,
        client_id: String,
        assertion: C,
        options: Option<ClientAssertionCredentialOptions>,
    ) -> azure_core::Result<Arc<Self>> {
        Ok(Arc::new(Self::new_exclusive(
            tenant_id, client_id, assertion, options,
        )?))
    }

    /// Create a new `ClientAssertionCredential` without wrapping it in an
    /// `Arc`. Intended for use by other credentials in the crate that will
    /// themselves be protected by an `Arc`.
    pub(crate) fn new_exclusive(
        tenant_id: String,
        client_id: String,
        assertion: C,
        options: Option<ClientAssertionCredentialOptions>,
    ) -> azure_core::Result<Self> {
        Ok(Self {
            tenant_id,
            client_id,
            assertion,
            cache: TokenCache::new(),
            options: options.unwrap_or_default(),
        })
    }

    /// Create a new `ClientAssertionCredential` from environment variables.
    ///
    /// # Variables
    ///
    /// * `AZURE_TENANT_ID`
    /// * `AZURE_CLIENT_ID`
    pub fn from_env(
        assertion: C,
        options: Option<ClientAssertionCredentialOptions>,
    ) -> azure_core::Result<Arc<Self>> {
        Ok(Arc::new(Self::from_env_exclusive(assertion, options)?))
    }

    /// Create a new `ClientAssertionCredential` from environment variables,
    /// without wrapping it in an `Arc`. Intended for use by other credentials
    /// in the crate that will themselves be protected by an `Arc`.
    ///
    /// # Variables
    ///
    /// * `AZURE_TENANT_ID`
    /// * `AZURE_CLIENT_ID`
    pub(crate) fn from_env_exclusive(
        assertion: C,
        options: Option<ClientAssertionCredentialOptions>,
    ) -> azure_core::Result<Self> {
        let options = options.unwrap_or_default();
        let env = options.credential_options.env();
        let tenant_id =
            env.var(AZURE_TENANT_ID_ENV_KEY)
                .with_context(ErrorKind::Credential, || {
                    format!(
                        "working identity credential requires {} environment variable",
                        AZURE_TENANT_ID_ENV_KEY
                    )
                })?;
        let client_id =
            env.var(AZURE_CLIENT_ID_ENV_KEY)
                .with_context(ErrorKind::Credential, || {
                    format!(
                        "working identity credential requires {} environment variable",
                        AZURE_CLIENT_ID_ENV_KEY
                    )
                })?;

        ClientAssertionCredential::new_exclusive(tenant_id, client_id, assertion, Some(options))
    }

    async fn get_token(&self, scopes: &[&str]) -> azure_core::Result<AccessToken> {
        let token = self.assertion.secret().await?;
        let credential_options = &self.options.credential_options;
        let res: AccessToken = federated_credentials_flow::authorize(
            credential_options.http_client().clone(),
            &self.client_id,
            &token,
            scopes,
            &self.tenant_id,
            &credential_options.authority_host()?,
        )
        .await
        .map(|r| {
            AccessToken::new(
                r.access_token().clone(),
                OffsetDateTime::now_utc() + Duration::from_secs(r.expires_in),
            )
        })
        .context(ErrorKind::Credential, "request token error")?;
        Ok(res)
    }
}

#[cfg_attr(target_arch = "wasm32", async_trait::async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait::async_trait)]
impl<C: ClientAssertion> TokenCredential for ClientAssertionCredential<C> {
    async fn get_token(&self, scopes: &[&str]) -> azure_core::Result<AccessToken> {
        self.cache.get_token(scopes, self.get_token(scopes)).await
    }

    async fn clear_cache(&self) -> azure_core::Result<()> {
        self.cache.clear().await
    }
}

// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use crate::{credentials::cache::TokenCache, federated_credentials_flow, TokenCredentialOptions};
use azure_core::{
    credentials::{AccessToken, TokenCredential},
    error::{ErrorKind, ResultExt},
};
use std::{fmt::Debug, str, sync::Arc, time::Duration};
use time::OffsetDateTime;

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
}

#[cfg(test)]
pub(crate) mod tests {
    use std::collections::HashMap;

    use super::*;
    use crate::tests::*;
    use azure_core::{
        authority_hosts::AZURE_PUBLIC_CLOUD,
        http::{
            headers::{self, content_type},
            Body, Method, Request,
        },
    };
    use url::form_urlencoded;

    pub const FAKE_ASSERTION: &str = "fake assertion";

    pub fn is_valid_request() -> impl Fn(&Request) -> azure_core::Result<()> {
        let expected_url = format!(
            "{}{}/oauth2/v2.0/token",
            AZURE_PUBLIC_CLOUD.as_str(),
            FAKE_TENANT_ID
        );
        move |req: &Request| {
            assert_eq!(&Method::Post, req.method());
            assert_eq!(expected_url, req.url().to_string());
            assert_eq!(
                content_type::APPLICATION_X_WWW_FORM_URLENCODED.as_str(),
                req.headers().get_str(&headers::CONTENT_TYPE).unwrap()
            );
            let expected_params = [
                ("client_assertion", FAKE_ASSERTION),
                (
                    "client_assertion_type",
                    "urn:ietf:params:oauth:client-assertion-type:jwt-bearer",
                ),
                ("client_id", FAKE_CLIENT_ID),
                ("grant_type", "client_credentials"),
                ("scope", &LIVE_TEST_SCOPES.join(" ")),
            ];
            let body = match req.body() {
                Body::Bytes(bytes) => str::from_utf8(bytes).unwrap(),
                _ => panic!("unexpected body type"),
            };
            let actual_params: HashMap<String, String> = form_urlencoded::parse(body.as_bytes())
                .map(|(k, v)| (k.to_string(), v.to_string()))
                .collect();
            for (key, value) in expected_params.iter() {
                assert_eq!(
                    *value,
                    actual_params
                        .get(*key)
                        .unwrap_or_else(|| panic!("no {} in request body", key))
                );
            }
            Ok(())
        }
    }
}

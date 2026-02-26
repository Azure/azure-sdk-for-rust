// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Clients used to communicate with the service.
use crate::authorizer::KeyVaultAuthorizer;
pub use crate::generated::clients::*;
use azure_core::{
    credentials::TokenCredential,
    fmt::SafeDebug,
    http::{
        policies::{auth::BearerTokenAuthorizationPolicy, Policy},
        ClientOptions, Pipeline, Url,
    },
    tracing, Result,
};
use std::sync::Arc;

/// Options used when creating a [`KeyClient`]
#[derive(Clone, SafeDebug)]
pub struct KeyClientOptions {
    /// The API version to use for this operation.
    pub api_version: String,
    /// Allows customization of the client.
    pub client_options: ClientOptions,
    /// Controls whether the client requires the resource specified in authentication
    /// challenges to match the Key Vault or Managed HSM domain. True by default.
    pub verify_challenge_resource: Option<bool>,
}

impl Default for KeyClientOptions {
    fn default() -> Self {
        Self {
            api_version: String::from("2025-07-01"),
            client_options: ClientOptions::default(),
            verify_challenge_resource: Some(true),
        }
    }
}

impl KeyClient {
    /// Creates a new KeyClient, using Entra ID authentication.
    ///
    /// # Arguments
    ///
    /// * `endpoint` - Service host
    /// * `credential` - An implementation of [`TokenCredential`](azure_core::credentials::TokenCredential) that can provide an
    ///   Entra ID token to use when authenticating.
    /// * `options` - Optional configuration for the client.
    #[tracing::new("KeyVault")]
    pub fn new(
        endpoint: &str,
        credential: Arc<dyn TokenCredential>,
        options: Option<KeyClientOptions>,
    ) -> Result<Self> {
        let options = options.unwrap_or_default();
        let endpoint = Url::parse(endpoint)?;
        if !endpoint.scheme().starts_with("http") {
            return Err(azure_core::Error::with_message(
                azure_core::error::ErrorKind::Other,
                format!("{endpoint} must use http(s)"),
            ));
        }
        let authorizer = KeyVaultAuthorizer::new(options.verify_challenge_resource.unwrap_or(true));
        let auth_policy: Arc<dyn Policy> = Arc::new(
            BearerTokenAuthorizationPolicy::new(credential, Vec::<String>::new())
                .with_on_request(authorizer.clone())
                .with_on_challenge(authorizer),
        );
        Ok(Self {
            endpoint,
            api_version: options.api_version,
            pipeline: Pipeline::new(
                option_env!("CARGO_PKG_NAME"),
                option_env!("CARGO_PKG_VERSION"),
                options.client_options,
                Vec::new(),
                vec![auth_policy],
                None,
            ),
        })
    }
}

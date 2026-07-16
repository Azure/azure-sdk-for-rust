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
            api_version: String::from(DEFAULT_API_VERSION),
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

#[cfg(test)]
mod tests {
    use super::*;
    use azure_core::{
        http::{headers::Headers, AsyncRawResponse, ClientOptions, StatusCode, Transport},
        Bytes,
    };
    use azure_core_test::{credentials::MockCredential, http::MockHttpClient};
    use futures::{FutureExt as _, TryStreamExt as _};

    // cspell:ignore skiptoken
    const LIST_KEY_PROPERTIES_RESPONSE: &[u8] =
        br#"{"nextLink":"/keys?api-version=2025-07-01&skiptoken=page-2","value":[]}"#;

    #[tokio::test]
    async fn list_key_properties_keeps_body_for_into_model() -> Result<()> {
        let mock_client = Arc::new(MockHttpClient::new(|req| {
            assert_eq!(req.url().path(), "/keys");
            async move {
                Ok(AsyncRawResponse::from_bytes(
                    StatusCode::Ok,
                    Headers::new(),
                    Bytes::from_static(LIST_KEY_PROPERTIES_RESPONSE),
                ))
            }
            .boxed()
        }));
        let credential = MockCredential::new()?;
        let client = KeyClient::new(
            "https://example.vault.azure.net",
            credential,
            Some(KeyClientOptions {
                client_options: ClientOptions {
                    transport: Some(Transport::new(mock_client)),
                    ..Default::default()
                },
                ..Default::default()
            }),
        )?;

        let mut pager = client.list_key_properties(None)?.into_pages();
        let page = pager.try_next().await?.expect("expected a page");

        assert_eq!(
            pager.continuation().map(AsRef::as_ref),
            Some("https://example.vault.azure.net/keys?api-version=2025-07-01&skiptoken=page-2")
        );

        let page = page.into_model()?;
        assert_eq!(
            page.next_link.as_deref(),
            Some("/keys?api-version=2025-07-01&skiptoken=page-2")
        );
        assert!(page.value.is_empty());

        Ok(())
    }
}

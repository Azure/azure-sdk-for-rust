// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Clients used to communicate with Azure Blob Storage.

use azure_core::{
    credentials::TokenCredential,
    http::{
        new_http_client,
        policies::{auth::BearerTokenAuthorizationPolicy, Policy},
        ClientOptions, HttpClientOptions, Transport, Url,
    },
    Result,
};
use std::sync::Arc;

use crate::{
    logging::apply_storage_logging_defaults,
    session::{
        options::SessionOptions,
        policy::SessionAuthenticationPolicy,
        provider::{ContainerSessionProvider, SessionProvider},
    },
};

mod append_blob_client;
mod blob_client;
mod blob_container_client;
mod blob_service_client;
mod block_blob_client;
mod page_blob_client;

pub use append_blob_client::{AppendBlobClient, AppendBlobClientOptions};
pub use blob_client::{BlobClient, BlobClientOptions};
pub use blob_container_client::{BlobContainerClient, BlobContainerClientOptions};
pub use blob_service_client::{BlobServiceClient, BlobServiceClientOptions};
pub use block_blob_client::{BlockBlobClient, BlockBlobClientOptions};
pub use page_blob_client::{PageBlobClient, PageBlobClientOptions};

/// The OAuth scope used for Entra ID authentication against Storage.
const STORAGE_SCOPE: &str = "https://storage.azure.com/.default";

#[allow(clippy::needless_update)]
fn apply_client_defaults(options: &mut ClientOptions) {
    if options.transport.is_none() {
        options.transport = Some(Transport::new(new_http_client(Some(HttpClientOptions {
            automatic_decompression: false,
            ..Default::default()
        }))))
    }
    apply_storage_logging_defaults(options);
}

/// Builds the per-retry authentication policies for a client.
///
/// With no credential there are none. With a credential, the bearer token
/// policy is used, wrapped by the [`SessionAuthenticationPolicy`] when session
/// authentication is enabled so eligible downloads can use a session token.
///
/// The `client_options` and `version` are those of the constructing client,
/// used to build the session provider's own session-free service client.
//
// This is a temporary shape: session options are threaded through a dedicated
// parameter because they cannot yet be carried on the generated client options.
// TODO: fold `SessionOptions` into the generated options once the generator
// supports additional fields, and drop the `new_with_session_options` constructors.
fn build_auth_policies(
    endpoint: &Url,
    credential: Option<Arc<dyn TokenCredential>>,
    session_options: &SessionOptions,
    client_options: &ClientOptions,
    version: &str,
) -> Result<Vec<Arc<dyn Policy>>> {
    let mut per_retry_policies: Vec<Arc<dyn Policy>> = Vec::default();

    let Some(credential) = credential else {
        return Ok(per_retry_policies);
    };
    if !endpoint.scheme().starts_with("https") {
        return Err(azure_core::Error::with_message(
            azure_core::error::ErrorKind::Other,
            format!("{endpoint} must use https"),
        ));
    }

    let bearer: Arc<dyn Policy> = Arc::new(BearerTokenAuthorizationPolicy::new(
        credential.clone(),
        vec![STORAGE_SCOPE],
    ));

    if !session_options.is_enabled() {
        per_retry_policies.push(bearer);
        return Ok(per_retry_policies);
    }

    // Session signing needs an account name, resolved once here. Mirror the
    // service SDK: fail fast when explicitly enabled but unresolvable, or quietly
    // fall back to bearer when sessions are enabled only by default.
    let Some(account) = resolve_session_account(endpoint, session_options) else {
        let endpoint = endpoint_for_log(endpoint);
        if session_options.is_explicitly_enabled() {
            tracing::warn!(
                %endpoint,
                "session authentication cannot be enabled: the storage account name could not be determined"
            );
            return Err(azure_core::Error::with_message(
                azure_core::error::ErrorKind::Other,
                format!(
                    "the storage account name is required to enable session authentication \
                     but could not be determined from {endpoint}; set SessionOptions::account_name"
                ),
            ));
        }
        tracing::warn!(
            %endpoint,
            "session authentication disabled: the storage account name could not be determined"
        );
        per_retry_policies.push(bearer);
        return Ok(per_retry_policies);
    };

    let provider: Arc<dyn SessionProvider> = match &session_options.session_provider {
        Some(provider) => provider.clone(),
        None => {
            let service_options = BlobServiceClientOptions {
                client_options: client_options.clone(),
                version: version.to_string(),
            };
            let provider: Arc<dyn SessionProvider> =
                ContainerSessionProvider::new(endpoint, credential, Some(service_options))?;
            provider
        }
    };
    per_retry_policies.push(Arc::new(SessionAuthenticationPolicy::new(
        provider, bearer, account,
    )));

    Ok(per_retry_policies)
}

/// Resolves the account name used to sign session requests: the configured
/// account name, or the first label of the endpoint host.
fn resolve_session_account(endpoint: &Url, options: &SessionOptions) -> Option<String> {
    if let Some(account) = options.account_name.as_deref() {
        if !account.is_empty() {
            return Some(account.to_string());
        }
    }
    endpoint
        .host_str()?
        .split('.')
        .next()
        .filter(|label| !label.is_empty())
        .map(str::to_string)
}

/// Reduces `endpoint` to scheme, host, and path for logging, dropping any query
/// string so a SAS token is never recorded.
fn endpoint_for_log(endpoint: &Url) -> String {
    let mut endpoint = endpoint.clone();
    endpoint.set_query(None);
    endpoint.set_fragment(None);
    endpoint.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::session::options::SessionMode;
    use crate::session::provider::{SessionTokenInfo, StubSessionProvider};
    use async_trait::async_trait;
    use azure_core::{
        credentials::TokenCredential,
        http::{
            headers::{Headers, AUTHORIZATION},
            AsyncRawResponse, Context, Method, Request, StatusCode,
        },
        time::OffsetDateTime,
        Bytes,
    };
    use azure_core_test::{credentials::MockCredential, http::MockHttpClient};
    use futures::FutureExt as _;
    use std::sync::Mutex;

    const SESSION_XML: &[u8] = br#"<?xml version="1.0" encoding="utf-8"?>
<CreateSessionResult>
  <AuthenticationType>HMAC</AuthenticationType>
  <Credentials>
    <SessionKey>c2Vzc2lvbi1rZXk=</SessionKey>
    <SessionToken>token-abc</SessionToken>
  </Credentials>
  <Expiration>Wed, 01 Jan 2031 00:00:00 GMT</Expiration>
  <Id>session-id</Id>
</CreateSessionResult>"#;

    /// A terminal policy that records the Authorization header it receives.
    #[derive(Debug)]
    struct CapturingTransport {
        seen_auth: Mutex<Vec<Option<String>>>,
    }

    #[async_trait]
    impl Policy for CapturingTransport {
        async fn send(
            &self,
            _ctx: &Context,
            request: &mut Request,
            _next: &[Arc<dyn Policy>],
        ) -> azure_core::http::policies::PolicyResult {
            self.seen_auth.lock().unwrap().push(
                request
                    .headers()
                    .get_optional_str(&AUTHORIZATION)
                    .map(str::to_string),
            );
            Ok(AsyncRawResponse::from_bytes(
                StatusCode::Ok,
                Headers::new(),
                Bytes::from_static(b""),
            ))
        }
    }

    /// Client options whose transport always answers Create Session with 201.
    fn options_with_create_session_mock() -> ClientOptions {
        let mock = Arc::new(MockHttpClient::new(|_req| {
            async {
                Ok(AsyncRawResponse::from_bytes(
                    StatusCode::Created,
                    Headers::new(),
                    Bytes::from_static(SESSION_XML),
                ))
            }
            .boxed()
        }));
        ClientOptions {
            transport: Some(Transport::new(mock)),
            ..Default::default()
        }
    }

    fn endpoint() -> Url {
        Url::parse("https://myaccount.blob.core.windows.net/").unwrap()
    }

    fn download_request() -> Request {
        Request::new(
            Url::parse("https://myaccount.blob.core.windows.net/mycontainer/myblob").unwrap(),
            Method::Get,
        )
    }

    async fn auth_scheme_used(policy: &Arc<dyn Policy>) -> Option<String> {
        let transport = Arc::new(CapturingTransport {
            seen_auth: Mutex::new(Vec::new()),
        });
        let next: [Arc<dyn Policy>; 1] = [transport.clone()];
        let mut request = download_request();
        policy
            .send(&Context::new(), &mut request, &next)
            .await
            .unwrap();
        let auth = transport
            .seen_auth
            .lock()
            .unwrap()
            .last()
            .cloned()
            .flatten();
        auth
    }

    #[test]
    fn no_credential_yields_no_auth_policy() {
        let policies = build_auth_policies(
            &endpoint(),
            None,
            &SessionOptions::default(),
            &ClientOptions::default(),
            "2026-02-06",
        )
        .unwrap();
        assert!(policies.is_empty());
    }

    #[tokio::test]
    async fn disabled_mode_wires_bearer_only() {
        let credential: Arc<dyn TokenCredential> = MockCredential::new().unwrap();
        let session_options = SessionOptions {
            mode: SessionMode::Disabled,
            account_name: None,
            ..Default::default()
        };

        let policies = build_auth_policies(
            &endpoint(),
            Some(credential),
            &session_options,
            &ClientOptions::default(),
            "2026-02-06",
        )
        .unwrap();

        assert_eq!(policies.len(), 1);
        let auth = auth_scheme_used(&policies[0]).await.unwrap();
        assert!(auth.starts_with("Bearer "), "got: {auth}");
    }

    #[tokio::test]
    async fn enabled_mode_wires_session_auth() {
        let credential: Arc<dyn TokenCredential> = MockCredential::new().unwrap();
        let session_options = SessionOptions {
            mode: SessionMode::Enabled,
            account_name: Some("myaccount".into()),
            ..Default::default()
        };

        let policies = build_auth_policies(
            &endpoint(),
            Some(credential),
            &session_options,
            &options_with_create_session_mock(),
            "2026-02-06",
        )
        .unwrap();

        assert_eq!(policies.len(), 1);
        let auth = auth_scheme_used(&policies[0]).await.unwrap();
        assert!(auth.starts_with("Session token-abc:"), "got: {auth}");
    }

    #[test]
    fn resolve_session_account_prefers_configured_name() {
        let options = SessionOptions {
            mode: SessionMode::Enabled,
            account_name: Some("configured".into()),
            ..Default::default()
        };
        assert_eq!(
            resolve_session_account(&endpoint(), &options).as_deref(),
            Some("configured")
        );
    }

    #[test]
    fn resolve_session_account_falls_back_to_host_label() {
        let options = SessionOptions {
            mode: SessionMode::Enabled,
            account_name: None,
            ..Default::default()
        };
        assert_eq!(
            resolve_session_account(&endpoint(), &options).as_deref(),
            Some("myaccount")
        );
    }

    #[test]
    fn resolve_session_account_ignores_empty_configured_name() {
        let options = SessionOptions {
            mode: SessionMode::Enabled,
            account_name: Some(String::new()),
            ..Default::default()
        };
        assert_eq!(
            resolve_session_account(&endpoint(), &options).as_deref(),
            Some("myaccount")
        );
    }

    #[test]
    fn endpoint_for_log_strips_query_and_fragment() {
        let url =
            Url::parse("https://myaccount.blob.core.windows.net/c/b?sig=secret#frag").unwrap();
        assert_eq!(
            endpoint_for_log(&url),
            "https://myaccount.blob.core.windows.net/c/b"
        );
    }

    #[tokio::test]
    async fn injected_provider_is_used() {
        let provider = Arc::new(StubSessionProvider::new(
            SessionTokenInfo::for_test(
                "injected-token",
                "c2Vzc2lvbi1rZXk=",
                OffsetDateTime::from_unix_timestamp(4_000_000_000).unwrap(),
            ),
            true,
        ));
        let credential: Arc<dyn TokenCredential> = MockCredential::new().unwrap();
        let session_options = SessionOptions {
            mode: SessionMode::Enabled,
            account_name: Some("myaccount".into()),
            session_provider: Some(provider.clone()),
        };

        let policies = build_auth_policies(
            &endpoint(),
            Some(credential),
            &session_options,
            &ClientOptions::default(),
            "2026-02-06",
        )
        .unwrap();

        assert_eq!(policies.len(), 1);
        let auth = auth_scheme_used(&policies[0]).await.unwrap();
        assert!(
            provider.get_calls() >= 1,
            "the injected provider was not consulted"
        );
        assert!(auth.starts_with("Session injected-token:"), "got: {auth}");
    }
}

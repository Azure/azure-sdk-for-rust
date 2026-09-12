// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Clients used to communicate with Azure Blob Storage.

use azure_core::{
    credentials::TokenCredential,
    http::{
        new_http_client,
        policies::{auth::BearerTokenAuthorizationPolicy, Policy},
        ClientOptions, HttpClientOptions, Pipeline, Transport, Url,
    },
    Result,
};
use std::{
    net::{Ipv4Addr, Ipv6Addr},
    sync::Arc,
};

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

/// The shape shared by every generated `*ClientOptions` type, which have no common type of their own.
trait GeneratedClientOptions {
    fn client_options(&self) -> &ClientOptions;
    fn version(&self) -> &str;
}

macro_rules! impl_generated_client_options {
    ($($t:ty),+ $(,)?) => {
        $(impl GeneratedClientOptions for $t {
            fn client_options(&self) -> &ClientOptions {
                &self.client_options
            }

            fn version(&self) -> &str {
                &self.version
            }
        })+
    };
}

impl_generated_client_options!(
    AppendBlobClientOptions,
    BlobClientOptions,
    BlobContainerClientOptions,
    BlobServiceClientOptions,
    BlockBlobClientOptions,
    PageBlobClientOptions,
);

/// Returns a copy of `options` with Storage defaults applied: a transport that does
/// not transparently decompress blob content, and the Storage logging allow lists.
fn with_client_defaults(options: &ClientOptions) -> ClientOptions {
    let mut options = options.clone();
    if options.transport.is_none() {
        options.transport = Some(Transport::new(new_http_client(Some(HttpClientOptions {
            automatic_decompression: false,
        }))));
    }
    apply_storage_logging_defaults(&mut options);
    options
}

/// Builds a client pipeline, sharing the configured transport with the session
/// provider so session creation and authenticated requests use the same network context.
fn build_pipeline(
    endpoint: &Url,
    credential: Option<Arc<dyn TokenCredential>>,
    session_options: Option<&SessionOptions>,
    options: &impl GeneratedClientOptions,
) -> Result<Pipeline> {
    let client_options = with_client_defaults(options.client_options());
    // The session provider clones these, so it must get the defaulted copy rather than the raw options, or it ends up on a different transport than this pipeline.
    let per_retry_policies = build_auth_policies(
        endpoint,
        credential,
        session_options,
        &client_options,
        options.version(),
    )?;

    Ok(Pipeline::new(
        option_env!("CARGO_PKG_NAME"),
        option_env!("CARGO_PKG_VERSION"),
        client_options,
        Vec::default(),
        per_retry_policies,
        None,
    ))
}

/// Builds the per-retry authentication policies for a client.
///
/// The resulting policy path depends on the supplied credential and session
/// configuration:
///
/// - Without a credential, no authentication policy is added.
/// - With a credential and sessions disabled, bearer authentication is used.
/// - With a credential and sessions enabled, session authentication wraps the
///   bearer policy so eligible downloads use session credentials and other
///   requests can continue using bearer authentication.
///
/// When session authentication is enabled without an explicit provider,
/// `client_options` and `version` are used to construct a session-free service
/// client that acquires and refreshes session credentials.
fn build_auth_policies(
    endpoint: &Url,
    credential: Option<Arc<dyn TokenCredential>>,
    session_options: Option<&SessionOptions>,
    client_options: &ClientOptions,
    version: &str,
) -> Result<Vec<Arc<dyn Policy>>> {
    let mut per_retry_policies: Vec<Arc<dyn Policy>> = Vec::default();

    // Anonymous and SAS-based clients do not need an authentication policy.
    let Some(credential) = credential else {
        return Ok(per_retry_policies);
    };

    if !endpoint.scheme().starts_with("https") {
        return Err(azure_core::Error::with_message(
            azure_core::error::ErrorKind::Other,
            format!("{endpoint} must use HTTPS."),
        ));
    }

    let bearer: Arc<dyn Policy> = Arc::new(BearerTokenAuthorizationPolicy::new(
        credential.clone(),
        vec![STORAGE_SCOPE],
    ));

    // Use the existing bearer-only path when sessions are disabled.
    let Some(session_options) = session_options.filter(|options| options.is_enabled()) else {
        per_retry_policies.push(bearer);
        return Ok(per_retry_policies);
    };

    // Session signing requires a storage account name. If sessions were explicitly
    // enabled, fail when it cannot be resolved; otherwise, preserve bearer authentication.
    let Some(account) = resolve_session_account(endpoint, session_options) else {
        let endpoint = endpoint_for_logging(endpoint);
        if session_options.is_explicitly_enabled() {
            return Err(azure_core::Error::with_message(
                azure_core::error::ErrorKind::Other,
                format!(
                    "Session authentication requires a storage account name, but one could not \
                     be determined from {endpoint}. Set `SessionOptions::account_name`."
                ),
            ));
        }
        tracing::warn!(
            %endpoint,
            "Session authentication is unavailable because the storage account name could not be determined. Falling back to bearer authentication."
        );
        per_retry_policies.push(bearer);
        return Ok(per_retry_policies);
    };

    // Reuse an injected provider, or create a session-free client to acquire sessions.
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
/// account name, or one derived from the endpoint.
fn resolve_session_account(endpoint: &Url, options: &SessionOptions) -> Option<String> {
    if let Some(account) = options.account_name.as_deref() {
        if !account.is_empty() {
            return Some(account.to_string());
        }
    }
    let host = endpoint.host_str()?;
    if host_is_ip_literal(host) {
        return endpoint
            .path_segments()?
            .find(|segment| !segment.is_empty())
            .map(str::to_string);
    }
    host.split('.')
        .next()
        .filter(|label| !label.is_empty())
        .map(str::to_string)
}

/// Whether `host` (as returned by [`Url::host_str`]) is an IPv4 or bracketed
/// IPv6 literal, which denotes a path-style endpoint such as the local emulator.
fn host_is_ip_literal(host: &str) -> bool {
    host.parse::<Ipv4Addr>().is_ok()
        || host
            .strip_prefix('[')
            .and_then(|inner| inner.strip_suffix(']'))
            .is_some_and(|inner| inner.parse::<Ipv6Addr>().is_ok())
}

/// Reduces `endpoint` to scheme, host, and path for logging, dropping any query
/// string so a SAS token is never recorded.
fn endpoint_for_logging(endpoint: &Url) -> String {
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
            Some(&SessionOptions::default()),
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
            Some(&session_options),
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
            Some(&session_options),
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
    fn resolve_session_account_uses_path_segment_for_ip_host() {
        let options = SessionOptions {
            mode: SessionMode::Enabled,
            account_name: None,
            ..Default::default()
        };
        let endpoint = Url::parse("https://127.0.0.1:10000/devstoreaccount1/c/b").unwrap();
        assert_eq!(
            resolve_session_account(&endpoint, &options).as_deref(),
            Some("devstoreaccount1")
        );
    }

    #[test]
    fn resolve_session_account_uses_path_segment_for_ipv6_host() {
        let options = SessionOptions {
            mode: SessionMode::Enabled,
            account_name: None,
            ..Default::default()
        };
        let endpoint = Url::parse("https://[::1]:10000/devstoreaccount1/c/b").unwrap();
        assert_eq!(
            resolve_session_account(&endpoint, &options).as_deref(),
            Some("devstoreaccount1")
        );
    }

    #[test]
    fn resolve_session_account_configured_name_wins_over_ip_host() {
        let options = SessionOptions {
            mode: SessionMode::Enabled,
            account_name: Some("explicit".into()),
            ..Default::default()
        };
        let endpoint = Url::parse("https://127.0.0.1:10000/devstoreaccount1").unwrap();
        assert_eq!(
            resolve_session_account(&endpoint, &options).as_deref(),
            Some("explicit")
        );
    }

    #[test]
    fn resolve_session_account_none_for_ip_host_without_path() {
        let options = SessionOptions {
            mode: SessionMode::Enabled,
            account_name: None,
            ..Default::default()
        };
        let endpoint = Url::parse("https://127.0.0.1/").unwrap();
        assert_eq!(resolve_session_account(&endpoint, &options), None);
    }

    #[test]
    fn endpoint_for_logging_strips_query_and_fragment() {
        let url =
            Url::parse("https://myaccount.blob.core.windows.net/c/b?sig=secret#frag").unwrap();
        assert_eq!(
            endpoint_for_logging(&url),
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
            Some(&session_options),
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

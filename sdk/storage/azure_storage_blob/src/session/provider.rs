// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Session providers: mint and cache session tokens for eligible requests.
//!
//! [`ContainerSessionProvider`] owns a per-container [`AutoRefreshingCache`] of
//! sessions and mints new ones by calling the container-level Create Session API
//! on a session-free [`BlobServiceClient`]. Fallback-eligible failures (`5xx`,
//! `403`, or `400 FeatureNotEnabled`) are converted into a fallback-to-bearer
//! sentinel that is cached for a cooldown period so the service is not stormed.

use crate::{
    models::{AuthenticationType, CreateSessionConfiguration, CreateSessionResponse},
    session::cache::{AcquireFn, AutoRefreshingCache, ExpiringValue},
    BlobServiceClient, BlobServiceClientOptions,
};
use async_trait::async_trait;
use azure_core::{
    credentials::TokenCredential,
    error::ErrorKind,
    fmt::SafeDebug,
    http::{headers::HeaderName, Method, Request, StatusCode, Url},
    time::{Duration, OffsetDateTime},
    Error, Result,
};
use std::{
    collections::HashMap,
    fmt,
    sync::{Arc, Mutex},
};

#[cfg(test)]
use std::sync::atomic::{AtomicUsize, Ordering};

/// How long before expiry a proactive background refresh is started.
const REFRESH_BUFFER: Duration = Duration::seconds(30);
/// Maximum time a background refresh may run before the current value is kept.
const BACKGROUND_ACQUIRE_TIMEOUT: Duration = Duration::seconds(30);
/// How long a fallback-to-bearer sentinel is cached after a fallback-eligible failure.
const FALLBACK_COOLDOWN: Duration = Duration::minutes(5);
/// Storage error code indicating the session feature is not enabled for the account.
const FEATURE_NOT_ENABLED: &str = "FeatureNotEnabled";

/// A cached session, or a sentinel indicating that callers should fall back to
/// bearer authentication for the duration of the cooldown.
///
/// This type is opaque: it is returned and consumed by the sealed
/// [`SessionProvider`] trait and cannot be constructed or inspected by callers.
#[derive(Clone, SafeDebug)]
pub struct SessionTokenInfo {
    session_token: Option<String>,
    session_key: Option<String>,
    expires_on: OffsetDateTime,
    refresh_on: OffsetDateTime,
    is_fallback_to_bearer: bool,
}

impl SessionTokenInfo {
    /// Builds a session from a successful Create Session response.
    fn from_response(response: CreateSessionResponse, refresh_buffer: Duration) -> Result<Self> {
        let missing = |field: &'static str| {
            Error::with_message(
                ErrorKind::DataConversion,
                format!("create session response is missing required field: {field}"),
            )
        };

        let credentials = response.credentials.ok_or_else(|| missing("Credentials"))?;
        let session_token = credentials
            .session_token
            .ok_or_else(|| missing("SessionToken"))?;
        let session_key = credentials
            .session_key
            .ok_or_else(|| missing("SessionKey"))?;
        let expires_on = response.expiration.ok_or_else(|| missing("Expiration"))?;

        Ok(Self {
            session_token: Some(session_token),
            session_key: Some(session_key),
            expires_on,
            refresh_on: expires_on - refresh_buffer,
            is_fallback_to_bearer: false,
        })
    }

    /// Builds a fallback-to-bearer sentinel that expires after `cooldown`. It
    /// carries no refresh buffer, so it is honored for the full cooldown with a
    /// single foreground re-acquire at expiry.
    fn fallback_to_bearer(cooldown: Duration, now: OffsetDateTime) -> Self {
        let expires_on = now + cooldown;
        Self {
            session_token: None,
            session_key: None,
            expires_on,
            refresh_on: expires_on,
            is_fallback_to_bearer: true,
        }
    }

    /// Whether callers should fall back to bearer authentication.
    pub(crate) fn is_fallback_to_bearer(&self) -> bool {
        self.is_fallback_to_bearer
    }

    /// The session token and key to sign a request with, or `None` when this is
    /// a fallback-to-bearer sentinel.
    pub(crate) fn credentials(&self) -> Option<(&str, &str)> {
        match (&self.session_token, &self.session_key) {
            (Some(token), Some(key)) if !self.is_fallback_to_bearer => Some((token, key)),
            _ => None,
        }
    }
}

#[cfg(test)]
impl SessionTokenInfo {
    /// Builds a usable session for tests.
    pub(crate) fn for_test(token: &str, key: &str, expires_on: OffsetDateTime) -> Self {
        Self {
            session_token: Some(token.into()),
            session_key: Some(key.into()),
            expires_on,
            refresh_on: expires_on,
            is_fallback_to_bearer: false,
        }
    }

    /// Builds a fallback-to-bearer sentinel for tests.
    pub(crate) fn fallback_for_test(expires_on: OffsetDateTime) -> Self {
        Self::fallback_to_bearer(Duration::seconds(0), expires_on)
    }
}

impl ExpiringValue for SessionTokenInfo {
    fn refresh_on(&self) -> OffsetDateTime {
        self.refresh_on
    }
    fn expires_on(&self) -> OffsetDateTime {
        self.expires_on
    }
    fn replaces_current(&self) -> bool {
        !self.is_fallback_to_bearer
    }
    fn with_refresh_on(&self, refresh_on: OffsetDateTime) -> Self {
        Self {
            refresh_on,
            ..self.clone()
        }
    }
}

/// Two sessions are equal when they carry the same token, so the cache can tell
/// whether a value it holds is still the one a caller used.
impl PartialEq for SessionTokenInfo {
    fn eq(&self, other: &Self) -> bool {
        self.session_token == other.session_token
    }
}

/// Seals [`SessionProvider`]; only types defined in this module can implement it.
mod sealed {
    pub trait Sealed {}
}

/// Provides and caches session tokens used to authenticate eligible blob download requests.
///
/// This trait is sealed and cannot be implemented outside this crate. Construct a
/// provider with [`ContainerSessionProvider::new`] and assign it to
/// [`SessionOptions::session_provider`](crate::SessionOptions) to share one
/// session cache across multiple clients.
#[async_trait]
pub trait SessionProvider: sealed::Sealed + fmt::Debug + Send + Sync {
    /// Returns a cached session for `request`, acquiring one on first access.
    async fn get_session(&self, request: &Request) -> Result<SessionTokenInfo>;

    /// Invalidates the cached session for `request`, but only if it still holds
    /// `current`, so a newer concurrently-refreshed session is not clobbered.
    async fn invalidate_session(&self, request: &Request, current: &SessionTokenInfo);

    /// Whether `request` is eligible for session-token authentication.
    fn is_request_eligible(&self, request: &Request) -> bool;
}

/// A [`SessionProvider`] that mints sessions with a [`TokenCredential`] and
/// caches them per container.
pub struct ContainerSessionProvider {
    service_client: Arc<BlobServiceClient>,
    refresh_buffer: Duration,
    caches: Mutex<HashMap<String, AutoRefreshingCache<SessionTokenInfo>>>,
}

impl fmt::Debug for ContainerSessionProvider {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ContainerSessionProvider")
            .finish_non_exhaustive()
    }
}

impl ContainerSessionProvider {
    /// Creates a provider that mints and caches per-container session tokens.
    ///
    /// Sessions are minted against the blob service endpoint derived from
    /// `service_url` (container, blob, and query stripped). Assign the returned
    /// provider to [`SessionOptions::session_provider`](crate::SessionOptions) to
    /// share one session cache across multiple clients.
    ///
    /// # Arguments
    ///
    /// * `service_url` - A URL for the target account, for example `https://myaccount.blob.core.windows.net/`.
    /// * `credential` - A [`TokenCredential`] used to mint sessions.
    /// * `options` - Optional configuration applied to the provider's own service client.
    pub fn new(
        service_url: &Url,
        credential: Arc<dyn TokenCredential>,
        options: Option<BlobServiceClientOptions>,
    ) -> Result<Arc<Self>> {
        let endpoint = service_endpoint(service_url);
        let service_client = Arc::new(BlobServiceClient::new(endpoint, Some(credential), options)?);
        Ok(Arc::new(Self {
            service_client,
            refresh_buffer: REFRESH_BUFFER,
            caches: Mutex::new(HashMap::new()),
        }))
    }

    /// Returns the per-container cache, creating it on first access.
    fn cache_for(&self, container: &str) -> AutoRefreshingCache<SessionTokenInfo> {
        let mut caches = self.caches.lock().unwrap();
        if let Some(cache) = caches.get(container) {
            return cache.clone();
        }
        let cache = self.build_cache(container);
        caches.insert(container.to_string(), cache.clone());
        cache
    }

    fn build_cache(&self, container: &str) -> AutoRefreshingCache<SessionTokenInfo> {
        let service_client = self.service_client.clone();
        let container = container.to_string();
        let refresh_buffer = self.refresh_buffer;
        let acquire: AcquireFn<SessionTokenInfo> = Arc::new(move || {
            let service_client = service_client.clone();
            let container = container.clone();
            Box::pin(
                async move { acquire_session(&service_client, &container, refresh_buffer).await },
            )
        });
        AutoRefreshingCache::new(acquire, BACKGROUND_ACQUIRE_TIMEOUT)
    }
}

#[async_trait]
impl SessionProvider for ContainerSessionProvider {
    async fn get_session(&self, request: &Request) -> Result<SessionTokenInfo> {
        let container = container_name(request.url()).ok_or_else(|| {
            Error::with_message(
                ErrorKind::Other,
                "could not determine the container name from the request URL",
            )
        })?;
        self.cache_for(&container).get().await
    }

    async fn invalidate_session(&self, request: &Request, current: &SessionTokenInfo) {
        if let Some(container) = container_name(request.url()) {
            self.cache_for(&container)
                .invalidate_if_current(current)
                .await;
        }
    }

    fn is_request_eligible(&self, request: &Request) -> bool {
        if request.method() != Method::Get {
            return false;
        }
        // Structured-message downloads are authenticated with bearer, not sessions.
        if request
            .headers()
            .get_optional_str(&STRUCTURED_MESSAGE_HEADER)
            .is_some()
        {
            return false;
        }
        let url = request.url();
        let Some(segments) = url.path_segments() else {
            return false;
        };
        let mut segments = segments.filter(|segment| !segment.is_empty());
        let has_container = segments.next().is_some();
        let has_blob = segments.next().is_some();
        // Eligible only for blob-level GET downloads, not sub-resource operations
        // (`comp`/`restype`).
        has_container && has_blob && !has_operation_query(url)
    }
}

impl sealed::Sealed for ContainerSessionProvider {}

/// Mints a new session by calling Create Session, converting fallback-eligible
/// failures into a fallback-to-bearer sentinel.
async fn acquire_session(
    service_client: &BlobServiceClient,
    container: &str,
    refresh_buffer: Duration,
) -> Result<SessionTokenInfo> {
    let container_client = service_client.blob_container_client(container);
    let config = CreateSessionConfiguration {
        authentication_type: Some(AuthenticationType::Hmac),
    };

    let response = match container_client
        .create_session(config.try_into()?, None)
        .await
    {
        Ok(response) => response,
        Err(error) => {
            return match fallback_cooldown(&error) {
                Some(cooldown) => Ok(SessionTokenInfo::fallback_to_bearer(
                    cooldown,
                    OffsetDateTime::now_utc(),
                )),
                None => Err(error),
            };
        }
    };

    SessionTokenInfo::from_response(response.into_model()?, refresh_buffer)
}

/// Returns the cooldown for which a fallback-to-bearer is cached when a Create
/// Session failure is fallback-eligible (`5xx`, `403`, or `400 FeatureNotEnabled`).
fn fallback_cooldown(error: &Error) -> Option<Duration> {
    let status = error.http_status()?;
    let feature_not_enabled = matches!(
        error.kind(),
        ErrorKind::HttpResponse { error_code: Some(code), .. } if code == FEATURE_NOT_ENABLED
    );
    let eligible = status.is_server_error()
        || status == StatusCode::Forbidden
        || (status == StatusCode::BadRequest && feature_not_enabled);
    eligible.then_some(FALLBACK_COOLDOWN)
}

/// Reduces `url` to the account's blob service endpoint by discarding the
/// container and blob path segments and every query-string component.
fn service_endpoint(url: &Url) -> Url {
    let mut endpoint = url.clone();
    endpoint.set_path("");
    endpoint.set_query(None);
    endpoint.set_fragment(None);
    endpoint
}

/// Extracts the container name (the first non-empty path segment) from `url`.
fn container_name(url: &Url) -> Option<String> {
    url.path_segments()?
        .find(|segment| !segment.is_empty())
        .map(str::to_string)
}

/// The header set on structured-message downloads, which are not session-eligible.
const STRUCTURED_MESSAGE_HEADER: HeaderName = HeaderName::from_static("x-ms-structured-body");

/// Whether `url` carries a `comp` or `restype` query parameter, which denote a
/// sub-resource operation rather than a blob download.
fn has_operation_query(url: &Url) -> bool {
    url.query_pairs()
        .any(|(name, _)| name.eq_ignore_ascii_case("comp") || name.eq_ignore_ascii_case("restype"))
}

/// A configurable [`SessionProvider`] test double, shared by the policy and
/// client-wiring tests. Has an internal counting mechanism to be used for test assertions.
#[cfg(test)]
#[derive(Debug)]
pub(crate) struct StubSessionProvider {
    session: SessionTokenInfo,
    eligible: bool,
    get_calls: AtomicUsize,
    invalidate_calls: AtomicUsize,
}

#[cfg(test)]
impl StubSessionProvider {
    pub(crate) fn new(session: SessionTokenInfo, eligible: bool) -> Self {
        Self {
            session,
            eligible,
            get_calls: AtomicUsize::new(0),
            invalidate_calls: AtomicUsize::new(0),
        }
    }

    pub(crate) fn get_calls(&self) -> usize {
        self.get_calls.load(Ordering::SeqCst)
    }

    pub(crate) fn invalidate_calls(&self) -> usize {
        self.invalidate_calls.load(Ordering::SeqCst)
    }
}

#[cfg(test)]
#[async_trait]
impl SessionProvider for StubSessionProvider {
    async fn get_session(&self, _request: &Request) -> Result<SessionTokenInfo> {
        self.get_calls.fetch_add(1, Ordering::SeqCst);
        Ok(self.session.clone())
    }
    async fn invalidate_session(&self, _request: &Request, _current: &SessionTokenInfo) {
        self.invalidate_calls.fetch_add(1, Ordering::SeqCst);
    }
    fn is_request_eligible(&self, _request: &Request) -> bool {
        self.eligible
    }
}

#[cfg(test)]
impl sealed::Sealed for StubSessionProvider {}

#[cfg(test)]
mod tests {
    use super::*;
    use azure_core::{
        http::{
            headers::Headers, AsyncRawResponse, ClientOptions, Method, Request, StatusCode,
            Transport, Url,
        },
        Bytes,
    };
    use azure_core_test::{credentials::MockCredential, http::MockHttpClient};
    use futures::FutureExt as _;

    const SESSION_XML: &str = r#"<?xml version="1.0" encoding="utf-8"?>
<CreateSessionResult>
  <AuthenticationType>HMAC</AuthenticationType>
  <Credentials>
    <SessionKey>c2Vzc2lvbi1rZXk=</SessionKey>
    <SessionToken>token-abc</SessionToken>
  </Credentials>
  <Expiration>Wed, 01 Jan 2031 00:00:00 GMT</Expiration>
  <Id>session-id</Id>
</CreateSessionResult>"#;

    fn get_request(url: &str) -> Request {
        Request::new(Url::parse(url).unwrap(), Method::Get)
    }

    fn http_error(status: StatusCode, error_code: Option<&str>) -> Error {
        Error::from(ErrorKind::HttpResponse {
            status,
            error_code: error_code.map(str::to_string),
            raw_response: None,
        })
    }

    fn provider_returning(
        status: StatusCode,
        body: &'static [u8],
    ) -> Arc<ContainerSessionProvider> {
        let mock = Arc::new(MockHttpClient::new(move |_req| {
            async move {
                Ok(AsyncRawResponse::from_bytes(
                    status,
                    Headers::new(),
                    Bytes::from_static(body),
                ))
            }
            .boxed()
        }));
        let options = BlobServiceClientOptions {
            client_options: ClientOptions {
                transport: Some(Transport::new(mock)),
                ..Default::default()
            },
            ..Default::default()
        };
        let credential: Arc<dyn TokenCredential> = MockCredential::new().unwrap();
        ContainerSessionProvider::new(
            &Url::parse("https://myaccount.blob.core.windows.net/").unwrap(),
            credential,
            Some(options),
        )
        .unwrap()
    }

    #[test]
    fn fallback_cooldown_classification() {
        assert!(fallback_cooldown(&http_error(StatusCode::InternalServerError, None)).is_some());
        assert!(fallback_cooldown(&http_error(StatusCode::ServiceUnavailable, None)).is_some());
        assert!(fallback_cooldown(&http_error(StatusCode::Forbidden, None)).is_some());
        assert!(fallback_cooldown(&http_error(
            StatusCode::BadRequest,
            Some(FEATURE_NOT_ENABLED)
        ))
        .is_some());

        assert!(
            fallback_cooldown(&http_error(StatusCode::BadRequest, Some("OtherError"))).is_none()
        );
        assert!(fallback_cooldown(&http_error(StatusCode::NotFound, None)).is_none());
        assert!(fallback_cooldown(&http_error(StatusCode::Unauthorized, None)).is_none());
        assert!(fallback_cooldown(&Error::with_message(ErrorKind::Other, "boom")).is_none());
    }

    #[test]
    fn eligibility_matrix() {
        let provider = provider_returning(StatusCode::Created, SESSION_XML.as_bytes());

        // GET blob download is eligible.
        assert!(provider.is_request_eligible(&get_request("https://a.blob.core.windows.net/c/b")));
        // Virtual-directory blob names remain eligible.
        assert!(
            provider.is_request_eligible(&get_request("https://a.blob.core.windows.net/c/d/e/b"))
        );

        // Non-GET is ineligible.
        assert!(!provider.is_request_eligible(&Request::new(
            Url::parse("https://a.blob.core.windows.net/c/b").unwrap(),
            Method::Put,
        )));
        // Service-level and container-level requests are ineligible.
        assert!(!provider.is_request_eligible(&get_request("https://a.blob.core.windows.net/")));
        assert!(!provider.is_request_eligible(&get_request("https://a.blob.core.windows.net/c")));
        // `comp` operations are ineligible.
        assert!(!provider.is_request_eligible(&get_request(
            "https://a.blob.core.windows.net/c/b?comp=blocklist"
        )));
        // `restype` operations are ineligible.
        assert!(!provider.is_request_eligible(&get_request(
            "https://a.blob.core.windows.net/c/b?restype=container"
        )));
        // Structured-message downloads are ineligible.
        let mut structured = get_request("https://a.blob.core.windows.net/c/b");
        structured.insert_header(STRUCTURED_MESSAGE_HEADER, "XSM/1.0");
        assert!(!provider.is_request_eligible(&structured));
    }

    #[test]
    fn service_endpoint_strips_path_and_query() {
        let url =
            Url::parse("https://a.blob.core.windows.net/container/blob?comp=x&timeout=30").unwrap();
        assert_eq!(
            service_endpoint(&url).as_str(),
            "https://a.blob.core.windows.net/"
        );
    }

    #[test]
    fn container_name_is_first_segment() {
        let url = Url::parse("https://a.blob.core.windows.net/mycontainer/dir/blob").unwrap();
        assert_eq!(container_name(&url).as_deref(), Some("mycontainer"));
        assert_eq!(
            container_name(&Url::parse("https://a.blob.core.windows.net/").unwrap()),
            None
        );
    }

    #[test]
    fn session_token_info_from_response_sets_refresh_window() {
        let response = CreateSessionResponse {
            authentication_type: Some(AuthenticationType::Hmac),
            credentials: Some(crate::models::SessionCredentials {
                session_key: Some("key".into()),
                session_token: Some("token".into()),
            }),
            expiration: Some(OffsetDateTime::from_unix_timestamp(1_700_000_300).unwrap()),
            id: Some("id".into()),
        };
        let info = SessionTokenInfo::from_response(response, Duration::seconds(30)).unwrap();
        assert_eq!(info.credentials(), Some(("token", "key")));
        assert!(!info.is_fallback_to_bearer());
        assert_eq!(info.refresh_on(), info.expires_on() - Duration::seconds(30));
    }

    #[test]
    fn fallback_sentinel_has_no_credentials_or_refresh_buffer() {
        let now = OffsetDateTime::from_unix_timestamp(1_700_000_000).unwrap();
        let info = SessionTokenInfo::fallback_to_bearer(Duration::minutes(5), now);
        assert!(info.is_fallback_to_bearer());
        assert_eq!(info.credentials(), None);
        assert_eq!(info.refresh_on(), info.expires_on());
        assert_eq!(info.expires_on(), now + Duration::minutes(5));
    }

    #[tokio::test]
    async fn get_session_returns_credentials_on_success() {
        let provider = provider_returning(StatusCode::Created, SESSION_XML.as_bytes());
        let request = get_request("https://myaccount.blob.core.windows.net/mycontainer/myblob");

        let info = provider.get_session(&request).await.unwrap();
        assert_eq!(info.credentials(), Some(("token-abc", "c2Vzc2lvbi1rZXk=")));
        assert!(!info.is_fallback_to_bearer());
    }

    #[tokio::test]
    async fn get_session_falls_back_to_bearer_on_forbidden() {
        let provider = provider_returning(
            StatusCode::Forbidden,
            b"<Error><Code>AuthorizationFailure</Code></Error>",
        );
        let request = get_request("https://myaccount.blob.core.windows.net/mycontainer/myblob");

        let info = provider.get_session(&request).await.unwrap();
        assert!(info.is_fallback_to_bearer());
        assert_eq!(info.credentials(), None);
    }
}

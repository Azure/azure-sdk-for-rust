// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! The pipeline policy that selects between session token and bearer token authentication.

use crate::session::{
    options::SessionOptions,
    provider::{SessionProvider, SessionTokenInfo},
    signer,
};
use async_trait::async_trait;
use azure_core::{
    error::ErrorKind,
    http::{
        headers::{AUTHORIZATION, MS_DATE},
        policies::{Policy, PolicyResult},
        Context, Request, StatusCode, Url,
    },
    time::{to_rfc7231, OffsetDateTime},
    Error, Result,
};
use std::sync::Arc;

/// A pipeline policy that authenticates eligible blob download requests with a
/// session token, falling back to bearer token authentication otherwise.
///
/// This policy occupies the authentication slot in the pipeline and wraps the
/// bearer token policy it delegates to. When sessions are enabled and a request
/// is eligible, it signs the request with a session token; otherwise, or when a
/// session cannot be used, it delegates to the wrapped bearer token policy.
#[derive(Debug)]
pub(crate) struct SessionAuthenticationPolicy {
    provider: Arc<dyn SessionProvider>,
    fallback: Arc<dyn Policy>,
    options: SessionOptions,
}

impl SessionAuthenticationPolicy {
    /// Creates a policy that delegates to `fallback` (the bearer token policy)
    /// whenever session authentication is disabled, ineligible, or unavailable.
    pub(crate) fn new(
        provider: Arc<dyn SessionProvider>,
        fallback: Arc<dyn Policy>,
        options: SessionOptions,
    ) -> Self {
        Self {
            provider,
            fallback,
            options,
        }
    }

    /// Signs `request` with the Shared Key protocol and sets the `Session`
    /// authorization header.
    fn sign_request(&self, request: &mut Request, session: &SessionTokenInfo) -> Result<()> {
        let account = self.resolve_account(request)?;
        let (token, key) = session.credentials().ok_or_else(|| {
            Error::with_message(ErrorKind::Other, "session is missing credentials")
        })?;

        // x-ms-date participates in the string-to-sign, so set it before signing.
        request.insert_header(MS_DATE, to_rfc7231(&OffsetDateTime::now_utc()));
        let signature = signer::sign(request, &account, key)?;
        request.insert_header(AUTHORIZATION, format!("Session {token}:{signature}"));
        Ok(())
    }

    /// Resolves the account name used for signing: the configured account name,
    /// or the one derived from the request URL.
    fn resolve_account(&self, request: &Request) -> Result<String> {
        if let Some(account) = self.options.account_name.as_deref() {
            if !account.is_empty() {
                return Ok(account.to_string());
            }
        }
        account_from_url(request.url()).ok_or_else(|| {
            Error::with_message(
                ErrorKind::Other,
                "the storage account name could not be determined from the request URL; \
                 set the session account name when using a custom endpoint",
            )
        })
    }
}

#[async_trait]
impl Policy for SessionAuthenticationPolicy {
    async fn send(
        &self,
        ctx: &Context,
        request: &mut Request,
        next: &[Arc<dyn Policy>],
    ) -> PolicyResult {
        // Disabled or ineligible requests use bearer authentication.
        if !self.options.is_enabled() || !self.provider.is_request_eligible(request) {
            return self.fallback.send(ctx, request, next).await;
        }

        // A fallback-to-bearer sentinel means a session could not be acquired.
        let session = self.provider.get_session(request).await?;
        if session.is_fallback_to_bearer() {
            return self.fallback.send(ctx, request, next).await;
        }

        self.sign_request(request, &session)?;
        let response = next[0].send(ctx, request, &next[1..]).await?;

        // On 401, drop the session, then retry exactly once with bearer auth.
        if response.status() == StatusCode::Unauthorized {
            clear_session_headers(request);
            self.provider.invalidate_session(request, &session).await;
            request.body_mut().reset().await?;
            return self.fallback.send(ctx, request, next).await;
        }

        Ok(response)
    }
}

/// Removes the headers set while signing so a subsequent bearer attempt starts clean.
fn clear_session_headers(request: &mut Request) {
    request.headers_mut().remove(AUTHORIZATION);
    request.headers_mut().remove(MS_DATE);
}

/// Derives the account name from the first label of the URL host.
fn account_from_url(url: &Url) -> Option<String> {
    url.host_str()?
        .split('.')
        .next()
        .filter(|label| !label.is_empty())
        .map(str::to_string)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::session::options::SessionMode;
    use azure_core::credentials::TokenCredential;
    use azure_core::http::policies::auth::BearerTokenAuthorizationPolicy;
    use azure_core::{
        http::{headers::Headers, AsyncRawResponse, Method},
        Bytes,
    };
    use azure_core_test::credentials::MockCredential;
    use std::sync::Mutex;

    /// A terminal policy that records each request's Authorization header and
    /// returns queued responses in order.
    #[derive(Debug)]
    struct CapturingTransport {
        responses: Mutex<Vec<AsyncRawResponse>>,
        seen_auth: Mutex<Vec<Option<String>>>,
    }

    impl CapturingTransport {
        fn new(responses: Vec<AsyncRawResponse>) -> Arc<Self> {
            Arc::new(Self {
                responses: Mutex::new(responses),
                seen_auth: Mutex::new(Vec::new()),
            })
        }
    }

    #[async_trait]
    impl Policy for CapturingTransport {
        async fn send(
            &self,
            _ctx: &Context,
            request: &mut Request,
            _next: &[Arc<dyn Policy>],
        ) -> PolicyResult {
            let auth = request
                .headers()
                .get_optional_str(&AUTHORIZATION)
                .map(str::to_string);
            self.seen_auth.lock().unwrap().push(auth);
            Ok(self.responses.lock().unwrap().remove(0))
        }
    }

    /// A provider with canned behavior for exercising the policy state machine.
    #[derive(Debug)]
    struct FakeProvider {
        session: SessionTokenInfo,
        eligible: bool,
        invalidated: Mutex<u32>,
    }

    impl FakeProvider {
        fn new(session: SessionTokenInfo, eligible: bool) -> Self {
            Self {
                session,
                eligible,
                invalidated: Mutex::new(0),
            }
        }
    }

    #[async_trait]
    impl SessionProvider for FakeProvider {
        async fn get_session(&self, _request: &Request) -> Result<SessionTokenInfo> {
            Ok(self.session.clone())
        }
        async fn invalidate_session(&self, _request: &Request, _current: &SessionTokenInfo) {
            *self.invalidated.lock().unwrap() += 1;
        }
        fn is_request_eligible(&self, _request: &Request) -> bool {
            self.eligible
        }
    }

    fn bearer_policy() -> Arc<dyn Policy> {
        let credential: Arc<dyn TokenCredential> = MockCredential::new().unwrap();
        Arc::new(BearerTokenAuthorizationPolicy::new(
            credential,
            vec!["https://storage.azure.com/.default"],
        ))
    }

    fn response(status: StatusCode) -> AsyncRawResponse {
        AsyncRawResponse::from_bytes(status, Headers::new(), Bytes::from_static(b""))
    }

    fn request() -> Request {
        Request::new(
            Url::parse("https://myaccount.blob.core.windows.net/mycontainer/myblob").unwrap(),
            Method::Get,
        )
    }

    fn valid_session() -> SessionTokenInfo {
        SessionTokenInfo::for_test(
            "token-abc",
            "c2Vzc2lvbi1rZXk=",
            OffsetDateTime::from_unix_timestamp(4_000_000_000).unwrap(),
        )
    }

    fn options(mode: SessionMode) -> SessionOptions {
        SessionOptions {
            mode,
            account_name: None,
        }
    }

    async fn run(
        provider: Arc<FakeProvider>,
        options: SessionOptions,
        transport: Arc<CapturingTransport>,
    ) -> (StatusCode, Vec<Option<String>>, u32) {
        let policy = SessionAuthenticationPolicy::new(provider.clone(), bearer_policy(), options);
        let next: [Arc<dyn Policy>; 1] = [transport.clone()];
        let mut request = request();
        let status = policy
            .send(&Context::new(), &mut request, &next)
            .await
            .unwrap()
            .status();
        let seen = transport.seen_auth.lock().unwrap().clone();
        let invalidated = *provider.invalidated.lock().unwrap();
        (status, seen, invalidated)
    }

    #[tokio::test]
    async fn eligible_request_is_signed_with_session_scheme() {
        let provider = Arc::new(FakeProvider::new(valid_session(), true));
        let transport = CapturingTransport::new(vec![response(StatusCode::Ok)]);

        let (status, seen, invalidated) =
            run(provider, options(SessionMode::Enabled), transport).await;

        assert_eq!(status, StatusCode::Ok);
        assert_eq!(seen.len(), 1);
        let auth = seen[0].as_deref().unwrap();
        assert!(auth.starts_with("Session token-abc:"), "got: {auth}");
        assert_eq!(invalidated, 0);
    }

    #[tokio::test]
    async fn disabled_mode_uses_bearer() {
        let provider = Arc::new(FakeProvider::new(valid_session(), true));
        let transport = CapturingTransport::new(vec![response(StatusCode::Ok)]);

        let (_status, seen, _) = run(provider, options(SessionMode::Disabled), transport).await;

        assert!(seen[0].as_deref().unwrap().starts_with("Bearer "));
    }

    #[tokio::test]
    async fn ineligible_request_uses_bearer() {
        let provider = Arc::new(FakeProvider::new(valid_session(), false));
        let transport = CapturingTransport::new(vec![response(StatusCode::Ok)]);

        let (_status, seen, _) = run(provider, options(SessionMode::Enabled), transport).await;

        assert!(seen[0].as_deref().unwrap().starts_with("Bearer "));
    }

    #[tokio::test]
    async fn fallback_sentinel_uses_bearer() {
        let sentinel = SessionTokenInfo::fallback_for_test(
            OffsetDateTime::from_unix_timestamp(4_000_000_000).unwrap(),
        );
        let provider = Arc::new(FakeProvider::new(sentinel, true));
        let transport = CapturingTransport::new(vec![response(StatusCode::Ok)]);

        let (_status, seen, _) = run(provider, options(SessionMode::Enabled), transport).await;

        assert_eq!(seen.len(), 1);
        assert!(seen[0].as_deref().unwrap().starts_with("Bearer "));
    }

    #[tokio::test]
    async fn unauthorized_invalidates_and_retries_once_with_bearer() {
        let provider = Arc::new(FakeProvider::new(valid_session(), true));
        let transport = CapturingTransport::new(vec![
            response(StatusCode::Unauthorized),
            response(StatusCode::Ok),
        ]);

        let (status, seen, invalidated) =
            run(provider, options(SessionMode::Enabled), transport).await;

        assert_eq!(status, StatusCode::Ok);
        assert_eq!(seen.len(), 2, "expected exactly one retry");
        assert!(
            seen[0].as_deref().unwrap().starts_with("Session "),
            "first attempt should use a session token"
        );
        assert!(
            seen[1].as_deref().unwrap().starts_with("Bearer "),
            "retry should use bearer authentication"
        );
        assert_eq!(invalidated, 1);
    }
}

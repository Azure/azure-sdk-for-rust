// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use crate::{
    credentials::{AccessToken, TokenCredential, TokenRequestOptions},
    http::{
        headers::{AUTHORIZATION, WWW_AUTHENTICATE},
        policies::{Policy, PolicyResult},
    },
    Error, Result,
};
use async_lock::RwLock;
use async_trait::async_trait;
use std::sync::Arc;
use typespec::{
    error::ErrorKind,
    http::{headers::Headers, StatusCode},
};
#[cfg(not(target_arch = "wasm32"))]
use typespec_client_core::http::Body::SeekableStream;
use typespec_client_core::http::{ClientMethodOptions, Context, Request};
use typespec_client_core::time::{Duration, OffsetDateTime};

/// Authentication policy for a bearer token.
#[derive(Debug, Clone)]
pub struct BearerTokenAuthorizationPolicy {
    authorizer: Arc<BearerTokenAuthorizer>,
    on_request: Arc<dyn OnRequest>,
    on_challenge: Option<Arc<dyn OnChallenge>>,
}

impl BearerTokenAuthorizationPolicy {
    /// Creates a new `BearerTokenAuthorizationPolicy`.
    pub fn new<A, B>(credential: Arc<dyn TokenCredential>, scopes: A) -> Self
    where
        A: IntoIterator<Item = B>,
        B: Into<String>,
    {
        let scopes: Vec<String> = scopes.into_iter().map(|s| s.into()).collect();
        Self {
            authorizer: Arc::new(BearerTokenAuthorizer::new(credential)),
            on_request: Arc::new(DefaultOnRequest { scopes }),
            on_challenge: None,
        }
    }

    /// Sets a callback for `send` to invoke once on each request it receives, before sending the request.
    ///
    /// See [`OnRequest`] for more details. When not set, the policy authorizes each request using the credential
    /// and scopes specified to `new`.
    pub fn with_on_request(mut self, on_request: Arc<dyn OnRequest>) -> Self {
        self.on_request = on_request;
        self
    }

    /// Sets a callback to invoke upon receiving a 401 Unauthorized response with an authentication challenge.
    ///
    /// See [`OnChallenge`] for more details. When not set, `send` returns 401 responses without attempting to
    /// handle their challenges.
    pub fn with_on_challenge(mut self, on_challenge: Arc<dyn OnChallenge>) -> Self {
        self.on_challenge = Some(on_challenge);
        self
    }
}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
impl Policy for BearerTokenAuthorizationPolicy {
    async fn send(
        &self,
        ctx: &Context,
        request: &mut Request,
        next: &[Arc<dyn Policy>],
    ) -> PolicyResult {
        self.on_request
            .on_request(ctx, request, self.authorizer.as_ref())
            .await?;

        let mut response = next[0].send(ctx, request, &next[1..]).await?;

        if response.status() == StatusCode::Unauthorized {
            self.authorizer.invalidate_cache().await;
            if let Some(ref on_challenge) = self.on_challenge {
                if response.headers().get_str(&WWW_AUTHENTICATE).is_ok() {
                    let should_retry = on_challenge
                        .on_challenge(ctx, request, self.authorizer.as_ref(), response.headers())
                        .await?;
                    if should_retry {
                        #[cfg(not(target_arch = "wasm32"))]
                        if let SeekableStream(stream) = request.body_mut() {
                            stream.reset().await?;
                        }
                        response = next[0].send(ctx, request, &next[1..]).await?
                    }
                }
            }
        }

        Ok(response)
    }
}

/// Callback [`BearerTokenAuthorizationPolicy`] invokes when it receives a 401 Unauthorized response with an authentication challenge (WWW-Authenticate header).
#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
pub trait OnChallenge: std::fmt::Debug + Send + Sync {
    /// Called when [`BearerTokenAuthorizationPolicy`] receives a 401 Unauthorized response with a challenge.
    ///
    /// Implementations are responsible for parsing authentication parameters from the challenge, authorizing the request via the provided [`Authorizer`],
    /// and indicating whether the policy should retry the request.
    ///
    /// # Arguments
    /// * `context` - The request context
    /// * `request` - The HTTP request that received the challenge
    /// * `authorizer` - Helper used to acquire an access token and set the request's authorization header
    /// * `headers` - The 401 response's headers
    ///
    /// # Returns
    /// * `Ok(true)` when the callback handled the challenge and [`BearerTokenAuthorizationPolicy`] should retry the request.
    /// * `Ok(false)` when the callback can't handle the challenge. [`BearerTokenAuthorizationPolicy`] will return the 401 response to the client in this case.
    /// * `Err` when an error occurs while handling the challenge.
    async fn on_challenge(
        &self,
        context: &Context,
        request: &mut Request,
        authorizer: &dyn Authorizer,
        headers: &Headers,
    ) -> Result<bool>;
}

/// Callback [`BearerTokenAuthorizationPolicy`] invokes on every request it receives, before sending the request.
#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
pub trait OnRequest: std::fmt::Debug + Send + Sync {
    /// Invoked once on every [`BearerTokenAuthorizationPolicy::send`] invocation, before the policy sends the request.
    ///
    /// `send` doesn't call this method before retrying a request after an authentication challenge (see [`OnChallenge`]
    /// for more about challenge handling). Implementations are responsible for authorizing each request via the provided
    /// [`Authorizer`]. The policy sends the request when this method returns Ok.
    ///
    /// # Arguments
    /// * `context` - The request context
    /// * `request` - The HTTP request being sent
    /// * `authorizer` - Helper used to acquire an access token and set the request's authorization header
    async fn on_request(
        &self,
        context: &Context,
        request: &mut Request,
        authorizer: &dyn Authorizer,
    ) -> Result<()>;
}

/// Helper trait used by [`OnChallenge`] and [`OnRequest`] to authorize requests. This trait is sealed and cannot
/// be implemented outside of this module.
#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
pub trait Authorizer: crate::private::Sealed + std::fmt::Debug + Send + Sync {
    /// Acquire an access token for the provided scopes and options, and set the request's authorization header.
    async fn authorize(
        &self,
        request: &mut Request,
        scopes: &[&str],
        options: TokenRequestOptions<'_>,
    ) -> Result<()>;
}

#[derive(Debug)]
struct BearerTokenAuthorizer {
    access_token: Arc<RwLock<Option<AccessToken>>>,
    credential: Arc<dyn TokenCredential>,
}

impl BearerTokenAuthorizer {
    fn new(credential: Arc<dyn TokenCredential>) -> Self {
        Self {
            access_token: Arc::new(RwLock::new(None)),
            credential,
        }
    }

    async fn invalidate_cache(&self) {
        let mut access_token = self.access_token.write().await;
        *access_token = None;
    }
}

impl crate::private::Sealed for BearerTokenAuthorizer {}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
impl Authorizer for BearerTokenAuthorizer {
    async fn authorize(
        &self,
        request: &mut Request,
        scopes: &[&str],
        options: TokenRequestOptions<'_>,
    ) -> Result<()> {
        let access_token = self.access_token.read().await;
        match access_token.as_ref() {
            None => {
                // cache is empty. Upgrade the lock and acquire a token, provided another thread hasn't already done so
                drop(access_token);
                let mut access_token = self.access_token.write().await;
                if access_token.is_none() {
                    *access_token = Some(self.credential.get_token(scopes, Some(options)).await?);
                }
            }
            Some(token) if should_refresh(&token.expires_on) => {
                // token is expired or within its refresh window. Upgrade the lock and
                // acquire a new token, provided another thread hasn't already done so
                let expires_on = token.expires_on;
                drop(access_token);
                let mut access_token = self.access_token.write().await;
                // access_token shouldn't be None here, but check anyway to guarantee unwrap won't panic
                if access_token.is_none() || access_token.as_ref().unwrap().expires_on == expires_on
                {
                    match self.credential.get_token(scopes, Some(options)).await {
                        Ok(new_token) => {
                            *access_token = Some(new_token);
                        }
                        Err(e)
                            if access_token.is_none()
                                || expires_on <= OffsetDateTime::now_utc() =>
                        {
                            // propagate this error because we can't proceed without a new token
                            return Err(e);
                        }
                        Err(_) => {
                            // ignore this error because the cached token is still valid
                        }
                    }
                }
            }
            Some(_) => {
                // do nothing; cached token is valid and not within its refresh window
                drop(access_token); // release the read lock so we don't try to acquire it while still holding it
            }
        }

        let access_token = self.access_token.read().await;
        let token = access_token
            .as_ref()
            .ok_or_else(|| {
                Error::with_message(
                    ErrorKind::Credential,
                    "The request failed due to an error while fetching the access token.",
                )
            })?
            .token
            .secret();
        request.insert_header(AUTHORIZATION, format!("Bearer {token}"));

        Ok(())
    }
}

fn should_refresh(expires_on: &OffsetDateTime) -> bool {
    *expires_on <= OffsetDateTime::now_utc() + Duration::minutes(5)
}

#[derive(Debug, Default)]
struct DefaultOnRequest {
    scopes: Vec<String>,
}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
impl OnRequest for DefaultOnRequest {
    async fn on_request(
        &self,
        context: &Context,
        request: &mut Request,
        authorizer: &dyn Authorizer,
    ) -> Result<()> {
        let options = TokenRequestOptions {
            method_options: ClientMethodOptions {
                context: context.clone(),
            },
        };
        let scopes: Vec<&str> = self.scopes.iter().map(String::as_str).collect();
        authorizer.authorize(request, &scopes, options).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        credentials::{Secret, TokenCredential, TokenRequestOptions},
        http::{
            headers::{HeaderName, Headers, AUTHORIZATION},
            policies::{Policy, TransportPolicy},
            Request, StatusCode,
        },
        time::OffsetDateTime,
        Bytes, Result,
    };
    use async_trait::async_trait;
    use azure_core_test::http::MockHttpClient;
    use futures::FutureExt;
    use std::sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    };
    use typespec::http::headers::HeaderValue;
    use typespec_client_core::{
        http::{AsyncRawResponse, ClientMethodOptions, Method, Transport},
        time::Duration,
    };

    #[derive(Debug, Clone)]
    struct MockCredential {
        calls: Arc<AtomicUsize>,
        tokens: Arc<[AccessToken]>,
    }

    impl MockCredential {
        fn new(tokens: &[AccessToken]) -> Self {
            Self {
                calls: Arc::new(AtomicUsize::new(0)),
                tokens: tokens.into(),
            }
        }

        fn get_token_calls(&self) -> usize {
            self.calls.load(Ordering::SeqCst)
        }
    }

    // ensure the number of get_token() calls matches the number of tokens
    // in a test case i.e., that the policy called get_token() as expected
    impl Drop for MockCredential {
        fn drop(&mut self) {
            if !self.tokens.is_empty() {
                assert_eq!(self.tokens.len(), self.calls.load(Ordering::SeqCst));
            }
        }
    }

    #[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
    #[cfg_attr(not(target_arch = "wasm32"), async_trait)]
    impl TokenCredential for MockCredential {
        async fn get_token(
            &self,
            _: &[&str],
            _: Option<TokenRequestOptions<'_>>,
        ) -> Result<AccessToken> {
            let i = self.calls.fetch_add(1, Ordering::SeqCst);
            self.tokens
                .get(i)
                .ok_or_else(|| Error::with_message(ErrorKind::Credential, "no more mock tokens"))
                .cloned()
        }
    }

    #[tokio::test]
    async fn authn_error() {
        // this mock's get_token() will return an error because it has no tokens
        let credential = MockCredential::new(&[]);
        let policy = BearerTokenAuthorizationPolicy::new(Arc::new(credential), ["scope"]);
        let client = MockHttpClient::new(|_| panic!("expected an error from get_token"));
        let transport = Arc::new(TransportPolicy::new(Transport::new(Arc::new(client))));
        let mut req = Request::new("https://localhost".parse().unwrap(), Method::Get);

        let err = policy
            .send(
                &Context::default(),
                &mut req,
                std::slice::from_ref(&(transport.clone() as Arc<dyn Policy>)),
            )
            .await
            .expect_err("request should fail");

        assert_eq!(ErrorKind::Credential, *err.kind());
    }

    async fn run_test(tokens: &[AccessToken]) {
        let credential = Arc::new(MockCredential::new(tokens));
        let policy = BearerTokenAuthorizationPolicy::new(credential.clone(), ["scope"]);
        let client = Arc::new(MockHttpClient::new(move |actual| {
            let credential = credential.clone();
            async move {
                let authz = actual.headers().get_str(&AUTHORIZATION)?;
                // e.g. if this is the first request, we expect 1 get_token call and tokens[0] in the header
                let i = credential.get_token_calls().saturating_sub(1);
                let expected = &credential.tokens[i];

                assert_eq!(format!("Bearer {}", expected.token.secret()), authz);

                Ok(AsyncRawResponse::from_bytes(
                    StatusCode::Ok,
                    Headers::new(),
                    Bytes::new(),
                ))
            }
            .boxed()
        }));
        let transport = Arc::new(TransportPolicy::new(Transport::new(client)));

        let mut handles = vec![];
        for _ in 0..4 {
            let policy = policy.clone();
            let transport = transport.clone();
            let handle = tokio::spawn(async move {
                let ctx = Context::default();
                let mut req = Request::new("https://localhost".parse().unwrap(), Method::Get);
                policy
                    .send(
                        &ctx,
                        &mut req,
                        std::slice::from_ref(&(transport.clone() as Arc<dyn Policy>)),
                    )
                    .await
                    .expect("successful request");
            });
            handles.push(handle);
        }

        for handle in handles {
            tokio::time::timeout(Duration::seconds(2).try_into().unwrap(), handle)
                .await
                .expect("task timed out after 2 seconds")
                .expect("completed task");
        }
    }

    #[tokio::test]
    async fn caches_token() {
        run_test(&[AccessToken {
            token: Secret::new("fake".to_string()),
            expires_on: OffsetDateTime::now_utc() + Duration::seconds(3600),
        }])
        .await;
    }

    #[tokio::test]
    async fn refreshes_token() {
        run_test(&[
            AccessToken {
                token: Secret::new("1".to_string()),
                expires_on: OffsetDateTime::now_utc() - Duration::seconds(1),
            },
            AccessToken {
                token: Secret::new("2".to_string()),
                expires_on: OffsetDateTime::now_utc() + Duration::seconds(3600),
            },
        ])
        .await;
    }

    #[derive(Debug)]
    struct TestOnChallenge {
        calls: Arc<AtomicUsize>,
        error: Option<Error>,
        should_retry: bool,
    }

    #[async_trait]
    impl OnChallenge for TestOnChallenge {
        async fn on_challenge(
            &self,
            context: &Context,
            request: &mut Request,
            authorizer: &dyn Authorizer,
            _headers: &Headers,
        ) -> Result<bool> {
            self.calls.fetch_add(1, Ordering::SeqCst);
            if let Some(ref e) = self.error {
                return Err(Error::with_message(e.kind().clone(), e.to_string()));
            }
            if self.should_retry {
                let options = TokenRequestOptions {
                    method_options: ClientMethodOptions {
                        context: context.clone(),
                    },
                };
                authorizer.authorize(request, &["scope"], options).await?;
            }
            Ok(self.should_retry)
        }
    }

    #[tokio::test]
    async fn on_challenge_error() {
        let calls = Arc::new(AtomicUsize::new(0));
        let on_challenge = Arc::new(TestOnChallenge {
            calls: calls.clone(),
            error: Some(Error::with_message(
                ErrorKind::Other,
                "something went wrong",
            )),
            should_retry: false,
        });

        let credential = Arc::new(MockCredential::new(&[AccessToken {
            token: Secret::new("fake".to_string()),
            expires_on: OffsetDateTime::now_utc() + Duration::seconds(3600),
        }]));

        let policy = BearerTokenAuthorizationPolicy::new(credential, ["scope"])
            .with_on_challenge(on_challenge);

        let client = MockHttpClient::new(|_| {
            async {
                let mut headers = Headers::new();
                headers.insert(WWW_AUTHENTICATE, "Bearer challenge");
                Ok(AsyncRawResponse::from_bytes(
                    StatusCode::Unauthorized,
                    headers,
                    Bytes::new(),
                ))
            }
            .boxed()
        });
        let transport = Arc::new(TransportPolicy::new(Transport::new(Arc::new(client))));

        let mut req = Request::new("https://localhost".parse().unwrap(), Method::Get);
        let err = policy
            .send(
                &Context::default(),
                &mut req,
                std::slice::from_ref(&(transport as Arc<dyn Policy>)),
            )
            .await
            .expect_err("request should fail");

        assert_eq!(ErrorKind::Other, *err.kind());
        assert_eq!("something went wrong", err.to_string());
        assert_eq!(1, calls.load(Ordering::SeqCst));
    }

    #[tokio::test]
    async fn on_challenge_not_called_without_header() {
        let calls = Arc::new(AtomicUsize::new(0));
        let on_challenge = Arc::new(TestOnChallenge {
            calls: calls.clone(),
            error: None,
            should_retry: false,
        });

        let credential = Arc::new(MockCredential::new(&[AccessToken {
            token: Secret::new("fake".to_string()),
            expires_on: OffsetDateTime::now_utc() + Duration::seconds(3600),
        }]));

        let policy = BearerTokenAuthorizationPolicy::new(credential, ["scope"])
            .with_on_challenge(on_challenge);

        let client = MockHttpClient::new(|_| {
            async {
                Ok(AsyncRawResponse::from_bytes(
                    StatusCode::Unauthorized,
                    Headers::new(),
                    Bytes::new(),
                ))
            }
            .boxed()
        });
        let transport = Arc::new(TransportPolicy::new(Transport::new(Arc::new(client))));

        let mut req = Request::new("https://localhost".parse().unwrap(), Method::Get);
        let response = policy
            .send(
                &Context::default(),
                &mut req,
                std::slice::from_ref(&(transport as Arc<dyn Policy>)),
            )
            .await
            .expect("successful request");

        assert_eq!(StatusCode::Unauthorized, response.status());
        assert_eq!(0, calls.load(Ordering::SeqCst));
    }

    #[tokio::test]
    async fn on_challenge_no_retry() {
        let calls = Arc::new(AtomicUsize::new(0));
        let on_challenge = Arc::new(TestOnChallenge {
            calls: calls.clone(),
            error: None,
            should_retry: false,
        });

        let credential = Arc::new(MockCredential::new(&[AccessToken::new(
            "token",
            OffsetDateTime::now_utc() + Duration::seconds(3600),
        )]));

        let policy = BearerTokenAuthorizationPolicy::new(credential, ["scope"])
            .with_on_challenge(on_challenge);

        let client = MockHttpClient::new(|_| {
            async {
                Ok(AsyncRawResponse::from_bytes(
                    StatusCode::Unauthorized,
                    Headers::from(std::collections::HashMap::from([(
                        WWW_AUTHENTICATE,
                        HeaderValue::from("Bearer challenge".to_string()),
                    )])),
                    Bytes::new(),
                ))
            }
            .boxed()
        });
        let transport: Arc<dyn Policy> =
            Arc::new(TransportPolicy::new(Transport::new(Arc::new(client))));

        let ctx = Context::default();
        let mut req = Request::new("https://localhost".parse().unwrap(), Method::Get);
        let res = policy
            .send(&ctx, &mut req, std::slice::from_ref(&transport))
            .await
            .expect("successful request");

        assert_eq!(1, calls.load(Ordering::SeqCst));
        assert_eq!(StatusCode::Unauthorized, res.status());
        assert_eq!(
            "Bearer challenge",
            res.headers().get_str(&WWW_AUTHENTICATE).unwrap()
        );
    }

    #[tokio::test]
    async fn on_challenge_with_retry() {
        let on_challenge_calls = Arc::new(AtomicUsize::new(0));
        let on_challenge = Arc::new(TestOnChallenge {
            calls: on_challenge_calls.clone(),
            error: None,
            should_retry: true,
        });

        let on_request_calls = Arc::new(AtomicUsize::new(0));
        let on_request = Arc::new(TestOnRequest {
            calls: on_request_calls.clone(),
            error: None,
        });

        let credential = Arc::new(MockCredential::new(&[
            AccessToken {
                token: Secret::new("first".to_string()),
                expires_on: OffsetDateTime::now_utc() + Duration::seconds(3600),
            },
            AccessToken {
                token: Secret::new("second".to_string()),
                expires_on: OffsetDateTime::now_utc() + Duration::seconds(3600),
            },
        ]));

        let policy = BearerTokenAuthorizationPolicy::new(credential.clone(), ["scope"])
            .with_on_request(on_request)
            .with_on_challenge(on_challenge);

        let request_count = Arc::new(AtomicUsize::new(0));
        let request_count_clone = request_count.clone();

        let client = MockHttpClient::new(move |actual| {
            let count = request_count_clone.fetch_add(1, Ordering::SeqCst);
            async move {
                let authz = actual.headers().get_str(&AUTHORIZATION)?;

                if count == 0 {
                    // First request gets 401
                    assert_eq!("Bearer first", authz);
                    Ok(AsyncRawResponse::from_bytes(
                        StatusCode::Unauthorized,
                        Headers::from(std::collections::HashMap::from([(
                            WWW_AUTHENTICATE,
                            HeaderValue::from("Bearer challenge".to_string()),
                        )])),
                        Bytes::new(),
                    ))
                } else {
                    // Retry with new token succeeds
                    assert_eq!("Bearer second", authz);
                    Ok(AsyncRawResponse::from_bytes(
                        StatusCode::Ok,
                        Headers::new(),
                        Bytes::new(),
                    ))
                }
            }
            .boxed()
        });
        let transport = Arc::new(TransportPolicy::new(Transport::new(Arc::new(client))));

        let mut req = Request::new("https://localhost".parse().unwrap(), Method::Get);
        let response = policy
            .send(
                &Context::default(),
                &mut req,
                std::slice::from_ref(&(transport as Arc<dyn Policy>)),
            )
            .await
            .expect("successful request");

        assert_eq!(StatusCode::Ok, response.status());
        assert_eq!(1, on_request_calls.load(Ordering::SeqCst));
        assert_eq!(1, on_challenge_calls.load(Ordering::SeqCst));
        assert_eq!(2, request_count.load(Ordering::SeqCst));
        assert_eq!(2, credential.get_token_calls());
    }

    #[derive(Debug)]
    struct TestOnRequest {
        calls: Arc<AtomicUsize>,
        error: Option<Error>,
    }

    #[async_trait]
    impl OnRequest for TestOnRequest {
        async fn on_request(
            &self,
            _context: &Context,
            request: &mut Request,
            authorizer: &dyn Authorizer,
        ) -> Result<()> {
            let calls = self.calls.fetch_add(1, Ordering::SeqCst) + 1;
            request.insert_header("on-request-calls", calls.to_string());

            if let Some(ref e) = self.error {
                Err(Error::with_message(e.kind().clone(), e.to_string()))
            } else {
                authorizer
                    .authorize(
                        request,
                        &["scope"],
                        TokenRequestOptions {
                            method_options: ClientMethodOptions {
                                context: Context::default(),
                            },
                        },
                    )
                    .await?;
                Ok(())
            }
        }
    }

    #[tokio::test]
    async fn on_request() {
        let called = Arc::new(AtomicUsize::new(0));
        let on_request = Arc::new(TestOnRequest {
            calls: called.clone(),
            error: None,
        });

        let credential = Arc::new(MockCredential::new(&[AccessToken::new(
            "token",
            OffsetDateTime::now_utc() + Duration::seconds(3600),
        )]));

        let policy =
            BearerTokenAuthorizationPolicy::new(credential, ["scope"]).with_on_request(on_request);

        let client = MockHttpClient::new(|actual| {
            async {
                assert_eq!(
                    "1",
                    actual
                        .headers()
                        .get_str(&HeaderName::from_static("on-request-calls"))?,
                    "on_request should have set the test header to 1",
                );
                Ok(AsyncRawResponse::from_bytes(
                    StatusCode::Ok,
                    Headers::new(),
                    Bytes::new(),
                ))
            }
            .boxed()
        });
        let transport: Arc<dyn Policy> =
            Arc::new(TransportPolicy::new(Transport::new(Arc::new(client))));

        let ctx = Context::default();
        let mut req = Request::new("https://localhost".parse().unwrap(), Method::Get);
        req.insert_header("on-request-calls", stringify!(TestOnRequest));
        policy
            .send(&ctx, &mut req, std::slice::from_ref(&transport))
            .await
            .expect("successful request");

        assert_eq!(1, called.load(Ordering::SeqCst));
    }
    #[tokio::test]
    async fn on_request_error() {
        let calls = Arc::new(AtomicUsize::new(0));
        let on_request = Arc::new(TestOnRequest {
            calls: calls.clone(),
            error: Some(Error::with_message(
                ErrorKind::Other,
                "something went wrong",
            )),
        });

        let credential = Arc::new(MockCredential::new(&[]));

        let policy =
            BearerTokenAuthorizationPolicy::new(credential, ["scope"]).with_on_request(on_request);

        let client =
            MockHttpClient::new(|_| panic!("request should not be sent when on_request errors"));
        let transport: Arc<dyn Policy> =
            Arc::new(TransportPolicy::new(Transport::new(Arc::new(client))));

        let ctx = Context::default();
        let mut req = Request::new("https://localhost".parse().unwrap(), Method::Get);

        let err = policy
            .send(&ctx, &mut req, std::slice::from_ref(&transport))
            .await
            .expect_err("request should fail");

        assert_eq!(ErrorKind::Other, *err.kind());
        assert_eq!("something went wrong", err.to_string());
        assert_eq!(1, calls.load(Ordering::SeqCst));
    }

    #[cfg_attr(not(target_arch = "wasm32"), tokio::test)]
    async fn resets_stream_for_retry_after_challenge() {
        use futures::StreamExt;
        use typespec_client_core::http::Body;
        use typespec_client_core::stream::BytesStream;

        let on_challenge_calls = Arc::new(AtomicUsize::new(0));
        let on_challenge = Arc::new(TestOnChallenge {
            calls: on_challenge_calls.clone(),
            error: None,
            should_retry: true,
        });

        let credential = Arc::new(MockCredential::new(&[
            AccessToken {
                token: Secret::new("first".to_string()),
                expires_on: OffsetDateTime::now_utc() + Duration::seconds(3600),
            },
            AccessToken {
                token: Secret::new("second".to_string()),
                expires_on: OffsetDateTime::now_utc() + Duration::seconds(3600),
            },
        ]));

        let policy = BearerTokenAuthorizationPolicy::new(credential, ["scope"])
            .with_on_challenge(on_challenge);

        let request_count = Arc::new(AtomicUsize::new(0));
        let request_count_clone = request_count.clone();

        let client = MockHttpClient::new(move |actual| {
            let count = request_count_clone.fetch_add(1, Ordering::SeqCst);
            async move {
                match actual.body() {
                    Body::SeekableStream(stream) => {
                        let mut stream = stream.clone();
                        let mut collected = Vec::new();
                        while let Some(chunk) = stream.next().await {
                            let chunk = chunk?;
                            collected.extend_from_slice(&chunk);
                        }
                        assert_eq!(b"test data", collected.as_slice());
                    }
                    _ => unreachable!("body is a SeekableStream"),
                }

                if count == 0 {
                    Ok(AsyncRawResponse::from_bytes(
                        StatusCode::Unauthorized,
                        Headers::from(std::collections::HashMap::from([(
                            WWW_AUTHENTICATE,
                            HeaderValue::from("Bearer challenge".to_string()),
                        )])),
                        Bytes::new(),
                    ))
                } else {
                    Ok(AsyncRawResponse::from_bytes(
                        StatusCode::Ok,
                        Headers::new(),
                        Bytes::new(),
                    ))
                }
            }
            .boxed()
        });
        let transport = Arc::new(TransportPolicy::new(Transport::new(Arc::new(client))));

        let mut req = Request::new("https://localhost".parse().unwrap(), Method::Get);
        let stream = BytesStream::new(b"test data".as_slice());
        req.set_body(Body::SeekableStream(Box::new(stream)));

        let res = policy
            .send(
                &Context::default(),
                &mut req,
                std::slice::from_ref(&(transport as Arc<dyn Policy>)),
            )
            .await
            .expect("policy should reset the body stream and succeed on retry");

        assert_eq!(StatusCode::Ok, res.status());
        assert_eq!(1, on_challenge_calls.load(Ordering::SeqCst));
        assert_eq!(2, request_count.load(Ordering::SeqCst));
    }
}

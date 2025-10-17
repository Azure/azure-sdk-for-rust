// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use crate::{
    credentials::{AccessToken, TokenCredential, TokenRequestOptions},
    error::{Error, ErrorKind},
    http::{
        headers::AUTHORIZATION,
        policies::{Policy, PolicyResult},
    },
};
use async_lock::RwLock;
use async_trait::async_trait;
use std::sync::Arc;
use typespec_client_core::http::{ClientMethodOptions, Context, Request};
use typespec_client_core::time::{Duration, OffsetDateTime};

/// Authentication policy for a bearer token.
#[derive(Debug, Clone)]
pub struct BearerTokenCredentialPolicy {
    credential: Arc<dyn TokenCredential>,
    scopes: Vec<String>,
    access_token: Arc<RwLock<Option<AccessToken>>>,
}

impl BearerTokenCredentialPolicy {
    /// Creates a new `BearerTokenCredentialPolicy`.
    pub fn new<A, B>(credential: Arc<dyn TokenCredential>, scopes: A) -> Self
    where
        A: IntoIterator<Item = B>,
        B: Into<String>,
    {
        Self {
            credential,
            scopes: scopes.into_iter().map(|s| s.into()).collect(),
            access_token: Arc::new(RwLock::new(None)),
        }
    }

    fn scopes(&self) -> Vec<&str> {
        self.scopes
            .iter()
            .map(String::as_str)
            .collect::<Vec<&str>>()
    }

    async fn access_token(&self) -> Option<String> {
        let access_token = self.access_token.read().await;
        access_token.as_ref().map(|s| s.token.secret().to_string())
    }
}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
impl Policy for BearerTokenCredentialPolicy {
    async fn send(
        &self,
        ctx: &Context,
        request: &mut Request,
        next: &[Arc<dyn Policy>],
    ) -> PolicyResult {
        let access_token = self.access_token.read().await;

        match access_token.as_ref() {
            None => {
                // cache is empty. Upgrade the lock and acquire a token, provided another thread hasn't already done so
                drop(access_token);
                let mut access_token = self.access_token.write().await;
                if access_token.is_none() {
                    let options = TokenRequestOptions {
                        method_options: ClientMethodOptions {
                            context: ctx.clone(),
                        },
                    };
                    *access_token = Some(
                        self.credential
                            .get_token(&self.scopes(), Some(options))
                            .await?,
                    );
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
                    let options = TokenRequestOptions {
                        method_options: ClientMethodOptions {
                            context: ctx.clone(),
                        },
                    };
                    match self
                        .credential
                        .get_token(&self.scopes(), Some(options))
                        .await
                    {
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
            }
        }

        let access_token = self.access_token().await.ok_or_else(|| {
            Error::with_message(
                ErrorKind::Credential,
                "The request failed due to an error while fetching the access token.",
            )
        })?;
        request.insert_header(AUTHORIZATION, format!("Bearer {}", access_token));

        next[0].send(ctx, request, &next[1..]).await
    }
}

fn should_refresh(expires_on: &OffsetDateTime) -> bool {
    *expires_on <= OffsetDateTime::now_utc() + Duration::minutes(5)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        credentials::{Secret, TokenCredential, TokenRequestOptions},
        http::{
            headers::{Headers, AUTHORIZATION},
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
    use typespec_client_core::{
        http::{BufResponse, Method, Transport},
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
        let policy = BearerTokenCredentialPolicy::new(Arc::new(credential), ["scope"]);
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
        let policy = BearerTokenCredentialPolicy::new(credential.clone(), ["scope"]);
        let client = Arc::new(MockHttpClient::new(move |actual| {
            let credential = credential.clone();
            async move {
                let authz = actual.headers().get_str(&AUTHORIZATION)?;
                // e.g. if this is the first request, we expect 1 get_token call and tokens[0] in the header
                let i = credential.get_token_calls().saturating_sub(1);
                let expected = &credential.tokens[i];

                assert_eq!(format!("Bearer {}", expected.token.secret()), authz);

                Ok(BufResponse::from_bytes(
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
}

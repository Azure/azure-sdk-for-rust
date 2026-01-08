// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

// Apply changes to this file to the copy in each Key Vault crate. These copies must be identical.

use async_lock::RwLock;
use async_trait::async_trait;
use azure_core::{
    credentials::TokenRequestOptions,
    error::{Error, ErrorKind},
    http::{
        headers::{Headers, CONTENT_LENGTH, CONTENT_TYPE, WWW_AUTHENTICATE},
        policies::auth::{Authorizer, OnChallenge, OnRequest},
        Body, Context, Request, Url,
    },
    Result,
};
use std::sync::Arc;

/// Discovers authentication parameters from a Key Vault by sending the client's first request without
/// authorization to prompt an authentication challenge.
#[derive(Debug)]
pub(crate) struct KeyVaultAuthorizer {
    scope: RwLock<String>,
    verify_challenge_resource: bool,
}

impl KeyVaultAuthorizer {
    pub fn new(verify_challenge_resource: bool) -> Arc<Self> {
        Arc::new(Self {
            scope: RwLock::new(String::new()),
            verify_challenge_resource,
        })
    }

    // Parses authentication parameters from a Key Vault authentication challenge.
    //
    // Example challenges:
    //   Bearer authorization="https://login.microsoftonline.com/tenant", scope="https://vault.azure.net/.default"
    //   Bearer authorization="https://login.microsoftonline.com/tenant", resource="https://vault.azure.net"
    fn parse_scope_from_challenge(challenge: &str) -> Result<String> {
        for (i, _) in challenge.match_indices(r#"=""#) {
            if let Some(sub) = challenge.get(i.saturating_sub(8)..i) {
                if sub.ends_with("scope") || sub == "resource" {
                    let value_start = i + 2;
                    if let Some(end) = challenge[value_start..].find('"') {
                        let value = &challenge[value_start..value_start + end];
                        return Ok(if sub.ends_with("scope") {
                            value.to_string()
                        } else {
                            format!("{value}/.default")
                        });
                    }
                }
            }
        }

        Err(Error::with_message(
            ErrorKind::DataConversion,
            format!("no scope or resource in authentication challenge: {challenge}"),
        ))
    }
}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
impl OnRequest for KeyVaultAuthorizer {
    /// Runs on each request before it is sent.
    ///
    /// When authentication parameters have previously been discovered, this function authorizes the request
    /// normally. When authentication parameters aren't known, for example because the client hasn't sent a
    /// request to Key Vault, this function removes the request body and stores it in the Context so
    /// [`Self::on_challenge`] can restore it after authenticating. Removing the body in this case is important
    /// because the request is certain to fail due to its lack of authorization and because Key Vault supports
    /// an authentication scheme that protects request body data. Azure SDK clients don't support this scheme
    /// but must avoid sending unprotected data to a vault that requires it.
    async fn on_request(
        &self,
        ctx: &mut Context,
        request: &mut Request,
        authorizer: &dyn Authorizer,
    ) -> azure_core::Result<()> {
        let scope = self.scope.read().await;
        if scope.is_empty() {
            if !request.body().is_empty() {
                let body = request.body_mut().take();
                ctx.insert(body);
                let headers = request.headers_mut();
                headers.remove(CONTENT_LENGTH);
                headers.remove(CONTENT_TYPE);
            }
            Ok(())
        } else {
            authorizer
                .authorize(
                    request,
                    &[scope.as_str()],
                    TokenRequestOptions {
                        method_options: azure_core::http::ClientMethodOptions {
                            context: ctx.to_owned(),
                        },
                    },
                )
                .await
        }
    }
}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
impl OnChallenge for KeyVaultAuthorizer {
    /// Runs when a request receives an authentication challenge.
    ///
    /// This function extracts authentication parameters from the challenge, restores the body
    /// saved by [`Self::on_request`], if any, and authorizes the request.
    async fn on_challenge(
        &self,
        context: &Context,
        request: &mut Request,
        authorizer: &dyn Authorizer,
        headers: &Headers,
    ) -> Result<()> {
        let challenge = headers.get_str(&WWW_AUTHENTICATE)?;
        let scope = KeyVaultAuthorizer::parse_scope_from_challenge(challenge)?;
        {
            let mut cached_scope = self.scope.write().await;
            *cached_scope = scope.clone();
        }
        if self.verify_challenge_resource {
            // the challenge resource's host must match the requested domain's host
            let challenge_url = Url::parse(&scope).map_err(|_| {
                Error::with_message(
                    ErrorKind::DataConversion,
                    format!("invalid audience in challenge: {challenge}"),
                )
            })?;
            let challenge_host = challenge_url.host_str().ok_or_else(|| {
                Error::with_message(
                    ErrorKind::DataConversion,
                    format!("invalid audience in challenge: {challenge}"),
                )
            })?;
            let request_host = request.url().host_str().ok_or_else(|| {
                // should be impossible because the client already sent the request and received a response
                Error::with_message(
                    ErrorKind::DataConversion,
                    format!("invalid request URL: {}", request.url()),
                )
            })?;
            if !request_host.ends_with(format!(".{challenge_host}").as_str()) {
                return Err(Error::with_message(
                    ErrorKind::Other,
                    format!(
                            "challenge resource '{scope}' doesn't match the requested domain '{request_host}'. Set verify_challenge_resource in client options to disable this validation if necessary. See https://aka.ms/azsdk/blog/vault-uri for more information`"
                )));
            }
        }
        if let Some(saved_body) = context.value::<Body>() {
            request.set_body(saved_body);
            request.insert_header(CONTENT_LENGTH, saved_body.len().to_string());
            request.insert_header(CONTENT_TYPE, "application/json");
        }
        let options = TokenRequestOptions {
            method_options: azure_core::http::ClientMethodOptions {
                context: context.to_owned(),
            },
        };
        authorizer
            .authorize(request, &[scope.as_str()], options)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use azure_core::{
        credentials::{AccessToken, Secret, TokenCredential, TokenRequestOptions},
        http::{
            headers::{HeaderName, Headers, WWW_AUTHENTICATE},
            policies::{auth::BearerTokenAuthorizationPolicy, Policy},
            AsyncRawResponse, Context, Method, Pipeline, Request, StatusCode, Transport, Url,
        },
        time::{Duration, OffsetDateTime},
        Bytes,
    };
    use azure_core_test::http::MockHttpClient;
    use futures::FutureExt;
    use serde_json::json;
    use std::sync::{
        atomic::{AtomicUsize, Ordering},
        Arc, Mutex,
    };

    #[derive(Clone, Debug)]
    struct MockCredential {
        calls: Arc<AtomicUsize>,
        expected_scope: String,
        tokens: Arc<[AccessToken]>,
    }

    impl MockCredential {
        fn new(tokens: Vec<AccessToken>, expected_scope: String) -> Self {
            Self {
                calls: Arc::new(AtomicUsize::new(0)),
                expected_scope,
                tokens: tokens.into(),
            }
        }

        fn call_count(&self) -> usize {
            self.calls.load(Ordering::SeqCst)
        }
    }

    #[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
    #[cfg_attr(not(target_arch = "wasm32"), async_trait)]
    impl TokenCredential for MockCredential {
        async fn get_token(
            &self,
            scopes: &[&str],
            _: Option<TokenRequestOptions<'_>>,
        ) -> azure_core::Result<AccessToken> {
            let index = self.calls.fetch_add(1, Ordering::SeqCst);
            assert_eq!(
                scopes,
                [self.expected_scope.as_str()],
                "unexpected scopes in token request"
            );
            self.tokens.get(index).cloned().ok_or_else(|| {
                azure_core::Error::with_message(
                    azure_core::error::ErrorKind::Credential,
                    "no more mock tokens",
                )
            })
        }
    }

    #[tokio::test]
    async fn challenge_retries_with_original_body() {
        let expected_body = json!({
            "value": "secret-value",
        })
        .to_string();
        let expected_bytes = Bytes::from(expected_body.clone());

        let observed_bodies = Arc::new(Mutex::new(Vec::new()));
        let requests = Arc::new(AtomicUsize::new(0));

        let transport = Transport::new(Arc::new(MockHttpClient::new({
            let observed_bodies = Arc::clone(&observed_bodies);
            let requests = Arc::clone(&requests);
            move |req| {
                let observed_bodies = Arc::clone(&observed_bodies);
                let attempts = Arc::clone(&requests);
                async move {
                    let body_bytes = Bytes::from(req.body());
                    observed_bodies
                        .lock()
                        .expect("failed to lock observed bodies")
                        .push(body_bytes);

                    let attempt = attempts.fetch_add(1, Ordering::SeqCst);
                    if attempt == 0 {
                        assert!(req.body().is_empty(), "first request should have empty body");
                        let mut headers = Headers::new();
                        headers.insert(WWW_AUTHENTICATE, r#"Bearer authorization="https://login.microsoftonline.com/tenant", resource="https://a.b""#);
                        Ok(AsyncRawResponse::from_bytes(
                            StatusCode::Unauthorized,
                            headers,
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
            }
        })));

        let mock_credential = Arc::new(MockCredential::new(
            vec![AccessToken {
                token: Secret::new("token".to_string()),
                expires_on: OffsetDateTime::now_utc() + Duration::seconds(600),
            }],
            "https://a.b/.default".to_string(),
        ));

        let authorizer = KeyVaultAuthorizer::new(true);
        let auth_policy: Arc<dyn Policy> = Arc::new(
            BearerTokenAuthorizationPolicy::new(mock_credential.clone(), Vec::<String>::new())
                .with_on_request(authorizer.clone())
                .with_on_challenge(authorizer),
        );

        let client_options = azure_core::http::ClientOptions {
            transport: Some(transport),
            ..Default::default()
        };

        let pipeline = Pipeline::new(
            option_env!("CARGO_PKG_NAME"),
            option_env!("CARGO_PKG_VERSION"),
            client_options,
            Vec::default(),
            vec![auth_policy],
            None,
        );

        let endpoint = Url::parse("https://vault.a.b").expect("valid url");
        let mut request = Request::new(endpoint, Method::Put);
        request.insert_header("content-type", "application/json");
        request.set_body(expected_bytes.clone());

        pipeline
            .send(&Context::default(), &mut request, None)
            .await
            .expect("request should succeed");

        assert_eq!(
            requests.load(Ordering::SeqCst),
            2,
            "expected retry after challenge"
        );
        assert_eq!(
            mock_credential.call_count(),
            1,
            "credential should be called once (during challenge)"
        );

        let bodies = observed_bodies
            .lock()
            .expect("failed to lock observed bodies for assertion");
        assert_eq!(bodies.len(), 2, "transport should observe two requests");
        assert_eq!(
            &bodies[0],
            &Bytes::new(),
            "first request body should be empty"
        );
        assert_eq!(
            &bodies[1], &expected_bytes,
            "second request should have the expected body"
        );
    }

    #[tokio::test]
    async fn challenge_resource_verification() {
        let mock_credential = Arc::new(MockCredential::new(
            vec![AccessToken {
                token: Secret::new("token".to_string()),
                expires_on: OffsetDateTime::now_utc() + Duration::seconds(3600),
            }],
            "https://a.b/.default".to_string(),
        ));

        let transport = Transport::new(Arc::new(MockHttpClient::new({
            move |_| {
                async move {
                    let mut headers = Headers::new();
                    headers.insert(WWW_AUTHENTICATE, r#"Bearer authorization="https://login.microsoftonline.com/tenant", resource="https://a.b""#);
                    Ok(AsyncRawResponse::from_bytes(
                        StatusCode::Unauthorized,
                        headers,
                        Bytes::new(),
                    ))
                }
                .boxed()
            }
        })));
        let client_options = azure_core::http::ClientOptions {
            transport: Some(transport),
            ..Default::default()
        };

        let authorizer = KeyVaultAuthorizer::new(true);
        let auth_policy: Arc<dyn Policy> = Arc::new(
            BearerTokenAuthorizationPolicy::new(mock_credential.clone(), Vec::<String>::new())
                .with_on_request(authorizer.clone())
                .with_on_challenge(authorizer),
        );
        let pipeline = Pipeline::new(
            option_env!("CARGO_PKG_NAME"),
            option_env!("CARGO_PKG_VERSION"),
            client_options,
            Vec::default(),
            vec![auth_policy],
            None,
        );

        let mut request = Request::new(
            Url::parse("https://vault.c.d/keys/foo").unwrap(),
            Method::Get,
        );
        let err = pipeline
            .send(&Context::default(), &mut request, None)
            .await
            .unwrap_err();
        match err.kind() {
            ErrorKind::Other => {
                let inner_message = err.into_inner().unwrap().to_string();
                assert!(inner_message.contains("https://aka.ms/azsdk/blog/vault-uri"));
            }
            _ => panic!("unexpected error kind: {err:?}"),
        }
    }

    #[tokio::test]
    async fn concurrency() {
        let num_tasks = 10;
        let mut handles = Vec::new();

        // maps request ID to the number of attempts made for that request
        let request_tracker =
            Arc::new(Mutex::new(std::collections::HashMap::<String, usize>::new()));

        let transport = Transport::new(Arc::new(MockHttpClient::new({
            let request_tracker = request_tracker.clone();
            move |req| {
                let request_tracker = request_tracker.clone();
                async move {
                    let request_id = req
                        .headers()
                        .get_str(&HeaderName::from_static("request-id"))
                        .unwrap()
                        .to_string();

                    let mut tracker = request_tracker.lock().unwrap();
                    let entry = tracker.entry(request_id.clone()).or_insert(0);
                    *entry += 1;
                    let attempt = *entry;

                    let body_bytes = Bytes::from(req.body());
                    if attempt == 1 {
                        let mut headers = Headers::new();
                        headers.insert(
                            WWW_AUTHENTICATE,
                            r#"Bearer authorization="https://login.microsoftonline.com/tenant", resource="https://a.b""#,
                        );
                        Ok(AsyncRawResponse::from_bytes(
                            StatusCode::Unauthorized,
                            headers,
                            Bytes::new(),
                        ))
                    } else {
                        let expected_body = Bytes::from(format!("body-{}", request_id));
                        if body_bytes != expected_body {
                            return Ok(AsyncRawResponse::from_bytes(
                                StatusCode::BadRequest,
                                Headers::new(),
                                Bytes::from(format!(
                                    "Body mismatch. Expected: {:?}, Got: {:?}",
                                    expected_body, body_bytes
                                )),
                            ));
                        }

                        Ok(AsyncRawResponse::from_bytes(
                            StatusCode::Ok,
                            Headers::new(),
                            Bytes::new(),
                        ))
                    }
                }
                .boxed()
            }
        })));

        let mock_credential = Arc::new(MockCredential::new(
            (0..num_tasks)
                .map(|_| AccessToken {
                    token: Secret::new("token".to_string()),
                    expires_on: OffsetDateTime::now_utc() + Duration::seconds(3600),
                })
                .collect(),
            "https://a.b/.default".to_string(),
        ));

        let authorizer = KeyVaultAuthorizer::new(true);
        let auth_policy: Arc<dyn Policy> = Arc::new(
            BearerTokenAuthorizationPolicy::new(mock_credential.clone(), Vec::<String>::new())
                .with_on_request(authorizer.clone())
                .with_on_challenge(authorizer),
        );

        let client_options = azure_core::http::ClientOptions {
            transport: Some(transport),
            ..Default::default()
        };

        let pipeline = Pipeline::new(
            option_env!("CARGO_PKG_NAME"),
            option_env!("CARGO_PKG_VERSION"),
            client_options,
            Vec::default(),
            vec![auth_policy],
            None,
        );
        let pipeline = Arc::new(pipeline);

        for i in 0..num_tasks {
            let pipeline = pipeline.clone();
            handles.push(tokio::spawn(async move {
                let endpoint = Url::parse("https://vault.a.b").expect("valid url");
                let mut request = Request::new(endpoint, Method::Put);
                let request_id = format!("{i}");
                request.insert_header("request-id", &request_id);
                request.insert_header("content-type", "application/json");
                request.set_body(Bytes::from(format!("body-{request_id}")));

                pipeline.send(&Context::default(), &mut request, None).await
            }));
        }

        for result in futures::future::join_all(handles).await {
            let response = result.expect("task failed").expect("request failed");
            let status = response.status();
            assert_eq!(
                StatusCode::Ok,
                status,
                "Request failed with status: {status}"
            );
        }
    }

    #[test]
    fn parse_scope_both_parameters() {
        for challenge in [
            r#"Bearer authorization="https://login.microsoftonline.com/tenant", resource="https://first", scope="https://second/.default""#,
            r#"Bearer authorization="https://login.microsoftonline.com/tenant", scope="https://first/.default", resource="https://second""#,
        ] {
            let scope = KeyVaultAuthorizer::parse_scope_from_challenge(challenge).unwrap();
            assert_eq!(
                "https://first/.default", scope,
                "should prefer the first value found"
            );
        }
    }

    #[test]
    fn parse_scope_no_audience() {
        for challenge in [
            r#"Bearer authorization="https://login.microsoftonline.com/tenant""#,
            "...",
        ] {
            let err = KeyVaultAuthorizer::parse_scope_from_challenge(challenge)
                .expect_err("challenge contained no audience");
            assert!(err.to_string().contains(challenge));
        }
    }

    #[test]
    fn parse_scope_with_resource_parameter() {
        for challenge in [
            r#"Bearer authorization="https://login.microsoftonline.com/tenant", resource="https://a.b""#,
            r#"Bearer resource="https://a.b", authorization="https://login.microsoftonline.com/tenant""#,
        ] {
            let scope = KeyVaultAuthorizer::parse_scope_from_challenge(challenge).unwrap();
            assert_eq!("https://a.b/.default", scope);
        }
    }

    #[test]
    fn parse_scope_with_scope_parameter() {
        for challenge in [
            r#"Bearer authorization="https://login.microsoftonline.com/tenant", scope="https://a.b/.default""#,
            r#"Bearer scope="https://a.b/.default", authorization="https://login.microsoftonline.com/tenant""#,
        ] {
            let scope = KeyVaultAuthorizer::parse_scope_from_challenge(challenge).unwrap();
            assert_eq!("https://a.b/.default", scope);
        }
    }
}

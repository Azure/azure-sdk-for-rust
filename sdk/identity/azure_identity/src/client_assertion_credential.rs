// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use crate::{
    deserialize, get_authority_host, validate_not_empty, validate_tenant_id, EntraIdErrorResponse,
    EntraIdTokenResponse, TokenCache,
};
use azure_core::{
    credentials::{AccessToken, TokenCredential, TokenRequestOptions},
    error::{ErrorKind, ResultExt},
    http::{
        headers::{self, content_type},
        ClientMethodOptions, ClientOptions, Method, Pipeline, Request, StatusCode, Url,
    },
    time::{Duration, OffsetDateTime},
    Error,
};
use std::{fmt::Debug, str, sync::Arc};
use url::form_urlencoded;

const ASSERTION_TYPE: &str = "urn:ietf:params:oauth:client-assertion-type:jwt-bearer";
const CLIENT_ASSERTION_CREDENTIAL: &str = "ClientAssertionCredential";

/// Enables authentication of a Microsoft Entra service principal using a signed client assertion.
#[derive(Debug)]
pub struct ClientAssertionCredential<C> {
    client_id: String,
    endpoint: Url,
    assertion: C,
    cache: TokenCache,
    pipeline: Pipeline,
}

/// Options for constructing a new [`ClientAssertionCredential`].
#[derive(Debug, Default)]
pub struct ClientAssertionCredentialOptions {
    /// Additional tenants for which the credential may acquire tokens.
    ///
    /// Add the wildcard value "*" to allow the credential to acquire tokens for any tenant in which the application is registered.
    pub additionally_allowed_tenants: Vec<String>,

    /// The base URL for token requests.
    ///
    /// The default is `https://login.microsoftonline.com`.
    pub authority_host: Option<String>,

    /// Should be set true only by applications authenticating in disconnected clouds, or private clouds such as Azure Stack.
    ///
    /// It determines whether the credential requests Microsoft Entra instance metadata
    /// from <https://login.microsoft.com> before authenticating. Setting this to true will skip this request, making
    /// the application responsible for ensuring the configured authority is valid and trustworthy.
    pub disable_instance_discovery: bool,

    /// Options for the credential's HTTP pipeline.
    pub client_options: ClientOptions,
}

#[cfg_attr(target_arch = "wasm32", async_trait::async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait::async_trait)]
/// Represents an entity capable of supplying a client assertion.
pub trait ClientAssertion: Send + Sync + Debug {
    /// Supply the client assertion secret.
    async fn secret(&self, options: Option<ClientMethodOptions<'_>>) -> azure_core::Result<String>;
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
        validate_tenant_id(&tenant_id)?;
        validate_not_empty(&client_id, "no client ID specified")?;
        let options = options.unwrap_or_default();
        let authority_host = get_authority_host(None, options.authority_host)?;
        let endpoint = authority_host
            .join(&format!("/{tenant_id}/oauth2/v2.0/token"))
            .with_context(ErrorKind::DataConversion, || {
                format!("tenant_id {tenant_id} could not be URL encoded")
            })?;
        let pipeline = Pipeline::new(
            option_env!("CARGO_PKG_NAME"),
            option_env!("CARGO_PKG_VERSION"),
            options.client_options.clone(),
            Vec::default(),
            Vec::default(),
            None,
        );
        Ok(Self {
            client_id,
            assertion,
            endpoint,
            cache: TokenCache::new(),
            pipeline,
        })
    }

    async fn get_token_impl(
        &self,
        scopes: &[&str],
        options: Option<TokenRequestOptions<'_>>,
    ) -> azure_core::Result<AccessToken> {
        let mut req = Request::new(self.endpoint.clone(), Method::Post);
        req.insert_header(
            headers::CONTENT_TYPE,
            content_type::APPLICATION_X_WWW_FORM_URLENCODED,
        );
        let options = options.unwrap_or_default();
        let assertion = self
            .assertion
            .secret(Some(options.method_options.to_owned()))
            .await?;
        let encoded: String = form_urlencoded::Serializer::new(String::new())
            .append_pair("client_assertion", assertion.as_str())
            .append_pair("client_assertion_type", ASSERTION_TYPE)
            .append_pair("client_id", self.client_id.as_str())
            .append_pair("grant_type", "client_credentials")
            .append_pair("scope", &scopes.join(" "))
            .finish();
        req.set_body(encoded);

        let ctx = options.method_options.context.to_borrowed();
        let res = self.pipeline.send(&ctx, &mut req).await?;

        match res.status() {
            StatusCode::Ok => {
                let token_response: EntraIdTokenResponse =
                    deserialize(CLIENT_ASSERTION_CREDENTIAL, res).await?;
                Ok(AccessToken::new(
                    token_response.access_token,
                    OffsetDateTime::now_utc() + Duration::seconds(token_response.expires_in),
                ))
            }
            _ => {
                let error_response: EntraIdErrorResponse =
                    deserialize(CLIENT_ASSERTION_CREDENTIAL, res).await?;
                let message = if error_response.error_description.is_empty() {
                    format!("{} authentication failed.", CLIENT_ASSERTION_CREDENTIAL)
                } else {
                    format!(
                        "{} authentication failed. {}",
                        CLIENT_ASSERTION_CREDENTIAL, error_response.error_description
                    )
                };
                Err(Error::message(ErrorKind::Credential, message))
            }
        }
    }
}

#[cfg_attr(target_arch = "wasm32", async_trait::async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait::async_trait)]
impl<C: ClientAssertion> TokenCredential for ClientAssertionCredential<C> {
    async fn get_token(
        &self,
        scopes: &[&str],
        options: Option<TokenRequestOptions<'_>>,
    ) -> azure_core::Result<AccessToken> {
        self.cache
            .get_token(scopes, options, |s, o| self.get_token_impl(s, o))
            .await
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::tests::*;
    use azure_core::{
        authority_hosts::AZURE_PUBLIC_CLOUD,
        http::{
            headers::{self, content_type, Headers},
            Body, BufResponse, Method, Request, Transport,
        },
        Bytes,
    };
    use std::{collections::HashMap, time::SystemTime};
    use time::UtcOffset;
    use url::form_urlencoded;

    pub const FAKE_ASSERTION: &str = "fake assertion";

    pub fn is_valid_request() -> impl Fn(&Request) -> azure_core::Result<()> {
        let expected_url = format!(
            "{}{}/oauth2/v2.0/token",
            AZURE_PUBLIC_CLOUD.as_str(),
            FAKE_TENANT_ID
        );
        move |req: &Request| {
            assert_eq!(Method::Post, req.method());
            assert_eq!(expected_url, req.url().to_string());
            assert_eq!(
                content_type::APPLICATION_X_WWW_FORM_URLENCODED.as_str(),
                req.headers().get_str(&headers::CONTENT_TYPE).unwrap()
            );
            let expected_params = [
                ("client_assertion", FAKE_ASSERTION),
                ("client_assertion_type", ASSERTION_TYPE),
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

    #[derive(Debug)]
    struct MockAssertion {}

    #[cfg_attr(target_arch = "wasm32", async_trait::async_trait(?Send))]
    #[cfg_attr(not(target_arch = "wasm32"), async_trait::async_trait)]
    impl ClientAssertion for MockAssertion {
        async fn secret(&self, _: Option<ClientMethodOptions<'_>>) -> azure_core::Result<String> {
            Ok(FAKE_ASSERTION.to_string())
        }
    }

    #[tokio::test]
    async fn get_token_error() {
        let expected = "error description from the response";
        let mock = MockSts::new(
            vec![BufResponse::from_bytes(
                StatusCode::BadRequest,
                Headers::default(),
                Bytes::from(format!(
                    r#"{{"error":"invalid_request","error_description":"{}","error_codes":[50027],"timestamp":"2025-04-18 16:04:37Z","trace_id":"...","correlation_id":"...","error_uri":"https://login.microsoftonline.com/error?code=50027"}}"#,
                    expected
                )),
            )],
            Some(Arc::new(is_valid_request())),
        );
        let credential = ClientAssertionCredential::new(
            FAKE_TENANT_ID.to_string(),
            FAKE_CLIENT_ID.to_string(),
            MockAssertion {},
            Some(ClientAssertionCredentialOptions {
                client_options: ClientOptions {
                    transport: Some(Transport::new(Arc::new(mock))),
                    ..Default::default()
                },
                ..Default::default()
            }),
        )
        .expect("valid credential");

        let error = credential
            .get_token(LIVE_TEST_SCOPES, None)
            .await
            .expect_err("authentication error");
        assert!(matches!(error.kind(), ErrorKind::Credential));
        assert!(
            error.to_string().contains(expected),
            "expected error description from the response, got '{}'",
            error
        );
    }

    #[tokio::test]
    async fn get_token_success() {
        let mock = MockSts::new(
            vec![BufResponse::from_bytes(
                StatusCode::Ok,
                Headers::default(),
                Bytes::from(format!(
                    r#"{{"access_token":"{}","expires_in":3600,"token_type":"Bearer"}}"#,
                    FAKE_TOKEN
                )),
            )],
            Some(Arc::new(is_valid_request())),
        );
        let credential = ClientAssertionCredential::new(
            FAKE_TENANT_ID.to_string(),
            FAKE_CLIENT_ID.to_string(),
            MockAssertion {},
            Some(ClientAssertionCredentialOptions {
                client_options: ClientOptions {
                    transport: Some(Transport::new(Arc::new(mock))),
                    ..Default::default()
                },
                ..Default::default()
            }),
        )
        .expect("valid credential");

        let token = credential
            .get_token(LIVE_TEST_SCOPES, None)
            .await
            .expect("token");
        assert_eq!(FAKE_TOKEN, token.token.secret());
        assert!(token.expires_on > SystemTime::now());
        assert_eq!(UtcOffset::UTC, token.expires_on.offset());

        // MockSts will return an error if the credential sends another request
        let cached_token = credential
            .get_token(LIVE_TEST_SCOPES, None)
            .await
            .expect("cached token");
        assert_eq!(token.token.secret(), cached_token.token.secret());
        assert_eq!(token.expires_on, cached_token.expires_on);
    }
}

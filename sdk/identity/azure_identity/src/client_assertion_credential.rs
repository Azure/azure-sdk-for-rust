// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use crate::{get_authority_host, validate_not_empty, validate_tenant_id, TokenCache};
use azure_core::{
    credentials::{AccessToken, TokenCredential, TokenRequestOptions},
    error::{ErrorKind, ResultExt},
    http::{
        headers::{self, content_type},
        ClientMethodOptions, ClientOptions, Method, Pipeline, PipelineSendOptions, Request, Url,
    },
};
use std::{fmt::Debug, str, sync::Arc};
use url::form_urlencoded;

const ASSERTION_TYPE: &str = "urn:ietf:params:oauth:client-assertion-type:jwt-bearer";

/// Enables authentication of a Microsoft Entra service principal using a signed client assertion.
#[derive(Debug)]
pub struct ClientAssertionCredential<C> {
    name: &'static str,
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
            tenant_id,
            client_id,
            assertion,
            stringify!(ClientAssertionCredential),
            options,
        )?))
    }

    /// Create a new `ClientAssertionCredential` without wrapping it in an
    /// `Arc`. Intended for use by other credentials in the crate that will
    /// themselves be protected by an `Arc`.
    pub(crate) fn new_exclusive(
        tenant_id: String,
        client_id: String,
        assertion: C,
        name: &'static str,
        options: Option<ClientAssertionCredentialOptions>,
    ) -> azure_core::Result<Self> {
        validate_tenant_id(&tenant_id)?;
        validate_not_empty(&client_id, "no client ID specified")?;
        let options = options.unwrap_or_default();
        let authority_host = get_authority_host(None, options.client_options.cloud.as_deref())?;
        let endpoint = authority_host
            .join(&format!("/{tenant_id}/oauth2/v2.0/token"))
            .with_context_fn(ErrorKind::DataConversion, || {
                format!("tenant_id {tenant_id} could not be URL encoded")
            })?;
        let pipeline = Pipeline::new(
            option_env!("CARGO_PKG_NAME"),
            option_env!("CARGO_PKG_VERSION"),
            options.client_options,
            Vec::default(),
            Vec::default(),
            None,
        );
        Ok(Self {
            name,
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
        let res = self
            .pipeline
            .send(
                &ctx,
                &mut req,
                Some(PipelineSendOptions {
                    skip_checks: true,
                    ..Default::default()
                }),
            )
            .await?;

        crate::handle_entra_response(res)
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
            .map_err(|err| crate::authentication_error(self.name, err))
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::tests::*;
    use azure_core::{
        http::{
            headers::{self, content_type, Headers},
            AsyncRawResponse, Body, Method, RawResponse, Request, StatusCode, Transport,
        },
        Bytes,
    };
    use std::{collections::HashMap, time::SystemTime};
    use time::UtcOffset;
    use url::form_urlencoded;

    pub const FAKE_ASSERTION: &str = "fake assertion";

    pub fn is_valid_request(
        expected_authority: String,
        expected_assertion: Option<String>,
    ) -> impl Fn(&Request) -> azure_core::Result<()> {
        let expected_url = format!("{expected_authority}/oauth2/v2.0/token");
        move |req: &Request| {
            assert_eq!(Method::Post, req.method());
            assert_eq!(expected_url, req.url().to_string());
            assert_eq!(
                content_type::APPLICATION_X_WWW_FORM_URLENCODED.as_str(),
                req.headers().get_str(&headers::CONTENT_TYPE).unwrap()
            );
            let body = match req.body() {
                Body::Bytes(bytes) => str::from_utf8(bytes).unwrap(),
                _ => panic!("unexpected body type"),
            };
            let actual_params: HashMap<String, String> = form_urlencoded::parse(body.as_bytes())
                .map(|(k, v)| (k.to_string(), v.to_string()))
                .collect();
            let assertion = actual_params
                .get("client_assertion")
                .expect("request body should contain client_assertion");
            match &expected_assertion {
                Some(expected) => assert_eq!(expected, assertion),
                None => assert!(
                    !assertion.is_empty(),
                    "expected client_assertion to be present"
                ),
            }
            let expected_params = [
                ("client_assertion_type", ASSERTION_TYPE),
                ("client_id", FAKE_CLIENT_ID),
                ("grant_type", "client_credentials"),
                ("scope", &LIVE_TEST_SCOPES.join(" ")),
            ];
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
        let body = Bytes::from(
            r#"{"error":"invalid_request","error_description":"error description from the response","error_codes":[50027],"timestamp":"2025-04-18 16:04:37Z","trace_id":"...","correlation_id":"...","error_uri":"https://login.microsoftonline.com/error?code=50027"}"#,
        );
        let mut headers = Headers::default();
        headers.insert("key", "value");
        let expected_status = StatusCode::BadRequest;
        let expected_response =
            RawResponse::from_bytes(expected_status, headers.clone(), body.clone());
        let mock_response = AsyncRawResponse::from_bytes(expected_status, headers, body);

        let mock = MockSts::new(
            vec![mock_response],
            Some(Arc::new(is_valid_request(
                FAKE_PUBLIC_CLOUD_AUTHORITY.to_string(),
                Some(FAKE_ASSERTION.to_string()),
            ))),
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

        let err = credential
            .get_token(LIVE_TEST_SCOPES, None)
            .await
            .expect_err("authentication error");
        assert!(matches!(err.kind(), ErrorKind::Credential));
        assert_eq!(
            "ClientAssertionCredential authentication failed. error description from the response",
            err.to_string(),
        );
        match err
            .downcast_ref::<azure_core::Error>()
            .expect("returned error should wrap an azure_core::Error")
            .kind()
        {
            ErrorKind::HttpResponse {
                error_code: Some(error_code),
                raw_response: Some(response),
                status,
            } => {
                assert_eq!("50027", error_code);
                assert_eq!(&expected_response, response.as_ref());
                assert_eq!(expected_status, *status);
            }
            err => panic!("unexpected {:?}", err),
        };
    }

    #[tokio::test]
    async fn get_token_success() {
        let mock = MockSts::new(
            vec![token_response()],
            Some(Arc::new(is_valid_request(
                FAKE_PUBLIC_CLOUD_AUTHORITY.to_string(),
                Some(FAKE_ASSERTION.to_string()),
            ))),
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

    #[tokio::test]
    async fn cloud_configuration() {
        for (cloud, expected_authority) in cloud_configuration_cases() {
            let mock = MockSts::new(
                vec![token_response()],
                Some(Arc::new(is_valid_request(
                    expected_authority,
                    Some(FAKE_ASSERTION.to_string()),
                ))),
            );
            let credential = ClientAssertionCredential::new(
                FAKE_TENANT_ID.to_string(),
                FAKE_CLIENT_ID.to_string(),
                MockAssertion {},
                Some(ClientAssertionCredentialOptions {
                    client_options: ClientOptions {
                        transport: Some(Transport::new(Arc::new(mock))),
                        cloud: Some(Arc::new(cloud)),
                        ..Default::default()
                    },
                    ..Default::default()
                }),
            )
            .expect("valid credential");

            credential
                .get_token(LIVE_TEST_SCOPES, None)
                .await
                .expect("token");
        }
    }
}

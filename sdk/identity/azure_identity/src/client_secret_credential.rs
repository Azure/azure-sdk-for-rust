// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use crate::{
    deserialize, EntraIdErrorResponse, EntraIdTokenResponse, TokenCache, TokenCredentialOptions,
};
use azure_core::credentials::TokenRequestOptions;
use azure_core::http::StatusCode;
use azure_core::Result;
use azure_core::{
    credentials::{AccessToken, Secret, TokenCredential},
    error::{ErrorKind, ResultExt},
    http::{
        headers::{self, content_type},
        Method, Request, Url,
    },
    time::{Duration, OffsetDateTime},
    Error,
};
use std::{str, sync::Arc};
use url::form_urlencoded;

const CLIENT_SECRET_CREDENTIAL: &str = "ClientSecretCredential";

/// Options for constructing a new [`ClientSecretCredential`].
#[derive(Debug, Default)]
pub struct ClientSecretCredentialOptions {
    /// Options for constructing credentials.
    pub credential_options: TokenCredentialOptions,
}

/// Authenticates an application with a client secret.
#[derive(Debug)]
pub struct ClientSecretCredential {
    cache: TokenCache,
    client_id: String,
    endpoint: Url,
    options: TokenCredentialOptions,
    secret: Secret,
}

impl ClientSecretCredential {
    pub fn new(
        tenant_id: &str,
        client_id: String,
        secret: Secret,
        options: Option<ClientSecretCredentialOptions>,
    ) -> Result<Arc<Self>> {
        crate::validate_tenant_id(tenant_id)?;
        crate::validate_not_empty(&client_id, "no client ID specified")?;
        crate::validate_not_empty(secret.secret(), "no secret specified")?;

        let options = options.unwrap_or_default();
        let endpoint = options
            .credential_options
            .authority_host()?
            .join(&format!("/{tenant_id}/oauth2/v2.0/token"))
            .with_context(ErrorKind::DataConversion, || {
                format!("tenant_id '{tenant_id}' could not be URL encoded")
            })?;

        Ok(Arc::new(Self {
            cache: TokenCache::new(),
            client_id,
            endpoint,
            options: options.credential_options,
            secret,
        }))
    }

    async fn get_token_impl(
        &self,
        scopes: &[&str],
        _: Option<TokenRequestOptions>,
    ) -> Result<AccessToken> {
        let mut req = Request::new(self.endpoint.clone(), Method::Post);
        req.insert_header(
            headers::CONTENT_TYPE,
            content_type::APPLICATION_X_WWW_FORM_URLENCODED,
        );
        let body = form_urlencoded::Serializer::new(String::new())
            .append_pair("client_id", &self.client_id)
            .append_pair("client_secret", self.secret.secret())
            .append_pair("grant_type", "client_credentials")
            .append_pair("scope", &scopes.join(" "))
            .finish();
        req.set_body(body);

        let res = self.options.http_client().execute_request(&req).await?;

        match res.status() {
            StatusCode::Ok => {
                let token_response: EntraIdTokenResponse =
                    deserialize(CLIENT_SECRET_CREDENTIAL, res).await?;
                Ok(AccessToken::new(
                    token_response.access_token,
                    OffsetDateTime::now_utc() + Duration::seconds(token_response.expires_in),
                ))
            }
            _ => {
                let error_response: EntraIdErrorResponse =
                    deserialize(CLIENT_SECRET_CREDENTIAL, res).await?;
                let message = if error_response.error_description.is_empty() {
                    format!("{} authentication failed.", CLIENT_SECRET_CREDENTIAL)
                } else {
                    format!(
                        "{} authentication failed. {}",
                        CLIENT_SECRET_CREDENTIAL, error_response.error_description
                    )
                };
                Err(Error::message(ErrorKind::Credential, message))
            }
        }
    }
}

#[cfg_attr(target_arch = "wasm32", async_trait::async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait::async_trait)]
impl TokenCredential for ClientSecretCredential {
    async fn get_token(
        &self,
        scopes: &[&str],
        options: Option<TokenRequestOptions>,
    ) -> Result<AccessToken> {
        if scopes.is_empty() {
            return Err(Error::message(ErrorKind::Credential, "no scopes specified"));
        }
        self.cache
            .get_token(scopes, options, |s, o| self.get_token_impl(s, o))
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::*;
    use azure_core::{
        authority_hosts::AZURE_PUBLIC_CLOUD,
        http::{headers::Headers, RawResponse, StatusCode},
        Bytes, Result,
    };
    use std::vec;

    const FAKE_SECRET: &str = "fake secret";

    fn is_valid_request(authority_host: &str, tenant_id: &str) -> impl Fn(&Request) -> Result<()> {
        let expected_url = format!("{}{}/oauth2/v2.0/token", authority_host, tenant_id);
        move |req: &Request| {
            assert_eq!(&Method::Post, req.method());
            assert_eq!(expected_url, req.url().to_string());
            assert_eq!(
                req.headers().get_str(&headers::CONTENT_TYPE).unwrap(),
                content_type::APPLICATION_X_WWW_FORM_URLENCODED.as_str()
            );
            Ok(())
        }
    }

    #[tokio::test]
    async fn get_token_error() {
        let description = "AADSTS7000215: Invalid client secret.";
        let sts = MockSts::new(
            vec![RawResponse::from_bytes(
                StatusCode::BadRequest,
                Headers::default(),
                Bytes::from(format!(
                    r#"{{"error":"invalid_client","error_description":"{}","error_codes":[7000215],"timestamp":"2025-04-04 21:10:04Z","trace_id":"...","correlation_id":"...","error_uri":"https://login.microsoftonline.com/error?code=7000215"}}"#,
                    description
                )),
            )],
            Some(Arc::new(is_valid_request(
                AZURE_PUBLIC_CLOUD.as_str(),
                FAKE_TENANT_ID,
            ))),
        );
        let cred = ClientSecretCredential::new(
            FAKE_TENANT_ID,
            FAKE_CLIENT_ID.to_string(),
            FAKE_SECRET.into(),
            Some(ClientSecretCredentialOptions {
                credential_options: TokenCredentialOptions {
                    http_client: Arc::new(sts),
                    ..Default::default()
                },
            }),
        )
        .expect("valid credential");

        let err = cred
            .get_token(LIVE_TEST_SCOPES, None)
            .await
            .expect_err("expected error");
        assert!(matches!(err.kind(), ErrorKind::Credential));
        assert!(
            err.to_string().contains(description),
            "expected error description from the response, got '{}'",
            err
        );
    }

    #[tokio::test]
    async fn get_token_success() {
        let expires_in = 3600;
        let sts = MockSts::new(
            vec![RawResponse::from_bytes(
                StatusCode::Ok,
                Headers::default(),
                Bytes::from(format!(
                    r#"{{"access_token":"{}","expires_in":{},"token_type":"Bearer"}}"#,
                    FAKE_TOKEN, expires_in
                )),
            )],
            Some(Arc::new(is_valid_request(
                AZURE_PUBLIC_CLOUD.as_str(),
                FAKE_TENANT_ID,
            ))),
        );
        let cred = ClientSecretCredential::new(
            FAKE_TENANT_ID,
            FAKE_CLIENT_ID.to_string(),
            FAKE_SECRET.into(),
            Some(ClientSecretCredentialOptions {
                credential_options: TokenCredentialOptions {
                    http_client: Arc::new(sts),
                    ..Default::default()
                },
            }),
        )
        .expect("valid credential");
        let token = cred.get_token(LIVE_TEST_SCOPES, None).await.expect("token");

        assert_eq!(FAKE_TOKEN, token.token.secret());

        // allow a small margin when validating expiration time because it's computed as
        // the current time plus a number of seconds (expires_in) and the system clock
        // may have ticked into the next second since we assigned expires_in above
        let lifetime =
            token.expires_on.unix_timestamp() - OffsetDateTime::now_utc().unix_timestamp();
        assert!(
            (expires_in..expires_in + 1).contains(&lifetime),
            "token should expire in ~{} seconds but actually expires in {} seconds",
            expires_in,
            lifetime
        );

        // sts will return an error if the credential sends another request
        let cached_token = cred
            .get_token(LIVE_TEST_SCOPES, None)
            .await
            .expect("cached token");
        assert_eq!(token.token.secret(), cached_token.token.secret());
        assert_eq!(token.expires_on, cached_token.expires_on);
    }

    #[test]
    fn invalid_tenant_id() {
        ClientSecretCredential::new(
            "not a valid tenant",
            FAKE_CLIENT_ID.to_string(),
            FAKE_SECRET.into(),
            None,
        )
        .expect_err("invalid tenant ID");
    }

    #[tokio::test]
    async fn no_scopes() {
        ClientSecretCredential::new(
            FAKE_TENANT_ID,
            FAKE_CLIENT_ID.to_string(),
            FAKE_SECRET.into(),
            None,
        )
        .expect("valid credential")
        .get_token(&[], None)
        .await
        .expect_err("no scopes specified");
    }
}

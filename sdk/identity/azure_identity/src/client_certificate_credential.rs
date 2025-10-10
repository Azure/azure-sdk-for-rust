// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use crate::{
    authentication_error, deserialize, get_authority_host, EntraIdErrorResponse,
    EntraIdTokenResponse, TokenCache,
};
use azure_core::{
    base64,
    credentials::{AccessToken, Secret, TokenCredential, TokenRequestOptions},
    error::{Error, ErrorKind, ResultExt},
    http::{
        headers::{self, content_type},
        request::Request,
        ClientOptions, Method, Pipeline, PipelineSendOptions, StatusCode, Url,
    },
    time::{Duration, OffsetDateTime},
    Uuid,
};

// cspell:ignore pkey
use openssl::{
    error::ErrorStack,
    hash::{hash, DigestBytes, MessageDigest},
    pkcs12::Pkcs12,
    pkey::{PKey, Private},
    sign::Signer,
    x509::X509,
};
use std::{str, sync::Arc};
use url::form_urlencoded;

/// Refresh time to use in seconds.
const DEFAULT_REFRESH_TIME: i64 = 300;

const AZURE_CLIENT_SEND_CERTIFICATE_CHAIN_ENV_KEY: &str = "AZURE_CLIENT_SEND_CERTIFICATE_CHAIN";

/// Provides options to configure how the Identity library makes authentication
/// requests to Azure Active Directory.
#[derive(Clone, Debug)]
pub struct ClientCertificateCredentialOptions {
    /// Options for the credential's HTTP pipeline.
    pub client_options: ClientOptions,

    /// Whether to send the certificate chain.
    pub send_certificate_chain: bool,
}

impl Default for ClientCertificateCredentialOptions {
    fn default() -> Self {
        let send_certificate_chain = std::env::var(AZURE_CLIENT_SEND_CERTIFICATE_CHAIN_ENV_KEY)
            .map(|s| s == "1" || s.to_lowercase() == "true")
            .unwrap_or(false);
        Self {
            client_options: ClientOptions::default(),
            send_certificate_chain,
        }
    }
}

/// Enables authentication to Azure Active Directory using a client certificate that
/// was generated for an App Registration.
///
/// In order to use subject name validation `send_cert_chain` option must be set to true
/// The certificate is expected to be in base64 encoded PKCS12 format.
#[derive(Debug)]
pub struct ClientCertificateCredential {
    client_id: String,
    certificate: Secret,
    password: Secret,
    endpoint: Url,
    pipeline: Pipeline,
    send_certificate_chain: bool,
    cache: TokenCache,
}

impl ClientCertificateCredential {
    /// Create a new `ClientCertificateCredential`.
    pub fn new<C, P>(
        tenant_id: String,
        client_id: String,
        client_certificate: C,
        client_certificate_password: P,
        options: Option<ClientCertificateCredentialOptions>,
    ) -> azure_core::Result<Arc<ClientCertificateCredential>>
    where
        C: Into<Secret>,
        P: Into<Secret>,
    {
        let options = options.unwrap_or_default();
        let authority_host = get_authority_host(None, options.client_options.cloud.as_deref())?;
        let endpoint = authority_host
            .join(&format!("/{tenant_id}/oauth2/v2.0/token"))
            .with_context_fn(ErrorKind::DataConversion, || {
                format!("tenant_id '{tenant_id}' could not be URL encoded")
            })?;

        let pipeline = Pipeline::new(
            option_env!("CARGO_PKG_NAME"),
            option_env!("CARGO_PKG_VERSION"),
            options.client_options,
            Vec::default(),
            Vec::default(),
            None,
        );

        Ok(Arc::new(ClientCertificateCredential {
            client_id,
            certificate: client_certificate.into(),
            password: client_certificate_password.into(),
            endpoint,
            pipeline,
            send_certificate_chain: options.send_certificate_chain,
            cache: TokenCache::new(),
        }))
    }

    fn sign(jwt: &str, pkey: &PKey<Private>) -> Result<Vec<u8>, ErrorStack> {
        let mut signer = Signer::new(MessageDigest::sha256(), pkey)?;
        signer.update(jwt.as_bytes())?;
        signer.sign_to_vec()
    }

    fn get_thumbprint(cert: &X509) -> Result<DigestBytes, ErrorStack> {
        let der = cert.to_der()?;
        let digest = hash(MessageDigest::sha1(), &der)?;
        Ok(digest)
    }

    fn as_jwt_part(part: &[u8]) -> String {
        base64::encode_url_safe(part)
    }

    async fn get_token_impl(
        &self,
        scopes: &[&str],
        options: Option<TokenRequestOptions<'_>>,
    ) -> azure_core::Result<AccessToken> {
        let certificate = base64::decode(self.certificate.secret())
            .map_err(|_| Error::with_message(ErrorKind::Credential, "Base64 decode failed"))?;

        let pkcs12_certificate = Pkcs12::from_der(&certificate)
            .map_err(openssl_error)?
            .parse2(self.password.secret())
            .map_err(openssl_error)?;

        let Some(cert) = pkcs12_certificate.cert.as_ref() else {
            return Err(Error::with_message(
                ErrorKind::Credential,
                "Certificate not found",
            ));
        };

        let Some(pkey) = pkcs12_certificate.pkey.as_ref() else {
            return Err(Error::with_message(
                ErrorKind::Credential,
                "Private key not found",
            ));
        };

        let thumbprint =
            ClientCertificateCredential::get_thumbprint(cert).map_err(openssl_error)?;

        let uuid = Uuid::new_v4();
        let current_time = OffsetDateTime::now_utc().unix_timestamp();
        let expiry_time = current_time + DEFAULT_REFRESH_TIME;
        let x5t = base64::encode(thumbprint);

        let header = match self.send_certificate_chain {
            true => {
                let base_signature = get_encoded_cert(cert)?;
                let x5c = match pkcs12_certificate.ca {
                    Some(chain) => {
                        let chain = chain
                            .into_iter()
                            .map(|x| get_encoded_cert(&x))
                            .collect::<azure_core::Result<Vec<String>>>()?
                            .join(",");
                        format! {"{},{}", base_signature, chain}
                    }
                    None => base_signature,
                };
                format!(
                    r#"{{"alg":"RS256","typ":"JWT", "x5t":"{}", "x5c":[{}]}}"#,
                    x5t, x5c
                )
            }
            false => format!(r#"{{"alg":"RS256","typ":"JWT", "x5t":"{}"}}"#, x5t),
        };
        let header = ClientCertificateCredential::as_jwt_part(header.as_bytes());

        let payload = format!(
            r#"{{"aud":"{}","exp":{},"iss": "{}", "jti": "{}", "nbf": {}, "sub": "{}"}}"#,
            self.endpoint, expiry_time, self.client_id, uuid, current_time, self.client_id
        );
        let payload = ClientCertificateCredential::as_jwt_part(payload.as_bytes());

        let jwt = format!("{}.{}", header, payload);
        let signature = ClientCertificateCredential::sign(&jwt, pkey).map_err(openssl_error)?;
        let sig = ClientCertificateCredential::as_jwt_part(&signature);
        let client_assertion = format!("{}.{}", jwt, sig);

        let encoded = {
            let mut encoded = &mut form_urlencoded::Serializer::new(String::new());
            encoded = encoded
                .append_pair("client_id", self.client_id.as_str())
                .append_pair("scope", &scopes.join(" "))
                .append_pair(
                    "client_assertion_type",
                    "urn:ietf:params:oauth:client-assertion-type:jwt-bearer",
                )
                .append_pair("client_assertion", client_assertion.as_str())
                .append_pair("grant_type", "client_credentials");
            encoded.finish()
        };

        let mut req = Request::new(self.endpoint.clone(), Method::Post);
        req.insert_header(
            headers::CONTENT_TYPE,
            content_type::APPLICATION_X_WWW_FORM_URLENCODED,
        );
        req.set_body(encoded);

        let options = options.unwrap_or_default();
        let ctx = options.method_options.context.to_borrowed();
        let rsp = self
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

        match rsp.status() {
            StatusCode::Ok => {
                let response: EntraIdTokenResponse =
                    deserialize(stringify!(ClientCertificateCredential), rsp)?;
                Ok(AccessToken::new(
                    response.access_token,
                    OffsetDateTime::now_utc() + Duration::seconds(response.expires_in),
                ))
            }
            _ => {
                let error_response: EntraIdErrorResponse =
                    deserialize(stringify!(ClientCertificateCredential), rsp)?;
                let message = if error_response.error_description.is_empty() {
                    "authentication failed".to_string()
                } else {
                    error_response.error_description.clone()
                };
                Err(Error::with_message(ErrorKind::Credential, message))
            }
        }
    }
}

fn get_encoded_cert(cert: &X509) -> azure_core::Result<String> {
    Ok(format!(
        "\"{}\"",
        base64::encode(cert.to_pem().map_err(openssl_error)?)
    ))
}

fn openssl_error(err: ErrorStack) -> azure_core::error::Error {
    Error::new(ErrorKind::Credential, err)
}

#[cfg_attr(target_arch = "wasm32", async_trait::async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait::async_trait)]
impl TokenCredential for ClientCertificateCredential {
    async fn get_token(
        &self,
        scopes: &[&str],
        options: Option<TokenRequestOptions<'_>>,
    ) -> azure_core::Result<AccessToken> {
        if scopes.is_empty() {
            return Err(Error::with_message(
                ErrorKind::Credential,
                "no scopes specified",
            ));
        }
        self.cache
            .get_token(scopes, options, |s, o| self.get_token_impl(s, o))
            .await
            .map_err(authentication_error::<Self>)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        client_assertion_credential::tests::is_valid_request, tests::*, TSG_LINK_ERROR_TEXT,
    };
    use azure_core::{
        http::{headers::Headers, BufResponse, StatusCode, Transport},
        Bytes,
    };
    use std::sync::{Arc, LazyLock};

    static TEST_CERT: LazyLock<String> = LazyLock::new(|| {
        let pfx = std::fs::read(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/certificate.pfx"
        ))
        .expect("failed to read test certificate");
        base64::encode(pfx)
    });

    #[tokio::test]
    async fn cloud_configuration() {
        for (cloud, expected_authority) in cloud_configuration_cases() {
            let sts = MockSts::new(
                vec![token_response()],
                Some(Arc::new(is_valid_request(expected_authority, None))),
            );
            let credential = ClientCertificateCredential::new(
                FAKE_TENANT_ID.to_string(),
                FAKE_CLIENT_ID.to_string(),
                Secret::new(TEST_CERT.to_string()),
                Secret::new(""),
                Some(ClientCertificateCredentialOptions {
                    client_options: ClientOptions {
                        transport: Some(Transport::new(Arc::new(sts))),
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

    #[tokio::test]
    async fn get_token_error() {
        let description = "AADSTS7000215: Invalid client certificate.";
        let sts = MockSts::new(
            vec![BufResponse::from_bytes(
                StatusCode::BadRequest,
                Headers::default(),
                Bytes::from(format!(
                    r#"{{"error":"invalid_client","error_description":"{description}","error_codes":[7000215],"timestamp":"2025-04-04 21:10:04Z","trace_id":"...","correlation_id":"...","error_uri":"https://login.microsoftonline.com/error?code=7000215"}}"#,
                )),
            )],
            Some(Arc::new(is_valid_request(
                FAKE_PUBLIC_CLOUD_AUTHORITY.to_string(),
                None,
            ))),
        );
        let credential = ClientCertificateCredential::new(
            FAKE_TENANT_ID.to_string(),
            FAKE_CLIENT_ID.to_string(),
            TEST_CERT.to_string(),
            Secret::new(""),
            Some(ClientCertificateCredentialOptions {
                client_options: ClientOptions {
                    transport: Some(Transport::new(Arc::new(sts))),
                    ..Default::default()
                },
                ..Default::default()
            }),
        )
        .expect("valid credential");

        let err = credential
            .get_token(LIVE_TEST_SCOPES, None)
            .await
            .expect_err("expected error");
        assert!(matches!(err.kind(), ErrorKind::Credential));
        assert!(
            err.to_string().contains(description),
            "expected error description from the response, got '{}'",
            err
        );
        assert!(
            err.to_string()
                .contains(&format!("{TSG_LINK_ERROR_TEXT}#client-cert")),
            "expected error to contain a link to the troubleshooting guide, got '{err}'",
        );
    }

    #[tokio::test]
    async fn get_token_success() {
        let sts = MockSts::new(
            vec![token_response()],
            Some(Arc::new(is_valid_request(
                FAKE_PUBLIC_CLOUD_AUTHORITY.to_string(),
                None,
            ))),
        );
        let credential = ClientCertificateCredential::new(
            FAKE_TENANT_ID.to_string(),
            FAKE_CLIENT_ID.to_string(),
            TEST_CERT.to_string(),
            Secret::new(""),
            Some(ClientCertificateCredentialOptions {
                client_options: ClientOptions {
                    transport: Some(Transport::new(Arc::new(sts))),
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
        let lifetime =
            token.expires_on.unix_timestamp() - OffsetDateTime::now_utc().unix_timestamp();
        assert!(
            (3600..3601).contains(&lifetime),
            "token should expire in ~3600 seconds but actually expires in {} seconds",
            lifetime
        );

        let cached_token = credential
            .get_token(LIVE_TEST_SCOPES, None)
            .await
            .expect("cached token");
        assert_eq!(token.token.secret(), cached_token.token.secret());
        assert_eq!(token.expires_on, cached_token.expires_on);
    }

    #[tokio::test]
    async fn no_scopes() {
        ClientCertificateCredential::new(
            FAKE_TENANT_ID.to_string(),
            FAKE_CLIENT_ID.to_string(),
            TEST_CERT.to_string(),
            Secret::new(""),
            None,
        )
        .expect("valid credential")
        .get_token(&[], None)
        .await
        .expect_err("no scopes provided");
    }
}

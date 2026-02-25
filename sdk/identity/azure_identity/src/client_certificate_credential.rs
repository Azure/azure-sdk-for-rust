// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use crate::{
    authentication_error, env::Env, get_authority_host, secret_bytes::SecretBytes,
    validate_not_empty, validate_tenant_id, TokenCache,
};
use azure_core::{
    base64,
    credentials::{AccessToken, Secret, TokenCredential, TokenRequestOptions},
    error::{Error, ErrorKind, ResultExt},
    http::{
        headers::{self, content_type},
        request::Request,
        ClientOptions, Method, Pipeline, PipelineSendOptions, Url,
    },
    time::OffsetDateTime,
    Uuid,
};

// cspell:ignore pkey
use openssl::{
    error::ErrorStack,
    hash::MessageDigest,
    pkcs12::Pkcs12,
    pkey::{Id, PKey, Private},
    sign::Signer,
    x509::X509,
};
use std::sync::Arc;
use url::form_urlencoded;

const DEFAULT_ASSERTION_LIFETIME: i64 = 300;

const AZURE_CLIENT_SEND_CERTIFICATE_CHAIN_ENV_KEY: &str = "AZURE_CLIENT_SEND_CERTIFICATE_CHAIN";

/// Options for constructing a new [`ClientCertificateCredential`].
#[derive(Clone, Debug, Default)]
pub struct ClientCertificateCredentialOptions {
    /// Options for the credential's HTTP pipeline.
    pub client_options: ClientOptions,

    /// The password for the certificate.
    pub password: Option<Secret>,

    #[cfg(test)]
    pub(crate) env: Option<Env>,
}

/// Authenticates an application with a certificate.
#[derive(Debug)]
pub struct ClientCertificateCredential {
    client_id: String,
    key: PKey<Private>,
    endpoint: Url,
    pipeline: Pipeline,
    header: String,
    cache: TokenCache,
}

impl ClientCertificateCredential {
    /// Create a new `ClientCertificateCredential`.
    ///
    /// # Arguments
    /// - `tenant_id`: The tenant (directory) ID of the service principal.
    /// - `client_id`: The client (application) ID of the service principal.
    /// - `certificate`: The PKCS12 certificate bytes with its RSA private key.
    /// - `options`: Options for configuring the credential. If `None`, the credential uses its default options.
    ///
    pub fn new(
        tenant_id: String,
        client_id: String,
        certificate: SecretBytes,
        options: Option<ClientCertificateCredentialOptions>,
    ) -> azure_core::Result<Arc<ClientCertificateCredential>> {
        validate_tenant_id(&tenant_id)?;
        validate_not_empty(&client_id, "no client ID specified")?;

        let options = options.unwrap_or_default();

        let (key, cert, ca_chain) =
            parse_certificate(certificate.bytes(), options.password.as_ref())?;
        let thumbprint = cert
            .digest(MessageDigest::sha1())
            .with_context(ErrorKind::Credential, "failed to compute thumbprint")?
            .to_vec();
        let thumbprint = base64::encode(thumbprint);

        #[cfg(test)]
        let env = options.env.unwrap_or_default();
        #[cfg(not(test))]
        let env = Env::default();

        let send_x5c = env
            .var(AZURE_CLIENT_SEND_CERTIFICATE_CHAIN_ENV_KEY)
            .map(|s| s == "1" || s.eq_ignore_ascii_case("true"))
            .unwrap_or(false);
        let header = if send_x5c {
            let base_signature = get_encoded_cert(&cert)?;
            let x5c = match &ca_chain {
                Some(chain) => {
                    let chain = chain
                        .iter()
                        .map(get_encoded_cert)
                        .collect::<azure_core::Result<Vec<String>>>()?
                        .join(",");
                    format!("{base_signature},{chain}")
                }
                None => base_signature,
            };
            format!(r#"{{"alg":"RS256","typ":"JWT","x5c":[{x5c}],"x5t":"{thumbprint}"}}"#)
        } else {
            format!(r#"{{"alg":"RS256","typ":"JWT","x5t":"{thumbprint}"}}"#)
        };

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
            key,
            endpoint,
            pipeline,
            header: ClientCertificateCredential::as_jwt_part(header.as_bytes()),
            cache: TokenCache::new(),
        }))
    }

    fn sign(jwt: &str, pkey: &PKey<Private>) -> Result<Vec<u8>, ErrorStack> {
        let mut signer = Signer::new(MessageDigest::sha256(), pkey)?;
        signer.update(jwt.as_bytes())?;
        signer.sign_to_vec()
    }

    fn as_jwt_part(part: &[u8]) -> String {
        base64::encode_url_safe(part)
    }

    async fn get_token_impl(
        &self,
        scopes: &[&str],
        options: Option<TokenRequestOptions<'_>>,
    ) -> azure_core::Result<AccessToken> {
        let uuid = Uuid::new_v4();
        let current_time = OffsetDateTime::now_utc().unix_timestamp();
        let expiry_time = current_time + DEFAULT_ASSERTION_LIFETIME;
        let payload = format!(
            r#"{{"aud":"{}","exp":{},"iss": "{}", "jti": "{}", "nbf": {}, "sub": "{}"}}"#,
            self.endpoint, expiry_time, self.client_id, uuid, current_time, self.client_id
        );
        let payload = ClientCertificateCredential::as_jwt_part(payload.as_bytes());

        let jwt = format!("{}.{}", self.header, payload);
        let signature = ClientCertificateCredential::sign(&jwt, &self.key)
            .with_context(ErrorKind::Credential, "failed to sign JWT")?;
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

        crate::handle_entra_response(rsp)
    }
}

/// Parse a PKCS12 certificate into key, certificate, and optional CA chain.
fn parse_certificate(
    cert_bytes: &[u8],
    password: Option<&Secret>,
) -> azure_core::Result<(PKey<Private>, X509, Option<Vec<X509>>)> {
    let pkcs12 = Pkcs12::from_der(cert_bytes).with_context(
        ErrorKind::Credential,
        "deserializing PKCS12 from DER failed",
    )?;
    let parsed = pkcs12
        .parse2(password.map(|p| p.secret()).unwrap_or(""))
        .with_context(ErrorKind::Credential, "PKCS12 parsing failed")?;
    let key = parsed.pkey.ok_or_else(|| {
        Error::with_message(
            ErrorKind::Credential,
            "PKCS12 bundle contains no private key",
        )
    })?;
    if key.id() != Id::RSA {
        return Err(Error::with_message(
            ErrorKind::Credential,
            "only RSA private keys are supported",
        ));
    }
    let cert = parsed.cert.ok_or_else(|| {
        Error::with_message(
            ErrorKind::Credential,
            "PKCS12 bundle contains no certificate",
        )
    })?;
    let ca_chain = parsed.ca.and_then(|stack| {
        let certs: Vec<X509> = stack.into_iter().collect();
        if certs.is_empty() {
            None
        } else {
            Some(certs)
        }
    });

    Ok((key, cert, ca_chain))
}

fn get_encoded_cert(cert: &X509) -> azure_core::Result<String> {
    Ok(format!(
        "\"{}\"",
        base64::encode(
            cert.to_pem()
                .with_context(ErrorKind::Credential, "PEM encoding failed")?
        )
    ))
}

#[async_trait::async_trait]
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
            .map_err(|err| authentication_error(stringify!(ClientCertificateCredential), err))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        client_assertion_credential::tests::is_valid_request, secret_bytes::SecretBytes, tests::*,
    };
    use azure_core::{
        http::{
            headers::Headers,
            policies::{Policy, PolicyResult},
            AsyncRawResponse, Context, RawResponse, StatusCode, Transport,
        },
        Bytes,
    };
    use openssl::{pkey::Public, sign::Verifier};
    use std::{
        collections::HashMap,
        sync::{Arc, LazyLock},
    };

    static TEST_CERT: LazyLock<Vec<u8>> = LazyLock::new(|| {
        std::fs::read(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/certificate.pfx"
        ))
        .expect("failed to read test certificate")
    });

    #[derive(Debug, Clone)]
    struct VerifyAssertionPolicy {
        public_key: PKey<Public>,
        cert_der: Vec<u8>,
        expect_x5c: bool,
    }

    impl VerifyAssertionPolicy {
        fn new(certificate: &[u8], expect_x5c: bool) -> Self {
            let (_, cert, _) = parse_certificate(certificate, None).expect("valid certificate");
            let public_key = cert.public_key().expect("public key");
            let cert_der = cert.to_der().expect("valid certificate");
            Self {
                public_key,
                cert_der,
                expect_x5c,
            }
        }
    }

    #[async_trait::async_trait]
    impl Policy for VerifyAssertionPolicy {
        async fn send(
            &self,
            ctx: &Context,
            request: &mut Request,
            next: &[Arc<dyn Policy>],
        ) -> PolicyResult {
            if !request.url().path().ends_with("/token") {
                return next[0].send(ctx, request, &next[1..]).await;
            }

            let body = request.body();
            let body_bytes: Bytes = body.into();

            let params: HashMap<String, String> = form_urlencoded::parse(body_bytes.as_ref())
                .map(|(k, v)| (k.to_string(), v.to_string()))
                .collect();

            let client_assertion = params
                .get("client_assertion")
                .expect("client_assertion should be present");

            let parts: Vec<_> = client_assertion.split('.').collect();
            let [header, payload, signature] = parts.as_slice() else {
                panic!("JWT should have 3 parts");
            };

            let header_bytes =
                base64::decode_url_safe(header).expect("header should be base64url encoded");
            let header_str = std::str::from_utf8(&header_bytes).expect("header should be UTF-8");

            let header_json: serde_json::Value =
                serde_json::from_str(header_str).expect("header should be JSON");

            if self.expect_x5c {
                let x5c_array = header_json
                    .get("x5c")
                    .expect("header should contain x5c field")
                    .as_array()
                    .expect("x5c should be an array");

                assert!(!x5c_array.is_empty(), "x5c array should not be empty");

                let x5c_cert_base64 = x5c_array[0]
                    .as_str()
                    .expect("x5c certificate should be a string");
                let x5c_cert_pem = base64::decode(x5c_cert_base64)
                    .expect("x5c certificate should be valid base64");
                let x5c_cert =
                    X509::from_pem(&x5c_cert_pem).expect("x5c certificate should be valid PEM");

                let x5c_der = x5c_cert
                    .to_der()
                    .expect("x5c certificate should convert to DER");
                assert_eq!(
                    self.cert_der, x5c_der,
                    "the first certificate in x5c should match the certificate provided to ClientCertificateCredential::new()"
                );
            } else {
                assert!(
                    header_json.get("x5c").is_none(),
                    "header should not contain x5c field when expect_x5c is false"
                );
            }

            let signature_bytes =
                base64::decode_url_safe(signature).expect("signature should be base64url encoded");

            let mut verifier =
                Verifier::new(openssl::hash::MessageDigest::sha256(), &self.public_key)
                    .expect("verifier creation should succeed");
            verifier
                .update(format!("{header}.{payload}").as_bytes())
                .expect("verifier update should succeed");
            let verified = verifier
                .verify(&signature_bytes)
                .expect("verification should complete");

            assert!(
                verified,
                "JWT signature should verify with the certificate's public key"
            );

            next[0].send(ctx, request, &next[1..]).await
        }
    }

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
                SecretBytes::from(TEST_CERT.as_slice()),
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
        let body = Bytes::from(
            r#"{"error":"invalid_client","error_description":"AADSTS7000215: Invalid client certificate.","error_codes":[7000215],"timestamp":"2025-04-04 21:10:04Z","trace_id":"...","correlation_id":"...","error_uri":"https://login.microsoftonline.com/error?code=7000215"}"#,
        );
        let expected_status = StatusCode::BadRequest;
        let headers = Headers::default();
        let expected_response =
            RawResponse::from_bytes(expected_status, headers.clone(), body.clone());
        let sts = MockSts::new(
            vec![AsyncRawResponse::from_bytes(expected_status, headers, body)],
            Some(Arc::new(is_valid_request(
                FAKE_PUBLIC_CLOUD_AUTHORITY.to_string(),
                None,
            ))),
        );
        let credential = ClientCertificateCredential::new(
            FAKE_TENANT_ID.to_string(),
            FAKE_CLIENT_ID.to_string(),
            TEST_CERT.as_slice().into(),
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
        assert_eq!(
            "ClientCertificateCredential authentication failed. AADSTS7000215: Invalid client certificate.\nTo troubleshoot, visit https://aka.ms/azsdk/rust/identity/troubleshoot#client-cert",
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
                assert_eq!("7000215", error_code);
                assert_eq!(&expected_response, response.as_ref());
                assert_eq!(expected_status, *status);
            }
            err => panic!("unexpected {:?}", err),
        };
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
            TEST_CERT.as_slice().into(),
            Some(ClientCertificateCredentialOptions {
                client_options: ClientOptions {
                    transport: Some(Transport::new(Arc::new(sts))),
                    per_try_policies: vec![Arc::new(VerifyAssertionPolicy::new(&TEST_CERT, false))],
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

    #[test]
    fn invalid_certificate() {
        ClientCertificateCredential::new(
            FAKE_TENANT_ID.to_string(),
            FAKE_CLIENT_ID.to_string(),
            b"not a certificate".as_slice().into(),
            None,
        )
        .expect_err("invalid certificate");
    }

    #[test]
    fn invalid_tenant_id() {
        ClientCertificateCredential::new(
            "not a valid tenant".to_string(),
            FAKE_CLIENT_ID.to_string(),
            TEST_CERT.as_slice().into(),
            None,
        )
        .expect_err("invalid tenant ID");
    }

    #[tokio::test]
    async fn no_scopes() {
        ClientCertificateCredential::new(
            FAKE_TENANT_ID.to_string(),
            FAKE_CLIENT_ID.to_string(),
            TEST_CERT.as_slice().into(),
            None,
        )
        .expect("valid credential")
        .get_token(&[], None)
        .await
        .expect_err("no scopes provided");
    }

    #[tokio::test]
    async fn sni() {
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
            TEST_CERT.as_slice().into(),
            Some(ClientCertificateCredentialOptions {
                client_options: ClientOptions {
                    transport: Some(Transport::new(Arc::new(sts))),
                    per_try_policies: vec![Arc::new(VerifyAssertionPolicy::new(&TEST_CERT, true))],
                    ..Default::default()
                },
                env: Some(Env::from(
                    &[(AZURE_CLIENT_SEND_CERTIFICATE_CHAIN_ENV_KEY, "true")][..],
                )),
                ..Default::default()
            }),
        )
        .expect("credential");

        let token = credential
            .get_token(LIVE_TEST_SCOPES, None)
            .await
            .expect("token");

        assert_eq!(FAKE_TOKEN, token.token.secret());
    }
}

use crate::token_credentials::cache::TokenCache;
use azure_core::{
    auth::{AccessToken, Secret, TokenCredential},
    authority_hosts::AZURE_PUBLIC_CLOUD,
    base64, content_type,
    error::{Error, ErrorKind},
    headers, new_http_client, HttpClient, Method, Request,
};
use openssl::{
    error::ErrorStack,
    hash::{hash, DigestBytes, MessageDigest},
    pkcs12::Pkcs12,
    pkey::{PKey, Private},
    sign::Signer,
    x509::X509,
};
use serde::Deserialize;
use std::{str, sync::Arc, time::Duration};
use time::OffsetDateTime;
use url::{form_urlencoded, Url};

/// Refresh time to use in seconds
const DEFAULT_REFRESH_TIME: i64 = 300;

/// Provides options to configure how the Identity library makes authentication
/// requests to Azure Active Directory.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CertificateCredentialOptions {
    authority_host: Url,
    send_certificate_chain: bool,
}

impl Default for CertificateCredentialOptions {
    fn default() -> Self {
        Self {
            authority_host: AZURE_PUBLIC_CLOUD.to_owned(),
            send_certificate_chain: false,
        }
    }
}

impl CertificateCredentialOptions {
    /// Create a new `TokenCredentialsOptions`. default() may also be used.
    pub fn new(authority_host: Url, send_certificate_chain: bool) -> Self {
        Self {
            authority_host,
            send_certificate_chain,
        }
    }
    /// Set the authority host for authentication requests.
    pub fn set_authority_host(&mut self, authority_host: Url) {
        self.authority_host = authority_host;
    }

    /// The authority host to use for authentication requests.  The default is
    /// <https://login.microsoftonline.com>.
    pub fn authority_host(&self) -> &Url {
        &self.authority_host
    }

    /// Enable/disable sending the certificate chain
    pub fn set_send_certificate_chain(&mut self, send_certificate_chain: bool) {
        self.send_certificate_chain = send_certificate_chain;
    }

    /// Whether certificate chain is sent as part of the request or not. Default is
    /// set to true
    pub fn send_certificate_chain(&self) -> bool {
        self.send_certificate_chain
    }
}

/// Enables authentication to Azure Active Directory using a client certificate that
/// was generated for an App Registration.
///
/// In order to use subject name validation `send_cert_chain` option must be set to true
/// The certificate is expected to be in base64 encoded PKCS12 format
#[derive(Debug)]
pub struct ClientCertificateCredential {
    tenant_id: String,
    client_id: String,
    client_certificate: Secret,
    client_certificate_pass: Secret,
    http_client: Arc<dyn HttpClient>,
    options: CertificateCredentialOptions,
    cache: TokenCache,
}

impl ClientCertificateCredential {
    /// Create a new `ClientCertificateCredential`
    pub fn new<C, P>(
        tenant_id: String,
        client_id: String,
        client_certificate: C,
        client_certificate_pass: P,
        options: CertificateCredentialOptions,
    ) -> ClientCertificateCredential
    where
        C: Into<Secret>,
        P: Into<Secret>,
    {
        ClientCertificateCredential {
            tenant_id,
            client_id,
            client_certificate: client_certificate.into(),
            client_certificate_pass: client_certificate_pass.into(),
            http_client: new_http_client(),
            options,
            cache: TokenCache::new(),
        }
    }

    fn options(&self) -> &CertificateCredentialOptions {
        &self.options
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

    async fn get_token(&self, scopes: &[&str]) -> azure_core::Result<AccessToken> {
        if scopes.len() != 1 {
            return Err(Error::message(
                ErrorKind::Credential,
                "only one scope is supported for IMDS authentication",
            ));
        }

        let Some(scope) = scopes.first() else {
            return Err(Error::message(
                ErrorKind::Credential,
                "no scopes were provided",
            ));
        };

        let options = self.options();

        let url = options
            .authority_host()
            .join(&format!("{}/oauth2/v2.0/token", self.tenant_id))?;

        let certificate = base64::decode(self.client_certificate.secret())
            .map_err(|_| Error::message(ErrorKind::Credential, "Base64 decode failed"))?;

        let pkcs12_certificate = Pkcs12::from_der(&certificate)
            .map_err(openssl_error)?
            .parse2(self.client_certificate_pass.secret())
            .map_err(openssl_error)?;

        let Some(cert) = pkcs12_certificate.cert.as_ref() else {
            return Err(Error::message(
                ErrorKind::Credential,
                "Certificate not found",
            ));
        };

        let Some(pkey) = pkcs12_certificate.pkey.as_ref() else {
            return Err(Error::message(
                ErrorKind::Credential,
                "Private key not found",
            ));
        };

        let thumbprint =
            ClientCertificateCredential::get_thumbprint(cert).map_err(openssl_error)?;

        let uuid = uuid::Uuid::new_v4();
        let current_time = OffsetDateTime::now_utc().unix_timestamp();
        let expiry_time = current_time + DEFAULT_REFRESH_TIME;
        let x5t = base64::encode(thumbprint);

        let header = match options.send_certificate_chain {
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
            url, expiry_time, self.client_id, uuid, current_time, self.client_id
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
                .append_pair("scope", scope)
                .append_pair(
                    "client_assertion_type",
                    "urn:ietf:params:oauth:client-assertion-type:jwt-bearer",
                )
                .append_pair("client_assertion", client_assertion.as_str())
                .append_pair("grant_type", "client_credentials");
            encoded.finish()
        };

        let mut req = Request::new(url, Method::Post);
        req.insert_header(
            headers::CONTENT_TYPE,
            content_type::APPLICATION_X_WWW_FORM_URLENCODED,
        );
        req.set_body(encoded);

        let rsp = self.http_client.execute_request(&req).await?;
        let rsp_status = rsp.status();

        if !rsp_status.is_success() {
            let rsp_body = rsp.into_body().collect().await?;
            return Err(ErrorKind::http_response_from_body(rsp_status, &rsp_body).into_error());
        }

        let response: AadTokenResponse = rsp.json().await?;
        Ok(AccessToken::new(
            response.access_token,
            OffsetDateTime::now_utc() + Duration::from_secs(response.expires_in),
        ))
    }
}

#[derive(Deserialize, Debug, Default)]
#[serde(default)]
struct AadTokenResponse {
    token_type: String,
    expires_in: u64,
    ext_expires_in: u64,
    access_token: String,
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
    async fn get_token(&self, scopes: &[&str]) -> azure_core::Result<AccessToken> {
        self.cache.get_token(scopes, self.get_token(scopes)).await
    }

    async fn clear_cache(&self) -> azure_core::Result<()> {
        self.cache.clear().await
    }
}

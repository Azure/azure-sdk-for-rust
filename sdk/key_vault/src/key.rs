use std::fmt::{Debug, Display};

use anyhow::Context;
use azure_core::TokenCredential;
use getset::Getters;
use serde::Deserialize;
use serde_json::{Map, Value};
use url::Url;

use crate::client::API_VERSION;
use crate::{KeyVaultClient, KeyVaultError};

/// A KeyBundle consisting of a WebKey plus its attributes.
#[derive(Debug, Deserialize, Getters)]
#[getset(get = "pub")]
pub struct KeyBundle {
    /// The key management attributes.
    attributes: KeyAttributes,
    /// The Json web key.
    key: JsonWebKey,
    /// True if the key's lifetime is managed by key vault. If this is a key backing a certificate, then managed will be true.
    managed: Option<bool>,
    /// Application specific metadata in the form of key-value pairs.
    tags: Option<Map<String, Value>>,
}

#[derive(Debug, Deserialize, Getters)]
#[getset(get = "pub")]
#[serde(rename_all = "camelCase")]
pub struct KeyAttributes {
    /// Creation time in UTC.
    created: u64,
    /// Determines whether the object is enabled.
    enabled: bool,
    /// Expiry date in UTC.
    exp: Option<u64>,
    /// Not before date in UTC.
    nbf: u64,
    /// softDelete data retention days. Value should be >=7 and <=90 when softDelete enabled, otherwise 0.
    recoverable_days: Option<u8>,
    /// Reflects the deletion recovery level currently in effect for keys in the current vault. If it contains 'Purgeable' the key can be permanently deleted by a privileged user; otherwise, only the system can purge the key, at the end of the retention interval.
    recovery_level: String,
    /// Last updated time in UTC.
    updated: u64,
}

#[derive(Debug, Deserialize, Getters)]
#[getset(get = "pub")]
pub struct JsonWebKey {
    /// Elliptic curve name. For valid values, see JsonWebKeyCurveName.
    crv: Option<String>,
    /// RSA private exponent, or the D component of an EC private key.
    #[serde(skip_serializing_if = "Option::is_none")]
    d: Option<String>,
    /// RSA private key parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    dp: Option<String>,
    /// RSA private key parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    dq: Option<String>,
    /// RSA public exponent.
    #[serde(skip_serializing_if = "Option::is_none")]
    e: Option<String>,
    /// Symmetric key.
    #[serde(skip_serializing_if = "Option::is_none")]
    k: Option<String>,
    /// HSM Token, used with 'Bring Your Own Key'.
    #[serde(skip_serializing_if = "Option::is_none")]
    key_hsm: Option<String>,
    /// Supported key operations.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    key_ops: Vec<String>,
    /// Key identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    kid: Option<String>,
    /// JsonWebKey Key Type (kty), as defined in https://tools.ietf.org/html/draft-ietf-jose-json-web-algorithms-40.
    kty: String,
    /// RSA modulus.
    #[serde(skip_serializing_if = "Option::is_none")]
    n: Option<String>,
    /// RSA secret prime.
    #[serde(skip_serializing_if = "Option::is_none")]
    p: Option<String>,
    /// RSA secret prime, with p < q.
    #[serde(skip_serializing_if = "Option::is_none")]
    q: Option<String>,
    /// RSA private key parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    qi: Option<String>,
    /// X component of an EC public key.
    #[serde(skip_serializing_if = "Option::is_none")]
    x: Option<String>,
    /// Y component of an EC public key.
    #[serde(skip_serializing_if = "Option::is_none")]
    y: Option<String>,
}

/// The signing/verification algorithm identifier
#[derive(Debug)]
pub enum JsonWebKeySignatureAlgorithm {
    ES256,  // ECDSA using P-256 and SHA-256, as described in https://tools.ietf.org/html/rfc7518.
    ES256K, // ECDSA using P-256K and SHA-256, as described in https://tools.ietf.org/html/rfc7518
    ES384,  // ECDSA using P-384 and SHA-384, as described in https://tools.ietf.org/html/rfc7518
    ES512,  // ECDSA using P-521 and SHA-512, as described in https://tools.ietf.org/html/rfc7518
    PS256, // RSASSA-PSS using SHA-256 and MGF1 with SHA-256, as described in https://tools.ietf.org/html/rfc7518
    PS384, // RSASSA-PSS using SHA-384 and MGF1 with SHA-384, as described in https://tools.ietf.org/html/rfc7518
    PS512, // RSASSA-PSS using SHA-512 and MGF1 with SHA-512, as described in https://tools.ietf.org/html/rfc7518
    RS256, // RSASSA-PKCS1-v1_5 using SHA-256, as described in https://tools.ietf.org/html/rfc7518
    RS384, // RSASSA-PKCS1-v1_5 using SHA-384, as described in https://tools.ietf.org/html/rfc7518
    RS512, // RSASSA-PKCS1-v1_5 using SHA-512, as described in https://tools.ietf.org/html/rfc7518
}

impl Display for JsonWebKeySignatureAlgorithm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self, f)
    }
}

impl<'a, T: TokenCredential> KeyVaultClient<'a, T> {
    /// Gets the public part of a stored key.
    /// The get key operation is applicable to all key types.
    /// If the requested key is symmetric, then no key material is released in the response.
    /// This operation requires the keys/get permission.
    ///
    /// GET {vaultBaseUrl}/keys/{key-name}/{key-version}?api-version=7.1
    pub async fn get_key(
        &mut self,
        key_name: &str,
        key_version: &str,
    ) -> Result<KeyBundle, KeyVaultError> {
        let uri = Url::parse_with_params(
            &format!(
                "{}/keys/{}/{}",
                self.keyvault_endpoint, key_name, key_version
            ),
            &[("api-version", API_VERSION)],
        )
        .unwrap();
        let resp_body = self.get_authed(uri.to_string()).await?;
        let response = serde_json::from_str::<KeyBundle>(&resp_body)
            .with_context(|| format!("Failed to parse response from Key Vault: {}", resp_body))?;
        Ok(response)
    }
}

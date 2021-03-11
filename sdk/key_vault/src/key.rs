use std::fmt::{Debug, Display};

use azure_core::TokenCredential;
use chrono::serde::ts_seconds_option;
use chrono::{DateTime, Utc};
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
    #[serde(with = "ts_seconds_option", default)]
    created: Option<DateTime<Utc>>,
    /// Determines whether the object is enabled.
    enabled: Option<bool>,
    /// Expiry date in UTC.
    #[serde(with = "ts_seconds_option", default)]
    exp: Option<DateTime<Utc>>,
    /// Not before date in UTC.
    #[serde(with = "ts_seconds_option", default)]
    nbf: Option<DateTime<Utc>>,
    /// softDelete data retention days. Value should be >=7 and <=90 when softDelete enabled, otherwise 0.
    recoverable_days: Option<u8>,
    /// Reflects the deletion recovery level currently in effect for keys in the current vault. If it contains 'Purgeable' the key can be permanently deleted by a privileged user; otherwise, only the system can purge the key, at the end of the retention interval.
    recovery_level: Option<String>,
    /// Last updated time in UTC.
    #[serde(with = "ts_seconds_option", default)]
    updated: Option<DateTime<Utc>>,
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

#[derive(Debug, Deserialize, Getters)]
#[getset(get = "pub")]
pub struct KeyOperationResult {
    kid: String,
    value: String,
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
        let response = serde_json::from_str::<KeyBundle>(&resp_body)?;
        Ok(response)
    }

    /// Creates a signature from a digest using the specified key.
    /// The SIGN operation is applicable to asymmetric and symmetric keys stored in Azure Key Vault since this operation uses the private portion of the key.
    /// This operation requires the keys/sign permission.
    pub async fn sign(
        &mut self,
        key_name: &str,
        key_version: &str,
        value: &str,
        alg: JsonWebKeySignatureAlgorithm,
    ) -> Result<KeyOperationResult, KeyVaultError> {
        // POST {vaultBaseUrl}/keys/{key-name}/{key-version}/sign?api-version=7.1
        let uri = Url::parse_with_params(
            &format!(
                "{}/keys/{}/{}/sign",
                self.keyvault_endpoint, key_name, key_version
            ),
            &[("api-version", API_VERSION)],
        )
        .unwrap();

        let mut request_body = Map::new();
        request_body.insert("alg".to_owned(), Value::String(alg.to_string()));
        request_body.insert("value".to_owned(), Value::String(value.to_owned()));

        let response = self
            .post_authed(
                uri.to_string(),
                Some(Value::Object(request_body).to_string()),
            )
            .await?;

        let result = serde_json::from_str::<KeyOperationResult>(&response)?;

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use azure_core::errors::AzureError;
    use azure_core::TokenResponse;
    use chrono::{DateTime, Duration, Utc};
    use mockito::{mock, Matcher};
    use oauth2::AccessToken;
    use serde_json::json;

    struct MockKeyCredential;

    #[async_trait::async_trait]
    impl TokenCredential for MockKeyCredential {
        async fn get_token(&self, _resource: &str) -> Result<TokenResponse, AzureError> {
            Ok(TokenResponse::new(
                AccessToken::new("TOKEN".to_owned()),
                Utc::now() + Duration::days(14),
            ))
        }
    }

    macro_rules! mock_client {
        ($creds:expr, $keyvault_name:expr) => {{
            let mut client = KeyVaultClient::new($creds, $keyvault_name);
            client.keyvault_endpoint = mockito::server_url();
            client
        }};
    }

    fn diff(first: DateTime<Utc>, second: DateTime<Utc>) -> Duration {
        if first > second {
            first - second
        } else {
            second - first
        }
    }

    #[tokio::test]
    async fn can_get_key() {
        let time_created = Utc::now() - Duration::days(7);
        let time_updated = Utc::now();
        let _m = mock("GET", "/keys/test-key/78deebed173b48e48f55abf87ed4cf71")
            .match_query(Matcher::UrlEncoded("api-version".into(), API_VERSION.into()))
            .with_header("content-type", "application/json")
            .with_body(
                json!({
                    "key": {
                        "kid": "https://test-keyvault.vault.azure.net/keys/test-key/78deebed173b48e48f55abf87ed4cf71",
                        "kty": "RSA",
                        "key_ops": [
                            "encrypt",
                            "decrypt",
                            "sign",
                            "verify",
                            "wrapKey",
                            "unwrapKey"
                        ],
                        "n": "2HJAE5fU3Cw2Rt9hEuq-F6XjINKGa-zskfISVqopqUy60GOs2eyhxbWbJBeUXNor_gf-tXtNeuqeBgitLeVa640UDvnEjYTKWjCniTxZRaU7ewY8BfTSk-7KxoDdLsPSpX_MX4rwlAx-_1UGk5t4sQgTbm9T6Fm2oqFd37dsz5-Gj27UP2GTAShfJPFD7MqU_zIgOI0pfqsbNL5xTQVM29K6rX4jSPtylZV3uWJtkoQIQnrIHhk1d0SC0KwlBV3V7R_LVYjiXLyIXsFzSNYgQ68ZjAwt8iL7I8Osa-ehQLM13DVvLASaf7Jnu3sC3CWl3Gyirgded6cfMmswJzY87w",
                        "e": "AQAB"
                    },
                    "attributes": {
                        "enabled": true,
                        "created": time_created.timestamp(),
                        "updated": time_updated.timestamp(),
                        "recoveryLevel": "Recoverable+Purgeable"
                      },
                    "tags": {
                        "purpose": "unit test",
                        "test name ": "CreateGetDeleteKeyTest"
                    }
                })
                .to_string(),
            )
            .with_status(200)
            .create();

        let creds = MockKeyCredential;
        let mut client = mock_client!(&creds, &"test-keyvault");

        let keybundle = client
            .get_key("test-key", "78deebed173b48e48f55abf87ed4cf71")
            .await
            .unwrap();

        let JsonWebKey { kid, n, .. } = keybundle.key;
        let KeyAttributes {
            created,
            enabled,
            updated,
            ..
        } = keybundle.attributes;
        assert_eq!("2HJAE5fU3Cw2Rt9hEuq-F6XjINKGa-zskfISVqopqUy60GOs2eyhxbWbJBeUXNor_gf-tXtNeuqeBgitLeVa640UDvnEjYTKWjCniTxZRaU7ewY8BfTSk-7KxoDdLsPSpX_MX4rwlAx-_1UGk5t4sQgTbm9T6Fm2oqFd37dsz5-Gj27UP2GTAShfJPFD7MqU_zIgOI0pfqsbNL5xTQVM29K6rX4jSPtylZV3uWJtkoQIQnrIHhk1d0SC0KwlBV3V7R_LVYjiXLyIXsFzSNYgQ68ZjAwt8iL7I8Osa-ehQLM13DVvLASaf7Jnu3sC3CWl3Gyirgded6cfMmswJzY87w", n.unwrap());
        assert_eq!(
            "https://test-keyvault.vault.azure.net/keys/test-key/78deebed173b48e48f55abf87ed4cf71",
            kid.unwrap()
        );

        assert_eq!(true, enabled.unwrap());
        assert!(diff(time_created, created.unwrap()) < Duration::seconds(1));
        assert!(diff(time_updated, updated.unwrap()) < Duration::seconds(1));
    }

    #[tokio::test]
    async fn can_sign() {
        let _m = mock("POST", "/keys/test-key/78deebed173b48e48f55abf87ed4cf71/sign")
            .match_query(Matcher::UrlEncoded("api-version".into(), API_VERSION.into()))
            .with_header("content-type", "application/json")
            .with_body(
                json!({
                    "kid": "https://myvault.vault.azure.net/keys/testkey/9885aa558e8d448789683188f8c194b0",
                    "value": "aKFG8NXcfTzqyR44rW42484K_zZI_T7zZuebvWuNgAoEI1gXYmxrshp42CunSmmu4oqo4-IrCikPkNIBkHXnAW2cv03Ad0UpwXhVfepK8zzDBaJPMKVGS-ZRz8CshEyGDKaLlb3J3zEkXpM3RrSEr0mdV6hndHD_mznLB5RmFui5DsKAhez4vUqajgtkgcPfCekMqeSwp6r9ItVL-gEoAohx8XMDsPedqu-7BuZcBcdayaPuBRL4wWoTDULA11P-UN_sJ5qMj3BbiRYhIlBWGR04wIGfZ3pkJjHJUpOvgH2QajdYPzUBauOCewMYbq9XkLRSzI_A7HkkDVycugSeAA"
                })
                .to_string(),
            )
            .with_status(200)
            .create();

        let creds = MockKeyCredential;
        let mut client = mock_client!(&creds, &"test-keyvault");

        let res = client
            .sign(
                "test-key",
                "78deebed173b48e48f55abf87ed4cf71",
                "base64msg2sign",
                JsonWebKeySignatureAlgorithm::RS512,
            )
            .await
            .unwrap();

        let kid = res.kid;
        let sig = res.value;

        assert_eq!(
            kid,
            "https://myvault.vault.azure.net/keys/testkey/9885aa558e8d448789683188f8c194b0"
        );
        assert_eq!(sig, "aKFG8NXcfTzqyR44rW42484K_zZI_T7zZuebvWuNgAoEI1gXYmxrshp42CunSmmu4oqo4-IrCikPkNIBkHXnAW2cv03Ad0UpwXhVfepK8zzDBaJPMKVGS-ZRz8CshEyGDKaLlb3J3zEkXpM3RrSEr0mdV6hndHD_mznLB5RmFui5DsKAhez4vUqajgtkgcPfCekMqeSwp6r9ItVL-gEoAohx8XMDsPedqu-7BuZcBcdayaPuBRL4wWoTDULA11P-UN_sJ5qMj3BbiRYhIlBWGR04wIGfZ3pkJjHJUpOvgH2QajdYPzUBauOCewMYbq9XkLRSzI_A7HkkDVycugSeAA");
    }
}

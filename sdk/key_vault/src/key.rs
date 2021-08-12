use std::fmt::{Debug, Display};

use azure_core::TokenCredential;
use base64::{CharacterSet, Config};
use chrono::serde::ts_seconds_option;
use chrono::{DateTime, Utc};
use getset::Getters;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_json::{Map, Value};

use crate::client::API_VERSION_PARAM;
use crate::Error;
use crate::KeyClient;

/// A KeyBundle consisting of a WebKey plus its attributes.
#[derive(Debug, Deserialize, Getters)]
#[getset(get = "pub")]
pub struct KeyVaultKey {
    /// The key management properties.
    #[serde(flatten)]
    properties: KeyProperties,
    /// The Json web key.
    key: JsonWebKey,
}

#[derive(Debug, Deserialize, Getters)]
#[getset(get = "pub")]
pub struct KeyProperties {
    attributes: KeyAttributes,
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
    #[serde(rename = "created", with = "ts_seconds_option", default)]
    created_on: Option<DateTime<Utc>>,
    /// Determines whether the object is enabled.
    enabled: Option<bool>,
    /// Expiry date in UTC.
    #[serde(rename = "exp", with = "ts_seconds_option", default)]
    expires_on: Option<DateTime<Utc>>,
    /// Not before date in UTC.
    #[serde(rename = "nbf", with = "ts_seconds_option", default)]
    not_before: Option<DateTime<Utc>>,
    /// softDelete data retention days. Value should be >=7 and <=90 when softDelete enabled, otherwise 0.
    recoverable_days: Option<u8>,
    /// Reflects the deletion recovery level currently in effect for keys in the current vault. If it contains 'Purgeable' the key can be permanently deleted by a privileged user; otherwise, only the system can purge the key, at the end of the retention interval.
    recovery_level: Option<String>,
    /// Last updated time in UTC.
    #[serde(rename = "updated", with = "ts_seconds_option", default)]
    updated_on: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize, Getters)]
#[getset(get = "pub")]
pub struct JsonWebKey {
    /// Elliptic curve name. For valid values, see JsonWebKeyCurveName.
    #[serde(rename = "crv")]
    curve_name: Option<String>,
    /// RSA private exponent, or the D component of an EC private key.
    #[serde(
        serialize_with = "ser_base64_opt",
        deserialize_with = "deser_base64_opt"
    )]
    #[serde(default)]
    d: Option<Vec<u8>>,
    /// RSA private key parameter.
    #[serde(
        serialize_with = "ser_base64_opt",
        deserialize_with = "deser_base64_opt"
    )]
    #[serde(default)]
    dp: Option<Vec<u8>>,
    /// RSA private key parameter.
    #[serde(
        serialize_with = "ser_base64_opt",
        deserialize_with = "deser_base64_opt"
    )]
    #[serde(default)]
    dq: Option<Vec<u8>>,
    /// RSA public exponent.
    #[serde(
        serialize_with = "ser_base64_opt",
        deserialize_with = "deser_base64_opt"
    )]
    #[serde(default)]
    e: Option<Vec<u8>>,
    /// Symmetric key.
    #[serde(
        serialize_with = "ser_base64_opt",
        deserialize_with = "deser_base64_opt"
    )]
    #[serde(default)]
    k: Option<Vec<u8>>,
    /// HSM Token, used with 'Bring Your Own Key'.
    #[serde(
        serialize_with = "ser_base64_opt",
        deserialize_with = "deser_base64_opt"
    )]
    #[serde(default)]
    #[serde(rename = "key_hsm")]
    t: Option<Vec<u8>>,
    /// Supported key operations.
    key_ops: Option<Vec<String>>,
    /// Key identifier.
    #[serde(rename = "kid")]
    id: Option<String>,
    /// JsonWebKey Key Type (kty), as defined in https://tools.ietf.org/html/draft-ietf-jose-json-web-algorithms-40.
    #[serde(rename = "kty")]
    key_type: String,
    /// RSA modulus.
    #[serde(
        serialize_with = "ser_base64_opt",
        deserialize_with = "deser_base64_opt"
    )]
    #[serde(default)]
    n: Option<Vec<u8>>,
    /// RSA secret prime.
    #[serde(
        serialize_with = "ser_base64_opt",
        deserialize_with = "deser_base64_opt"
    )]
    #[serde(default)]
    p: Option<Vec<u8>>,
    /// RSA secret prime, with p < q.
    #[serde(
        serialize_with = "ser_base64_opt",
        deserialize_with = "deser_base64_opt"
    )]
    #[serde(default)]
    q: Option<Vec<u8>>,
    /// RSA private key parameter.
    #[serde(
        serialize_with = "ser_base64_opt",
        deserialize_with = "deser_base64_opt"
    )]
    #[serde(default)]
    qi: Option<Vec<u8>>,
    /// X component of an EC public key.
    #[serde(
        serialize_with = "ser_base64_opt",
        deserialize_with = "deser_base64_opt"
    )]
    #[serde(default)]
    x: Option<Vec<u8>>,
    /// Y component of an EC public key.
    #[serde(
        serialize_with = "ser_base64_opt",
        deserialize_with = "deser_base64_opt"
    )]
    #[serde(default)]
    y: Option<Vec<u8>>,
}

const BASE64_URL_SAFE: Config = Config::new(CharacterSet::UrlSafe, false);

fn ser_base64<S>(bytes: &Vec<u8>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let base_64 = base64::encode_config(bytes, BASE64_URL_SAFE);
    serializer.serialize_str(&base_64)
}

fn ser_base64_opt<S>(bytes: &Option<Vec<u8>>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    if let Some(bytes) = bytes {
        let base_64 = base64::encode_config(bytes, BASE64_URL_SAFE);
        serializer.serialize_str(&base_64)
    } else {
        serializer.serialize_none()
    }
}

fn deser_base64<'de, D>(deserializer: D) -> Result<Vec<u8>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = String::deserialize(deserializer)?;
    let res = base64::decode_config(s, BASE64_URL_SAFE).map_err(serde::de::Error::custom)?;
    Ok(res)
}

fn deser_base64_opt<'de, D>(deserializer: D) -> Result<Option<Vec<u8>>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: Option<&str> = Option::deserialize(deserializer)?;
    let res = match s {
        Some(s) => {
            Some(base64::decode_config(s, BASE64_URL_SAFE).map_err(serde::de::Error::custom)?)
        }
        None => None,
    };
    Ok(res)
}

#[derive(Debug, Deserialize, Getters)]
#[getset(get = "pub")]
pub struct SignResult {
    #[serde(
        rename = "value",
        serialize_with = "ser_base64",
        deserialize_with = "deser_base64"
    )]
    signature: Vec<u8>,
    #[serde(skip)]
    algorithm: SignatureAlgorithm,
    #[serde(rename = "kid")]
    key_id: String,
}

/// The signing/verification algorithm identifier
#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum SignatureAlgorithm {
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
    Custom(String),
}

impl Default for SignatureAlgorithm {
    fn default() -> Self {
        SignatureAlgorithm::Custom("".to_string())
    }
}

impl Display for SignatureAlgorithm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self, f)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum EncryptionAlgorithm {
    #[serde(rename = "A128CBC")]
    A128Cbc,
    #[serde(rename = "A128CBCPAD")]
    A128CbcPad,
    #[serde(rename = "A128GCM")]
    A128Gcm,
    #[serde(rename = "A192CBC")]
    A192Cbc,
    #[serde(rename = "A192CBCPAD")]
    A192CbcPad,
    #[serde(rename = "A192GCM")]
    A192Gcm,
    #[serde(rename = "A256CBC")]
    A256Cbc,
    #[serde(rename = "A256CBCPAD")]
    A256CbcPad,
    #[serde(rename = "A256GCM")]
    A256Gcm,
    #[serde(rename = "RSA-OAEP")]
    RsaOaep,
    #[serde(rename = "RSA-OAEP-256")]
    RsaOaep256,
    #[serde(rename = "RSA1_5")]
    Rsa15,
}

impl Default for EncryptionAlgorithm {
    fn default() -> Self {
        EncryptionAlgorithm::A128Cbc
    }
}

impl Display for EncryptionAlgorithm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self, f)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DecryptParameters {
    pub decrypt_parameters_encryption: DecryptParametersEncryption,
    #[serde(serialize_with = "ser_base64", deserialize_with = "deser_base64")]
    pub ciphertext: Vec<u8>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DecryptParametersEncryption {
    Rsa15(Rsa15Parameters),
    RsaOaep(RsaOaepParameters),
    RsaOaep256(RsaOaep256Parameters),
    A128Gcm(A128GcmParameters),
    A192Gcm(A192GcmParameters),
    A256Gcm(A256GcmParameters),
    A128Cbc(A128CbcParameters),
    A192Cbc(A192CbcParameters),
    A256Cbc(A256CbcParameters),
    A128CbcPad(A128CbcPadParameters),
    A192CbcPad(A192CbcPadParameters),
    A256CbcPad(A256CbcPadParameters),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Rsa15Parameters {
    algorithm: EncryptionAlgorithm,
}

impl Rsa15Parameters {
    pub fn new() -> Self {
        Self {
            algorithm: EncryptionAlgorithm::Rsa15,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RsaOaepParameters {
    algorithm: EncryptionAlgorithm,
}

impl RsaOaepParameters {
    pub fn new() -> Self {
        Self {
            algorithm: EncryptionAlgorithm::RsaOaep,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RsaOaep256Parameters {
    algorithm: EncryptionAlgorithm,
}

impl RsaOaep256Parameters {
    pub fn new() -> Self {
        Self {
            algorithm: EncryptionAlgorithm::RsaOaep256,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct A128GcmParameters {
    algorithm: EncryptionAlgorithm,
    parameters: AesGcmDecryptParameters,
}

impl A128GcmParameters {
    pub fn new(parameters: AesGcmDecryptParameters) -> Self {
        Self {
            algorithm: EncryptionAlgorithm::A128Gcm,
            parameters,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct A192GcmParameters {
    algorithm: EncryptionAlgorithm,
    parameters: AesGcmDecryptParameters,
}

impl A192GcmParameters {
    pub fn new(parameters: AesGcmDecryptParameters) -> Self {
        Self {
            algorithm: EncryptionAlgorithm::A192Gcm,
            parameters,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct A256GcmParameters {
    algorithm: EncryptionAlgorithm,
    parameters: AesGcmDecryptParameters,
}

impl A256GcmParameters {
    pub fn new(parameters: AesGcmDecryptParameters) -> Self {
        Self {
            algorithm: EncryptionAlgorithm::A256Gcm,
            parameters,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct A128CbcParameters {
    algorithm: EncryptionAlgorithm,
    parameters: AesCbcDecryptParameters,
}

impl A128CbcParameters {
    pub fn new(parameters: AesCbcDecryptParameters) -> Self {
        Self {
            algorithm: EncryptionAlgorithm::A128Cbc,
            parameters,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct A192CbcParameters {
    algorithm: EncryptionAlgorithm,
    parameters: AesCbcDecryptParameters,
}

impl A192CbcParameters {
    pub fn new(parameters: AesCbcDecryptParameters) -> Self {
        Self {
            algorithm: EncryptionAlgorithm::A192Cbc,
            parameters,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct A256CbcParameters {
    algorithm: EncryptionAlgorithm,
    parameters: AesCbcDecryptParameters,
}

impl A256CbcParameters {
    pub fn new(parameters: AesCbcDecryptParameters) -> Self {
        Self {
            algorithm: EncryptionAlgorithm::A256Cbc,
            parameters,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct A128CbcPadParameters {
    algorithm: EncryptionAlgorithm,
    parameters: AesCbcDecryptParameters,
}

impl A128CbcPadParameters {
    pub fn new(parameters: AesCbcDecryptParameters) -> Self {
        Self {
            algorithm: EncryptionAlgorithm::A128CbcPad,
            parameters,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct A192CbcPadParameters {
    algorithm: EncryptionAlgorithm,
    parameters: AesCbcDecryptParameters,
}

impl A192CbcPadParameters {
    pub fn new(parameters: AesCbcDecryptParameters) -> Self {
        Self {
            algorithm: EncryptionAlgorithm::A192CbcPad,
            parameters,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct A256CbcPadParameters {
    algorithm: EncryptionAlgorithm,
    parameters: AesCbcDecryptParameters,
}

impl A256CbcPadParameters {
    pub fn new(parameters: AesCbcDecryptParameters) -> Self {
        Self {
            algorithm: EncryptionAlgorithm::A256CbcPad,
            parameters,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AesGcmDecryptParameters {
    #[serde(serialize_with = "ser_base64", deserialize_with = "deser_base64")]
    pub iv: Vec<u8>,
    #[serde(serialize_with = "ser_base64", deserialize_with = "deser_base64")]
    pub authentication_tag: Vec<u8>,
    #[serde(
        serialize_with = "ser_base64_opt",
        deserialize_with = "deser_base64_opt"
    )]
    pub additional_authenticated_data: Option<Vec<u8>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AesCbcDecryptParameters {
    #[serde(serialize_with = "ser_base64", deserialize_with = "deser_base64")]
    pub iv: Vec<u8>,
}

#[derive(Debug, Deserialize, Getters)]
#[getset(get = "pub")]
pub struct DecryptResult {
    #[serde(skip)]
    algorithm: EncryptionAlgorithm,
    #[serde(rename = "kid")]
    key_id: String,
    #[serde(
        rename = "value",
        serialize_with = "ser_base64",
        deserialize_with = "deser_base64"
    )]
    result: Vec<u8>,
}

impl<'a, T: TokenCredential> KeyClient<'a, T> {
    /// Gets the public part of a stored key.
    /// The get key operation is applicable to all key types.
    /// If the requested key is symmetric, then no key material is released in the response.
    /// This operation requires the keys/get permission.
    ///
    /// GET {vaultBaseUrl}/keys/{key-name}/{key-version}?api-version=7.1
    pub async fn get_key(
        &mut self,
        key_name: &str,
        key_version: Option<&str>,
    ) -> Result<KeyVaultKey, Error> {
        let mut uri = self.vault_url.clone();
        let path = if let Some(ver) = key_version {
            format!("keys/{}/{}", key_name, ver)
        } else {
            format!("keys/{}", key_name)
        };
        uri.set_path(&path);
        uri.set_query(Some(API_VERSION_PARAM));

        let resp_body = self.get_authed(uri.to_string()).await?;
        let response = serde_json::from_str::<KeyVaultKey>(&resp_body)?;
        Ok(response)
    }

    /// Creates a signature from a digest using the specified key.
    /// The SIGN operation is applicable to asymmetric and symmetric keys stored in Azure Key Vault since this operation uses the private portion of the key.
    /// This operation requires the keys/sign permission.
    pub async fn sign(
        &mut self,
        algorithm: SignatureAlgorithm,
        key_name: &str,
        key_version: &str,
        digest: &str,
    ) -> Result<SignResult, Error> {
        // POST {vaultBaseUrl}/keys/{key-name}/{key-version}/sign?api-version=7.1

        let mut uri = self.vault_url.clone();
        uri.set_path(&format!("keys/{}/{}/sign", key_name, key_version));
        uri.set_query(Some(API_VERSION_PARAM));

        let mut request_body = Map::new();
        request_body.insert("alg".to_owned(), Value::String(algorithm.to_string()));
        request_body.insert("value".to_owned(), Value::String(digest.to_owned()));

        let response = self
            .post_authed(
                uri.to_string(),
                Some(Value::Object(request_body).to_string()),
            )
            .await?;

        let mut result = serde_json::from_str::<SignResult>(&response)?;
        result.algorithm = algorithm;
        Ok(result)
    }

    /// Decrypt a single block of encrypted data.
    /// The DECRYPT operation decrypts a well-formed block of ciphertext using the target encryption key and specified algorithm.
    /// This operation is the reverse of the ENCRYPT operation; only a single block of data may be decrypted, the size of this block is dependent on the target key and the algorithm to be used.
    /// The DECRYPT operation applies to asymmetric and symmetric keys stored in Vault or HSM since it uses the private portion of the key. This operation requires the keys/decrypt permission.
    pub async fn decrypt(
        &mut self,
        key_name: &str,
        key_version: Option<&str>,
        decrypt_parameters: DecryptParameters,
    ) -> Result<DecryptResult, Error> {
        // POST {vaultBaseUrl}/keys/{key-name}/{key-version}/decrypt?api-version=7.2

        let mut uri = self.vault_url.clone();
        let path = format!("keys/{}/{}/decrypt", key_name, key_version);

        uri.set_path(&path);
        uri.set_query(Some(API_VERSION_PARAM));

        let mut request_body = Map::new();
        request_body.insert(
            "value".to_owned(),
            Value::String(base64::encode(decrypt_parameters.ciphertext.to_owned())),
        );

        let algorithm = match decrypt_parameters.decrypt_parameters_encryption {
            DecryptParametersEncryption::Rsa15(Rsa15Parameters { algorithm: alg })
            | DecryptParametersEncryption::RsaOaep(RsaOaepParameters { algorithm: alg })
            | DecryptParametersEncryption::RsaOaep256(RsaOaep256Parameters { algorithm: alg }) => {
                request_body.insert("alg".to_owned(), serde_json::to_value(&alg).unwrap());
                alg
            }
            DecryptParametersEncryption::A128Gcm(A128GcmParameters {
                algorithm: alg,
                parameters: params,
            })
            | DecryptParametersEncryption::A192Gcm(A192GcmParameters {
                algorithm: alg,
                parameters: params,
            })
            | DecryptParametersEncryption::A256Gcm(A256GcmParameters {
                algorithm: alg,
                parameters: params,
            }) => {
                request_body.insert("alg".to_owned(), serde_json::to_value(&alg).unwrap());
                request_body.insert("iv".to_owned(), serde_json::to_value(params.iv).unwrap());
                request_body.insert(
                    "tag".to_owned(),
                    serde_json::to_value(params.authentication_tag).unwrap(),
                );
                if let Some(aad) = params.additional_authenticated_data {
                    request_body.insert("aad".to_owned(), serde_json::to_value(aad).unwrap());
                };
                alg
            }
            DecryptParametersEncryption::A128Cbc(A128CbcParameters {
                algorithm: alg,
                parameters: params,
            })
            | DecryptParametersEncryption::A192Cbc(A192CbcParameters {
                algorithm: alg,
                parameters: params,
            })
            | DecryptParametersEncryption::A256Cbc(A256CbcParameters {
                algorithm: alg,
                parameters: params,
            })
            | DecryptParametersEncryption::A128CbcPad(A128CbcPadParameters {
                algorithm: alg,
                parameters: params,
            })
            | DecryptParametersEncryption::A192CbcPad(A192CbcPadParameters {
                algorithm: alg,
                parameters: params,
            })
            | DecryptParametersEncryption::A256CbcPad(A256CbcPadParameters {
                algorithm: alg,
                parameters: params,
            }) => {
                request_body.insert("alg".to_owned(), serde_json::to_value(&alg).unwrap());
                request_body.insert("iv".to_owned(), serde_json::to_value(params.iv).unwrap());
                alg
            }
        };

        let response = self
            .post_authed(
                uri.to_string(),
                Some(Value::Object(request_body).to_string()),
            )
            .await?;

        let mut result = serde_json::from_str::<DecryptResult>(&response)?;
        result.algorithm = algorithm;
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use chrono::{DateTime, Duration, Utc};
    use mockito::{mock, Matcher};
    use serde_json::json;

    use crate::client::API_VERSION;
    use crate::mock_client;
    use crate::tests::MockCredential;

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
                            "unwrapKey",
                            "destroy!"
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

        let creds = MockCredential;
        let mut client = mock_client!(&"test-keyvault", &creds,);

        let key = client
            .get_key("test-key", Some("78deebed173b48e48f55abf87ed4cf71"))
            .await
            .unwrap();

        let JsonWebKey { id, n, .. } = key.key();
        let KeyProperties {
            attributes,
            managed,
            tags,
        } = key.properties();
        let KeyAttributes {
            created_on,
            enabled,
            updated_on,
            ..
        } = attributes;
        let expected_n = base64::decode_config("2HJAE5fU3Cw2Rt9hEuq-F6XjINKGa-zskfISVqopqUy60GOs2eyhxbWbJBeUXNor_gf-tXtNeuqeBgitLeVa640UDvnEjYTKWjCniTxZRaU7ewY8BfTSk-7KxoDdLsPSpX_MX4rwlAx-_1UGk5t4sQgTbm9T6Fm2oqFd37dsz5-Gj27UP2GTAShfJPFD7MqU_zIgOI0pfqsbNL5xTQVM29K6rX4jSPtylZV3uWJtkoQIQnrIHhk1d0SC0KwlBV3V7R_LVYjiXLyIXsFzSNYgQ68ZjAwt8iL7I8Osa-ehQLM13DVvLASaf7Jnu3sC3CWl3Gyirgded6cfMmswJzY87w", BASE64_URL_SAFE).unwrap();
        assert_eq!(expected_n, n.to_owned().unwrap());
        assert_eq!(
            "https://test-keyvault.vault.azure.net/keys/test-key/78deebed173b48e48f55abf87ed4cf71",
            id.to_owned().unwrap()
        );

        assert!(managed.is_none());
        assert_eq!(
            tags.to_owned().unwrap().get("purpose").unwrap(),
            "unit test"
        );
        assert_eq!(true, enabled.unwrap());
        assert!(diff(time_created, created_on.unwrap()) < Duration::seconds(1));
        assert!(diff(time_updated, updated_on.unwrap()) < Duration::seconds(1));
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

        let creds = MockCredential;
        let mut client = mock_client!(&"test-keyvault", &creds,);

        let res = client
            .sign(
                SignatureAlgorithm::RS512,
                "test-key",
                "78deebed173b48e48f55abf87ed4cf71",
                "base64msg2sign",
            )
            .await
            .unwrap();

        let kid = res.key_id();
        let sig = res.signature();
        let alg = res.algorithm();

        assert_eq!(
            kid,
            "https://myvault.vault.azure.net/keys/testkey/9885aa558e8d448789683188f8c194b0"
        );
        let expected_sig = base64::decode_config("aKFG8NXcfTzqyR44rW42484K_zZI_T7zZuebvWuNgAoEI1gXYmxrshp42CunSmmu4oqo4-IrCikPkNIBkHXnAW2cv03Ad0UpwXhVfepK8zzDBaJPMKVGS-ZRz8CshEyGDKaLlb3J3zEkXpM3RrSEr0mdV6hndHD_mznLB5RmFui5DsKAhez4vUqajgtkgcPfCekMqeSwp6r9ItVL-gEoAohx8XMDsPedqu-7BuZcBcdayaPuBRL4wWoTDULA11P-UN_sJ5qMj3BbiRYhIlBWGR04wIGfZ3pkJjHJUpOvgH2QajdYPzUBauOCewMYbq9XkLRSzI_A7HkkDVycugSeAA", BASE64_URL_SAFE).unwrap();
        assert_eq!(expected_sig, sig.to_owned());
        assert!(matches!(alg, SignatureAlgorithm::RS512));
    }

    #[tokio::test]
    async fn can_decrypt() {
        let _m = mock("POST", "/keys/test-key/78deebed173b48e48f55abf87ed4cf71/decrypt")
            .match_query(Matcher::UrlEncoded("api-version".into(), API_VERSION.into()))
            .with_header("content-type", "application/json")
            .with_body(
                json!({
                    "kid": "https://myvault.vault.azure.net/keys/test-key/78deebed173b48e48f55abf87ed4cf71",
                    "value": "dvDmrSBpjRjtYg"
                  })
                .to_string(),
            )
            .with_status(200)
            .create();

        let creds = MockCredential;
        let mut client = mock_client!(&"test-keyvault", &creds,);

        let decrypt_parameters = DecryptParameters {
            ciphertext: base64::decode("dvDmrSBpjRjtYg").unwrap(),
            decrypt_parameters_encryption: DecryptParametersEncryption::RsaOaep256(
                RsaOaep256Parameters::new(),
            ),
        };

        let res = client
            .decrypt(
                "test-key",
                Some("78deebed173b48e48f55abf87ed4cf71"),
                decrypt_parameters,
            )
            .await
            .unwrap();

        let kid = res.key_id();
        let val = res.result();
        let alg = res.algorithm();

        assert_eq!(
            kid,
            "https://myvault.vault.azure.net/keys/test-key/78deebed173b48e48f55abf87ed4cf71"
        );
        let expected_val = base64::decode_config("dvDmrSBpjRjtYg", BASE64_URL_SAFE).unwrap();
        assert_eq!(expected_val, val.to_owned());

        assert!(matches!(alg, &EncryptionAlgorithm::RsaOaep256));
    }
}

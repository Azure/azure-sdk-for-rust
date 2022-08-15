use azure_core::error::{Error, ErrorKind};
use base64::{CharacterSet, Config};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_json::{Map, Value};
use std::fmt::{Debug, Display};
use time::OffsetDateTime;

/// A KeyBundle consisting of a WebKey plus its attributes.
#[derive(Debug, Deserialize)]
pub struct KeyVaultKey {
    /// The key management properties.
    #[serde(flatten)]
    pub properties: KeyProperties,
    /// The Json web key.
    pub key: JsonWebKey,
}

#[derive(Debug, Deserialize)]
pub struct KeyProperties {
    pub attributes: KeyAttributes,
    /// True if the key's lifetime is managed by key vault. If this is a key backing a certificate, then managed will be true.
    pub managed: Option<bool>,
    /// Application specific metadata in the form of key-value pairs.
    pub tags: Option<Map<String, Value>>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KeyAttributes {
    /// Creation time in UTC.
    #[serde(
        rename = "created",
        with = "azure_core::date::timestamp::option",
        default
    )]
    pub created_on: Option<OffsetDateTime>,
    /// Determines whether the object is enabled.
    pub enabled: Option<bool>,
    /// Expiry date in UTC.
    #[serde(rename = "exp", with = "azure_core::date::timestamp::option", default)]
    pub expires_on: Option<OffsetDateTime>,
    /// Not before date in UTC.
    #[serde(rename = "nbf", with = "azure_core::date::timestamp::option", default)]
    pub not_before: Option<OffsetDateTime>,
    /// softDelete data retention days. Value should be >=7 and <=90 when softDelete enabled, otherwise 0.
    pub recoverable_days: Option<u8>,
    /// Reflects the deletion recovery level currently in effect for keys in the current vault. If it contains 'Purgeable' the key can be permanently deleted by a privileged user; otherwise, only the system can purge the key, at the end of the retention interval.
    pub recovery_level: Option<String>,
    /// Last updated time in UTC.
    #[serde(
        rename = "updated",
        with = "azure_core::date::timestamp::option",
        default
    )]
    pub updated_on: Option<OffsetDateTime>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JsonWebKey {
    /// Elliptic curve name. For valid values, see JsonWebKeyCurveName.
    #[serde(rename = "crv")]
    pub curve_name: Option<String>,
    /// RSA private exponent, or the D component of an EC private key.
    #[serde(
        serialize_with = "ser_base64_opt",
        deserialize_with = "deser_base64_opt"
    )]
    #[serde(default)]
    pub d: Option<Vec<u8>>,
    /// RSA private key parameter.
    #[serde(
        serialize_with = "ser_base64_opt",
        deserialize_with = "deser_base64_opt"
    )]
    #[serde(default)]
    pub dp: Option<Vec<u8>>,
    /// RSA private key parameter.
    #[serde(
        serialize_with = "ser_base64_opt",
        deserialize_with = "deser_base64_opt"
    )]
    #[serde(default)]
    pub dq: Option<Vec<u8>>,
    /// RSA public exponent.
    #[serde(
        serialize_with = "ser_base64_opt",
        deserialize_with = "deser_base64_opt"
    )]
    #[serde(default)]
    pub e: Option<Vec<u8>>,
    /// Symmetric key.
    #[serde(
        serialize_with = "ser_base64_opt",
        deserialize_with = "deser_base64_opt"
    )]
    #[serde(default)]
    pub k: Option<Vec<u8>>,
    /// HSM Token, used with 'Bring Your Own Key'.
    #[serde(
        serialize_with = "ser_base64_opt",
        deserialize_with = "deser_base64_opt"
    )]
    #[serde(default)]
    #[serde(rename = "key_hsm")]
    pub t: Option<Vec<u8>>,
    /// Supported key operations.
    pub key_ops: Option<Vec<String>>,
    /// Key identifier.
    #[serde(rename = "kid")]
    pub id: Option<String>,
    /// JsonWebKey Key Type (kty), as defined in <https://tools.ietf.org/html/draft-ietf-jose-json-web-algorithms-40>.
    #[serde(rename = "kty")]
    pub key_type: String,
    /// RSA modulus.
    #[serde(
        serialize_with = "ser_base64_opt",
        deserialize_with = "deser_base64_opt"
    )]
    #[serde(default)]
    pub n: Option<Vec<u8>>,
    /// RSA secret prime.
    #[serde(
        serialize_with = "ser_base64_opt",
        deserialize_with = "deser_base64_opt"
    )]
    #[serde(default)]
    pub p: Option<Vec<u8>>,
    /// RSA secret prime, with p < q.
    #[serde(
        serialize_with = "ser_base64_opt",
        deserialize_with = "deser_base64_opt"
    )]
    #[serde(default)]
    pub q: Option<Vec<u8>>,
    /// RSA private key parameter.
    #[serde(
        serialize_with = "ser_base64_opt",
        deserialize_with = "deser_base64_opt"
    )]
    #[serde(default)]
    pub qi: Option<Vec<u8>>,
    /// X component of an EC public key.
    #[serde(
        serialize_with = "ser_base64_opt",
        deserialize_with = "deser_base64_opt"
    )]
    #[serde(default)]
    pub x: Option<Vec<u8>>,
    /// Y component of an EC public key.
    #[serde(
        serialize_with = "ser_base64_opt",
        deserialize_with = "deser_base64_opt"
    )]
    #[serde(default)]
    pub y: Option<Vec<u8>>,
}

pub(crate) const BASE64_URL_SAFE: Config = Config::new(CharacterSet::UrlSafe, false);

fn ser_base64<S>(bytes: &[u8], serializer: S) -> Result<S::Ok, S::Error>
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

#[derive(Debug, Deserialize)]
pub struct SignResult {
    #[serde(
        rename = "value",
        serialize_with = "ser_base64",
        deserialize_with = "deser_base64"
    )]
    pub signature: Vec<u8>,
    #[serde(skip)]
    pub algorithm: SignatureAlgorithm,
    #[serde(rename = "kid")]
    pub key_id: String,
}

/// The signing/verification algorithm identifier
#[derive(Debug, Deserialize, Serialize, Clone)]
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

#[derive(Debug, Serialize, Deserialize, Clone)]
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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DecryptParameters {
    pub decrypt_parameters_encryption: DecryptParametersEncryption,
    #[serde(serialize_with = "ser_base64", deserialize_with = "deser_base64")]
    pub ciphertext: Vec<u8>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum DecryptParametersEncryption {
    Rsa(RsaDecryptParameters),
    AesGcm(AesGcmDecryptParameters),
    AesCbc(AesCbcDecryptParameters),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RsaDecryptParameters {
    pub algorithm: EncryptionAlgorithm,
}

impl RsaDecryptParameters {
    pub fn new(algorithm: EncryptionAlgorithm) -> Result<Self, Error> {
        match algorithm {
            EncryptionAlgorithm::Rsa15
            | EncryptionAlgorithm::RsaOaep
            | EncryptionAlgorithm::RsaOaep256 => Ok(Self { algorithm }),
            _ => Err(Error::with_message(ErrorKind::Other, || {
                format!("unexpected encryption algorithm: {algorithm}")
            })),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AesGcmDecryptParameters {
    pub algorithm: EncryptionAlgorithm,
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

impl AesGcmDecryptParameters {
    pub fn new(
        algorithm: EncryptionAlgorithm,
        iv: Vec<u8>,
        authentication_tag: Vec<u8>,
        additional_authenticated_data: Option<Vec<u8>>,
    ) -> Result<Self, Error> {
        match algorithm {
            EncryptionAlgorithm::A128Gcm
            | EncryptionAlgorithm::A192Gcm
            | EncryptionAlgorithm::A256Gcm => Ok(Self {
                algorithm,
                iv,
                authentication_tag,
                additional_authenticated_data,
            }),
            _ => Err(Error::with_message(ErrorKind::Other, || {
                format!("unexpected encryption algorithm: {algorithm}")
            })),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AesCbcDecryptParameters {
    pub algorithm: EncryptionAlgorithm,
    #[serde(serialize_with = "ser_base64", deserialize_with = "deser_base64")]
    pub iv: Vec<u8>,
}

impl AesCbcDecryptParameters {
    pub fn new(algorithm: EncryptionAlgorithm, iv: Vec<u8>) -> Result<Self, Error> {
        match algorithm {
            EncryptionAlgorithm::A128Cbc
            | EncryptionAlgorithm::A192Cbc
            | EncryptionAlgorithm::A256Cbc
            | EncryptionAlgorithm::A128CbcPad
            | EncryptionAlgorithm::A192CbcPad
            | EncryptionAlgorithm::A256CbcPad => Ok(Self { algorithm, iv }),
            _ => Err(Error::with_message(ErrorKind::Other, || {
                format!("unexpected encryption algorithm: {algorithm}")
            })),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct DecryptResult {
    #[serde(skip)]
    pub algorithm: EncryptionAlgorithm,
    #[serde(rename = "kid")]
    pub key_id: String,
    #[serde(
        rename = "value",
        serialize_with = "ser_base64",
        deserialize_with = "deser_base64"
    )]
    pub result: Vec<u8>,
}

// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

// cspell:ignore Hdvcmxk

//! Base64 encoding and decoding functions.

use base64::{
    alphabet,
    engine::{
        general_purpose::{GeneralPurpose, GeneralPurposeConfig},
        DecodePaddingMode,
    },
    Engine,
};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

const STANDARD: GeneralPurpose = GeneralPurpose::new(
    &alphabet::STANDARD,
    GeneralPurposeConfig::new().with_decode_padding_mode(DecodePaddingMode::Indifferent),
);

const URL_SAFE: GeneralPurpose = GeneralPurpose::new(
    &alphabet::URL_SAFE,
    GeneralPurposeConfig::new()
        .with_decode_padding_mode(DecodePaddingMode::Indifferent)
        .with_encode_padding(false),
);

/// Encode the input into a base64 string using the standard base64 encoding scheme.
///
/// # Arguments
/// * `input` - The input data to encode, which can be any type that implements `AsRef<[u8]>`.
///
/// # Returns
/// A `String` containing the base64 encoded representation of the input data.
///
/// # Examples
/// ```rust
/// # use typespec_client_core::base64;
/// let data = b"Hello, world!";
/// let encoded = base64::encode(data);
/// assert_eq!(encoded, "SGVsbG8sIHdvcmxkIQ==");
/// ```
pub fn encode<T>(input: T) -> String
where
    T: AsRef<[u8]>,
{
    STANDARD.encode(input)
}

/// Decode a base64 encoded string using the standard base64 decoding scheme.
///
/// # Arguments
/// * `input` - The base64 encoded input data to decode, which can be any type that implements `AsRef<[u8]>`.
///
/// # Returns
/// A `Result` containing a `Vec<u8>` with the decoded data, or an error if the input is not valid base64.
///
/// # Examples
/// ```rust
/// # use typespec_client_core::base64;
/// let encoded = "SGVsbG8sIHdvcmxkIQ==";
/// let decoded = base64::decode(encoded).expect("Decoding should succeed");
/// assert_eq!(decoded, b"Hello, world!");
/// ```
pub fn decode<T>(input: T) -> crate::Result<Vec<u8>>
where
    T: AsRef<[u8]>,
{
    Ok(STANDARD.decode(input)?)
}

/// Encode the input into a base64 string using the URL safe base64 encoding scheme (base64url).
///
/// # Arguments
/// * `input` - The input data to encode, which can be any type that implements `AsRef<[u8]>`.
///
/// # Returns
/// A `String` containing the base64url encoded representation of the input data.
///
/// # Examples
/// ```rust
/// # use typespec_client_core::base64;
/// let data = b"Hello, world!";
/// let encoded = base64::encode_url_safe(data);
/// assert_eq!(encoded, "SGVsbG8sIHdvcmxkIQ");
/// ```
pub fn encode_url_safe<T>(input: T) -> String
where
    T: AsRef<[u8]>,
{
    URL_SAFE.encode(input)
}

/// Decode a base64url encoded string using the URL safe base64 decoding scheme. (base64url)
///
/// # Arguments
/// * `input` - The base64url encoded input data to decode, which can be any type that implements `AsRef<[u8]>`.
///
/// # Returns
/// A `Result` containing a `Vec<u8>` with the decoded data, or an error if the input is not valid base64url.
///
/// # Examples
/// ```rust
/// # use typespec_client_core::base64;
/// let encoded = "SGVsbG8sIHdvcmxkIQ";
/// let decoded = base64::decode_url_safe(encoded).expect("Decoding should succeed");
/// assert_eq!(decoded, b"Hello, world!");
/// ```
pub fn decode_url_safe<T>(input: T) -> crate::Result<Vec<u8>>
where
    T: AsRef<[u8]>,
{
    Ok(URL_SAFE.decode(input)?)
}

/// Helper that can be used in a serde deserialize_with derive macro
/// for struct fields that contain base64 encoded data.
///
/// Uses the standard base64 decoder.
///
/// # Examples
///
/// ```rust,no_run
/// # use serde::{Deserialize};
/// # use typespec_client_core::base64;
/// #[derive(Deserialize)]
/// struct SomeType {
///     #[serde(deserialize_with = "base64::deserialize")]
///     pub value: Vec<u8>,
/// }
/// ```
pub fn deserialize<'de, D>(deserializer: D) -> Result<Vec<u8>, D::Error>
where
    D: Deserializer<'de>,
{
    let decoded = String::deserialize(deserializer)?;
    decode(decoded).map_err(serde::de::Error::custom)
}

/// Helper that can be used in a serde deserialize_with derive macro
/// for struct fields that contain base64 encoded data.
///
/// Uses the URL safe base64 decoder.
///
/// # Examples
///
/// ```rust,no_run
/// # use serde::{Deserialize};
/// # use typespec_client_core::base64;
/// #[derive(Deserialize)]
/// struct SomeType {
///     #[serde(deserialize_with = "base64::deserialize_url_safe")]
///     pub value: Vec<u8>,
/// }
/// ```
pub fn deserialize_url_safe<'de, D>(deserializer: D) -> Result<Vec<u8>, D::Error>
where
    D: Deserializer<'de>,
{
    let decoded = String::deserialize(deserializer)?;
    decode_url_safe(decoded).map_err(serde::de::Error::custom)
}

/// Helper that can be used in a serde serialize_with derive macro
/// for struct fields that contain base64 encoded data.
///
/// Uses the standard base64 encoder.
///
/// # Examples
///
/// ```rust,no_run
/// # use serde::{Serialize};
/// # use typespec_client_core::base64;
/// #[derive(Serialize)]
/// struct SomeType {
///     #[serde(serialize_with = "base64::serialize")]
///     pub value: Vec<u8>,
/// }
/// ```
pub fn serialize<S, T>(to_serialize: &T, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
    T: AsRef<[u8]>,
{
    let encoded = encode(to_serialize.as_ref());
    String::serialize(&encoded, serializer)
}

/// Helper that can be used in a serde serialize_with derive macro
/// for struct fields that contain base64 encoded data.
///
/// Uses the URL safe base64 encoder.
///
/// # Examples
///
/// ```rust,no_run
/// # use serde::{Serialize};
/// # use typespec_client_core::base64;
/// #[derive(Serialize)]
/// struct SomeType {
///     #[serde(serialize_with = "base64::serialize_url_safe")]
///     pub value: Vec<u8>,
/// }
/// ```
pub fn serialize_url_safe<S, T>(to_serialize: &T, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
    T: AsRef<[u8]>,
{
    let encoded = encode_url_safe(to_serialize.as_ref());
    String::serialize(&encoded, serializer)
}

pub mod option {
    //! Serialization helpers for optional fields.

    use super::{decode, decode_url_safe, encode, encode_url_safe};
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    /// Helper that can be used in a serde deserialize_with derive macro
    /// for struct fields that contain base64 encoded data.
    ///
    /// Uses the standard base64 decoder.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use serde::{Deserialize};
    /// # use typespec_client_core::base64;
    /// #[derive(Deserialize)]
    /// struct SomeType {
    ///     #[serde(deserialize_with = "base64::option::deserialize")]
    ///     pub value: Option<Vec<u8>>,
    /// }
    /// ```
    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<Vec<u8>>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let decoded = <Option<String>>::deserialize(deserializer)?;
        match decoded {
            Some(d) => {
                let d = decode(d).map_err(serde::de::Error::custom)?;
                Ok(Some(d))
            }
            None => Ok(None),
        }
    }

    /// Helper that can be used in a serde deserialize_with derive macro
    /// for struct fields that contain base64 encoded data.
    ///
    /// Uses the URL safe base64 decoder.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use serde::{Deserialize};
    /// # use typespec_client_core::base64;
    /// #[derive(Deserialize)]
    /// struct SomeType {
    ///     #[serde(deserialize_with = "base64::option::deserialize_url_safe")]
    ///     pub value: Option<Vec<u8>>,
    /// }
    /// ```
    pub fn deserialize_url_safe<'de, D>(deserializer: D) -> Result<Option<Vec<u8>>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let decoded = <Option<String>>::deserialize(deserializer)?;
        match decoded {
            Some(d) => {
                let d = decode_url_safe(d).map_err(serde::de::Error::custom)?;
                Ok(Some(d))
            }
            None => Ok(None),
        }
    }

    /// Helper that can be used in a serde serialize_with derive macro
    /// for struct fields that contain base64 encoded data.
    ///
    /// Uses the standard base64 encoder.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use serde::{Serialize};
    /// # use typespec_client_core::base64;
    /// #[derive(Serialize)]
    /// struct SomeType {
    ///     #[serde(serialize_with = "base64::option::serialize")]
    ///     pub value: Option<Vec<u8>>,
    /// }
    /// ```
    pub fn serialize<S, T>(to_serialize: &Option<T>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
        T: AsRef<[u8]>,
    {
        let encoded = to_serialize.as_ref().map(encode);
        <Option<String>>::serialize(&encoded, serializer)
    }

    /// Helper that can be used in a serde serialize_with derive macro
    /// for struct fields that contain base64 encoded data.
    ///
    /// Uses the URL safe base64 encoder.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use serde::{Serialize};
    /// # use typespec_client_core::base64;
    /// #[derive(Serialize)]
    /// struct SomeType {
    ///     #[serde(serialize_with = "base64::option::serialize_url_safe")]
    ///     pub value: Option<Vec<u8>>,
    /// }
    /// ```
    pub fn serialize_url_safe<S, T>(
        to_serialize: &Option<T>,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
        T: AsRef<[u8]>,
    {
        let encoded = to_serialize.as_ref().map(encode_url_safe);
        <Option<String>>::serialize(&encoded, serializer)
    }
}

#[cfg(test)]
mod tests {
    use super::{
        decode, decode_url_safe, deserialize, deserialize_url_safe, encode, encode_url_safe,
        option, serialize, serialize_url_safe,
    };
    use serde::{Deserialize, Serialize};

    #[test]
    fn standard_encode() {
        assert_eq!(encode(b"Hello, world!"), "SGVsbG8sIHdvcmxkIQ==");
        assert_eq!(encode(b""), "");
        assert_eq!(encode(b"f"), "Zg==");
        assert_eq!(encode(b"fo"), "Zm8=");
        assert_eq!(encode(b"foo"), "Zm9v");
    }

    #[test]
    fn standard_decode() {
        assert_eq!(decode("SGVsbG8sIHdvcmxkIQ==").unwrap(), b"Hello, world!");
        assert_eq!(decode("").unwrap(), b"");
        assert_eq!(decode("Zg==").unwrap(), b"f");
        assert_eq!(decode("Zm8=").unwrap(), b"fo");
        assert_eq!(decode("Zm9v").unwrap(), b"foo");
    }

    #[test]
    fn url_safe_encode() {
        assert_eq!(encode_url_safe(b"Hello, world!"), "SGVsbG8sIHdvcmxkIQ");
        assert_eq!(encode_url_safe(b""), "");
        assert_eq!(encode_url_safe(b"f"), "Zg");
        assert_eq!(encode_url_safe(b"fo"), "Zm8");
        assert_eq!(encode_url_safe(b"foo"), "Zm9v");
    }

    #[test]
    fn url_safe_decode() {
        assert_eq!(
            decode_url_safe("SGVsbG8sIHdvcmxkIQ").unwrap(),
            b"Hello, world!"
        );
        assert_eq!(decode_url_safe("").unwrap(), b"");
        assert_eq!(decode_url_safe("Zg").unwrap(), b"f");
        assert_eq!(decode_url_safe("Zm8").unwrap(), b"fo");
        assert_eq!(decode_url_safe("Zm9v").unwrap(), b"foo");
    }

    #[test]
    fn roundtrip_standard() {
        let data = b"The quick brown fox jumps over the lazy dog";
        assert_eq!(decode(encode(data)).unwrap(), data);
    }

    #[test]
    fn roundtrip_url_safe() {
        let data = b"The quick brown fox jumps over the lazy dog";
        assert_eq!(decode_url_safe(encode_url_safe(data)).unwrap(), data);
    }

    #[derive(Serialize, Deserialize)]
    struct TestStruct {
        #[serde(serialize_with = "serialize", deserialize_with = "deserialize")]
        data: Vec<u8>,
    }

    #[test]
    fn serde_standard() {
        let original = TestStruct {
            data: b"test data".to_vec(),
        };
        let json = serde_json::to_string(&original).unwrap();
        assert!(json.contains("dGVzdCBkYXRh"));
        let deserialized: TestStruct = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.data, original.data);
    }

    #[derive(Serialize, Deserialize)]
    struct TestStructUrlSafe {
        #[serde(
            serialize_with = "serialize_url_safe",
            deserialize_with = "deserialize_url_safe"
        )]
        data: Vec<u8>,
    }

    #[test]
    fn serde_url_safe() {
        let original = TestStructUrlSafe {
            data: b"test data".to_vec(),
        };
        let json = serde_json::to_string(&original).unwrap();
        assert!(json.contains("dGVzdCBkYXRh"));
        let deserialized: TestStructUrlSafe = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.data, original.data);
    }

    #[derive(Serialize, Deserialize)]
    struct TestOptionalStruct {
        #[serde(
            serialize_with = "option::serialize",
            deserialize_with = "option::deserialize"
        )]
        data: Option<Vec<u8>>,
    }

    #[test]
    fn serde_option_some() {
        let original = TestOptionalStruct {
            data: Some(b"test data".to_vec()),
        };
        let json = serde_json::to_string(&original).unwrap();
        assert!(json.contains("dGVzdCBkYXRh"));
        let deserialized: TestOptionalStruct = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.data, original.data);
    }

    #[test]
    fn serde_option_none() {
        let original = TestOptionalStruct { data: None };
        let json = serde_json::to_string(&original).unwrap();
        assert!(json.contains("null"));
        let deserialized: TestOptionalStruct = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.data, None);
    }

    #[derive(Serialize, Deserialize)]
    struct TestOptionalStructUrlSafe {
        #[serde(
            serialize_with = "option::serialize_url_safe",
            deserialize_with = "option::deserialize_url_safe"
        )]
        data: Option<Vec<u8>>,
    }

    #[test]
    fn serde_option_url_safe_some() {
        let original = TestOptionalStructUrlSafe {
            data: Some(b"test data".to_vec()),
        };
        let json = serde_json::to_string(&original).unwrap();
        assert!(json.contains("dGVzdCBkYXRh"));
        let deserialized: TestOptionalStructUrlSafe = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.data, original.data);
    }

    #[test]
    fn serde_option_url_safe_none() {
        let original = TestOptionalStructUrlSafe { data: None };
        let json = serde_json::to_string(&original).unwrap();
        assert!(json.contains("null"));
        let deserialized: TestOptionalStructUrlSafe = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.data, None);
    }

    #[test]
    fn decode_invalid_standard() {
        assert!(decode("invalid!@#$").is_err());
    }

    #[test]
    fn decode_invalid_url_safe() {
        assert!(decode_url_safe("invalid!@#$").is_err());
    }
}

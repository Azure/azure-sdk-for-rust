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
///     #[serde(deserialize_with = "base64::deserialize_url_safe")]
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
///     #[serde(serialize_with = "base64::serialize")]
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
///     #[serde(serialize_with = "base64::serialize_url_safe")]
///     pub value: Option<Vec<u8>>,
/// }
/// ```
pub fn serialize_url_safe<S, T>(to_serialize: &Option<T>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
    T: AsRef<[u8]>,
{
    let encoded = to_serialize.as_ref().map(encode_url_safe);
    <Option<String>>::serialize(&encoded, serializer)
}

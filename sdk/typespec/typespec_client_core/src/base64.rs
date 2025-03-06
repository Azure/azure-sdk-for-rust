// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

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

pub fn encode<T>(input: T) -> String
where
    T: AsRef<[u8]>,
{
    STANDARD.encode(input)
}

pub fn decode<T>(input: T) -> crate::Result<Vec<u8>>
where
    T: AsRef<[u8]>,
{
    Ok(STANDARD.decode(input)?)
}

pub fn encode_url_safe<T>(input: T) -> String
where
    T: AsRef<[u8]>,
{
    URL_SAFE.encode(input)
}

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
    let decoded = <Option<String>>::deserialize(deserializer)?;
    match decoded {
        Some(d) => {
            let d = decode(d).map_err(serde::de::Error::custom)?;
            Ok(d)
        }
        None => Ok(Vec::default()),
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
///     pub value: Vec<u8>,
/// }
/// ```
pub fn deserialize_url_safe<'de, D>(deserializer: D) -> Result<Vec<u8>, D::Error>
where
    D: Deserializer<'de>,
{
    let decoded = <Option<String>>::deserialize(deserializer)?;
    match decoded {
        Some(d) => {
            let d = decode_url_safe(d).map_err(serde::de::Error::custom)?;
            Ok(d)
        }
        None => Ok(Vec::default()),
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
///     pub value: Vec<u8>,
/// }
/// ```
pub fn serialize<S, T>(to_serialize: &T, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
    T: AsRef<[u8]>,
{
    let encoded = encode(to_serialize);
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
    let encoded = encode_url_safe(to_serialize);
    String::serialize(&encoded, serializer)
}

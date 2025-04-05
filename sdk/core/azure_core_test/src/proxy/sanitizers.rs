// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Sanitize request and response headers, query string parameters, and body properties.
use super::join;
use azure_core::http::headers::{AsHeaders, HeaderName, HeaderValue};
use serde::Serialize;
#[cfg(test)]
use std::collections::HashMap;
use std::{
    convert::Infallible,
    fmt,
    iter::{once, Once},
};

/// Sanitizes `$..id`.
pub const SANITIZE_BODY_ID: &str = "AZSDK3431";

/// Sanitizes `$..name`.
pub const SANITIZE_BODY_NAME: &str = "AZSDK3430";

/// Sanitizes `$..etag`.
pub const SANITIZE_BODY_ETAG: &str = "AZSDK3490";

/// Default sanitizers to remove.
// See <https://github.com/Azure/azure-sdk-tools/blob/528e97f7de3a97375beaf7b85e1334df013c8290/tools/test-proxy/Azure.Sdk.Tools.TestProxy/Common/SanitizerDictionary.cs>.
pub const DEFAULT_SANITIZERS_TO_REMOVE: &[&str; 2] = &[
    SANITIZE_BODY_ID,   // $..id
    SANITIZE_BODY_ETAG, // $..etag
];

/// Default sanitization replacement value, "Sanitized";
pub const DEFAULT_SANITIZED_VALUE: &str = "Sanitized";

/// Represents a sanitizer.
pub trait Sanitizer: AsHeaders + fmt::Debug + Serialize {}

macro_rules! impl_sanitizer {
    ($name:ident) => {
        impl Sanitizer for $name {}

        impl AsHeaders for $name {
            type Error = Infallible;
            type Iter = Once<(HeaderName, HeaderValue)>;
            fn as_headers(&self) -> Result<Self::Iter, Self::Error> {
                Ok(once((
                    super::ABSTRACTION_IDENTIFIER,
                    HeaderValue::from_static(stringify!($name)),
                )))
            }
        }
    };

    ($($name:ident),+ $(,)?) => {
        $(impl_sanitizer!($name);)*
    };
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct ApplyCondition {
    #[serde(rename = "UriRegex")]
    pub uri_regex: String,
}

impl_sanitizer!(BodyKeySanitizer, BodyRegexSanitizer, BodyStringSanitizer);

/// This sanitizer offers regular expression replacements within a returned JSON body for a specific JSONPath.
///
/// This sanitizer only applies to JSON bodies.
#[derive(Clone, Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BodyKeySanitizer {
    /// The JSONPath that will be checked for replacements.
    pub json_path: String,

    /// The substitution value. The default is [`DEFAULT_SANITIZED_VALUE`].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,

    /// The regular expression to search for.
    ///
    /// Can be defined as a simple regular expression replacement or, if [`BodyKeySanitizer::group_for_replace`] is set, a substitution operation.
    /// Defaults to replacing the entire string.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regex: Option<String>,

    /// The regular expression capture group to substitute.
    ///
    /// Do not set if you're invoking a simple replacement operation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_for_replace: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition: Option<ApplyCondition>,
}

#[test]
fn test_body_key_sanitizer_as_headers() {
    let sut = BodyKeySanitizer {
        json_path: String::from("$.values"),
        ..Default::default()
    };
    let actual = sut.as_headers().expect("expect headers");
    let expected: HashMap<HeaderName, HeaderValue> = HashMap::from_iter(vec![(
        "x-abstraction-identifier".into(),
        "BodyKeySanitizer".into(),
    )]);
    assert!(actual.eq(expected.into_iter()));
}

/// This sanitizer offers regular expression replacements within raw request and response bodies.
///
/// Specifically, this means the regular expression applies to the raw JSON.
/// If you are attempting to simply replace a specific JSON key, the [`BodyKeySanitizer`] is probably what you want to use.
#[derive(Clone, Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BodyRegexSanitizer {
    /// The substitution value. The default is [`DEFAULT_SANITIZED_VALUE`].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,

    /// The regular expression to search for.
    ///
    /// Can be defined as a simple regular expression replacement or, if [`BodyKeySanitizer::group_for_replace`] is set, a substitution operation.
    /// Defaults to replacing the entire string.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regex: Option<String>,

    /// The regular expression capture group to substitute.
    ///
    /// Do not set if you're invoking a simple replacement operation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_for_replace: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition: Option<ApplyCondition>,
}

#[test]
fn test_body_regex_sanitizer_as_headers() {
    let sut = BodyRegexSanitizer::default();
    let actual = sut.as_headers().expect("expect headers");
    let expected: HashMap<HeaderName, HeaderValue> = HashMap::from_iter(vec![(
        "x-abstraction-identifier".into(),
        "BodyRegexSanitizer".into(),
    )]);
    assert!(actual.eq(expected.into_iter()));
}

#[derive(Clone, Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BodyStringSanitizer {
    pub target: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition: Option<ApplyCondition>,
}

impl_sanitizer!(GeneralRegexSanitizer, GeneralStringSanitizer);

#[derive(Clone, Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GeneralRegexSanitizer {
    /// The substitution value. The default is [`DEFAULT_SANITIZED_VALUE`].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,

    /// The regular expression to search for.
    ///
    /// Can be defined as a simple regular expression replacement or, if [`BodyKeySanitizer::group_for_replace`] is set, a substitution operation.
    /// Defaults to replacing the entire string.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regex: Option<String>,

    /// The regular expression capture group to substitute.
    ///
    /// Do not set if you're invoking a simple replacement operation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_for_replace: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition: Option<ApplyCondition>,
}

#[derive(Clone, Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GeneralStringSanitizer {
    pub target: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition: Option<ApplyCondition>,
}

impl_sanitizer!(HeaderRegexSanitizer, HeaderStringSanitizer);

#[derive(Clone, Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct HeaderRegexSanitizer {
    pub key: String,

    /// The substitution value. The default is [`DEFAULT_SANITIZED_VALUE`].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,

    /// The regular expression to search for.
    ///
    /// Can be defined as a simple regular expression replacement or, if [`BodyKeySanitizer::group_for_replace`] is set, a substitution operation.
    /// Defaults to replacing the entire string.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regex: Option<String>,

    /// The regular expression capture group to substitute.
    ///
    /// Do not set if you're invoking a simple replacement operation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_for_replace: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition: Option<ApplyCondition>,
}

#[derive(Clone, Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct HeaderStringSanitizer {
    pub key: String,
    pub target: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition: Option<ApplyCondition>,
}

impl_sanitizer!(OAuthResponseSanitizer);

#[derive(Clone, Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OAuthResponseSanitizer;

#[derive(Clone, Copy, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum RegexEntryValues {
    Body,
    Header,
    Uri,
}

impl_sanitizer!(RegexEntrySanitizer);

#[derive(Clone, Debug, Serialize)]
pub struct RegexEntrySanitizer {
    pub target: RegexEntryValues,
    pub regex: String,
}

impl_sanitizer!(RemoveHeaderSanitizer);

#[derive(Clone, Debug, Serialize)]
pub struct RemoveHeaderSanitizer {
    #[serde(serialize_with = "join")]
    pub headers_for_removal: Vec<&'static str>,
}

impl_sanitizer!(
    UriRegexSanitizer,
    UriStringSanitizer,
    UriSubscriptionIdSanitizer,
);

#[derive(Clone, Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UriRegexSanitizer {
    /// The substitution value. The default is [`DEFAULT_SANITIZED_VALUE`].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,

    /// The regular expression to search for.
    ///
    /// Can be defined as a simple regular expression replacement or, if [`BodyKeySanitizer::group_for_replace`] is set, a substitution operation.
    /// Defaults to replacing the entire string.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regex: Option<String>,

    /// The regular expression capture group to substitute.
    ///
    /// Do not set if you're invoking a simple replacement operation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_for_replace: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition: Option<ApplyCondition>,
}

#[derive(Clone, Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UriStringSanitizer {
    pub target: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition: Option<ApplyCondition>,
}

#[derive(Clone, Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UriSubscriptionIdSanitizer {
    pub value: Option<String>,
    pub condition: Option<ApplyCondition>,
}

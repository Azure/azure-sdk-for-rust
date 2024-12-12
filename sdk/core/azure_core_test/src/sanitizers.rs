// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use azure_core::headers::{AsHeaders, HeaderName, HeaderValue};
use serde::Serialize;
#[cfg(test)]
use std::collections::HashMap;
use std::{
    convert::Infallible,
    fmt,
    iter::{once, Once},
};

/// Default sanitization replacement value, "Sanitized";
pub const SANITIZED_VALUE: &str = "Sanitized";
const ABSTRACTION_IDENTIFIER: HeaderName = HeaderName::from_static("x-abstraction-identifier");

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
                    ABSTRACTION_IDENTIFIER,
                    HeaderValue::from_static(stringify!($name)),
                )))
            }
        }
    };

    ($($name:ident),+) => {
        $(impl_sanitizer!($name))*

    };
}

/// This sanitizer offers regular expression replacements within a returned JSON body for a specific JSONPath.
///
/// This sanitizer only applies to JSON bodies.
#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BodyKeySanitizer {
    /// The JSONPath that will be checked for replacements.
    pub json_path: String,

    /// The substitution value. The default is [`SANITIZED_VALUE`].
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
}
impl_sanitizer!(BodyKeySanitizer);

#[test]
fn test_body_key_sanitizer_as_headers() {
    let sut = BodyKeySanitizer {
        json_path: String::from("$.values"),
        value: None,
        regex: None,
        group_for_replace: None,
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
    /// The substitution value. The default is [`SANITIZED_VALUE`].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,

    /// The regular expression to search for or the entire body if `None`.
    ///
    /// Can be defined as a simple regular expression replacement or, if [`BodyRegexSanitizer::group_for_replace`] is set, a substitution operation.
    /// Defaults to replacing the entire string.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regex: Option<String>,

    /// The regular expression capture group to substitute.
    ///
    /// Do not set if you're invoking a simple replacement operation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_for_replace: Option<String>,
}
impl_sanitizer!(BodyRegexSanitizer);

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

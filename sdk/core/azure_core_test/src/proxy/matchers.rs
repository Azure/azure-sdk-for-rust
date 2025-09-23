// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use super::join;
use azure_core::{
    http::headers::{AsHeaders, HeaderName, HeaderValue},
    Bytes,
};
use serde::Serialize;
use std::{convert::Infallible, iter};

// cspell:ignore headerless

/// The headers ignored by default for [`CustomDefaultMatcher`];
pub const DEFAULT_IGNORED_HEADERS: &[&str; 6] = &[
    "date",
    "request-id",
    // cspell:disable-next-line
    "traceparent",
    "user-agent",
    "x-ms-client-request-id",
    "x-ms-date",
];

/// Matchers to use for a recording or playback.
#[derive(Debug, Serialize)]
pub enum Matcher {
    BodilessMatcher,
    HeaderlessMatcher,
    #[serde(untagged)]
    CustomDefaultMatcher(CustomDefaultMatcher),
}

impl AsHeaders for Matcher {
    type Error = Infallible;
    type Iter = iter::Once<(HeaderName, HeaderValue)>;

    fn as_headers(&self) -> Result<Self::Iter, Self::Error> {
        let v = match self {
            Matcher::BodilessMatcher => stringify!(BodilessMatcher),
            Matcher::CustomDefaultMatcher(_) => stringify!(CustomDefaultMatcher),
            Matcher::HeaderlessMatcher => stringify!(HeaderlessMatcher),
        };
        Ok(iter::once((
            super::ABSTRACTION_IDENTIFIER,
            HeaderValue::from_static(v),
        )))
    }
}

impl TryFrom<Matcher> for Bytes {
    type Error = serde_json::Error;
    fn try_from(matcher: Matcher) -> std::result::Result<Self, Self::Error> {
        let v = serde_json::to_vec(&matcher)?;
        Ok(Bytes::from(v))
    }
}

/// A custom matcher.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CustomDefaultMatcher {
    /// Whether to compare bodies during playback.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compare_bodies: Option<bool>,

    /// Header to exclude from the recording.
    #[serde(serialize_with = "join", skip_serializing_if = "Vec::is_empty")]
    pub excluded_headers: Vec<&'static str>,

    /// Headers to ignore during playback.
    ///
    /// The default is [`DEFAULT_IGNORED_HEADERS`].
    #[serde(serialize_with = "join", skip_serializing_if = "Vec::is_empty")]
    pub ignored_headers: Vec<&'static str>,

    /// Whether to ignore the order of query parameters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ignore_query_ordering: Option<bool>,

    /// Query parameter names to ignore.
    #[serde(serialize_with = "join", skip_serializing_if = "Vec::is_empty")]
    pub ignored_query_parameters: Vec<&'static str>,
}

impl Default for CustomDefaultMatcher {
    fn default() -> Self {
        CustomDefaultMatcher {
            compare_bodies: None,
            excluded_headers: Vec::new(),
            ignored_headers: DEFAULT_IGNORED_HEADERS.to_vec(),
            ignore_query_ordering: None,
            ignored_query_parameters: Vec::new(),
        }
    }
}

impl From<CustomDefaultMatcher> for Matcher {
    fn from(matcher: CustomDefaultMatcher) -> Self {
        Matcher::CustomDefaultMatcher(matcher)
    }
}

#[test]
fn serialize_custom_default_matcher() {
    let v: Matcher = CustomDefaultMatcher {
        compare_bodies: Some(false),
        ignored_headers: vec!["foo", "bar"],
        ..Default::default()
    }
    .into();
    assert_eq!(
        serde_json::to_string(&v).unwrap(),
        r#"{"compareBodies":false,"ignoredHeaders":"foo,bar"}"#
    )
}

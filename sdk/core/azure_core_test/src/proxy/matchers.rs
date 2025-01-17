// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use super::join;
use azure_core::headers::{AsHeaders, HeaderName, HeaderValue};
use serde::Serialize;
use std::{convert::Infallible, iter};

// cspell:ignore headerless

/// Matchers to use for a recording or playback.
#[derive(Debug)]
pub enum Matcher {
    BodilessMatcher,
    CustomDefaultMatcher(CustomDefaultMatcherBody),
    HeaderlessMatcher,
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

/// A custom matcher.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CustomDefaultMatcherBody {
    /// Whether to compare bodies during playback.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compare_bodies: Option<bool>,

    /// Header to exclude from the recording.
    #[serde(serialize_with = "join", skip_serializing_if = "Vec::is_empty")]
    pub excluded_headers: Vec<&'static str>,

    /// Headers to ignore during playback.
    #[serde(serialize_with = "join", skip_serializing_if = "Vec::is_empty")]
    pub ignored_headers: Vec<&'static str>,

    /// Whether to ignore the order of query parameters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ignore_query_ordering: Option<bool>,

    /// Query parameter names to ignore.
    #[serde(serialize_with = "join", skip_serializing_if = "Vec::is_empty")]
    pub ignored_query_parameters: Vec<&'static str>,
}

#[test]
fn serialize_custom_default_matcher() {
    let v = CustomDefaultMatcherBody {
        compare_bodies: Some(false),
        ignored_headers: vec!["foo", "bar"],
        ..Default::default()
    };
    assert_eq!(
        serde_json::to_string(&v).unwrap(),
        r#"{"compareBodies":false,"ignoredHeaders":"foo,bar"}"#
    )
}

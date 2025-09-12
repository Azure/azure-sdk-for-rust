// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

#![allow(missing_docs)]

//! Shared utilities for testing client libraries built on `azure_core`.
//!
//! For a comprehensive suite of utilities for testing client libraries built on `azure_core`,
//! read documentation for the `azure_core_test` crate.

use crate::{
    error::{Error, ErrorKind},
    http::headers::{FromHeaders, Header, HeaderName, HeaderValue, Headers},
};
use std::{fmt, str::FromStr};

/// Whether to test client methods by playing back recordings, recording live sessions, or executing live sessions without recording.
#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub enum TestMode {
    /// Test client methods by playing back recordings.
    #[default]
    Playback,

    /// Record live sessions.
    Record,

    /// Execute live sessions without recording.
    Live,
}

impl TestMode {
    /// Gets the `TestMode` from the `AZURE_TEST_MODE` environment variable or returns the default if undefined.
    pub fn current() -> typespec::Result<Self> {
        std::env::var("AZURE_TEST_MODE").map_or_else(|_| Ok(TestMode::default()), |v| v.parse())
    }
}

impl fmt::Debug for TestMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.into())
    }
}

impl From<TestMode> for &'static str {
    fn from(mode: TestMode) -> Self {
        match mode {
            TestMode::Playback => "playback",
            TestMode::Record => "record",
            TestMode::Live => "live",
        }
    }
}

impl From<&TestMode> for &'static str {
    fn from(mode: &TestMode) -> Self {
        TestMode::into(*mode)
    }
}

impl FromStr for TestMode {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "playback" => Ok(Self::Playback),
            "record" => Ok(Self::Record),
            "live" => Ok(Self::Live),
            _ => Err(Error::message(
                ErrorKind::DataConversion,
                "expected 'playback', 'record', or 'live'",
            )),
        }
    }
}

/// The `x-recording-mode` header name added by test-proxy.
pub const RECORDING_MODE: HeaderName = HeaderName::from_static("x-recording-mode");

/// Represents the `x-recording-mode` header added by the test-proxy.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum RecordingMode {
    #[default]
    Playback,
    Record,
}

impl From<RecordingMode> for &'static str {
    fn from(mode: RecordingMode) -> Self {
        match mode {
            RecordingMode::Playback => "playback",
            RecordingMode::Record => "record",
        }
    }
}

impl From<&RecordingMode> for &'static str {
    fn from(mode: &RecordingMode) -> Self {
        RecordingMode::into(*mode)
    }
}

impl FromStr for RecordingMode {
    type Err = crate::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "playback" => Ok(Self::Playback),
            "record" => Ok(Self::Record),
            _ => Err(Error::message(
                ErrorKind::DataConversion,
                "expected 'playback' or 'record'",
            )),
        }
    }
}

impl Header for RecordingMode {
    fn name(&self) -> HeaderName {
        RECORDING_MODE
    }

    fn value(&self) -> HeaderValue {
        HeaderValue::from_static(self.into())
    }
}

impl FromHeaders for RecordingMode {
    type Error = Error;

    fn header_names() -> &'static [&'static str] {
        &["x-recording-mode"]
    }

    fn from_headers(headers: &Headers) -> Result<Option<Self>, Self::Error> {
        let Some(value) = headers.iter().find_map(|(name, value)| {
            if name.as_str().eq_ignore_ascii_case(RECORDING_MODE.as_str()) {
                return Some(value);
            }

            None
        }) else {
            return Ok(None);
        };

        Ok(Some(value.as_str().parse()?))
    }
}

impl TryFrom<TestMode> for RecordingMode {
    type Error = Error;

    fn try_from(value: TestMode) -> crate::Result<Self> {
        match value {
            TestMode::Playback => Ok(RecordingMode::Playback),
            TestMode::Record => Ok(RecordingMode::Record),
            _ => Err(Error::new(
                ErrorKind::DataConversion,
                "expected `Playback` or `Record`",
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::http::headers::{FromHeaders, Header, HeaderName, HeaderValue, Headers};
    use std::str::FromStr;

    #[test]
    fn from_str_playback() {
        assert_eq!(
            RecordingMode::from_str("playback").unwrap(),
            RecordingMode::Playback
        );
        assert_eq!(
            RecordingMode::from_str("Playback").unwrap(),
            RecordingMode::Playback
        );
    }

    #[test]
    fn from_str_record() {
        assert_eq!(
            RecordingMode::from_str("record").unwrap(),
            RecordingMode::Record
        );
        assert_eq!(
            RecordingMode::from_str("Record").unwrap(),
            RecordingMode::Record
        );
    }

    #[test]
    fn from_str_invalid() {
        assert!(RecordingMode::from_str("invalid").is_err());
        assert!(RecordingMode::from_str("").is_err());
        assert!(RecordingMode::from_str("live").is_err());
    }

    #[test]
    fn into_str_ref() {
        let playback = RecordingMode::Playback;
        let record = RecordingMode::Record;

        assert_eq!(<&str>::from(&playback), "playback");
        assert_eq!(<&str>::from(&record), "record");
    }

    #[test]
    fn header_trait() {
        let playback = RecordingMode::Playback;
        let record = RecordingMode::Record;

        assert_eq!(playback.name(), HeaderName::from_static("x-recording-mode"));
        assert_eq!(record.name(), HeaderName::from_static("x-recording-mode"));

        assert_eq!(playback.value(), HeaderValue::from_static("playback"));
        assert_eq!(record.value(), HeaderValue::from_static("record"));
    }

    #[test]
    fn from_headers_present() {
        let mut headers = Headers::new();
        headers.insert(
            HeaderName::from_static("x-recording-mode"),
            HeaderValue::from_static("playback"),
        );

        let result = RecordingMode::from_headers(&headers).unwrap();
        assert_eq!(result, Some(RecordingMode::Playback));

        let mut headers = Headers::new();
        headers.insert(
            HeaderName::from_static("x-recording-mode"),
            HeaderValue::from_static("record"),
        );

        let result = RecordingMode::from_headers(&headers).unwrap();
        assert_eq!(result, Some(RecordingMode::Record));
    }

    #[test]
    fn from_headers_case_insensitive() {
        // Test that header parsing is case-insensitive for header values
        let mut headers = Headers::new();
        headers.insert(
            HeaderName::from_static("x-recording-mode"),
            HeaderValue::from_static("playback"),
        );

        let result = RecordingMode::from_headers(&headers).unwrap();
        assert_eq!(result, Some(RecordingMode::Playback));

        let mut headers = Headers::new();
        headers.insert(
            HeaderName::from_static("x-recording-mode"),
            HeaderValue::from_static("RECORD"),
        );

        let result = RecordingMode::from_headers(&headers).unwrap();
        assert_eq!(result, Some(RecordingMode::Record));
    }

    #[test]
    fn from_headers_not_present() {
        let headers = Headers::new();
        let result = RecordingMode::from_headers(&headers).unwrap();
        assert_eq!(result, None);
    }

    #[test]
    fn from_headers_invalid_value() {
        let mut headers = Headers::new();
        headers.insert(
            HeaderName::from_static("x-recording-mode"),
            HeaderValue::from_static("invalid"),
        );

        let result = RecordingMode::from_headers(&headers);
        assert!(result.is_err());
    }

    #[test]
    fn header_names() {
        let names = RecordingMode::header_names();
        assert_eq!(names, &["x-recording-mode"]);
    }

    #[test]
    fn default_value() {
        let default = RecordingMode::default();
        assert_eq!(default, RecordingMode::Playback);
    }
}

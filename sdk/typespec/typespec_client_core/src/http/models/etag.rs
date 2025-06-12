// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use serde::{Deserialize, Serialize};
use std::{fmt, str::FromStr};

use crate::http::headers::HeaderValue;

/// Represents an ETag for versioned resources.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Etag(String);

// Implementation for common string types
impl From<&str> for Etag {
    fn from(s: &str) -> Self {
        Self(s.to_string())
    }
}

impl From<String> for Etag {
    fn from(s: String) -> Self {
        Self(s)
    }
}

impl AsRef<str> for Etag {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl FromStr for Etag {
    type Err = crate::error::Error;
    fn from_str(s: &str) -> crate::Result<Self> {
        Ok(Self(s.into()))
    }
}

impl fmt::Display for Etag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.0, f)
    }
}

impl From<Etag> for String {
    fn from(etag: Etag) -> Self {
        etag.0
    }
}

impl From<Etag> for HeaderValue {
    fn from(etag: Etag) -> Self {
        HeaderValue::from(String::from(etag))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::http::headers::HeaderValue;

    #[test]
    fn from_string() {
        let etag = Etag::from("test-etag");
        assert_eq!("test-etag", etag.0);
    }

    #[test]
    fn from_str() {
        let etag = "test-etag".parse::<Etag>().unwrap();
        assert_eq!("test-etag", etag.0);
    }

    #[test]
    fn as_ref() {
        let etag = Etag::from("test-etag");
        assert_eq!("test-etag", etag.as_ref());
    }

    #[test]
    fn display() {
        let etag = Etag::from("test-etag");
        assert_eq!("test-etag", etag.to_string());
    }

    #[test]
    fn to_string() {
        let etag = Etag::from("test-etag");
        let s: String = etag.into();
        assert_eq!("test-etag", s);
    }

    #[test]
    fn to_header_value() {
        let etag = Etag::from("test-etag");
        let header_value: HeaderValue = etag.into();
        assert_eq!("test-etag", header_value.as_str());
    }
}

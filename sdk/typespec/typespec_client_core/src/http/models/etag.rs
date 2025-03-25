// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use serde::{Deserialize, Serialize};
use std::{fmt, str::FromStr};

/// Represents an ETag for versioned resources.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Etag(String);

impl<T> From<T> for Etag
where
    T: Into<String>,
{
    fn from(t: T) -> Self {
        Self(t.into())
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

// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! RFC 7231 date and time parsing and formatting functions.
use crate::time::{parse_rfc7231, to_rfc7231};
use serde::{de, Deserialize, Deserializer, Serializer};
use time::OffsetDateTime;

/// Deserialize an RFC 7231 date and time string into an [`OffsetDateTime`].
pub fn deserialize<'de, D>(deserializer: D) -> Result<OffsetDateTime, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    parse_rfc7231(&s).map_err(de::Error::custom)
}

/// Serialize an [`OffsetDateTime`] to an RFC 7231 date and time string.
pub fn serialize<S>(date: &OffsetDateTime, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(&to_rfc7231(date))
}

pub mod option {
    use crate::time::{parse_rfc7231, to_rfc7231};
    use serde::{Deserialize, Deserializer, Serializer};
    use time::OffsetDateTime;

    /// Deserialize an RFC 7231 date and time string into an optional [`OffsetDateTime`].
    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<OffsetDateTime>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: Option<String> = Option::deserialize(deserializer)?;
        s.map(|s| parse_rfc7231(&s).map_err(serde::de::Error::custom))
            .transpose()
    }

    /// Serialize an optional [`OffsetDateTime`] to an RFC 7231 date and time string.
    pub fn serialize<S>(date: &Option<OffsetDateTime>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if let Some(date) = date {
            serializer.serialize_str(&to_rfc7231(date))
        } else {
            serializer.serialize_none()
        }
    }
}

// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Internal use only -- referenced by code generated from TypeSpec.
//!
//! Serde helpers that format [`OffsetDateTime`] values as RFC 3339 strings for
//! Azure Storage requests and parse RFC 3339 responses. Use the `seconds_only`
//! sub-module for endpoints that reject fractional seconds.

use azure_core::time::{parse_rfc3339, OffsetDateTime};
use serde::{Deserialize, Deserializer, Serializer};
use time::format_description::well_known::Rfc3339;

/// Serializes an [`OffsetDateTime`] as a standard RFC 3339 string.
pub fn serialize<S>(value: &OffsetDateTime, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let s = value.format(&Rfc3339).map_err(serde::ser::Error::custom)?;
    serializer.serialize_str(&s)
}

/// Deserializes an [`OffsetDateTime`] from a standard RFC 3339 string.
pub fn deserialize<'de, D>(deserializer: D) -> Result<OffsetDateTime, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    parse_rfc3339(&s).map_err(serde::de::Error::custom)
}

/// Serde helpers for `Option<OffsetDateTime>` fields using standard RFC 3339.
pub mod option {
    use azure_core::time::{parse_rfc3339, OffsetDateTime};
    use serde::{Deserialize, Deserializer, Serializer};
    use time::format_description::well_known::Rfc3339;

    /// Serializes an `Option<OffsetDateTime>` as a standard RFC 3339 string, or
    /// `null` if `None`.
    pub fn serialize<S>(value: &Option<OffsetDateTime>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match value {
            Some(dt) => {
                let s = dt.format(&Rfc3339).map_err(serde::ser::Error::custom)?;
                serializer.serialize_str(&s)
            }
            None => serializer.serialize_none(),
        }
    }

    /// Deserializes an `Option<OffsetDateTime>` from a standard RFC 3339 string.
    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<OffsetDateTime>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = <Option<String>>::deserialize(deserializer)?;
        match value {
            Some(s) => parse_rfc3339(&s)
                .map(Some)
                .map_err(serde::de::Error::custom),
            None => Ok(None),
        }
    }
}

/// Serde helpers that format [`OffsetDateTime`] truncated to whole seconds
/// (e.g. `2022-08-12T20:55:02Z`).
///
/// Some Azure Storage service endpoints reject timestamps with fractional
/// seconds. Use this for request bodies; deserialization is unaffected and
/// can use the standard RFC 3339 helpers in the parent module.
pub mod seconds_only {
    use azure_core::time::OffsetDateTime;
    use serde::Serializer;
    use time::format_description::FormatItem;

    const FORMAT: &[FormatItem<'static>] =
        time::macros::format_description!("[year]-[month]-[day]T[hour]:[minute]:[second]Z");

    /// Serializes an [`OffsetDateTime`] as an RFC 3339 string with no
    /// fractional seconds, normalized to UTC.
    pub fn serialize<S>(value: &OffsetDateTime, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = value
            .to_offset(time::UtcOffset::UTC)
            .format(FORMAT)
            .map_err(serde::ser::Error::custom)?;
        serializer.serialize_str(&s)
    }

    /// Serde helpers for `Option<OffsetDateTime>` fields using the seconds-only
    /// format.
    pub mod option {
        use azure_core::time::OffsetDateTime;
        use serde::Serializer;

        /// Serializes an `Option<OffsetDateTime>` as an RFC 3339 string with no
        /// fractional seconds, or skips if `None`.
        pub fn serialize<S>(
            value: &Option<OffsetDateTime>,
            serializer: S,
        ) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match value {
                Some(dt) => super::serialize(dt, serializer),
                None => serializer.serialize_none(),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use azure_core::time::OffsetDateTime;
    use serde::{Deserialize, Serialize};
    use time::macros::datetime;

    #[derive(Debug, Deserialize, PartialEq, Serialize)]
    struct Wrap {
        #[serde(with = "super")]
        ts: OffsetDateTime,
    }

    #[derive(Debug, Deserialize, PartialEq, Serialize)]
    struct WrapOption {
        #[serde(with = "super::option")]
        ts: Option<OffsetDateTime>,
    }

    #[derive(Debug, Serialize)]
    struct WrapSecondsOnly {
        #[serde(serialize_with = "super::seconds_only::serialize")]
        ts: OffsetDateTime,
    }

    #[derive(Debug, Serialize)]
    struct WrapSecondsOnlyOption {
        #[serde(serialize_with = "super::seconds_only::option::serialize")]
        ts: Option<OffsetDateTime>,
    }

    #[test]
    fn rfc3339_roundtrip() {
        let value = Wrap {
            ts: datetime!(2026-06-05 00:03:53.300956 UTC),
        };
        let json = serde_json::to_string(&value).unwrap();
        assert_eq!(json, r#"{"ts":"2026-06-05T00:03:53.300956Z"}"#);
        let parsed: Wrap = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed, value);
    }

    #[test]
    fn rfc3339_option_some_roundtrip() {
        let value = WrapOption {
            ts: Some(datetime!(2026-06-05 00:03:53 UTC)),
        };
        let json = serde_json::to_string(&value).unwrap();
        assert_eq!(json, r#"{"ts":"2026-06-05T00:03:53Z"}"#);
        let parsed: WrapOption = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed, value);
    }

    #[test]
    fn rfc3339_option_none_roundtrip() {
        let value = WrapOption { ts: None };
        let json = serde_json::to_string(&value).unwrap();
        assert_eq!(json, r#"{"ts":null}"#);
        let parsed: WrapOption = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed, value);
    }

    #[test]
    fn seconds_only_truncates_fractional() {
        let value = WrapSecondsOnly {
            ts: datetime!(2026-06-05 00:03:53.300956 UTC),
        };
        let json = serde_json::to_string(&value).unwrap();
        assert_eq!(json, r#"{"ts":"2026-06-05T00:03:53Z"}"#);
    }

    #[test]
    fn seconds_only_normalizes_to_utc() {
        // 12:00:00 at +02:00 == 10:00:00 UTC.
        let value = WrapSecondsOnly {
            ts: datetime!(2026-06-05 12:00:00 +2),
        };
        let json = serde_json::to_string(&value).unwrap();
        assert_eq!(json, r#"{"ts":"2026-06-05T10:00:00Z"}"#);
    }

    #[test]
    fn seconds_only_option_some() {
        let value = WrapSecondsOnlyOption {
            ts: Some(datetime!(2022-08-12 20:55:02.123 UTC)),
        };
        let json = serde_json::to_string(&value).unwrap();
        assert_eq!(json, r#"{"ts":"2022-08-12T20:55:02Z"}"#);
    }

    #[test]
    fn seconds_only_option_none() {
        let value = WrapSecondsOnlyOption { ts: None };
        let json = serde_json::to_string(&value).unwrap();
        assert_eq!(json, r#"{"ts":null}"#);
    }
}

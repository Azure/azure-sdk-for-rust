// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Opaque parsed session token segment for merge operations.

use super::vector_session_token::SessionTokenValue;
use azure_core::{error::ErrorKind, fmt::SafeDebug};
use std::fmt;

/// An opaque parsed session token segment supporting merge operations.
///
/// A session token segment has the format `<pkRangeId>:<value>` where `<value>`
/// is either a V1 simple LSN or a V2 version vector. This type supports parsing,
/// merging, and formatting session token segments without exposing the internal
/// token format details.
#[derive(Clone, SafeDebug)]
pub struct SessionTokenSegment {
    pk_range_id: String,
    value: SessionTokenValue,
}

impl SessionTokenSegment {
    /// Parses a session token segment string.
    ///
    /// The expected format is `<pkRangeId>:<value>`.
    pub fn parse(s: &str) -> azure_core::Result<Self> {
        let (pk_range_id, value_str) = s.trim().split_once(':').ok_or_else(|| {
            azure_core::Error::with_message(
                ErrorKind::DataConversion,
                "invalid session token segment: missing ':'",
            )
        })?;
        let value = SessionTokenValue::parse(value_str)?;
        Ok(Self {
            pk_range_id: pk_range_id.to_owned(),
            value,
        })
    }

    /// Returns the partition key range ID.
    pub fn pk_range_id(&self) -> &str {
        &self.pk_range_id
    }

    /// Returns the global logical sequence number of this token.
    pub fn global_lsn(&self) -> u64 {
        self.value.global_lsn()
    }

    /// Merges another segment's token value into this one, returning `true` if modified.
    ///
    /// Only the token values are merged; the partition key range ID is unchanged.
    pub fn merge_value(&mut self, other: &Self) -> bool {
        self.value.merge(&other.value)
    }

    /// Sets the partition key range ID.
    pub fn set_pk_range_id(&mut self, id: impl Into<String>) {
        self.pk_range_id = id.into();
    }
}

impl fmt::Display for SessionTokenSegment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.pk_range_id, self.value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_v2_segment() {
        let seg = SessionTokenSegment::parse("0:1#100#1=10").unwrap();
        assert_eq!(seg.pk_range_id(), "0");
        assert_eq!(seg.global_lsn(), 100);
    }

    #[test]
    fn parse_v1_segment() {
        let seg = SessionTokenSegment::parse("5:500").unwrap();
        assert_eq!(seg.pk_range_id(), "5");
        assert_eq!(seg.global_lsn(), 500);
    }

    #[test]
    fn parse_missing_colon() {
        assert!(SessionTokenSegment::parse("no_colon").is_err());
    }

    #[test]
    fn display_round_trip() {
        let seg = SessionTokenSegment::parse("0:1#100#1=10").unwrap();
        let s = seg.to_string();
        let seg2 = SessionTokenSegment::parse(&s).unwrap();
        assert_eq!(seg2.pk_range_id(), "0");
        assert_eq!(seg2.global_lsn(), 100);
    }

    #[test]
    fn merge_takes_max() {
        let mut seg1 = SessionTokenSegment::parse("0:1#100#3=50").unwrap();
        let seg2 = SessionTokenSegment::parse("0:1#200#3=60").unwrap();
        assert!(seg1.merge_value(&seg2));
        assert_eq!(seg1.global_lsn(), 200);
    }

    #[test]
    fn set_pk_range_id_updates_display() {
        let mut seg = SessionTokenSegment::parse("0:1#100").unwrap();
        seg.set_pk_range_id("5");
        assert_eq!(seg.pk_range_id(), "5");
        assert_eq!(seg.to_string(), "5:1#100");
    }
}

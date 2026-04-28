// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Opaque parsed session token segment for merge operations.

use super::vector_session_token::SessionTokenValue;
use azure_core::{error::ErrorKind, fmt::SafeDebug};
use std::fmt;
use std::str::FromStr;

/// An opaque parsed session token segment supporting merge operations.
///
/// A session token segment has the format `<pkRangeId>:<value>` where `<value>`
/// is either a V1 simple LSN or a V2 version vector. This type supports parsing,
/// merging, and formatting session token segments without exposing the internal
/// token format details.
#[derive(Clone, SafeDebug)]
#[non_exhaustive]
pub struct SessionTokenSegment {
    pk_range_id: String,
    value: SessionTokenValue,
}

impl FromStr for SessionTokenSegment {
    type Err = azure_core::Error;

    fn from_str(s: &str) -> azure_core::Result<Self> {
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
}

impl SessionTokenSegment {
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

    /// Returns `true` if this segment is at least as recent as `other`.
    ///
    /// Recency is determined by comparing token versions first (higher version = newer
    /// topology), then global LSN and per-region LSNs when versions match. This matches
    /// the .NET SDK's `VectorSessionToken.IsValid` semantics.
    pub fn is_as_recent_as(&self, other: &Self) -> bool {
        self.value.is_as_recent_as(&other.value)
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
        let seg: SessionTokenSegment = "0:1#100#1=10".parse().unwrap();
        assert_eq!(seg.pk_range_id(), "0");
        assert_eq!(seg.global_lsn(), 100);
    }

    #[test]
    fn parse_v1_segment() {
        let seg: SessionTokenSegment = "5:500".parse().unwrap();
        assert_eq!(seg.pk_range_id(), "5");
        assert_eq!(seg.global_lsn(), 500);
    }

    #[test]
    fn parse_missing_colon() {
        assert!("no_colon".parse::<SessionTokenSegment>().is_err());
    }

    #[test]
    fn display_formats_expected_string() {
        let seg: SessionTokenSegment = "0:1#100#1=10".parse().unwrap();
        assert_eq!(seg.to_string(), "0:1#100#1=10");
    }

    #[test]
    fn merge_takes_max() {
        let mut seg1: SessionTokenSegment = "0:1#100#3=50".parse().unwrap();
        let seg2: SessionTokenSegment = "0:1#200#3=60".parse().unwrap();
        assert!(seg1.merge_value(&seg2));
        assert_eq!(seg1.global_lsn(), 200);
    }

    #[test]
    fn set_pk_range_id_updates_display() {
        let mut seg: SessionTokenSegment = "0:1#100".parse().unwrap();
        seg.set_pk_range_id("5");
        assert_eq!(seg.pk_range_id(), "5");
        assert_eq!(seg.to_string(), "5:1#100");
    }

    #[test]
    fn is_as_recent_as_higher_version_wins() {
        let newer: SessionTokenSegment = "0:2#50#1=5".parse().unwrap();
        let older: SessionTokenSegment = "0:1#100#1=10".parse().unwrap();
        assert!(newer.is_as_recent_as(&older));
        assert!(!older.is_as_recent_as(&newer));
    }

    #[test]
    fn is_as_recent_as_same_version_uses_lsn() {
        let higher: SessionTokenSegment = "0:1#200#3=60".parse().unwrap();
        let lower: SessionTokenSegment = "0:1#100#3=50".parse().unwrap();
        assert!(higher.is_as_recent_as(&lower));
        assert!(!lower.is_as_recent_as(&higher));
    }
}

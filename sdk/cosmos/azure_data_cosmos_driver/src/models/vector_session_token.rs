// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Internal representation of a Cosmos DB session token version vector.
//!
//! A session token on the wire looks like `<pkRangeId>:<version>#<globalLSN>#<region>=<lsn>#…`
//! (V2 vector format) or `<pkRangeId>:<lsn>` (V1 simple format).
//!
//! This module handles the portion **after** the colon — the token value itself.

use std::{collections::HashMap, fmt};

use azure_core::fmt::SafeDebug;

/// A parsed session-token version vector (the part after the `:`).
///
/// Layout: `<version>#<global_lsn>#<region_id>=<region_lsn>#…`
#[derive(Clone, SafeDebug, PartialEq, Eq)]
pub(crate) struct VectorSessionToken {
    version: u64,
    global_lsn: u64,
    region_progress: HashMap<u64, u64>,
}

impl VectorSessionToken {
    /// Parses the version-vector portion of a session token string.
    ///
    /// Returns `None` if the string is malformed.
    pub(crate) fn parse(s: &str) -> Option<Self> {
        // Expected: version#globalLSN#region=lsn#region=lsn#...
        let mut parts = s.split('#');
        let version: u64 = parts.next()?.parse().ok()?;
        let global_lsn: u64 = parts.next()?.parse().ok()?;

        let mut region_progress = HashMap::new();
        for segment in parts {
            if segment.is_empty() {
                continue;
            }
            let (region_str, lsn_str) = segment.split_once('=')?;
            let region_id: u64 = region_str.parse().ok()?;
            let lsn: u64 = lsn_str.parse().ok()?;
            region_progress.insert(region_id, lsn);
        }

        Some(Self {
            version,
            global_lsn,
            region_progress,
        })
    }

    /// Returns `true` if this token is at least as recent as `other`.
    ///
    /// A token with a higher version is always considered more recent (captures
    /// partition topology changes). For same-version tokens, compares
    /// `global_lsn` and per-region LSN values.
    ///
    /// Regions present in `other` but missing in `self` are treated as behind.
    #[allow(dead_code)] // Will be used by PKRange cache resolution
    pub(crate) fn is_at_least_as_recent_as(&self, other: &Self) -> bool {
        if self.version > other.version {
            return true;
        }
        if self.version < other.version {
            return false;
        }
        // Same version: compare LSNs
        if self.global_lsn < other.global_lsn {
            return false;
        }
        for (&region, &other_lsn) in &other.region_progress {
            match self.region_progress.get(&region) {
                Some(&self_lsn) if self_lsn >= other_lsn => {}
                _ => return false,
            }
        }
        true
    }

    /// Merges `other` into `self`, keeping the higher version and
    /// per-region maximum LSN values. Returns `true` if `self` was modified.
    ///
    /// # Divergence from Java SDK
    ///
    /// Java's `VectorSessionToken.merge()` throws `InternalServerErrorException`
    /// if the versions differ or if region sets don't match. Our implementation
    /// uses a silent-union approach (take max of each region, ignore missing),
    /// following the .NET/Python pattern which is more tolerant of topology
    /// changes during splits/merges.
    pub(crate) fn merge(&mut self, other: &Self) -> bool {
        let mut changed = false;

        if other.version > self.version {
            self.version = other.version;
            changed = true;
        }
        if other.global_lsn > self.global_lsn {
            self.global_lsn = other.global_lsn;
            changed = true;
        }
        for (&region, &other_lsn) in &other.region_progress {
            let entry = self.region_progress.entry(region).or_insert(0);
            if other_lsn > *entry {
                *entry = other_lsn;
                changed = true;
            }
        }

        changed
    }
}

impl fmt::Display for VectorSessionToken {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}#{}", self.version, self.global_lsn)?;
        // Sort region IDs for deterministic output
        let mut regions: Vec<_> = self.region_progress.iter().collect();
        regions.sort_by_key(|&(&k, _)| k);
        for (&region, &lsn) in &regions {
            write!(f, "#{}={}", region, lsn)?;
        }
        Ok(())
    }
}

/// A parsed session token value, either V1 (simple LSN) or V2 (version vector).
///
/// V1 tokens are a bare integer LSN (e.g., `500`), used by older Cosmos DB
/// configurations. V2 tokens carry a version, global LSN, and per-region
/// progress (e.g., `1#100#1=20#2=5`).
///
/// Parsing tries V2 first; if that fails, falls back to V1.
#[derive(Clone, SafeDebug, PartialEq, Eq)]
pub(crate) enum SessionTokenValue {
    /// V1 simple token: a single LSN value.
    Simple(u64),
    /// V2 vector token: version + global LSN + per-region progress.
    Vector(VectorSessionToken),
}

impl SessionTokenValue {
    /// Parses a session token value string, trying V2 (vector) first, then V1 (simple).
    pub(crate) fn parse(s: &str) -> Option<Self> {
        if let Some(vector) = VectorSessionToken::parse(s) {
            return Some(Self::Vector(vector));
        }
        // V1 fallback: bare integer
        let lsn: u64 = s.parse().ok()?;
        Some(Self::Simple(lsn))
    }

    /// Merges `other` into `self`, returning `true` if `self` was modified.
    ///
    /// Merging across token types (V1 with V2) is not meaningful — the newer
    /// token type wins outright when types differ.
    pub(crate) fn merge(&mut self, other: &Self) -> bool {
        match (self as &Self, other) {
            (Self::Vector(_), Self::Vector(other_v)) => {
                if let Self::Vector(ref mut self_v) = self {
                    self_v.merge(other_v)
                } else {
                    unreachable!()
                }
            }
            (Self::Simple(self_lsn), Self::Simple(other_lsn)) => {
                if other_lsn > self_lsn {
                    *self = Self::Simple(*other_lsn);
                    true
                } else {
                    false
                }
            }
            // V2 supersedes V1
            (Self::Simple(_), Self::Vector(_)) => {
                *self = other.clone();
                true
            }
            // V2 stays; V1 is obsolete
            (Self::Vector(_), Self::Simple(_)) => false,
        }
    }

    /// Returns `true` if this token is at least as recent as `other`.
    #[allow(dead_code)] // Will be used by PKRange cache resolution
    pub(crate) fn is_at_least_as_recent_as(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Vector(a), Self::Vector(b)) => a.is_at_least_as_recent_as(b),
            (Self::Simple(a), Self::Simple(b)) => a >= b,
            // V2 is always more recent than V1
            (Self::Vector(_), Self::Simple(_)) => true,
            (Self::Simple(_), Self::Vector(_)) => false,
        }
    }
}

impl fmt::Display for SessionTokenValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Simple(lsn) => write!(f, "{lsn}"),
            Self::Vector(v) => v.fmt(f),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_simple_token() {
        let t = VectorSessionToken::parse("1#100#1=20#2=5#3=30").unwrap();
        assert_eq!(t.version, 1);
        assert_eq!(t.global_lsn, 100);
        assert_eq!(t.region_progress.len(), 3);
        assert_eq!(t.region_progress[&1], 20);
        assert_eq!(t.region_progress[&2], 5);
        assert_eq!(t.region_progress[&3], 30);
    }

    #[test]
    fn parse_no_regions() {
        let t = VectorSessionToken::parse("2#50").unwrap();
        assert_eq!(t.version, 2);
        assert_eq!(t.global_lsn, 50);
        assert!(t.region_progress.is_empty());
    }

    #[test]
    fn parse_invalid_no_hash() {
        assert!(VectorSessionToken::parse("nope").is_none());
    }

    #[test]
    fn parse_invalid_lsn() {
        assert!(VectorSessionToken::parse("1#abc").is_none());
    }

    #[test]
    fn parse_invalid_region_format() {
        assert!(VectorSessionToken::parse("1#100#bad").is_none());
    }

    #[test]
    fn display_roundtrip() {
        let t = VectorSessionToken::parse("1#100#1=20").unwrap();
        let s = t.to_string();
        let t2 = VectorSessionToken::parse(&s).unwrap();
        assert_eq!(t, t2);
    }

    #[test]
    fn display_regions_sorted() {
        let t = VectorSessionToken::parse("1#100#3=30#1=10#2=20").unwrap();
        assert_eq!(t.to_string(), "1#100#1=10#2=20#3=30");
    }

    #[test]
    fn merge_takes_max_global_lsn() {
        let mut a = VectorSessionToken::parse("1#100#1=10").unwrap();
        let b = VectorSessionToken::parse("1#200#1=10").unwrap();
        assert!(a.merge(&b));
        assert_eq!(a.global_lsn, 200);
    }

    #[test]
    fn merge_takes_max_region_lsn() {
        let mut a = VectorSessionToken::parse("1#100#1=10#2=20").unwrap();
        let b = VectorSessionToken::parse("1#100#1=30#2=5").unwrap();
        assert!(a.merge(&b));
        assert_eq!(a.region_progress[&1], 30);
        assert_eq!(a.region_progress[&2], 20);
    }

    #[test]
    fn merge_union_regions() {
        let mut a = VectorSessionToken::parse("1#100#1=10").unwrap();
        let b = VectorSessionToken::parse("1#100#2=20").unwrap();
        assert!(a.merge(&b));
        assert_eq!(a.region_progress[&2], 20);
    }

    #[test]
    fn merge_higher_version_wins() {
        let mut a = VectorSessionToken::parse("1#100").unwrap();
        let b = VectorSessionToken::parse("5#100").unwrap();
        assert!(a.merge(&b));
        assert_eq!(a.version, 5);
    }

    #[test]
    fn is_at_least_as_recent_same() {
        let a = VectorSessionToken::parse("1#100#1=10#2=20").unwrap();
        assert!(a.is_at_least_as_recent_as(&a));
    }

    #[test]
    fn is_at_least_as_recent_higher() {
        let a = VectorSessionToken::parse("1#200#1=30#2=25").unwrap();
        let b = VectorSessionToken::parse("1#100#1=10#2=20").unwrap();
        assert!(a.is_at_least_as_recent_as(&b));
    }

    #[test]
    fn is_at_least_as_recent_lower() {
        let a = VectorSessionToken::parse("1#50#1=10").unwrap();
        let b = VectorSessionToken::parse("1#100#1=10").unwrap();
        assert!(!a.is_at_least_as_recent_as(&b));
    }

    #[test]
    fn is_at_least_as_recent_missing_region() {
        let a = VectorSessionToken::parse("1#100#1=10").unwrap();
        let b = VectorSessionToken::parse("1#100#1=10#2=20").unwrap();
        assert!(!a.is_at_least_as_recent_as(&b));
    }

    #[test]
    fn is_at_least_as_recent_higher_version() {
        // A higher version token is always more recent, even with lower LSNs
        let a = VectorSessionToken::parse("2#50#1=5").unwrap();
        let b = VectorSessionToken::parse("1#100#1=10").unwrap();
        assert!(a.is_at_least_as_recent_as(&b));
        assert!(!b.is_at_least_as_recent_as(&a));
    }

    // === SessionTokenValue tests ===

    #[test]
    fn parse_v2_token() {
        let t = SessionTokenValue::parse("1#100#1=10").unwrap();
        assert!(matches!(t, SessionTokenValue::Vector(_)));
    }

    #[test]
    fn parse_v1_simple_token() {
        let t = SessionTokenValue::parse("500").unwrap();
        assert!(matches!(t, SessionTokenValue::Simple(500)));
    }

    #[test]
    fn parse_v1_display_roundtrip() {
        let t = SessionTokenValue::parse("12345").unwrap();
        assert_eq!(t.to_string(), "12345");
    }

    #[test]
    fn parse_invalid_returns_none() {
        assert!(SessionTokenValue::parse("not_a_token").is_none());
        assert!(SessionTokenValue::parse("").is_none());
    }

    #[test]
    fn v1_merge_takes_max() {
        let mut a = SessionTokenValue::parse("100").unwrap();
        let b = SessionTokenValue::parse("200").unwrap();
        assert!(a.merge(&b));
        assert_eq!(a, SessionTokenValue::Simple(200));
    }

    #[test]
    fn v1_merge_no_change_when_lower() {
        let mut a = SessionTokenValue::parse("200").unwrap();
        let b = SessionTokenValue::parse("100").unwrap();
        assert!(!a.merge(&b));
        assert_eq!(a, SessionTokenValue::Simple(200));
    }

    #[test]
    fn v2_supersedes_v1_on_merge() {
        let mut a = SessionTokenValue::parse("500").unwrap();
        let b = SessionTokenValue::parse("1#100#1=10").unwrap();
        assert!(a.merge(&b));
        assert!(matches!(a, SessionTokenValue::Vector(_)));
    }

    #[test]
    fn v2_not_replaced_by_v1_on_merge() {
        let mut a = SessionTokenValue::parse("1#100#1=10").unwrap();
        let b = SessionTokenValue::parse("99999").unwrap();
        assert!(!a.merge(&b));
        assert!(matches!(a, SessionTokenValue::Vector(_)));
    }

    #[test]
    fn v1_is_at_least_as_recent() {
        let a = SessionTokenValue::parse("200").unwrap();
        let b = SessionTokenValue::parse("100").unwrap();
        assert!(a.is_at_least_as_recent_as(&b));
        assert!(!b.is_at_least_as_recent_as(&a));
    }

    #[test]
    fn v2_always_more_recent_than_v1() {
        let v2 = SessionTokenValue::parse("1#50#1=5").unwrap();
        let v1 = SessionTokenValue::parse("99999").unwrap();
        assert!(v2.is_at_least_as_recent_as(&v1));
        assert!(!v1.is_at_least_as_recent_as(&v2));
    }
}

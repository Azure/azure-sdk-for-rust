// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Internal representation of a Cosmos DB session token version vector.
//!
//! A session token on the wire looks like `<pkRangeId>:<version>#<globalLSN>#<region>=<lsn>#…`
//! (V2 vector format) or `<pkRangeId>:<lsn>` (V1 simple format).
//!
//! This module handles the portion **after** the colon — the token value itself.

use std::{collections::HashMap, fmt};

use azure_core::{error::ErrorKind, fmt::SafeDebug};

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
    /// Returns an error if the string is malformed.
    pub(crate) fn parse(s: &str) -> azure_core::Result<Self> {
        // Expected: version#globalLSN#region=lsn#region=lsn#...
        let mut parts = s.split('#');

        let version_str = parts.next().ok_or_else(|| {
            azure_core::Error::with_message(
                ErrorKind::DataConversion,
                "invalid session token: empty input",
            )
        })?;
        let version: u64 = version_str.parse().map_err(|_| {
            azure_core::Error::with_message_fn(ErrorKind::DataConversion, || {
                format!("invalid session token: bad version '{version_str}'")
            })
        })?;

        let global_str = parts.next().ok_or_else(|| {
            azure_core::Error::with_message_fn(ErrorKind::DataConversion, || {
                format!("invalid session token: missing global LSN in '{s}'")
            })
        })?;
        let global_lsn: u64 = global_str.parse().map_err(|_| {
            azure_core::Error::with_message_fn(ErrorKind::DataConversion, || {
                format!("invalid session token: bad global LSN '{global_str}'")
            })
        })?;

        let mut region_progress = HashMap::new();
        for segment in parts {
            if segment.is_empty() {
                continue;
            }
            let (region_str, lsn_str) = segment.split_once('=').ok_or_else(|| {
                azure_core::Error::with_message_fn(ErrorKind::DataConversion, || {
                    format!("invalid session token: malformed region segment '{segment}'")
                })
            })?;
            let region_id: u64 = region_str.parse().map_err(|_| {
                azure_core::Error::with_message_fn(ErrorKind::DataConversion, || {
                    format!("invalid session token: bad region id '{region_str}'")
                })
            })?;
            let lsn: u64 = lsn_str.parse().map_err(|_| {
                azure_core::Error::with_message_fn(ErrorKind::DataConversion, || {
                    format!("invalid session token: bad region LSN '{lsn_str}'")
                })
            })?;
            region_progress.insert(region_id, lsn);
        }

        Ok(Self {
            version,
            global_lsn,
            region_progress,
        })
    }

    /// Returns the global logical sequence number.
    pub(crate) fn global_lsn(&self) -> u64 {
        self.global_lsn
    }

    /// Returns `true` if this token is at least as recent as `other`.
    ///
    /// A token with a higher version is always considered more recent (captures
    /// partition topology changes). For same-version tokens, compares
    /// `global_lsn` and per-region LSN values.
    ///
    /// Regions present in `other` but missing in `self` are treated as behind.
    #[allow(dead_code)] // Will be used by PKRange cache resolution
    pub(crate) fn is_as_recent_as(&self, other: &Self) -> bool {
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

    /// Merges `other` into `self`, returning `true` if `self` was modified.
    ///
    /// # False-Progress Protection
    ///
    /// When versions differ (topology changed due to partition split/merge/failover),
    /// the higher-version token's `global_lsn` is used directly — **not** `max(a, b)`.
    /// Taking the max would create a "false progress" token combining a newer topology
    /// version with an older LSN that never existed in that topology.
    ///
    /// For region progress, only the higher-version's regions are kept. Regions
    /// present only in the lower-version token are dropped (they may no longer exist
    /// after the topology change). Regions present in both use `max(regionLSN)`.
    ///
    /// When versions are equal, `max(globalLSN)` and per-region `max(regionLSN)` are
    /// used, matching the behavior of all SDKs.
    ///
    /// This matches Java's `isSessionTokenFalseProgressMergeEnabled=true` (the default).
    pub(crate) fn merge(&mut self, other: &Self) -> bool {
        if self.version == other.version {
            // Same topology: take max of everything (standard merge)
            let mut changed = false;
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
        } else {
            // Different topology: higher-version wins for globalLSN and region set.
            let (higher, lower) = if other.version > self.version {
                (other, &*self)
            } else {
                (&*self, other)
            };

            // Build merged region map: iterate higher-version's regions,
            // take max with lower-version where both exist.
            let mut merged_regions = HashMap::with_capacity(higher.region_progress.len());
            for (&region, &higher_lsn) in &higher.region_progress {
                let merged_lsn = match lower.region_progress.get(&region) {
                    Some(&lower_lsn) => std::cmp::max(higher_lsn, lower_lsn),
                    None => higher_lsn,
                };
                merged_regions.insert(region, merged_lsn);
            }
            // Regions only in lower-version are intentionally dropped.

            let new_version = higher.version;
            let new_global_lsn = higher.global_lsn;

            let changed = self.version != new_version
                || self.global_lsn != new_global_lsn
                || self.region_progress != merged_regions;

            if changed {
                self.version = new_version;
                self.global_lsn = new_global_lsn;
                self.region_progress = merged_regions;
            }
            changed
        }
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
    /// Returns the global logical sequence number.
    pub(crate) fn global_lsn(&self) -> u64 {
        match self {
            Self::Simple(lsn) => *lsn,
            Self::Vector(v) => v.global_lsn(),
        }
    }

    /// Parses a session token value string, trying V2 (vector) first, then V1 (simple).
    pub(crate) fn parse(s: &str) -> azure_core::Result<Self> {
        if let Ok(vector) = VectorSessionToken::parse(s) {
            return Ok(Self::Vector(vector));
        }
        // V1 fallback: bare integer
        let lsn: u64 = s.parse().map_err(|_| {
            azure_core::Error::with_message_fn(ErrorKind::DataConversion, || {
                format!("invalid session token value: '{s}' is not a valid V2 vector or V1 integer")
            })
        })?;
        Ok(Self::Simple(lsn))
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
    pub(crate) fn is_as_recent_as(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Vector(a), Self::Vector(b)) => a.is_as_recent_as(b),
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

    /// Helper to build a `VectorSessionToken` for assertions without parsing.
    fn make_token(version: u64, global_lsn: u64, regions: &[(u64, u64)]) -> VectorSessionToken {
        VectorSessionToken {
            version,
            global_lsn,
            region_progress: regions.iter().copied().collect(),
        }
    }

    #[test]
    fn parse_simple_token() {
        let t = VectorSessionToken::parse("1#100#1=20#2=5#3=30").unwrap();
        assert_eq!(t, make_token(1, 100, &[(1, 20), (2, 5), (3, 30)]));
    }

    #[test]
    fn parse_no_regions() {
        let t = VectorSessionToken::parse("2#50").unwrap();
        assert_eq!(t, make_token(2, 50, &[]));
    }

    #[test]
    fn parse_invalid_no_hash() {
        assert!(VectorSessionToken::parse("nope").is_err());
    }

    #[test]
    fn parse_invalid_lsn() {
        assert!(VectorSessionToken::parse("1#abc").is_err());
    }

    #[test]
    fn parse_invalid_region_format() {
        assert!(VectorSessionToken::parse("1#100#bad").is_err());
    }

    #[test]
    fn parse_produces_expected_structure() {
        let t = VectorSessionToken::parse("1#100#1=20#2=5").unwrap();
        assert_eq!(t, make_token(1, 100, &[(1, 20), (2, 5)]));
    }

    #[test]
    fn display_formats_expected_string() {
        let t = make_token(1, 100, &[(1, 20)]);
        assert_eq!(t.to_string(), "1#100#1=20");
    }

    #[test]
    fn display_regions_sorted() {
        let t = make_token(1, 100, &[(3, 30), (1, 10), (2, 20)]);
        assert_eq!(t.to_string(), "1#100#1=10#2=20#3=30");
    }

    #[test]
    fn merge_takes_max_global_lsn() {
        let mut a = VectorSessionToken::parse("1#100#1=10").unwrap();
        let b = VectorSessionToken::parse("1#200#1=10").unwrap();
        assert!(a.merge(&b));
        assert_eq!(a, make_token(1, 200, &[(1, 10)]));
    }

    #[test]
    fn merge_takes_max_region_lsn() {
        let mut a = VectorSessionToken::parse("1#100#1=10#2=20").unwrap();
        let b = VectorSessionToken::parse("1#100#1=30#2=5").unwrap();
        assert!(a.merge(&b));
        assert_eq!(a, make_token(1, 100, &[(1, 30), (2, 20)]));
    }

    #[test]
    fn merge_union_regions() {
        let mut a = VectorSessionToken::parse("1#100#1=10").unwrap();
        let b = VectorSessionToken::parse("1#100#2=20").unwrap();
        assert!(a.merge(&b));
        assert_eq!(a, make_token(1, 100, &[(1, 10), (2, 20)]));
    }

    #[test]
    fn merge_higher_version_wins() {
        let mut a = VectorSessionToken::parse("1#100").unwrap();
        let b = VectorSessionToken::parse("5#100").unwrap();
        assert!(a.merge(&b));
        assert_eq!(a, make_token(5, 100, &[]));
    }

    #[test]
    fn merge_cross_version_uses_higher_version_global_lsn() {
        // False-progress protection: when versions differ, use the
        // higher-version's globalLSN, not max(both).
        let mut a = VectorSessionToken::parse("1#500#1=100").unwrap();
        let b = VectorSessionToken::parse("2#200#1=50").unwrap();
        assert!(a.merge(&b));
        // Higher version (2) wins — its globalLSN=200 is used, not max(500,200).
        // Region 1 exists in both: max(100, 50) = 100.
        assert_eq!(a, make_token(2, 200, &[(1, 100)]));
    }

    #[test]
    fn merge_cross_version_drops_lower_version_only_regions() {
        // Region 2 only exists in the lower-version token — it should be dropped
        // because the topology changed and that region may no longer exist.
        let mut a = VectorSessionToken::parse("1#100#1=10#2=20").unwrap();
        let b = VectorSessionToken::parse("2#50#1=5").unwrap();
        assert!(a.merge(&b));
        // Version 2 wins. Region 1: max(10, 5) = 10. Region 2: dropped.
        assert_eq!(a, make_token(2, 50, &[(1, 10)]));
    }

    #[test]
    fn merge_cross_version_takes_max_of_shared_regions() {
        // When a region exists in both tokens, take max even across versions.
        let mut a = VectorSessionToken::parse("1#100#1=50#2=30").unwrap();
        let b = VectorSessionToken::parse("2#80#1=10#2=40").unwrap();
        assert!(a.merge(&b));
        // Version 2 wins for globalLSN. Region 1: max(50, 10). Region 2: max(30, 40).
        assert_eq!(a, make_token(2, 80, &[(1, 50), (2, 40)]));
    }

    #[test]
    fn merge_cross_version_higher_version_new_region() {
        // Higher-version has a region that lower-version doesn't — keep it.
        let mut a = VectorSessionToken::parse("1#100#1=10").unwrap();
        let b = VectorSessionToken::parse("2#80#1=5#3=25").unwrap();
        assert!(a.merge(&b));
        // Version 2 wins. Region 1: max(10, 5). Region 3: new from higher version.
        assert_eq!(a, make_token(2, 80, &[(1, 10), (3, 25)]));
    }

    #[test]
    fn is_as_recent_as_same() {
        let a = VectorSessionToken::parse("1#100#1=10#2=20").unwrap();
        assert!(a.is_as_recent_as(&a));
    }

    #[test]
    fn is_as_recent_as_higher() {
        let a = VectorSessionToken::parse("1#200#1=30#2=25").unwrap();
        let b = VectorSessionToken::parse("1#100#1=10#2=20").unwrap();
        assert!(a.is_as_recent_as(&b));
    }

    #[test]
    fn is_as_recent_as_lower() {
        let a = VectorSessionToken::parse("1#50#1=10").unwrap();
        let b = VectorSessionToken::parse("1#100#1=10").unwrap();
        assert!(!a.is_as_recent_as(&b));
    }

    #[test]
    fn is_as_recent_as_missing_region() {
        let a = VectorSessionToken::parse("1#100#1=10").unwrap();
        let b = VectorSessionToken::parse("1#100#1=10#2=20").unwrap();
        assert!(!a.is_as_recent_as(&b));
    }

    #[test]
    fn is_as_recent_as_higher_version() {
        // A higher version token is always more recent, even with lower LSNs
        let a = VectorSessionToken::parse("2#50#1=5").unwrap();
        let b = VectorSessionToken::parse("1#100#1=10").unwrap();
        assert!(a.is_as_recent_as(&b));
        assert!(!b.is_as_recent_as(&a));
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
    fn v1_display_formats_expected_string() {
        assert_eq!(SessionTokenValue::Simple(12345).to_string(), "12345");
    }

    #[test]
    fn parse_invalid_returns_err() {
        assert!(SessionTokenValue::parse("not_a_token").is_err());
        assert!(SessionTokenValue::parse("").is_err());
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
    fn v1_is_as_recent_as() {
        let a = SessionTokenValue::parse("200").unwrap();
        let b = SessionTokenValue::parse("100").unwrap();
        assert!(a.is_as_recent_as(&b));
        assert!(!b.is_as_recent_as(&a));
    }

    #[test]
    fn v2_always_more_recent_than_v1() {
        let v2 = SessionTokenValue::parse("1#50#1=5").unwrap();
        let v1 = SessionTokenValue::parse("99999").unwrap();
        assert!(v2.is_as_recent_as(&v1));
        assert!(!v1.is_as_recent_as(&v2));
    }
}

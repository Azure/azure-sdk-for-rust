// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Internal vector session token for Cosmos DB session consistency.
//!
//! Parses and merges the version-vector portion of a Cosmos session token.
//! The full wire format is `<PKRangeId>:<Version>#<GlobalLSN>#<RegionId>=<LocalLSN>#...`.
//! This type only handles the part after the colon.

use std::collections::HashMap;
use std::fmt;
use std::str::FromStr;

/// Internal representation of a session token's version vector.
///
/// Tracks a global LSN and per-region local LSNs to enable merge
/// (point-wise max) and comparison across replicas.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct VectorSessionToken {
    version: u64,
    global_lsn: u64,
    region_progress: HashMap<u64, u64>,
}

impl VectorSessionToken {
    /// Merges two session tokens by taking the point-wise maximum.
    ///
    /// Returns a new token with:
    /// - The higher `version`
    /// - The higher `global_lsn`
    /// - For each region present in either token, the higher local LSN
    pub(crate) fn merge(&self, other: &Self) -> Self {
        let mut region_progress = self.region_progress.clone();
        for (&region_id, &other_lsn) in &other.region_progress {
            let entry = region_progress.entry(region_id).or_insert(0);
            *entry = (*entry).max(other_lsn);
        }
        Self {
            version: self.version.max(other.version),
            global_lsn: self.global_lsn.max(other.global_lsn),
            region_progress,
        }
    }

    /// Returns `true` if this token is at least as recent as `other`.
    ///
    /// A token is at least as recent when its `global_lsn` is >= the other's
    /// and every region tracked by `other` also appears in `self` with a
    /// local LSN >= the other's value.
    pub(crate) fn is_at_least_as_recent_as(&self, other: &Self) -> bool {
        if self.global_lsn < other.global_lsn {
            return false;
        }
        for (&region_id, &other_lsn) in &other.region_progress {
            match self.region_progress.get(&region_id) {
                Some(&self_lsn) if self_lsn >= other_lsn => {}
                _ => return false,
            }
        }
        true
    }
}

impl FromStr for VectorSessionToken {
    type Err = azure_core::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut segments = s.split('#');

        let version_str = segments.next().ok_or_else(|| {
            azure_core::Error::new(
                azure_core::error::ErrorKind::DataConversion,
                "missing version segment",
            )
        })?;
        let version: u64 = version_str.parse().map_err(|e| {
            azure_core::Error::new(
                azure_core::error::ErrorKind::DataConversion,
                format!("invalid version: {e}"),
            )
        })?;

        let global_lsn_str = segments.next().ok_or_else(|| {
            azure_core::Error::new(
                azure_core::error::ErrorKind::DataConversion,
                "missing global LSN segment",
            )
        })?;
        let global_lsn: u64 = global_lsn_str.parse().map_err(|e| {
            azure_core::Error::new(
                azure_core::error::ErrorKind::DataConversion,
                format!("invalid global LSN: {e}"),
            )
        })?;

        let mut region_progress = HashMap::new();
        for segment in segments {
            let (region_str, lsn_str) = segment.split_once('=').ok_or_else(|| {
                azure_core::Error::new(
                    azure_core::error::ErrorKind::DataConversion,
                    format!("invalid region progress segment: '{segment}'"),
                )
            })?;
            let region_id: u64 = region_str.parse().map_err(|e| {
                azure_core::Error::new(
                    azure_core::error::ErrorKind::DataConversion,
                    format!("invalid region id: {e}"),
                )
            })?;
            let local_lsn: u64 = lsn_str.parse().map_err(|e| {
                azure_core::Error::new(
                    azure_core::error::ErrorKind::DataConversion,
                    format!("invalid local LSN: {e}"),
                )
            })?;
            region_progress.insert(region_id, local_lsn);
        }

        Ok(Self {
            version,
            global_lsn,
            region_progress,
        })
    }
}

impl fmt::Display for VectorSessionToken {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}#{}", self.version, self.global_lsn)?;
        let mut entries: Vec<_> = self.region_progress.iter().collect();
        entries.sort_by_key(|(&region_id, _)| region_id);
        for (&region_id, &local_lsn) in &entries {
            write!(f, "#{}={}", region_id, local_lsn)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_simple_token() {
        let token: VectorSessionToken = "1#100#1=20#2=5#3=30".parse().unwrap();
        assert_eq!(token.version, 1);
        assert_eq!(token.global_lsn, 100);
        assert_eq!(token.region_progress.len(), 3);
        assert_eq!(token.region_progress[&1], 20);
        assert_eq!(token.region_progress[&2], 5);
        assert_eq!(token.region_progress[&3], 30);
    }

    #[test]
    fn parse_no_regions() {
        let token: VectorSessionToken = "1#100".parse().unwrap();
        assert_eq!(token.version, 1);
        assert_eq!(token.global_lsn, 100);
        assert!(token.region_progress.is_empty());
    }

    #[test]
    fn parse_invalid_no_hash() {
        assert!("abc".parse::<VectorSessionToken>().is_err());
    }

    #[test]
    fn parse_invalid_region_format() {
        assert!("1#100#bad".parse::<VectorSessionToken>().is_err());
    }

    #[test]
    fn parse_invalid_lsn() {
        assert!("1#abc#1=20".parse::<VectorSessionToken>().is_err());
    }

    #[test]
    fn display_roundtrip() {
        let input = "1#100#1=20#2=5#3=30";
        let token: VectorSessionToken = input.parse().unwrap();
        assert_eq!(token.to_string(), input);
    }

    #[test]
    fn display_regions_sorted() {
        let mut region_progress = HashMap::new();
        region_progress.insert(3, 30);
        region_progress.insert(1, 10);
        region_progress.insert(2, 20);
        let token = VectorSessionToken {
            version: 1,
            global_lsn: 100,
            region_progress,
        };
        assert_eq!(token.to_string(), "1#100#1=10#2=20#3=30");
    }

    #[test]
    fn merge_takes_max_global_lsn() {
        let a: VectorSessionToken = "1#100#1=20".parse().unwrap();
        let b: VectorSessionToken = "1#200#1=20".parse().unwrap();
        let merged = a.merge(&b);
        assert_eq!(merged.global_lsn, 200);
    }

    #[test]
    fn merge_takes_max_region_lsn() {
        let a: VectorSessionToken = "1#100#1=50#2=10".parse().unwrap();
        let b: VectorSessionToken = "1#100#1=20#2=40".parse().unwrap();
        let merged = a.merge(&b);
        assert_eq!(merged.region_progress[&1], 50);
        assert_eq!(merged.region_progress[&2], 40);
    }

    #[test]
    fn merge_union_regions() {
        let a: VectorSessionToken = "1#100#1=20".parse().unwrap();
        let b: VectorSessionToken = "1#100#2=30".parse().unwrap();
        let merged = a.merge(&b);
        assert_eq!(merged.region_progress.len(), 2);
        assert_eq!(merged.region_progress[&1], 20);
        assert_eq!(merged.region_progress[&2], 30);
    }

    #[test]
    fn merge_higher_version_wins() {
        let a: VectorSessionToken = "3#100#1=20".parse().unwrap();
        let b: VectorSessionToken = "5#100#1=20".parse().unwrap();
        let merged = a.merge(&b);
        assert_eq!(merged.version, 5);
    }

    #[test]
    fn is_at_least_as_recent_same() {
        let a: VectorSessionToken = "1#100#1=20#2=30".parse().unwrap();
        assert!(a.is_at_least_as_recent_as(&a));
    }

    #[test]
    fn is_at_least_as_recent_higher() {
        let a: VectorSessionToken = "1#200#1=40#2=50".parse().unwrap();
        let b: VectorSessionToken = "1#100#1=20#2=30".parse().unwrap();
        assert!(a.is_at_least_as_recent_as(&b));
    }

    #[test]
    fn is_at_least_as_recent_lower() {
        let a: VectorSessionToken = "1#50#1=20".parse().unwrap();
        let b: VectorSessionToken = "1#100#1=20".parse().unwrap();
        assert!(!a.is_at_least_as_recent_as(&b));
    }

    #[test]
    fn is_at_least_as_recent_missing_region() {
        let a: VectorSessionToken = "1#100#1=20".parse().unwrap();
        let b: VectorSessionToken = "1#100#1=20#2=30".parse().unwrap();
        assert!(!a.is_at_least_as_recent_as(&b));
    }
}

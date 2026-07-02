// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

// cspell:ignore behaviour epks
//! Session token parsing, formatting, and state tracking.
//!
//! Supports V2 vector-clock session tokens with the wire format:
//! `{pkrangeId}:{version}#{globalLSN}#{regionId}={localLSN}`
//!
//! Also accepts V1 tokens (`{pkrangeId}:-1#{lsn}`) for backward compatibility.

use std::collections::HashSet;
use std::sync::Mutex;

/// Newtype wrapper around the region-id `u64` carried in V2 session tokens.
///
/// Exists purely to make swapped-argument bugs in `SessionToken::format_v2`
/// (where region-id and local-LSN are adjacent `u64` parameters) a compile
/// error.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub(crate) struct RegionId(pub u64);

/// Newtype wrapper around the per-region local-LSN `u64` carried in V2
/// session tokens. See [`RegionId`] for rationale.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub(crate) struct LocalLsn(pub u64);

/// Parsed session token for a single partition key range.
///
/// V2 format: `{pkrange_id}:{version}#{global_lsn}#{region_id}={local_lsn}`
/// V1 format: `{pkrange_id}:-1#{lsn}` (parsed as version=0, global_lsn=lsn)
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct SessionToken {
    pub pkrange_id: u32,
    pub version: u64,
    pub global_lsn: u64,
    pub region_progress: Vec<(u64, u64)>,
}

impl SessionToken {
    /// Parses a session token. Supports both V1 and V2 formats.
    ///
    /// V2: `"0:1#100#0=100"` → pkrange_id=0, version=1, global_lsn=100, region 0=100
    /// V1: `"0:-1#5"` → pkrange_id=0, version=0, global_lsn=5, no regions
    /// Convenience wrapper around `parse_detailed` for callers that don't
    /// care which segment failed.
    #[cfg(test)]
    pub fn parse(s: &str) -> Option<Self> {
        Self::parse_detailed(s).ok()
    }

    /// Parses a session token, returning a structured error that identifies
    /// which segment failed to parse.
    ///
    /// Used by handlers that need to surface the failure reason (e.g. to a
    /// 400 response body) rather than just "Invalid session token". Drift
    /// between the emulator's parser and the SDK's parser is a common source
    /// of dual-backend test flakes — pointing at the offending segment makes
    /// triage cheap.
    pub fn parse_detailed(s: &str) -> Result<Self, SessionTokenParseError> {
        use SessionTokenParseError::*;
        let (pkrange_str, rest) = s.split_once(':').ok_or(MissingPkRangeSeparator)?;
        let pkrange_id = pkrange_str
            .parse::<u32>()
            .map_err(|_| InvalidPkRangeId(pkrange_str.to_string()))?;

        let mut hash_parts = rest.split('#');
        let version_str = hash_parts.next().ok_or(MissingVersion)?;

        // V1 check: version is "-1"
        if version_str == "-1" {
            let lsn_str = hash_parts.next().ok_or(MissingV1Lsn)?;
            let lsn = lsn_str
                .parse::<u64>()
                .map_err(|_| InvalidV1Lsn(lsn_str.to_string()))?;
            return Ok(SessionToken {
                pkrange_id,
                version: 0,
                global_lsn: lsn,
                region_progress: vec![],
            });
        }

        // V2: version#globalLSN#region=lsn#...
        let version = version_str
            .parse::<u64>()
            .map_err(|_| InvalidVersion(version_str.to_string()))?;
        let global_lsn_str = hash_parts.next().ok_or(MissingGlobalLsn)?;
        let global_lsn = global_lsn_str
            .parse::<u64>()
            .map_err(|_| InvalidGlobalLsn(global_lsn_str.to_string()))?;

        let mut region_progress = Vec::new();
        for segment in hash_parts {
            if segment.is_empty() {
                continue;
            }
            let (region_str, lsn_str) = segment
                .split_once('=')
                .ok_or_else(|| InvalidRegionSegment(segment.to_string()))?;
            let region_id = region_str
                .parse::<u64>()
                .map_err(|_| InvalidRegionId(region_str.to_string()))?;
            let lsn = lsn_str
                .parse::<u64>()
                .map_err(|_| InvalidRegionLsn(lsn_str.to_string()))?;
            region_progress.push((region_id, lsn));
        }

        Ok(SessionToken {
            pkrange_id,
            version,
            global_lsn,
            region_progress,
        })
    }

    /// Formats a V2 session token.
    ///
    /// `local_lsn` is the number of mutations applied at this region for the
    /// partition, which differs from `global_lsn` for any partition that
    /// receives writes via replication rather than originating them.
    ///
    /// `prior_progress`, if non-empty, carries per-region LSNs from a token a
    /// caller previously observed (typically the incoming `x-ms-session-token`
    /// of the request being served). Entries are merged into the output by
    /// taking `max(prior, local)` per region so the session token a driver
    /// accumulates against the emulator does not lose multi-region progress
    /// on every roundtrip — matching the behavior of the real gateway.
    ///
    /// `region` and `local_lsn` are wrapped in newtypes (`RegionId`,
    /// `LocalLsn`) to make swapped-argument bugs a compile error: in the wire
    /// format these are adjacent `u64` values and trivial to transpose.
    pub fn format_v2(
        pkrange_id: u32,
        version: u64,
        global_lsn: u64,
        region: RegionId,
        local_lsn: LocalLsn,
        prior_progress: &[(u64, u64)],
    ) -> String {
        let RegionId(region_id) = region;
        let LocalLsn(local_lsn) = local_lsn;
        if prior_progress.is_empty() {
            return format!(
                "{}:{}#{}#{}={}",
                pkrange_id, version, global_lsn, region_id, local_lsn
            );
        }
        // Merge: start from prior progress, overwrite/insert the local region
        // with max(prior_local, local_lsn), then emit in stable region-id
        // order.
        let mut merged: Vec<(u64, u64)> = prior_progress.to_vec();
        let mut found = false;
        for (rid, lsn) in merged.iter_mut() {
            if *rid == region_id {
                *lsn = (*lsn).max(local_lsn);
                found = true;
                break;
            }
        }
        if !found {
            merged.push((region_id, local_lsn));
        }
        merged.sort_by_key(|(rid, _)| *rid);
        let mut out = format!("{}:{}#{}", pkrange_id, version, global_lsn);
        for (rid, lsn) in merged {
            use std::fmt::Write;
            let _ = write!(out, "#{}={}", rid, lsn);
        }
        out
    }

    /// Formats a V1 session token (backward compatibility).
    pub fn format(pkrange_id: u32, lsn: u64) -> String {
        format!("{}:-1#{}", pkrange_id, lsn)
    }
}

/// Identifies which segment of a session token failed to parse.
///
/// Surfaced through `SessionToken::parse_detailed` and
/// `parse_composite_session_token` so handlers can include the offending
/// fragment in error responses (and dual-backend tests can pinpoint
/// emulator/SDK parser drift).
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum SessionTokenParseError {
    /// Token did not contain the `:` separating pkrange id from version.
    MissingPkRangeSeparator,
    /// Pkrange id segment was not a valid `u32`.
    InvalidPkRangeId(String),
    /// Token was missing the version segment after `:`.
    MissingVersion,
    /// V1 token (`pkrange:-1#lsn`) was missing the LSN segment.
    MissingV1Lsn,
    /// V1 LSN was not a valid `u64`.
    InvalidV1Lsn(String),
    /// V2 version segment was not a valid `u64`.
    InvalidVersion(String),
    /// V2 token was missing the global LSN segment after the version.
    MissingGlobalLsn,
    /// V2 global LSN was not a valid `u64`.
    InvalidGlobalLsn(String),
    /// A V2 region segment did not contain `=` (expected `region=lsn`).
    InvalidRegionSegment(String),
    /// V2 region id was not a valid `u64`.
    InvalidRegionId(String),
    /// V2 region LSN was not a valid `u64`.
    InvalidRegionLsn(String),
    /// A composite token had an empty entry (e.g. trailing `,`).
    EmptyComposite,
}

impl std::fmt::Display for SessionTokenParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use SessionTokenParseError::*;
        match self {
            MissingPkRangeSeparator => write!(f, "missing ':' separator after pkrange id"),
            InvalidPkRangeId(s) => write!(f, "invalid pkrange id '{}'", s),
            MissingVersion => write!(f, "missing version segment"),
            MissingV1Lsn => write!(f, "missing V1 LSN after '-1'"),
            InvalidV1Lsn(s) => write!(f, "invalid V1 LSN '{}'", s),
            InvalidVersion(s) => write!(f, "invalid version '{}'", s),
            MissingGlobalLsn => write!(f, "missing global LSN segment"),
            InvalidGlobalLsn(s) => write!(f, "invalid global LSN '{}'", s),
            InvalidRegionSegment(s) => {
                write!(f, "region segment '{}' missing '=' separator", s)
            }
            InvalidRegionId(s) => write!(f, "invalid region id '{}'", s),
            InvalidRegionLsn(s) => write!(f, "invalid region LSN '{}'", s),
            EmptyComposite => write!(f, "empty composite session token entry"),
        }
    }
}

/// Parses a composite session token string (comma-separated) into individual tokens.
///
/// Returns the segment-specific failure on the first malformed entry so the
/// caller can include it in an error response.
pub(crate) fn parse_composite_session_token(
    s: &str,
) -> Result<Vec<SessionToken>, SessionTokenParseError> {
    s.split(',')
        .map(|part| {
            let trimmed = part.trim();
            if trimmed.is_empty() {
                return Err(SessionTokenParseError::EmptyComposite);
            }
            SessionToken::parse_detailed(trimmed)
        })
        .collect()
}

/// Per-partition session state, tracking forced unavailability scoped to
/// specific effective partition keys.
///
/// Earlier the flag was a single `AtomicBool` per physical partition, which
/// meant `force_session_not_available(region, db, coll, "[\"pk1\"]")` would
/// trip the *next* read of any logical partition key that happened to hash
/// to the same physical partition. With the default 4 physical partitions
/// (and many user PKs typically sharing each), targeted edge-case tests were
/// brittle. The set keys on the EPK already computed by the caller, so we
/// match exactly the logical partition the test asked for.
pub(crate) struct SessionState {
    forced_epks: Mutex<HashSet<String>>,
}

impl SessionState {
    pub fn new() -> Self {
        Self {
            forced_epks: Mutex::new(HashSet::new()),
        }
    }

    /// Marks the given EPK as forced-unavailable on the next read (one-shot).
    pub fn set_force_unavailable_for(&self, epk: &str) {
        self.forced_epks.lock().unwrap().insert(epk.to_string());
    }

    /// Checks and clears the forced-unavailability marker for pk.
    /// Returns true if it was set (one-shot: only fires once).
    pub fn check_and_clear_forced_for(&self, epk: &str) -> bool {
        self.forced_epks.lock().unwrap().remove(epk)
    }

    /// Returns a snapshot of the currently-pending forced-unavailable EPKs.
    /// Used during partition split/merge so child partitions can inherit any
    /// pending markers whose EPK falls within their new range.
    pub fn snapshot_forced_epks(&self) -> Vec<String> {
        self.forced_epks.lock().unwrap().iter().cloned().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_v1_token() {
        let token = SessionToken::parse("0:-1#5").unwrap();
        assert_eq!(token.pkrange_id, 0);
        assert_eq!(token.version, 0);
        assert_eq!(token.global_lsn, 5);
        assert!(token.region_progress.is_empty());
    }

    #[test]
    fn parse_v2_token() {
        let token = SessionToken::parse("0:1#100#0=100").unwrap();
        assert_eq!(token.pkrange_id, 0);
        assert_eq!(token.version, 1);
        assert_eq!(token.global_lsn, 100);
        assert_eq!(token.region_progress, vec![(0, 100)]);
    }

    #[test]
    fn parse_v2_multi_region() {
        let token = SessionToken::parse("2:3#50#0=50#1=45").unwrap();
        assert_eq!(token.pkrange_id, 2);
        assert_eq!(token.version, 3);
        assert_eq!(token.global_lsn, 50);
        assert_eq!(token.region_progress, vec![(0, 50), (1, 45)]);
    }

    #[test]
    fn format_v2_token() {
        // Single-region: local LSN equals global LSN.
        let s = SessionToken::format_v2(0, 1, 100, RegionId(0), LocalLsn(100), &[]);
        assert_eq!(s, "0:1#100#0=100");
    }

    #[test]
    fn format_v2_token_distinguishes_local_from_global() {
        // Multi-region: a region that received this write via replication
        // reports the local-LSN component independently of the global LSN.
        let s = SessionToken::format_v2(2, 4, 100, RegionId(1), LocalLsn(73), &[]);
        assert_eq!(s, "2:4#100#1=73");
    }

    #[test]
    fn format_v2_preserves_prior_multi_region_progress() {
        // Incoming token tracks three regions; we re-emit at region 1 with a
        // newer local LSN. Regions 0 and 2 must survive the roundtrip.
        let incoming = SessionToken::parse("0:5#200#0=200#1=150#2=120").unwrap();
        let s = SessionToken::format_v2(
            0,
            5,
            200,
            RegionId(1),
            LocalLsn(180),
            &incoming.region_progress,
        );
        // Sorted by region id; region 1 advanced from 150 -> 180.
        assert_eq!(s, "0:5#200#0=200#1=180#2=120");
    }

    #[test]
    fn format_v2_local_lsn_takes_max_against_prior() {
        // Stale incoming local LSN must not regress the emitted value.
        let prior = vec![(0, 100), (1, 80)];
        let s = SessionToken::format_v2(0, 1, 100, RegionId(1), LocalLsn(40), &prior);
        assert_eq!(s, "0:1#100#0=100#1=80");
    }

    #[test]
    fn format_v2_inserts_new_region_when_absent_from_prior() {
        let prior = vec![(0, 100)];
        let s = SessionToken::format_v2(0, 1, 100, RegionId(2), LocalLsn(7), &prior);
        assert_eq!(s, "0:1#100#0=100#2=7");
    }

    #[test]
    fn format_v1_token() {
        assert_eq!(SessionToken::format(2, 10), "2:-1#10");
    }

    #[test]
    fn parse_composite_v2() {
        let tokens = parse_composite_session_token("0:1#5#0=5,1:1#3#0=3").unwrap();
        assert_eq!(tokens.len(), 2);
        assert_eq!(tokens[0].global_lsn, 5);
        assert_eq!(tokens[1].global_lsn, 3);
    }

    #[test]
    fn parse_composite_mixed() {
        let tokens = parse_composite_session_token("0:-1#5,1:1#3#0=3").unwrap();
        assert_eq!(tokens.len(), 2);
        assert_eq!(tokens[0].version, 0);
        assert_eq!(tokens[1].version, 1);
    }

    #[test]
    fn parse_composite_rejects_malformed_session_token() {
        assert!(parse_composite_session_token("0:1#5#0=5,broken-token").is_err());
        assert!(parse_composite_session_token("0:1#5#0=5,").is_err());
    }

    #[test]
    fn forced_unavailability_one_shot() {
        let state = SessionState::new();
        let epk = "ABCD";
        assert!(!state.check_and_clear_forced_for(epk));

        state.set_force_unavailable_for(epk);
        assert!(state.check_and_clear_forced_for(epk));
        assert!(!state.check_and_clear_forced_for(epk));
    }

    #[test]
    fn forced_unavailability_scoped_per_epk() {
        let state = SessionState::new();
        state.set_force_unavailable_for("AAAA");
        // Other EPKs in the same physical partition must not trip.
        assert!(!state.check_and_clear_forced_for("BBBB"));
        // The targeted EPK still trips on its own next read.
        assert!(state.check_and_clear_forced_for("AAAA"));
    }
}

// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Session token parsing, formatting, and state tracking.
//!
//! Supports V2 vector-clock session tokens with the wire format:
//! `{pkrangeId}:{version}#{globalLSN}#{regionId}={localLSN}`
//!
//! Also accepts V1 tokens (`{pkrangeId}:-1#{lsn}`) for backward compatibility.

use std::sync::atomic::{AtomicBool, Ordering};

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
    pub fn parse(s: &str) -> Option<Self> {
        let (pkrange_str, rest) = s.split_once(':')?;
        let pkrange_id = pkrange_str.parse::<u32>().ok()?;

        let mut hash_parts = rest.split('#');
        let version_str = hash_parts.next()?;

        // V1 check: version is "-1"
        if version_str == "-1" {
            let lsn_str = hash_parts.next()?;
            let lsn = lsn_str.parse::<u64>().ok()?;
            return Some(SessionToken {
                pkrange_id,
                version: 0,
                global_lsn: lsn,
                region_progress: vec![],
            });
        }

        // V2: version#globalLSN#region=lsn#...
        let version = version_str.parse::<u64>().ok()?;
        let global_lsn_str = hash_parts.next()?;
        let global_lsn = global_lsn_str.parse::<u64>().ok()?;

        let mut region_progress = Vec::new();
        for segment in hash_parts {
            if segment.is_empty() {
                continue;
            }
            let (region_str, lsn_str) = segment.split_once('=')?;
            let region_id = region_str.parse::<u64>().ok()?;
            let lsn = lsn_str.parse::<u64>().ok()?;
            region_progress.push((region_id, lsn));
        }

        Some(SessionToken {
            pkrange_id,
            version,
            global_lsn,
            region_progress,
        })
    }

    /// Formats a V2 session token.
    pub fn format_v2(pkrange_id: u32, version: u64, global_lsn: u64, region_id: u64) -> String {
        format!(
            "{}:{}#{}#{}={}",
            pkrange_id, version, global_lsn, region_id, global_lsn
        )
    }

    /// Formats a V1 session token (backward compatibility).
    pub fn format(pkrange_id: u32, lsn: u64) -> String {
        format!("{}:-1#{}", pkrange_id, lsn)
    }
}

/// Parses a composite session token string (comma-separated) into individual tokens.
pub(crate) fn parse_composite_session_token(s: &str) -> Vec<SessionToken> {
    s.split(',')
        .filter_map(|part| SessionToken::parse(part.trim()))
        .collect()
}

/// Formats a composite V2 session token from partition state.
#[allow(dead_code)]
pub(crate) fn format_composite_session_token(pairs: &[(u32, u64)]) -> String {
    pairs
        .iter()
        .map(|(id, lsn)| SessionToken::format(*id, *lsn))
        .collect::<Vec<_>>()
        .join(",")
}

/// Per-partition session state, tracking forced unavailability.
pub(crate) struct SessionState {
    force_unavailable: AtomicBool,
}

impl SessionState {
    pub fn new() -> Self {
        Self {
            force_unavailable: AtomicBool::new(false),
        }
    }

    /// Sets the forced-unavailability flag (one-shot).
    pub fn set_force_unavailable(&self) {
        self.force_unavailable.store(true, Ordering::SeqCst);
    }

    /// Checks and clears the forced-unavailability flag.
    /// Returns true if it was set (one-shot: only fires once).
    pub fn check_and_clear_forced(&self) -> bool {
        self.force_unavailable.swap(false, Ordering::SeqCst)
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
        let s = SessionToken::format_v2(0, 1, 100, 0);
        assert_eq!(s, "0:1#100#0=100");
    }

    #[test]
    fn format_v1_token() {
        assert_eq!(SessionToken::format(2, 10), "2:-1#10");
    }

    #[test]
    fn parse_composite_v2() {
        let tokens = parse_composite_session_token("0:1#5#0=5,1:1#3#0=3");
        assert_eq!(tokens.len(), 2);
        assert_eq!(tokens[0].global_lsn, 5);
        assert_eq!(tokens[1].global_lsn, 3);
    }

    #[test]
    fn parse_composite_mixed() {
        let tokens = parse_composite_session_token("0:-1#5,1:1#3#0=3");
        assert_eq!(tokens.len(), 2);
        assert_eq!(tokens[0].version, 0);
        assert_eq!(tokens[1].version, 1);
    }

    #[test]
    fn forced_unavailability_one_shot() {
        let state = SessionState::new();
        assert!(!state.check_and_clear_forced());

        state.set_force_unavailable();
        assert!(state.check_and_clear_forced());
        assert!(!state.check_and_clear_forced());
    }
}

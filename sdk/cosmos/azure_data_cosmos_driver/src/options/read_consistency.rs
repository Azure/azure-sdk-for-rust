// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use crate::models::DefaultConsistencyLevel;

/// Read consistency strategies supported by Azure Cosmos DB.
///
/// The requested read consistency strategy is independent of the default consistency level
/// provisioned for the account. When set to anything other than [`Default`](Self::Default),
/// it overrides the consistency level configured on the request options, on the client, or
/// on the account for the read path.
///
/// The strategy is honored across all supported transport modes (Gateway V1 / compute
/// gateway and Gateway V2 / thin client proxy). Wire emission:
///
/// - Gateway V1 (HTTP) sends `x-ms-cosmos-read-consistency-strategy: <Strategy>` and
///   omits `x-ms-consistency-level` whenever a non-`Default` strategy is in effect.
/// - Gateway V2 (RNTBD) serializes the strategy as token `0x00FE` (Byte) and omits the
///   `ConsistencyLevel` token under the same conditions.
///
/// `ReadConsistencyStrategy::Default` is transparent: no header / token is emitted and the
/// existing consistency-level behavior is preserved.
///
/// See [Cosmos DB consistency levels](https://learn.microsoft.com/azure/cosmos-db/consistency-levels)
/// for the underlying semantics.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum ReadConsistencyStrategy {
    /// Use the default read behavior for the consistency level applied to the operation,
    /// the client, or the account. No RCS header / token is emitted on the wire.
    Default,

    /// Eventual consistency guarantees that reads will return a subset of writes.
    /// All writes will eventually be available for reads.
    Eventual,

    /// Session consistency guarantees monotonic reads, monotonic writes, and
    /// read-your-writes within any single session.
    Session,

    /// Returns the latest committed version of the requested item across replicas.
    ///
    /// On accounts whose default consistency is Session, ConsistentPrefix, or Eventual,
    /// this strategy upgrades the read to a quorum read (1:2 client-to-backend
    /// amplification) without weakening any other operation. On Strong / BoundedStaleness
    /// accounts it behaves like the account default.
    LatestCommitted,

    /// Reads the latest version across all regions.
    ///
    /// Replication with global strong consistency is synchronous, so this strategy
    /// returns the latest successfully written version across regions.
    ///
    /// **NOTE**: Only valid on single-master accounts whose default consistency is
    /// Strong. Requesting `GlobalStrong` against any other account fails client-side
    /// with a `BadRequest` error before the request reaches the wire.
    GlobalStrong,
}

impl ReadConsistencyStrategy {
    /// Parses a read consistency strategy from its wire format representation.
    ///
    /// Parsing is case-sensitive for exact matches, with case-insensitive fallback.
    ///
    /// Returns `None` if the string does not match any known strategy.
    fn parse(s: &str) -> Option<Self> {
        match s {
            "Default" => Some(Self::Default),
            "Eventual" => Some(Self::Eventual),
            "Session" => Some(Self::Session),
            "LatestCommitted" => Some(Self::LatestCommitted),
            "GlobalStrong" => Some(Self::GlobalStrong),
            _ => {
                if s.eq_ignore_ascii_case("Default") {
                    Some(Self::Default)
                } else if s.eq_ignore_ascii_case("Eventual") {
                    Some(Self::Eventual)
                } else if s.eq_ignore_ascii_case("Session") {
                    Some(Self::Session)
                } else if s.eq_ignore_ascii_case("LatestCommitted") {
                    Some(Self::LatestCommitted)
                } else if s.eq_ignore_ascii_case("GlobalStrong") {
                    Some(Self::GlobalStrong)
                } else {
                    None
                }
            }
        }
    }

    /// Returns the wire format representation of this read consistency strategy.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Default => "Default",
            Self::Eventual => "Eventual",
            Self::Session => "Session",
            Self::LatestCommitted => "LatestCommitted",
            Self::GlobalStrong => "GlobalStrong",
        }
    }

    /// Returns `true` when this strategy is something other than [`Default`](Self::Default).
    ///
    /// `Default` is transparent on the wire: no RCS header / token is emitted and the
    /// existing `ConsistencyLevel` header / token continues to flow normally.
    pub(crate) fn is_non_default(&self) -> bool {
        !matches!(self, Self::Default)
    }

    /// Wire byte for the RNTBD `ReadConsistencyStrategy` token (id `0x00FE`, `Byte`).
    ///
    /// Returns `None` for [`Default`](Self::Default), which is transparent on the
    /// wire and MUST never be serialized.
    pub(crate) fn rntbd_wire_byte(self) -> Option<u8> {
        match self {
            Self::Default => None,
            Self::Eventual => Some(0x01),
            Self::Session => Some(0x02),
            Self::LatestCommitted => Some(0x03),
            Self::GlobalStrong => Some(0x04),
        }
    }

    /// Returns `true` if session consistency is effective for this strategy
    /// given the account's default consistency level.
    ///
    /// Session consistency is effective when:
    /// - The strategy explicitly requests [`Session`](Self::Session), or
    /// - The strategy is [`Default`](Self::Default) and the account default is
    ///   [`DefaultConsistencyLevel::Session`].
    ///
    /// [`LatestCommitted`](Self::LatestCommitted) and [`GlobalStrong`](Self::GlobalStrong)
    /// both upgrade the read out of the session lane, so they are never session-effective.
    pub(crate) fn is_session_effective(&self, account_default: DefaultConsistencyLevel) -> bool {
        match self {
            Self::Session => true,
            Self::Default => account_default.is_session(),
            Self::Eventual | Self::LatestCommitted | Self::GlobalStrong => false,
        }
    }
}

/// Resolves the effective consistency level for a read consistency strategy.
///
/// This collapsed [`DefaultConsistencyLevel`] is what flows through the SDK for
/// session-token bookkeeping and for the legacy `ConsistencyLevel` token/header
/// emitted when RCS is [`ReadConsistencyStrategy::Default`].
///
/// [`LatestCommitted`](ReadConsistencyStrategy::LatestCommitted) resolves to the
/// account default rather than `Strong`: the quorum-read upgrade is requested
/// server-side via the RCS header / token, and downstream client logic should
/// continue to treat the operation as if it ran at the account's native level.
pub(crate) fn resolve_effective_consistency(
    strategy: ReadConsistencyStrategy,
    account_default: DefaultConsistencyLevel,
) -> DefaultConsistencyLevel {
    match strategy {
        ReadConsistencyStrategy::Default | ReadConsistencyStrategy::LatestCommitted => {
            account_default
        }
        ReadConsistencyStrategy::Eventual => DefaultConsistencyLevel::Eventual,
        ReadConsistencyStrategy::Session => DefaultConsistencyLevel::Session,
        ReadConsistencyStrategy::GlobalStrong => DefaultConsistencyLevel::Strong,
    }
}

impl std::fmt::Display for ReadConsistencyStrategy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::str::FromStr for ReadConsistencyStrategy {
    type Err = crate::error::CosmosError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::parse(s).ok_or_else(|| {
            crate::error::CosmosError::builder()
                .with_status(crate::error::CosmosStatus::new(
                    azure_core::http::StatusCode::BadRequest,
                ))
                .with_message(format!("Unknown read consistency strategy: {s}"))
                .build()
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_all_strategies() {
        assert_eq!(
            "Default".parse::<ReadConsistencyStrategy>().ok(),
            Some(ReadConsistencyStrategy::Default)
        );
        assert_eq!(
            "Eventual".parse::<ReadConsistencyStrategy>().ok(),
            Some(ReadConsistencyStrategy::Eventual)
        );
        assert_eq!(
            "Session".parse::<ReadConsistencyStrategy>().ok(),
            Some(ReadConsistencyStrategy::Session)
        );
        assert_eq!(
            "LatestCommitted".parse::<ReadConsistencyStrategy>().ok(),
            Some(ReadConsistencyStrategy::LatestCommitted)
        );
        assert_eq!(
            "GlobalStrong".parse::<ReadConsistencyStrategy>().ok(),
            Some(ReadConsistencyStrategy::GlobalStrong)
        );
    }

    #[test]
    fn parse_unknown_returns_none() {
        assert!("Unknown".parse::<ReadConsistencyStrategy>().is_err());
    }

    #[test]
    fn parse_case_insensitive_fallback() {
        assert_eq!(
            "eventual".parse::<ReadConsistencyStrategy>().ok(),
            Some(ReadConsistencyStrategy::Eventual)
        );
        assert_eq!(
            "latestcommitted".parse::<ReadConsistencyStrategy>().ok(),
            Some(ReadConsistencyStrategy::LatestCommitted)
        );
    }

    #[test]
    fn to_string_roundtrip() {
        for strategy in &[
            ReadConsistencyStrategy::Default,
            ReadConsistencyStrategy::Eventual,
            ReadConsistencyStrategy::Session,
            ReadConsistencyStrategy::LatestCommitted,
            ReadConsistencyStrategy::GlobalStrong,
        ] {
            let s = strategy.to_string();
            assert_eq!(s.parse::<ReadConsistencyStrategy>().ok(), Some(*strategy));
        }
    }

    #[test]
    fn is_non_default_table() {
        assert!(!ReadConsistencyStrategy::Default.is_non_default());
        assert!(ReadConsistencyStrategy::Eventual.is_non_default());
        assert!(ReadConsistencyStrategy::Session.is_non_default());
        assert!(ReadConsistencyStrategy::LatestCommitted.is_non_default());
        assert!(ReadConsistencyStrategy::GlobalStrong.is_non_default());
    }

    #[test]
    fn session_effective_when_strategy_is_session() {
        assert!(
            ReadConsistencyStrategy::Session.is_session_effective(DefaultConsistencyLevel::Strong)
        );
        assert!(ReadConsistencyStrategy::Session
            .is_session_effective(DefaultConsistencyLevel::Eventual));
    }

    #[test]
    fn session_effective_when_default_and_account_is_session() {
        assert!(
            ReadConsistencyStrategy::Default.is_session_effective(DefaultConsistencyLevel::Session)
        );
    }

    #[test]
    fn not_session_effective_when_default_and_account_is_not_session() {
        assert!(
            !ReadConsistencyStrategy::Default.is_session_effective(DefaultConsistencyLevel::Strong)
        );
        assert!(!ReadConsistencyStrategy::Default
            .is_session_effective(DefaultConsistencyLevel::BoundedStaleness));
        assert!(!ReadConsistencyStrategy::Default
            .is_session_effective(DefaultConsistencyLevel::ConsistentPrefix));
        assert!(!ReadConsistencyStrategy::Default
            .is_session_effective(DefaultConsistencyLevel::Eventual));
    }

    #[test]
    fn not_session_effective_for_eventual_or_global_strong_or_latest_committed() {
        assert!(!ReadConsistencyStrategy::Eventual
            .is_session_effective(DefaultConsistencyLevel::Session));
        assert!(!ReadConsistencyStrategy::GlobalStrong
            .is_session_effective(DefaultConsistencyLevel::Session));
        assert!(!ReadConsistencyStrategy::LatestCommitted
            .is_session_effective(DefaultConsistencyLevel::Session));
    }

    #[test]
    fn resolve_effective_consistency_table() {
        let account_defaults = [
            DefaultConsistencyLevel::Strong,
            DefaultConsistencyLevel::BoundedStaleness,
            DefaultConsistencyLevel::Session,
            DefaultConsistencyLevel::ConsistentPrefix,
            DefaultConsistencyLevel::Eventual,
        ];

        for account_default in account_defaults {
            assert_eq!(
                resolve_effective_consistency(ReadConsistencyStrategy::Default, account_default),
                account_default
            );
            assert_eq!(
                resolve_effective_consistency(ReadConsistencyStrategy::Eventual, account_default),
                DefaultConsistencyLevel::Eventual
            );
            assert_eq!(
                resolve_effective_consistency(ReadConsistencyStrategy::Session, account_default),
                DefaultConsistencyLevel::Session
            );
            // LatestCommitted intentionally resolves to the account default — the quorum-read
            // upgrade is requested server-side via the RCS header / token, not by collapsing to
            // Strong on the client (which would mis-trigger Strong-only client code paths).
            assert_eq!(
                resolve_effective_consistency(
                    ReadConsistencyStrategy::LatestCommitted,
                    account_default
                ),
                account_default
            );
            assert_eq!(
                resolve_effective_consistency(
                    ReadConsistencyStrategy::GlobalStrong,
                    account_default
                ),
                DefaultConsistencyLevel::Strong
            );
        }
    }
}

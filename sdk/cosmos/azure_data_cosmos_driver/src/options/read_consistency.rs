// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

/// Read consistency strategies supported by Azure Cosmos DB.
///
/// The requested read consistency strategy can be chosen independent of the consistency level
/// provisioned for the database account.
///
/// The `ReadConsistencyStrategy` setting will override whatever `ConsistencyLevel` is chosen
/// in request options, client options, or the default consistency level for an account unless
/// `ReadConsistencyStrategy::Default` is used.
///
/// **NOTE**: `ReadConsistencyStrategy` is currently only supported when using direct mode.
///
/// See [Cosmos DB consistency levels](https://learn.microsoft.com/azure/cosmos-db/consistency-levels)
/// for detailed semantics.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum ReadConsistencyStrategy {
    /// Use the default read behavior for the consistency level applied to the operation,
    /// the client, or the account.
    Default,

    /// Eventual consistency guarantees that reads will return a subset of writes.
    /// All writes will eventually be available for reads.
    Eventual,

    /// Session consistency guarantees monotonic reads (you never read old data, then new,
    /// then old again), monotonic writes (writes are ordered), and read your writes
    /// (your writes are immediately visible to your reads) within any single session.
    Session,

    /// Reads the latest version across all regions.
    ///
    /// Since replication with global strong consistency is synchronous, this read
    /// consistency strategy ensures that the latest successfully written version
    /// across regions is returned.
    ///
    /// **NOTE**: Only supported for single-master accounts with Strong consistency
    /// enabled as default consistency.
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
            "GlobalStrong" => Some(Self::GlobalStrong),
            _ => {
                // Case-insensitive fallback
                if s.eq_ignore_ascii_case("Default") {
                    Some(Self::Default)
                } else if s.eq_ignore_ascii_case("Eventual") {
                    Some(Self::Eventual)
                } else if s.eq_ignore_ascii_case("Session") {
                    Some(Self::Session)
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
            Self::GlobalStrong => "GlobalStrong",
        }
    }
}

impl std::fmt::Display for ReadConsistencyStrategy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::str::FromStr for ReadConsistencyStrategy {
    type Err = azure_core::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::parse(s).ok_or_else(|| {
            azure_core::Error::with_message(
                azure_core::error::ErrorKind::DataConversion,
                format!("Unknown read consistency strategy: {}", s),
            )
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
        // Case-insensitive fallback works
        assert_eq!(
            "eventual".parse::<ReadConsistencyStrategy>().ok(),
            Some(ReadConsistencyStrategy::Eventual)
        );
    }

    #[test]
    fn to_string_roundtrip() {
        for strategy in &[
            ReadConsistencyStrategy::Default,
            ReadConsistencyStrategy::Eventual,
            ReadConsistencyStrategy::Session,
            ReadConsistencyStrategy::GlobalStrong,
        ] {
            let s = strategy.to_string();
            assert_eq!(s.parse::<ReadConsistencyStrategy>().ok(), Some(*strategy));
        }
    }
}

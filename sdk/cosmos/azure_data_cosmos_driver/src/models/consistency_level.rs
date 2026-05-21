// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Cosmos DB default consistency level model.

use serde::{Deserialize, Serialize};

/// The account-level default consistency level configured for a Cosmos DB account.
///
/// This type represents the *default* consistency of the account and should
/// **only** be used in account metadata (`AccountProperties`). For per-request
/// consistency overrides use [`ReadConsistencyStrategy`](crate::options::ReadConsistencyStrategy).
///
/// See [Cosmos DB consistency levels](https://learn.microsoft.com/azure/cosmos-db/consistency-levels)
/// for detailed semantics.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub(crate) enum DefaultConsistencyLevel {
    /// Reads are guaranteed to return the most recent committed version.
    Strong,

    /// Reads lag behind writes by at most K versions or T time interval.
    BoundedStaleness,

    /// Monotonic reads, monotonic writes, and read-your-writes within a session.
    Session,

    /// Reads may lag behind writes but are guaranteed to be in order.
    ConsistentPrefix,

    /// Reads may return any committed version; no ordering guarantees.
    Eventual,
}

impl DefaultConsistencyLevel {
    /// Returns `true` if this consistency level is [`Session`](Self::Session).
    pub(crate) fn is_session(&self) -> bool {
        matches!(self, Self::Session)
    }
}

impl std::fmt::Display for DefaultConsistencyLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Strong => f.write_str("Strong"),
            Self::BoundedStaleness => f.write_str("BoundedStaleness"),
            Self::Session => f.write_str("Session"),
            Self::ConsistentPrefix => f.write_str("ConsistentPrefix"),
            Self::Eventual => f.write_str("Eventual"),
        }
    }
}

impl std::str::FromStr for DefaultConsistencyLevel {
    type Err = azure_core::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Case-sensitive first, then case-insensitive fallback.
        match s {
            "Strong" => Ok(Self::Strong),
            "BoundedStaleness" => Ok(Self::BoundedStaleness),
            "Session" => Ok(Self::Session),
            "ConsistentPrefix" => Ok(Self::ConsistentPrefix),
            "Eventual" => Ok(Self::Eventual),
            _ => {
                if s.eq_ignore_ascii_case("Strong") {
                    Ok(Self::Strong)
                } else if s.eq_ignore_ascii_case("BoundedStaleness") {
                    Ok(Self::BoundedStaleness)
                } else if s.eq_ignore_ascii_case("Session") {
                    Ok(Self::Session)
                } else if s.eq_ignore_ascii_case("ConsistentPrefix") {
                    Ok(Self::ConsistentPrefix)
                } else if s.eq_ignore_ascii_case("Eventual") {
                    Ok(Self::Eventual)
                } else {
                    Err(azure_core::Error::with_message(
                        azure_core::error::ErrorKind::DataConversion,
                        format!("Unknown consistency level: {s}"),
                    ))
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_all_levels() {
        assert_eq!(
            "Strong".parse::<DefaultConsistencyLevel>().unwrap(),
            DefaultConsistencyLevel::Strong
        );
        assert_eq!(
            "BoundedStaleness"
                .parse::<DefaultConsistencyLevel>()
                .unwrap(),
            DefaultConsistencyLevel::BoundedStaleness
        );
        assert_eq!(
            "Session".parse::<DefaultConsistencyLevel>().unwrap(),
            DefaultConsistencyLevel::Session
        );
        assert_eq!(
            "ConsistentPrefix"
                .parse::<DefaultConsistencyLevel>()
                .unwrap(),
            DefaultConsistencyLevel::ConsistentPrefix
        );
        assert_eq!(
            "Eventual".parse::<DefaultConsistencyLevel>().unwrap(),
            DefaultConsistencyLevel::Eventual
        );
    }

    #[test]
    fn parse_case_insensitive() {
        assert_eq!(
            "session".parse::<DefaultConsistencyLevel>().unwrap(),
            DefaultConsistencyLevel::Session
        );
        assert_eq!(
            "STRONG".parse::<DefaultConsistencyLevel>().unwrap(),
            DefaultConsistencyLevel::Strong
        );
        assert_eq!(
            "eventual".parse::<DefaultConsistencyLevel>().unwrap(),
            DefaultConsistencyLevel::Eventual
        );
    }

    #[test]
    fn parse_unknown_fails() {
        assert!("Unknown".parse::<DefaultConsistencyLevel>().is_err());
    }

    #[test]
    fn display_formats_correctly() {
        assert_eq!(DefaultConsistencyLevel::Strong.to_string(), "Strong");
        assert_eq!(
            DefaultConsistencyLevel::BoundedStaleness.to_string(),
            "BoundedStaleness"
        );
        assert_eq!(DefaultConsistencyLevel::Session.to_string(), "Session");
        assert_eq!(
            DefaultConsistencyLevel::ConsistentPrefix.to_string(),
            "ConsistentPrefix"
        );
        assert_eq!(DefaultConsistencyLevel::Eventual.to_string(), "Eventual");
    }

    #[test]
    fn serde_serializes_correctly() {
        assert_eq!(
            serde_json::to_string(&DefaultConsistencyLevel::Strong).unwrap(),
            "\"Strong\""
        );
        assert_eq!(
            serde_json::to_string(&DefaultConsistencyLevel::BoundedStaleness).unwrap(),
            "\"BoundedStaleness\""
        );
        assert_eq!(
            serde_json::to_string(&DefaultConsistencyLevel::Session).unwrap(),
            "\"Session\""
        );
        assert_eq!(
            serde_json::to_string(&DefaultConsistencyLevel::ConsistentPrefix).unwrap(),
            "\"ConsistentPrefix\""
        );
        assert_eq!(
            serde_json::to_string(&DefaultConsistencyLevel::Eventual).unwrap(),
            "\"Eventual\""
        );
    }

    #[test]
    fn serde_deserializes_correctly() {
        assert_eq!(
            serde_json::from_str::<DefaultConsistencyLevel>("\"Strong\"").unwrap(),
            DefaultConsistencyLevel::Strong
        );
        assert_eq!(
            serde_json::from_str::<DefaultConsistencyLevel>("\"BoundedStaleness\"").unwrap(),
            DefaultConsistencyLevel::BoundedStaleness
        );
        assert_eq!(
            serde_json::from_str::<DefaultConsistencyLevel>("\"Session\"").unwrap(),
            DefaultConsistencyLevel::Session
        );
        assert_eq!(
            serde_json::from_str::<DefaultConsistencyLevel>("\"ConsistentPrefix\"").unwrap(),
            DefaultConsistencyLevel::ConsistentPrefix
        );
        assert_eq!(
            serde_json::from_str::<DefaultConsistencyLevel>("\"Eventual\"").unwrap(),
            DefaultConsistencyLevel::Eventual
        );
    }

    #[test]
    fn is_session() {
        assert!(DefaultConsistencyLevel::Session.is_session());
        assert!(!DefaultConsistencyLevel::Strong.is_session());
        assert!(!DefaultConsistencyLevel::Eventual.is_session());
    }
}

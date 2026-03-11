// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Cosmos DB consistency level model.

use serde::{Deserialize, Serialize};

/// The five consistency levels supported by Azure Cosmos DB.
///
/// Every Cosmos DB account is configured with one of these as its default.
/// Individual operations can relax (but not strengthen) the consistency level.
///
/// See [Cosmos DB consistency levels](https://learn.microsoft.com/azure/cosmos-db/consistency-levels)
/// for detailed semantics.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum ConsistencyLevel {
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

impl ConsistencyLevel {
    /// Returns `true` if this consistency level is [`Session`](Self::Session).
    pub fn is_session(&self) -> bool {
        matches!(self, Self::Session)
    }
}

impl std::fmt::Display for ConsistencyLevel {
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

impl std::str::FromStr for ConsistencyLevel {
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
            "Strong".parse::<ConsistencyLevel>().unwrap(),
            ConsistencyLevel::Strong
        );
        assert_eq!(
            "BoundedStaleness".parse::<ConsistencyLevel>().unwrap(),
            ConsistencyLevel::BoundedStaleness
        );
        assert_eq!(
            "Session".parse::<ConsistencyLevel>().unwrap(),
            ConsistencyLevel::Session
        );
        assert_eq!(
            "ConsistentPrefix".parse::<ConsistencyLevel>().unwrap(),
            ConsistencyLevel::ConsistentPrefix
        );
        assert_eq!(
            "Eventual".parse::<ConsistencyLevel>().unwrap(),
            ConsistencyLevel::Eventual
        );
    }

    #[test]
    fn parse_case_insensitive() {
        assert_eq!(
            "session".parse::<ConsistencyLevel>().unwrap(),
            ConsistencyLevel::Session
        );
        assert_eq!(
            "STRONG".parse::<ConsistencyLevel>().unwrap(),
            ConsistencyLevel::Strong
        );
        assert_eq!(
            "eventual".parse::<ConsistencyLevel>().unwrap(),
            ConsistencyLevel::Eventual
        );
    }

    #[test]
    fn parse_unknown_fails() {
        assert!("Unknown".parse::<ConsistencyLevel>().is_err());
    }

    #[test]
    fn display_roundtrip() {
        for level in &[
            ConsistencyLevel::Strong,
            ConsistencyLevel::BoundedStaleness,
            ConsistencyLevel::Session,
            ConsistencyLevel::ConsistentPrefix,
            ConsistencyLevel::Eventual,
        ] {
            assert_eq!(
                level.to_string().parse::<ConsistencyLevel>().unwrap(),
                *level
            );
        }
    }

    #[test]
    fn is_session() {
        assert!(ConsistencyLevel::Session.is_session());
        assert!(!ConsistencyLevel::Strong.is_session());
        assert!(!ConsistencyLevel::Eventual.is_session());
    }

    #[test]
    fn serde_roundtrip() {
        let level = ConsistencyLevel::Session;
        let json = serde_json::to_string(&level).unwrap();
        assert_eq!(json, "\"Session\"");
        let parsed: ConsistencyLevel = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed, level);
    }
}

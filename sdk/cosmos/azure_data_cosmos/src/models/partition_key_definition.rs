// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use azure_core::fmt::SafeDebug;
use serde::{Deserialize, Serialize};
use std::fmt;

/// Represents the partition key kind.
///
/// This is a string new type that allows for forward-compatible deserialization of
/// partition key kinds. Known values are defined as constants on this type.
#[derive(Clone, SafeDebug, Deserialize, Serialize)]
#[safe(true)]
pub struct PartitionKeyKind(String);

impl PartitionKeyKind {
    /// Standard single-path hashing.
    pub const HASH: &str = "Hash";

    /// Multi-path (hierarchical) partition keys.
    pub const MULTI_HASH: &str = "MultiHash";

    /// Creates a new [`PartitionKeyKind`] from the given string value.
    pub fn new(value: impl Into<String>) -> Self {
        PartitionKeyKind(value.into())
    }

    /// Returns the string representation of the partition key kind.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Default for PartitionKeyKind {
    fn default() -> Self {
        PartitionKeyKind(Self::HASH.to_string())
    }
}

impl fmt::Display for PartitionKeyKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl From<&str> for PartitionKeyKind {
    fn from(value: &str) -> Self {
        PartitionKeyKind(value.to_string())
    }
}

impl From<String> for PartitionKeyKind {
    fn from(value: String) -> Self {
        PartitionKeyKind(value)
    }
}

impl PartialEq for PartitionKeyKind {
    fn eq(&self, other: &Self) -> bool {
        self.0.eq_ignore_ascii_case(&other.0)
    }
}

impl Eq for PartitionKeyKind {}

/// Represents the partition key definition for a container.
#[derive(Clone, SafeDebug, Default, Deserialize, Serialize, PartialEq, Eq)]
#[safe(true)]
#[serde(rename_all = "camelCase")]
pub struct PartitionKeyDefinition {
    /// The list of partition keys paths.
    pub paths: Vec<String>,

    /// The partition key kind.
    pub kind: PartitionKeyKind,

    /// The version of the partition key hash in use.
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<i32>,
}

impl PartitionKeyDefinition {
    /// Creates a new [`PartitionKeyDefinition`] from the provided list of partition key paths.
    ///
    /// The [`PartitionKeyDefinition::kind`] will be set automatically, depending on how many paths are provided.
    pub fn new(paths: impl Into<Vec<String>>) -> Self {
        let paths = paths.into();
        let kind = if paths.len() > 1 {
            PartitionKeyKind::new(PartitionKeyKind::MULTI_HASH)
        } else {
            PartitionKeyKind::new(PartitionKeyKind::HASH)
        };
        PartitionKeyDefinition {
            paths,
            kind,
            version: Some(2),
        }
    }
}

impl From<&str> for PartitionKeyDefinition {
    fn from(value: &str) -> Self {
        PartitionKeyDefinition::new(vec![value.into()])
    }
}

impl From<String> for PartitionKeyDefinition {
    fn from(value: String) -> Self {
        PartitionKeyDefinition::new(vec![value])
    }
}

impl<S1: Into<String>, S2: Into<String>> From<(S1, S2)> for PartitionKeyDefinition {
    fn from(value: (S1, S2)) -> Self {
        PartitionKeyDefinition::new(vec![value.0.into(), value.1.into()])
    }
}

impl<S1: Into<String>, S2: Into<String>, S3: Into<String>> From<(S1, S2, S3)>
    for PartitionKeyDefinition
{
    fn from(value: (S1, S2, S3)) -> Self {
        PartitionKeyDefinition::new(vec![value.0.into(), value.1.into(), value.2.into()])
    }
}

#[cfg(test)]
mod tests {
    use crate::models::{PartitionKeyDefinition, PartitionKeyKind};

    #[test]
    pub fn from_single() {
        assert_eq!(
            PartitionKeyDefinition {
                paths: vec!["/a".to_string()],
                kind: PartitionKeyKind::new(PartitionKeyKind::HASH),
                version: Some(2),
            },
            "/a".into()
        );
        assert_eq!(
            PartitionKeyDefinition {
                paths: vec!["/a".to_string()],
                kind: PartitionKeyKind::new(PartitionKeyKind::HASH),
                version: Some(2),
            },
            "/a".to_string().into()
        );
    }

    #[test]
    pub fn from_pair() {
        assert_eq!(
            PartitionKeyDefinition {
                paths: vec!["/a".to_string(), "/b".to_string()],
                kind: PartitionKeyKind::new(PartitionKeyKind::MULTI_HASH),
                version: Some(2),
            },
            ("/a", "/b").into()
        );
    }

    #[test]
    pub fn from_triple() {
        assert_eq!(
            PartitionKeyDefinition {
                paths: vec!["/a".to_string(), "/b".to_string(), "/c".to_string()],
                kind: PartitionKeyKind::new(PartitionKeyKind::MULTI_HASH),
                version: Some(2),
            },
            ("/a", "/b", "/c").into()
        );
    }
}

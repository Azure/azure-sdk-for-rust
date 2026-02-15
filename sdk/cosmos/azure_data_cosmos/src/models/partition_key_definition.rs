// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use azure_core::fmt::SafeDebug;
use serde::{Deserialize, Serialize};

/// Represents the partition key definition for a container.
///
/// # Required fields
///
/// * `paths` — The list of partition key paths.
///
/// Use [`PartitionKeyDefinition::new()`] or one of the `From` impls to construct an instance:
///
/// ```rust
/// # use azure_data_cosmos::models::PartitionKeyDefinition;
/// let pk: PartitionKeyDefinition = "/partitionKey".into();
/// ```
#[non_exhaustive]
#[derive(Clone, SafeDebug, Deserialize, Serialize, PartialEq, Eq)]
#[safe(true)]
#[serde(rename_all = "camelCase")]
pub struct PartitionKeyDefinition {
    /// The list of partition keys paths.
    paths: Vec<String>,

    /// The partition key kind.
    #[serde(default)]
    kind: PartitionKeyKind,

    /// The version of the partition key hash in use.
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    version: Option<i32>,
}

impl PartitionKeyDefinition {
    /// Creates a new [`PartitionKeyDefinition`] from the provided list of partition key paths.
    ///
    /// The kind will be set automatically, depending on how many paths are provided.
    pub fn new(paths: impl Into<Vec<String>>) -> Self {
        let paths = paths.into();
        let kind = if paths.len() > 1 {
            PartitionKeyKind::MultiHash
        } else {
            PartitionKeyKind::Hash
        };
        PartitionKeyDefinition {
            paths,
            kind,
            version: Some(2),
        }
    }

    /// Gets the list of partition key paths.
    pub fn paths(&self) -> &[String] {
        &self.paths
    }

    /// Gets the partition key kind.
    pub fn kind(&self) -> &PartitionKeyKind {
        &self.kind
    }

    /// Sets the partition key kind.
    pub fn with_kind(mut self, kind: PartitionKeyKind) -> Self {
        self.kind = kind;
        self
    }

    /// Gets the partition key hash version.
    pub fn version(&self) -> Option<i32> {
        self.version
    }

    /// Sets the partition key hash version.
    pub fn with_version(mut self, version: Option<i32>) -> Self {
        self.version = version;
        self
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

/// Represents the kind of a partition key.
#[derive(Clone, SafeDebug, Default, Deserialize, Serialize, PartialEq, Eq)]
#[safe(true)]
#[serde(rename_all = "PascalCase")]
pub enum PartitionKeyKind {
    /// The container is partitioned by hashing the value of a single partition key.
    #[default]
    Hash,

    /// The container is partitioned by hashing multiple, hierarchical, partition keys.
    MultiHash,
}

#[cfg(test)]
mod tests {
    use crate::models::{PartitionKeyDefinition, PartitionKeyKind};

    #[test]
    pub fn from_single() {
        assert_eq!(
            PartitionKeyDefinition {
                paths: vec!["/a".to_string()],
                kind: PartitionKeyKind::Hash,
                version: Some(2),
            },
            "/a".into()
        );
        assert_eq!(
            PartitionKeyDefinition {
                paths: vec!["/a".to_string()],
                kind: PartitionKeyKind::Hash,
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
                kind: PartitionKeyKind::MultiHash,
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
                kind: PartitionKeyKind::MultiHash,
                version: Some(2),
            },
            ("/a", "/b", "/c").into()
        );
    }
}

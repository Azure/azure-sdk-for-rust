// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use serde::{Deserialize, Serialize};

/// Represents the partition key definition for a container.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
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
}

/// Represents the kind of a partition key.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "PascalCase")]
pub enum PartitionKeyKind {
    /// The container is partitioned by hashing the value of a single partition key.
    Hash,

    /// The container is partitioned by hashing multiple, hierarchical, partition keys.
    MultiHash,
}

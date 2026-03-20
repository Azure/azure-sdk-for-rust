// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Eagerly-resolved immutable container metadata.

use crate::models::{PartitionKeyDefinition, PartitionKeyKind};

/// Eagerly-resolved immutable metadata about a Cosmos DB container.
///
/// Created once when a [`ContainerClient`](crate::clients::ContainerClient) is constructed
/// by calling [`CosmosDriver::resolve_container()`](azure_data_cosmos_driver::CosmosDriver::resolve_container).
/// Holds the container's name, resource ID (RID), and partition key definition —
/// all of which are immutable after container creation.
///
/// This is the SDK's own type, adapted from the driver's
/// [`ContainerReference`](azure_data_cosmos_driver::models::ContainerReference)
/// to maintain independent versioning between the two crates.
#[derive(Clone, Debug)]
pub(crate) struct ContainerReference {
    /// Container user-facing name (e.g., "MyContainer").
    container_id: String,
    /// Container resource ID from the service (e.g., "ZH0DAJhRLgA=").
    collection_rid: String,
    /// Partition key definition (paths, kind, version) — immutable after creation.
    partition_key: PartitionKeyDefinition,
}

impl ContainerReference {
    /// Creates an SDK [`ContainerReference`] from the driver's resolved container metadata.
    ///
    /// Converts the driver's `PartitionKeyDefinition` (which uses enum-based `PartitionKeyKind`
    /// and `PartitionKeyVersion`) into the SDK's types (string-based `PartitionKeyKind` with
    /// `Option<i32>` version).
    pub(crate) fn from_driver_ref(
        driver_ref: &azure_data_cosmos_driver::models::ContainerReference,
    ) -> Self {
        let driver_pk = driver_ref.partition_key_definition();
        let paths: Vec<String> = driver_pk.paths().iter().map(|p| p.to_string()).collect();

        use azure_data_cosmos_driver::models::PartitionKeyKind as DriverKind;
        let kind = match driver_pk.kind() {
            DriverKind::Hash => {
                // Within Hash, the SDK distinguishes single-path (Hash) from
                // multi-path hierarchical partitioning (MultiHash) by path count,
                // matching how PartitionKeyDefinition::new() works.
                if paths.len() > 1 {
                    PartitionKeyKind::new(PartitionKeyKind::MULTI_HASH)
                } else {
                    PartitionKeyKind::new(PartitionKeyKind::HASH)
                }
            }
            DriverKind::Range => {
                // Legacy range partitioning — preserve the kind so EPK calculation
                // falls back to binary encoding instead of hashing.
                PartitionKeyKind::new("Range")
            }
            // PartitionKeyKind is #[non_exhaustive]; treat unknown variants as Hash.
            _ => PartitionKeyKind::new(PartitionKeyKind::HASH),
        };

        let version = Some(driver_pk.version().value() as i32);

        Self {
            container_id: driver_ref.name().to_string(),
            collection_rid: driver_ref.rid().to_string(),
            partition_key: PartitionKeyDefinition {
                paths,
                kind,
                version,
            },
        }
    }

    /// Returns the container's user-facing name.
    pub(crate) fn container_id(&self) -> &str {
        &self.container_id
    }

    /// Returns the container's resource ID (RID).
    ///
    /// Used as the cache key for `PartitionKeyRangeCache` lookups, ensuring
    /// all code paths share the same cache entry.
    pub(crate) fn collection_rid(&self) -> &str {
        &self.collection_rid
    }

    /// Returns the container's partition key definition.
    pub(crate) fn partition_key(&self) -> &PartitionKeyDefinition {
        &self.partition_key
    }

    /// Creates a `ContainerReference` from individual parts.
    ///
    /// Used in tests and when constructing from non-driver sources.
    pub(crate) fn from_parts(
        container_id: String,
        collection_rid: String,
        partition_key: PartitionKeyDefinition,
    ) -> Self {
        Self {
            container_id,
            collection_rid,
            partition_key,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accessors_return_stored_values() {
        let container_ref = ContainerReference {
            container_id: "test-container".to_string(),
            collection_rid: "ZH0DAJhRLgA=".to_string(),
            partition_key: PartitionKeyDefinition::new(vec!["/id".to_string()]),
        };

        assert_eq!(container_ref.container_id(), "test-container");
        assert_eq!(container_ref.collection_rid(), "ZH0DAJhRLgA=");
        assert_eq!(container_ref.partition_key().paths, vec!["/id".to_string()]);
    }
}

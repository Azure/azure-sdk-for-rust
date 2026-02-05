// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Defines conditions for when fault injection rules should be applied.

use crate::regions::RegionName;

/// Defines the condition under which a fault injection rule should be applied.
#[derive(Clone, Default, Debug)]
pub struct FaultInjectionCondition {
    /// The type of operation to which the fault injection applies.
    /// By default, the fault injection applies to all operation types.
    pub(crate) operation_type: Option<FaultOperationType>,
    /// The region to which the fault injection applies.
    /// By default, the fault injection applies to all regions.
    pub(crate) region: Option<RegionName>,
    /// The partition key range ID to which the fault injection applies.
    /// By default, the fault injection applies to all partition key ranges.
    pub(crate) partition_key_range_id: Option<String>,
    /// The container ID to which the fault injection applies.
    /// By default, the fault injection applies to all containers.
    pub(crate) container_id: Option<String>,
}

/// The type of operation to which the fault injection applies.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum FaultOperationType {
    /// Read items.
    #[default]
    ReadItem,
    /// Query items.
    QueryItem,
    /// Create item.
    CreateItem,
    /// Upsert item.
    UpsertItem,
    /// Replace item.
    ReplaceItem,
    /// Delete item.
    DeleteItem,
    /// Patch item.
    PatchItem,
    /// Batch item.
    BatchItem,
    /// Read change feed items.
    ChangeFeedItem,
    /// Read container request.
    MetadataReadContainer,
    /// Read database account request.
    MetadataReadDatabaseAccount,
    /// Query query plan request.
    MetadataQueryPlan,
    /// Partition key ranges request.
    MetadataPartitionKeyRanges,
}

impl From<FaultOperationType> for &'static str {
    fn from(op: FaultOperationType) -> Self {
        match op {
            FaultOperationType::ReadItem => "ReadItem",
            FaultOperationType::QueryItem => "QueryItem",
            FaultOperationType::CreateItem => "CreateItem",
            FaultOperationType::UpsertItem => "UpsertItem",
            FaultOperationType::ReplaceItem => "ReplaceItem",
            FaultOperationType::DeleteItem => "DeleteItem",
            FaultOperationType::PatchItem => "PatchItem",
            FaultOperationType::BatchItem => "BatchItem",
            FaultOperationType::ChangeFeedItem => "ChangeFeedItem",
            FaultOperationType::MetadataReadContainer => "MetadataReadContainer",
            FaultOperationType::MetadataReadDatabaseAccount => "MetadataReadDatabaseAccount",
            FaultOperationType::MetadataQueryPlan => "MetadataQueryPlan",
            FaultOperationType::MetadataPartitionKeyRanges => "MetadataPartitionKeyRanges",
        }
    }
}

impl FaultOperationType {
    /// Attempts to parse a FaultOperationType from a string.
    /// Returns None if the string does not match any known operation type.
    pub fn from_str(s: &str) -> Option<FaultOperationType> {
        match s {
            "ReadItem" => Some(FaultOperationType::ReadItem),
            "QueryItem" => Some(FaultOperationType::QueryItem),
            "CreateItem" => Some(FaultOperationType::CreateItem),
            "UpsertItem" => Some(FaultOperationType::UpsertItem),
            "ReplaceItem" => Some(FaultOperationType::ReplaceItem),
            "DeleteItem" => Some(FaultOperationType::DeleteItem),
            "PatchItem" => Some(FaultOperationType::PatchItem),
            "BatchItem" => Some(FaultOperationType::BatchItem),
            "ChangeFeedItem" => Some(FaultOperationType::ChangeFeedItem),
            "MetadataReadContainer" => Some(FaultOperationType::MetadataReadContainer),
            "MetadataReadDatabaseAccount" => Some(FaultOperationType::MetadataReadDatabaseAccount),
            "MetadataQueryPlan" => Some(FaultOperationType::MetadataQueryPlan),
            "MetadataPartitionKeyRanges" => Some(FaultOperationType::MetadataPartitionKeyRanges),
            _ => None,
        }
    }
}

/// Builder for creating a FaultInjectionCondition.
pub struct FaultInjectionConditionBuilder {
    operation_type: Option<FaultOperationType>,
    region: Option<RegionName>,
    partition_key_range_id: Option<String>,
    container_id: Option<String>,
}

impl FaultInjectionConditionBuilder {
    /// Creates a new FaultInjectionConditionBuilder with default values.
    pub fn new() -> Self {
        Self {
            operation_type: None,
            region: None,
            partition_key_range_id: None,
            container_id: None,
        }
    }

    /// Sets the operation type to which the fault injection applies.
    pub fn with_operation_type(mut self, operation_type: FaultOperationType) -> Self {
        self.operation_type = Some(operation_type);
        self
    }

    /// Sets the region to which the fault injection applies.
    pub fn with_region(mut self, region: RegionName) -> Self {
        self.region = Some(region);
        self
    }

    /// Sets the partition to which the fault injection applies.
    pub fn with_partition_key_range_id(
        mut self,
        partition_key_range_id: impl Into<String>,
    ) -> Self {
        self.partition_key_range_id = Some(partition_key_range_id.into());
        self
    }

    /// Sets the container ID to which the fault injection applies.
    pub fn with_container_id(mut self, container_id: impl Into<String>) -> Self {
        self.container_id = Some(container_id.into());
        self
    }

    /// Builds the FaultInjectionCondition.
    pub fn build(self) -> FaultInjectionCondition {
        FaultInjectionCondition {
            operation_type: self.operation_type,
            region: self.region,
            partition_key_range_id: self.partition_key_range_id,
            container_id: self.container_id,
        }
    }
}

impl Default for FaultInjectionConditionBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::{FaultInjectionConditionBuilder, FaultOperationType};
    use crate::regions;

    #[test]
    fn builder_default() {
        let builder = FaultInjectionConditionBuilder::default();
        let condition = builder.build();
        assert!(condition.operation_type.is_none());
        assert!(condition.region.is_none());
        assert!(condition.partition_key_range_id.is_none());
        assert!(condition.container_id.is_none());
    }

    #[test]
    fn builder_chained() {
        let condition = FaultInjectionConditionBuilder::new()
            .with_operation_type(FaultOperationType::DeleteItem)
            .with_region(regions::WEST_US)
            .with_partition_key_range_id("range-2")
            .with_container_id("container-2")
            .build();

        assert_eq!(
            condition.operation_type,
            Some(FaultOperationType::DeleteItem)
        );
        assert_eq!(condition.region, Some(regions::WEST_US));
        assert_eq!(
            condition.partition_key_range_id,
            Some("range-2".to_string())
        );
        assert_eq!(condition.container_id, Some("container-2".to_string()));
    }
}

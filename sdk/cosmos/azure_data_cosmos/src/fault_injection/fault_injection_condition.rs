// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Defines conditions for when fault injection rules should be applied.

use std::fmt;
use std::str::FromStr;

use crate::operation_context::OperationType;
use crate::regions::RegionName;
use crate::resource_context::ResourceType;

/// Defines the condition under which a fault injection rule should be applied.
#[derive(Clone, Default, Debug)]
pub struct FaultInjectionCondition {
    /// The type of operation to which the fault injection applies.
    operation_type: Option<FaultOperationType>,
    /// The region to which the fault injection applies.
    region: Option<RegionName>,
    /// The container ID to which the fault injection applies.
    container_id: Option<String>,
}

impl FaultInjectionCondition {
    /// Returns the operation type this condition matches, if set.
    pub fn operation_type(&self) -> Option<FaultOperationType> {
        self.operation_type
    }

    /// Returns the region this condition matches, if set.
    pub fn region(&self) -> Option<&RegionName> {
        self.region.as_ref()
    }

    /// Returns the container ID this condition matches, if set.
    pub fn container_id(&self) -> Option<&str> {
        self.container_id.as_deref()
    }
}

/// The type of operation to which the fault injection applies.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FaultOperationType {
    /// Read items.
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

impl FaultOperationType {
    /// Returns the string representation of this operation type.
    pub fn as_str(&self) -> &'static str {
        match self {
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

    /// Converts an operation type and resource type pair into a fault injection operation type.
    ///
    /// Returns `None` if the combination does not map to a known fault operation type.
    pub fn from_operation_and_resource(
        operation_type: &OperationType,
        resource_type: &ResourceType,
    ) -> Option<Self> {
        match (operation_type, resource_type) {
            (OperationType::Read, ResourceType::Documents) => Some(FaultOperationType::ReadItem),
            (OperationType::Query, ResourceType::Documents) => Some(FaultOperationType::QueryItem),
            (OperationType::Create, ResourceType::Documents) => {
                Some(FaultOperationType::CreateItem)
            }
            (OperationType::Upsert, ResourceType::Documents) => {
                Some(FaultOperationType::UpsertItem)
            }
            (OperationType::Replace, ResourceType::Documents) => {
                Some(FaultOperationType::ReplaceItem)
            }
            (OperationType::Delete, ResourceType::Documents) => {
                Some(FaultOperationType::DeleteItem)
            }
            (OperationType::Patch, ResourceType::Documents) => Some(FaultOperationType::PatchItem),
            (OperationType::Batch, ResourceType::Documents) => Some(FaultOperationType::BatchItem),
            (OperationType::ReadFeed, ResourceType::Documents) => {
                Some(FaultOperationType::ChangeFeedItem)
            }
            (OperationType::Read, ResourceType::Containers) => {
                Some(FaultOperationType::MetadataReadContainer)
            }
            (OperationType::Read, ResourceType::DatabaseAccount) => {
                Some(FaultOperationType::MetadataReadDatabaseAccount)
            }
            (OperationType::QueryPlan, ResourceType::Documents) => {
                Some(FaultOperationType::MetadataQueryPlan)
            }
            (OperationType::ReadFeed, ResourceType::PartitionKeyRanges) => {
                Some(FaultOperationType::MetadataPartitionKeyRanges)
            }
            _ => None,
        }
    }
}

impl fmt::Display for FaultOperationType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl FromStr for FaultOperationType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ReadItem" => Ok(FaultOperationType::ReadItem),
            "QueryItem" => Ok(FaultOperationType::QueryItem),
            "CreateItem" => Ok(FaultOperationType::CreateItem),
            "UpsertItem" => Ok(FaultOperationType::UpsertItem),
            "ReplaceItem" => Ok(FaultOperationType::ReplaceItem),
            "DeleteItem" => Ok(FaultOperationType::DeleteItem),
            "PatchItem" => Ok(FaultOperationType::PatchItem),
            "BatchItem" => Ok(FaultOperationType::BatchItem),
            "ChangeFeedItem" => Ok(FaultOperationType::ChangeFeedItem),
            "MetadataReadContainer" => Ok(FaultOperationType::MetadataReadContainer),
            "MetadataReadDatabaseAccount" => Ok(FaultOperationType::MetadataReadDatabaseAccount),
            "MetadataQueryPlan" => Ok(FaultOperationType::MetadataQueryPlan),
            "MetadataPartitionKeyRanges" => Ok(FaultOperationType::MetadataPartitionKeyRanges),
            _ => Err(format!("unknown FaultOperationType: {}", s)),
        }
    }
}

/// Builder for creating a FaultInjectionCondition.
pub struct FaultInjectionConditionBuilder {
    operation_type: Option<FaultOperationType>,
    region: Option<RegionName>,
    container_id: Option<String>,
}

impl FaultInjectionConditionBuilder {
    /// Creates a new FaultInjectionConditionBuilder with default values.
    pub fn new() -> Self {
        Self {
            operation_type: None,
            region: None,
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
    use super::FaultInjectionConditionBuilder;

    #[test]
    fn builder_default() {
        let builder = FaultInjectionConditionBuilder::default();
        let condition = builder.build();
        assert!(condition.operation_type().is_none());
        assert!(condition.region().is_none());
        assert!(condition.container_id().is_none());
    }
}

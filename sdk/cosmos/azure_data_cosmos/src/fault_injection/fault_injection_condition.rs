// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Defines conditions for when fault injection rules should be applied.

/// Defines the condition under which a fault injection rule should be applied.
#[derive(Clone, Default, Debug)]
pub struct FaultInjectionCondition {
    /// The endpoints to which the fault injection applies.
    /// Either the region or the endpoints must be specified.
    pub endpoints: Option<Vec<String>>,
    /// The type of operation to which the fault injection applies.
    pub operation_type: Option<FaultOperationType>,
    /// The region to which the fault injection applies.
    /// Either the endpoints or the region must be specified.
    pub region: Option<String>,
    /// The partition key range ID to which the fault injection applies.
    pub partition_key_range_id: Option<String>,
    /// The container ID to which the fault injection applies.
    pub container_id: Option<String>,
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

/// Builder for creating a FaultInjectionCondition.
pub struct FaultInjectionConditionBuilder {
    endpoints: Option<Vec<String>>,
    operation_type: Option<FaultOperationType>,
    region: Option<String>,
    partition_key_range_id: Option<String>,
    container_id: Option<String>,
}

impl FaultInjectionConditionBuilder {
    /// Creates a new FaultInjectionConditionBuilder with default values.
    pub fn new() -> Self {
        Self {
            endpoints: None,
            operation_type: None,
            region: None,
            partition_key_range_id: None,
            container_id: None,
        }
    }

    /// Sets the endpoints to which the fault injection applies.
    pub fn with_endpoints(mut self, endpoints: Vec<String>) -> Self {
        self.endpoints = Some(endpoints);
        self
    }

    /// Adds an endpoint to which the fault injection applies.
    pub fn with_endpoint(mut self, endpoint: impl Into<String>) -> Self {
        self.endpoints
            .get_or_insert_with(Vec::new)
            .push(endpoint.into());
        self
    }

    /// Sets the operation type to which the fault injection applies.
    pub fn with_operation_type(mut self, operation_type: FaultOperationType) -> Self {
        self.operation_type = Some(operation_type);
        self
    }

    /// Sets the region to which the fault injection applies.
    pub fn with_region(mut self, region: impl Into<String>) -> Self {
        self.region = Some(region.into());
        self
    }

    /// Sets the feed range to which the fault injection applies.
    pub fn with_partition_key_range_id(mut self, feed_range: impl Into<String>) -> Self {
        self.partition_key_range_id = Some(feed_range.into());
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
            endpoints: self.endpoints,
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

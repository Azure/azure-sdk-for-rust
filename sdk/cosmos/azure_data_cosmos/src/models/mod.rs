// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Resource model types sent to and received from the Azure Cosmos DB API.

// =========================================================================
// Public API
// =========================================================================

#[doc(inline)]
pub use azure_data_cosmos_driver::models::{
    ContainerReference, CosmosNumber, CosmosStatus, EffectivePartitionKey, PartitionKey,
    PartitionKeyDefinition, PartitionKeyKind, PartitionKeyValue, PartitionKeyVersion,
    PatchInstructions, PatchOperation,
};
pub use batch_response::BatchResponse;
pub use change_feed_item::{
    ChangeFeedItem, ChangeFeedMetadata, ChangeFeedOperationType, LogicalSequenceNumber,
};
pub use container_properties::{
    ConflictResolutionMode, ConflictResolutionPolicy, ContainerProperties, TimeToLive, UniqueKey,
    UniqueKeyPolicy, VectorDataType, VectorDistanceFunction, VectorEmbedding,
    VectorEmbeddingPolicy,
};
pub use database_properties::DatabaseProperties;
pub use indexing_policy::{
    CompositeIndex, CompositeIndexOrder, CompositeIndexProperty, IndexingMode, IndexingPolicy,
    PropertyPath, SpatialIndex, SpatialType, VectorIndex, VectorIndexType,
};
pub use item_response::ItemResponse;
pub use resource_response::ResourceResponse;
pub use response_body::ResponseBody;
pub use response_headers::ResponseHeaders;
pub use system_properties::SystemProperties;
pub use throughput_properties::ThroughputProperties;
pub use transactional_batch::{
    TransactionalBatch, TransactionalBatchOperationResult, TransactionalBatchResponse,
};

// =========================================================================
// Crate-internal exports
// =========================================================================

pub(crate) use cosmos_response::CosmosResponse;
pub(crate) use response_headers::into_driver_headers;

// =========================================================================
// Internal modules
// =========================================================================

mod batch_response;
mod change_feed_item;
mod container_properties;
mod cosmos_response;
mod database_properties;
mod indexing_policy;
mod item_response;
mod resource_response;
mod response_body;
mod response_headers;
mod system_properties;
mod throughput_properties;
mod transactional_batch;

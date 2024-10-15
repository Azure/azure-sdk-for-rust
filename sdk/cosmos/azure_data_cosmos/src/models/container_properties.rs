// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use std::time::Duration;

use azure_core::Model;
use serde::{Deserialize, Deserializer};

use crate::models::{IndexingPolicy, PartitionKeyDefinition, SystemProperties};

#[cfg(doc)]
use crate::clients::ContainerClientMethods;

fn deserialize_ttl<'de, D>(deserializer: D) -> Result<Option<Duration>, D::Error>
where
    D: Deserializer<'de>,
{
    Ok(Option::<u64>::deserialize(deserializer)?.map(Duration::from_secs))
}

/// Properties of a Cosmos DB container.
///
/// Returned by [`ContainerClient::read()`](crate::clients::ContainerClient::read()).
#[non_exhaustive]
#[derive(Model, Clone, Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ContainerProperties {
    /// The ID of the container.
    pub id: String,

    /// The time-to-live for items in the container.
    ///
    /// For more information see <https://docs.microsoft.com/azure/cosmos-db/time-to-live#time-to-live-configurations>
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_ttl")]
    pub default_ttl: Option<Duration>,

    /// The time-to-live for the analytical store in the container.
    ///
    /// For more information see <https://docs.microsoft.com/azure/cosmos-db/analytical-store-introduction#analytical-ttl>
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_ttl")]
    pub analytical_storage_ttl: Option<Duration>,

    /// The definition of the partition key for the container.
    pub partition_key: PartitionKeyDefinition,

    /// The indexing policy for the container.
    pub indexing_policy: Option<IndexingPolicy>,

    /// The unique key policy for the container.
    pub unique_key_policy: Option<UniqueKeyPolicy>,

    /// The conflict resolution policy for the container.
    pub conflict_resolution_policy: Option<ConflictResolutionPolicy>,

    /// The vector embedding policy for the container.
    pub vector_embedding_policy: Option<VectorEmbeddingPolicy>,

    /// A [`SystemProperties`] object containing common system properties for the container.
    #[serde(flatten)]
    pub system_properties: SystemProperties,
}

/// Represents the vector embedding policy for a container.
#[non_exhaustive]
#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct VectorEmbeddingPolicy {
    /// The [`VectorEmbedding`]s that describe the vector embeddings of items in the container.
    #[serde(rename = "vectorEmbeddings")]
    pub embeddings: Vec<VectorEmbedding>,
}

/// Represents the vector embedding policy for a container.
#[non_exhaustive]
#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct VectorEmbedding {
    /// The path to the property containing the vector.
    pub path: String,

    /// The data type of the elements stored in the vector.
    pub data_type: VectorDataType,

    /// The number of dimensions in the vector.
    pub dimensions: u32,

    /// The [`VectorDistanceFunction`] used to calculate the distance between vectors.
    pub distance_function: VectorDistanceFunction,
}

/// Defines the data types of the elements of a vector.
#[non_exhaustive]
#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum VectorDataType {
    /// Represents the `float16` data type.
    Float16,

    /// Represents the `float32` data type.
    Float32,

    /// Represents the `uint8` data type.
    Uint8,

    /// Represents the `int8` data type.
    Int8,
}

/// Defines the distance functions that can be used to calculate the distance between vectors.
#[non_exhaustive]
#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum VectorDistanceFunction {
    /// Represents the `euclidian` distance function.
    Euclidean,

    /// Represents the `cosine` distance function.
    Cosine,

    /// Represents the `dotproduct` distance function.
    #[serde(rename = "dotproduct")]
    DotProduct,
}

/// Represents a unique key policy for a container.
///
/// For more information see <https://docs.microsoft.com/azure/cosmos-db/unique-keys>
#[non_exhaustive]
#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct UniqueKeyPolicy {
    /// The keys defined in this policy.
    pub unique_keys: Vec<UniqueKey>,
}

/// Represents a single unique key for a container.
#[non_exhaustive]
#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct UniqueKey {
    /// The set of paths which must be unique for each item.
    pub paths: Vec<String>,
}

/// Represents a conflict resolution policy for a container
///
/// For more information, see <https://learn.microsoft.com/en-us/azure/cosmos-db/conflict-resolution-policies>
#[non_exhaustive]
#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ConflictResolutionPolicy {
    /// The conflict resolution mode.
    pub mode: ConflictResolutionMode,

    /// The path within the item to use to perform [`LastWriterWins`](ConflictResolutionMode::LastWriterWins) conflict resolution.
    #[serde(rename = "conflictResolutionPath")]
    pub resolution_path: String,

    /// The stored procedure path to use to perform [`Custom`](ConflictResolutionMode::Custom) conflict resolution.
    #[serde(rename = "conflictResolutionProcedure")]
    pub resolution_procedure: String,
}

/// Defines conflict resolution types available in Azure Cosmos DB
#[non_exhaustive]
#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "PascalCase")]
pub enum ConflictResolutionMode {
    /// Conflict resolution will be performed by using the highest value of the property specified by [`ConflictResolutionPolicy::resolution_path`].
    LastWriterWins,

    /// Conflict resolution will be performed by executing the stored procedure specified by [`ConflictResolutionPolicy::resolution_procedure`].
    Custom,
}

// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use std::{borrow::Cow, time::Duration};

use azure_core::fmt::SafeDebug;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::models::{IndexingPolicy, PartitionKeyDefinition, SystemProperties};

fn deserialize_ttl<'de, D>(deserializer: D) -> Result<Option<Duration>, D::Error>
where
    D: Deserializer<'de>,
{
    Ok(Option::<u64>::deserialize(deserializer)?.map(Duration::from_secs))
}

fn serialize_ttl<S>(duration: &Option<Duration>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match duration {
        Some(d) => serializer.serialize_some(&d.as_secs()),
        None => serializer.serialize_none(),
    }
}

/// Properties of a Cosmos DB container.
///
/// # Constructing
///
/// When constructing this type, you should **always** use [Struct Update] syntax using `..Default::default()`, for example:
///
/// ```rust
/// # use azure_data_cosmos::models::ContainerProperties;
/// let properties = ContainerProperties {
///     id: "NewContainer".into(),
///     partition_key: "/partitionKey".into(),
///     ..Default::default()
/// };
/// ```
///
/// Using this syntax has two purposes:
///
/// 1. It allows you to construct the type even though [`SystemProperties`] is not constructable (these properties should always be empty when you send a request).
/// 2. It protects you if we add additional properties to this struct.
///
/// Also, note that the `id` and `partition_key` values are **required** by the server. You will get an error from the server if you omit them.
///
/// [Struct Update]: https://doc.rust-lang.org/stable/book/ch05-01-defining-structs.html?highlight=Struct#creating-instances-from-other-instances-with-struct-update-syntax
#[derive(Clone, Default, SafeDebug, Deserialize, Serialize, PartialEq, Eq)]
#[safe(true)]
#[serde(rename_all = "camelCase")]
pub struct ContainerProperties {
    /// The ID of the container.
    pub id: Cow<'static, str>,

    /// The definition of the partition key for the container.
    pub partition_key: PartitionKeyDefinition,

    /// The indexing policy for the container.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub indexing_policy: Option<IndexingPolicy>,

    /// The unique key policy for the container.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unique_key_policy: Option<UniqueKeyPolicy>,

    /// The conflict resolution policy for the container.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conflict_resolution_policy: Option<ConflictResolutionPolicy>,

    /// The vector embedding policy for the container.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vector_embedding_policy: Option<VectorEmbeddingPolicy>,

    /// The time-to-live for items in the container.
    ///
    /// For more information see <https://learn.microsoft.com/azure/cosmos-db/time-to-live#time-to-live-configurations>
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(deserialize_with = "deserialize_ttl")]
    #[serde(serialize_with = "serialize_ttl")]
    pub default_ttl: Option<Duration>,

    /// The time-to-live for the analytical store in the container.
    ///
    /// For more information see <https://learn.microsoft.com/azure/cosmos-db/analytical-store-introduction#analytical-ttl>
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(deserialize_with = "deserialize_ttl")]
    #[serde(serialize_with = "serialize_ttl")]
    pub analytical_storage_ttl: Option<Duration>,

    /// A [`SystemProperties`] object containing common system properties for the container.
    #[serde(flatten)]
    pub system_properties: SystemProperties,
}

/// Represents the vector embedding policy for a container.
#[derive(Clone, SafeDebug, Deserialize, Serialize, PartialEq, Eq)]
#[safe(true)]
#[serde(rename_all = "camelCase")]
pub struct VectorEmbeddingPolicy {
    /// The [`VectorEmbedding`]s that describe the vector embeddings of items in the container.
    #[serde(rename = "vectorEmbeddings")]
    pub embeddings: Vec<VectorEmbedding>,
}

/// Represents the vector embedding policy for a container.
#[derive(Clone, SafeDebug, Deserialize, Serialize, PartialEq, Eq)]
#[safe(true)]
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
#[derive(Clone, SafeDebug, Deserialize, Serialize, PartialEq, Eq)]
#[safe(true)]
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
#[derive(Clone, SafeDebug, Deserialize, Serialize, PartialEq, Eq)]
#[safe(true)]
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
/// For more information see <https://learn.microsoft.com/azure/cosmos-db/unique-keys>
#[derive(Clone, SafeDebug, Deserialize, Serialize, PartialEq, Eq)]
#[safe(true)]
#[serde(rename_all = "camelCase")]
pub struct UniqueKeyPolicy {
    /// The keys defined in this policy.
    pub unique_keys: Vec<UniqueKey>,
}

/// Represents a single unique key for a container.
#[derive(Clone, SafeDebug, Deserialize, Serialize, PartialEq, Eq)]
#[safe(true)]
#[serde(rename_all = "camelCase")]
pub struct UniqueKey {
    /// The set of paths which must be unique for each item.
    pub paths: Vec<String>,
}

/// Represents a conflict resolution policy for a container
///
/// For more information, see <https://learn.microsoft.com/en-us/azure/cosmos-db/conflict-resolution-policies>
#[derive(Clone, SafeDebug, Deserialize, Serialize, PartialEq, Eq)]
#[safe(true)]
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
#[derive(Clone, SafeDebug, Deserialize, Serialize, PartialEq, Eq)]
#[safe(true)]
#[serde(rename_all = "PascalCase")]
pub enum ConflictResolutionMode {
    /// Conflict resolution will be performed by using the highest value of the property specified by [`ConflictResolutionPolicy::resolution_path`].
    LastWriterWins,

    /// Conflict resolution will be performed by executing the stored procedure specified by [`ConflictResolutionPolicy::resolution_procedure`].
    Custom,
}

#[cfg(test)]
mod tests {
    use serde::{Deserialize, Serialize};
    use std::time::Duration;

    use crate::models::ContainerProperties;

    #[cfg(test)]
    #[derive(Debug, Deserialize, Serialize)]
    struct DurationHolder {
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(deserialize_with = "super::deserialize_ttl")]
        #[serde(serialize_with = "super::serialize_ttl")]
        pub duration: Option<Duration>,
    }

    #[test]
    pub fn serialize_ttl() {
        let value = DurationHolder {
            duration: Some(Duration::from_secs(4200)),
        };
        let json = serde_json::to_string(&value).unwrap();
        assert_eq!(r#"{"duration":4200}"#, json);
    }

    #[test]
    pub fn serialize_missing_ttl() {
        let value = DurationHolder { duration: None };
        let json = serde_json::to_string(&value).unwrap();
        assert_eq!(r#"{}"#, json);
    }

    #[test]
    pub fn deserialize_ttl() {
        let value: DurationHolder = serde_json::from_str(r#"{"duration":4200}"#).unwrap();
        assert_eq!(Some(Duration::from_secs(4200)), value.duration);
    }

    #[test]
    pub fn deserialize_missing_ttl() {
        let value: DurationHolder = serde_json::from_str(r#"{}"#).unwrap();
        assert_eq!(None, value.duration);
    }

    #[test]
    pub fn deserialize_null_ttl() {
        let value: DurationHolder = serde_json::from_str(r#"{"duration":null}"#).unwrap();
        assert_eq!(None, value.duration);
    }

    #[test]
    pub fn container_properties_default_serialization() {
        // This test asserts that the default value serializes the same way across SDK versions.
        // When new properties are added to ContainerProperties, this test should not break.
        // If it does, users who are using `..Default::default()` syntax will start sending an unexpected payload to the server.
        // In rare cases, it's reasonable to update this test, if the new generated JSON is considered _equivalent_ to the original by the server.
        // But in general, a failure in this test means that the same user code will send an unexpected value in a new version of the SDK.
        let properties = ContainerProperties {
            id: "MyContainer".into(),
            partition_key: "/partitionKey".into(),
            ..Default::default()
        };
        let json = serde_json::to_string(&properties).unwrap();

        assert_eq!(
            "{\"id\":\"MyContainer\",\"partitionKey\":{\"paths\":[\"/partitionKey\"],\"kind\":\"Hash\",\"version\":2}}",
            json
        );
    }
}

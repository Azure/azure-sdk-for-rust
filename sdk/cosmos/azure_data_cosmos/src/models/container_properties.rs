// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use std::{borrow::Cow, time::Duration};

use azure_core::fmt::SafeDebug;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::models::{IndexingPolicy, PartitionKeyDefinition, SystemProperties};

/// Represents the time-to-live configuration for a Cosmos DB container.
///
/// Cosmos DB supports three TTL states:
/// - **Off**: TTL is disabled; items never expire. This is the default.
/// - **NoDefault**: TTL is enabled at the container level, but items have no default expiration.
///   Individual items can still set their own TTL via the `ttl` property.
///   Corresponds to the value `-1` on the wire.
/// - **Seconds**: TTL is enabled with a default expiration. Items expire after the given duration
///   unless they override it with their own `ttl` property.
///
/// For more information see <https://learn.microsoft.com/azure/cosmos-db/time-to-live#time-to-live-configurations>
#[derive(Clone, Default, SafeDebug, PartialEq, Eq)]
#[safe(true)]
#[non_exhaustive]
pub enum TimeToLive {
    /// TTL is disabled; items never expire.
    #[default]
    Off,

    /// TTL is enabled, but items have no default expiration.
    ///
    /// Individual items can still define their own TTL.
    NoDefault,

    /// TTL is enabled with a default expiration of the given duration.
    Seconds(Duration),
}

impl TimeToLive {
    fn is_off(&self) -> bool {
        matches!(self, TimeToLive::Off)
    }
}

impl From<Duration> for TimeToLive {
    fn from(d: Duration) -> Self {
        TimeToLive::Seconds(d)
    }
}

fn deserialize_ttl<'de, D>(deserializer: D) -> Result<TimeToLive, D::Error>
where
    D: Deserializer<'de>,
{
    match Option::<i64>::deserialize(deserializer)? {
        None => Ok(TimeToLive::Off),
        Some(-1) => Ok(TimeToLive::NoDefault),
        Some(n) if n >= 0 => Ok(TimeToLive::Seconds(Duration::from_secs(n as u64))),
        Some(n) => Err(serde::de::Error::invalid_value(
            serde::de::Unexpected::Signed(n),
            &"a non-negative integer or -1",
        )),
    }
}

fn serialize_ttl<S>(ttl: &TimeToLive, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match ttl {
        TimeToLive::Off => serializer.serialize_none(),
        TimeToLive::NoDefault => serializer.serialize_i64(-1),
        TimeToLive::Seconds(d) => serializer.serialize_u64(d.as_secs()),
    }
}

/// Properties of a Cosmos DB container.
///
/// # Constructing
///
/// When constructing this type, use [`ContainerProperties::new()`] with the required values, for example:
///
/// ```rust
/// # use azure_data_cosmos::models::ContainerProperties;
/// let properties = ContainerProperties::new("NewContainer", "/partitionKey".into());
/// ```
///
/// Also, note that the `id` and `partition_key` values are **required** by the server. You will get an error from the server if you omit them.
#[derive(Clone, SafeDebug, Deserialize, Serialize, PartialEq, Eq)]
#[safe(true)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
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
    #[serde(skip_serializing_if = "TimeToLive::is_off")]
    #[serde(deserialize_with = "deserialize_ttl")]
    #[serde(serialize_with = "serialize_ttl")]
    pub default_ttl: TimeToLive,

    /// The time-to-live for the analytical store in the container.
    ///
    /// For more information see <https://learn.microsoft.com/azure/cosmos-db/analytical-store-introduction#analytical-ttl>
    #[serde(default)]
    #[serde(skip_serializing_if = "TimeToLive::is_off")]
    #[serde(deserialize_with = "deserialize_ttl")]
    #[serde(serialize_with = "serialize_ttl")]
    pub analytical_storage_ttl: TimeToLive,

    /// A [`SystemProperties`] object containing common system properties for the container.
    #[serde(flatten)]
    pub system_properties: SystemProperties,
}

impl ContainerProperties {
    pub fn new(id: impl Into<Cow<'static, str>>, partition_key: PartitionKeyDefinition) -> Self {
        Self {
            id: id.into(),
            partition_key,
            indexing_policy: None,
            unique_key_policy: None,
            conflict_resolution_policy: None,
            vector_embedding_policy: None,
            default_ttl: TimeToLive::Off,
            analytical_storage_ttl: TimeToLive::Off,
            system_properties: SystemProperties::default(),
        }
    }

    pub fn with_indexing_policy(mut self, indexing_policy: IndexingPolicy) -> Self {
        self.indexing_policy = Some(indexing_policy);
        self
    }

    pub fn with_unique_key_policy(mut self, unique_key_policy: UniqueKeyPolicy) -> Self {
        self.unique_key_policy = Some(unique_key_policy);
        self
    }

    pub fn with_conflict_resolution_policy(
        mut self,
        conflict_resolution_policy: ConflictResolutionPolicy,
    ) -> Self {
        self.conflict_resolution_policy = Some(conflict_resolution_policy);
        self
    }

    pub fn with_vector_embedding_policy(
        mut self,
        vector_embedding_policy: VectorEmbeddingPolicy,
    ) -> Self {
        self.vector_embedding_policy = Some(vector_embedding_policy);
        self
    }

    pub fn with_default_ttl(mut self, default_ttl: TimeToLive) -> Self {
        self.default_ttl = default_ttl;
        self
    }

    pub fn with_analytical_storage_ttl(mut self, analytical_storage_ttl: TimeToLive) -> Self {
        self.analytical_storage_ttl = analytical_storage_ttl;
        self
    }
}

/// Represents the vector embedding policy for a container.
#[derive(Clone, SafeDebug, Deserialize, Serialize, PartialEq, Eq)]
#[safe(true)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct VectorEmbeddingPolicy {
    /// The [`VectorEmbedding`]s that describe the vector embeddings of items in the container.
    #[serde(rename = "vectorEmbeddings")]
    pub embeddings: Vec<VectorEmbedding>,
}

/// Represents the vector embedding policy for a container.
#[derive(Clone, SafeDebug, Deserialize, Serialize, PartialEq, Eq)]
#[safe(true)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
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
#[non_exhaustive]
pub struct UniqueKeyPolicy {
    /// The keys defined in this policy.
    pub unique_keys: Vec<UniqueKey>,
}

/// Represents a single unique key for a container.
#[derive(Clone, SafeDebug, Deserialize, Serialize, PartialEq, Eq)]
#[safe(true)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
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
#[non_exhaustive]
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

    use super::TimeToLive;
    use crate::models::ContainerProperties;

    #[derive(Debug, Deserialize, Serialize)]
    struct TtlHolder {
        #[serde(default)]
        #[serde(skip_serializing_if = "TimeToLive::is_off")]
        #[serde(deserialize_with = "super::deserialize_ttl")]
        #[serde(serialize_with = "super::serialize_ttl")]
        pub ttl: TimeToLive,
    }

    #[test]
    fn serialize_ttl_seconds() {
        let value = TtlHolder {
            ttl: TimeToLive::Seconds(Duration::from_secs(4200)),
        };
        let json = serde_json::to_string(&value).unwrap();
        assert_eq!(r#"{"ttl":4200}"#, json);
    }

    #[test]
    fn serialize_ttl_off() {
        let value = TtlHolder {
            ttl: TimeToLive::Off,
        };
        let json = serde_json::to_string(&value).unwrap();
        assert_eq!(r#"{}"#, json);
    }

    #[test]
    fn serialize_ttl_no_default() {
        let value = TtlHolder {
            ttl: TimeToLive::NoDefault,
        };
        let json = serde_json::to_string(&value).unwrap();
        assert_eq!(r#"{"ttl":-1}"#, json);
    }

    #[test]
    fn deserialize_ttl_seconds() {
        let value: TtlHolder = serde_json::from_str(r#"{"ttl":4200}"#).unwrap();
        assert_eq!(TimeToLive::Seconds(Duration::from_secs(4200)), value.ttl);
    }

    #[test]
    fn deserialize_ttl_missing() {
        let value: TtlHolder = serde_json::from_str(r#"{}"#).unwrap();
        assert_eq!(TimeToLive::Off, value.ttl);
    }

    #[test]
    fn deserialize_ttl_null() {
        let value: TtlHolder = serde_json::from_str(r#"{"ttl":null}"#).unwrap();
        assert_eq!(TimeToLive::Off, value.ttl);
    }

    #[test]
    fn deserialize_ttl_negative_one() {
        let value: TtlHolder = serde_json::from_str(r#"{"ttl":-1}"#).unwrap();
        assert_eq!(TimeToLive::NoDefault, value.ttl);
    }

    #[test]
    fn deserialize_ttl_zero() {
        let value: TtlHolder = serde_json::from_str(r#"{"ttl":0}"#).unwrap();
        assert_eq!(TimeToLive::Seconds(Duration::ZERO), value.ttl);
    }

    #[test]
    fn deserialize_ttl_invalid_negative() {
        let result = serde_json::from_str::<TtlHolder>(r#"{"ttl":-2}"#);
        assert!(result.is_err());
    }

    #[test]
    pub fn container_properties_default_serialization() {
        // This test asserts that the default value serializes the same way across SDK versions.
        // When new properties are added to ContainerProperties, this test should not break.
        // If it does, users may start sending an unexpected payload to the server.
        // In rare cases, it's reasonable to update this test, if the new generated JSON is considered _equivalent_ to the original by the server.
        // But in general, a failure in this test means that the same user code will send an unexpected value in a new version of the SDK.
        let properties = ContainerProperties::new("MyContainer", "/partitionKey".into());
        let json = serde_json::to_string(&properties).unwrap();

        assert_eq!(
            "{\"id\":\"MyContainer\",\"partitionKey\":{\"paths\":[\"/partitionKey\"],\"kind\":\"Hash\",\"version\":2}}",
            json
        );
    }
}

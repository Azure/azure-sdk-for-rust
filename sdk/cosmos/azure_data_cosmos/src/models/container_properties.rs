// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use std::borrow::Cow;

use azure_core::fmt::SafeDebug;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::models::{IndexingPolicy, PartitionKeyDefinition, SystemProperties};

/// Represents the time-to-live configuration for a Cosmos DB container.
///
/// Cosmos DB supports three TTL states:
/// - **Forever**: TTL is disabled; items never expire. This is the default.
/// - **NoDefault**: TTL is enabled at the container level, but items have no default expiration.
///   Individual items can still set their own TTL via the `ttl` property.
///   Corresponds to the value `-1` on the wire.
/// - **Seconds**: TTL is enabled with a default expiration in seconds. Items expire after the given
///   number of seconds unless they override it with their own `ttl` property.
///
/// For more information see <https://learn.microsoft.com/azure/cosmos-db/time-to-live#time-to-live-configurations>
#[derive(Clone, Default, SafeDebug, PartialEq, Eq)]
#[safe(true)]
#[non_exhaustive]
pub enum TimeToLive {
    /// TTL is disabled; items never expire.
    #[default]
    Forever,

    /// TTL is enabled, but items have no default expiration.
    ///
    /// Individual items can still define their own TTL.
    NoDefault,

    /// TTL is enabled with a default expiration of the given number of seconds.
    Seconds(u32),
}

impl TimeToLive {
    /// Returns `true` if TTL is [`Forever`](TimeToLive::Forever).
    pub fn is_forever(&self) -> bool {
        matches!(self, TimeToLive::Forever)
    }
}

impl From<u32> for TimeToLive {
    fn from(n: u32) -> Self {
        TimeToLive::Seconds(n)
    }
}

impl Serialize for TimeToLive {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            TimeToLive::Forever => serializer.serialize_none(),
            TimeToLive::NoDefault => serializer.serialize_i32(-1),
            TimeToLive::Seconds(n) => serializer.serialize_u32(*n),
        }
    }
}

impl<'de> Deserialize<'de> for TimeToLive {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        match Option::<i32>::deserialize(deserializer)? {
            None => Ok(TimeToLive::Forever),
            Some(-1) => Ok(TimeToLive::NoDefault),
            Some(n) if n > 0 => Ok(TimeToLive::Seconds(n as u32)),
            Some(n) => Err(serde::de::Error::invalid_value(
                serde::de::Unexpected::Signed(n as i64),
                &"a nonzero positive integer or -1",
            )),
        }
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
    #[serde(skip_serializing_if = "TimeToLive::is_forever")]
    pub default_ttl: TimeToLive,

    /// The time-to-live for the analytical store in the container.
    ///
    /// For more information see <https://learn.microsoft.com/azure/cosmos-db/analytical-store-introduction#analytical-ttl>
    #[serde(default)]
    #[serde(skip_serializing_if = "TimeToLive::is_forever")]
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
            default_ttl: TimeToLive::Forever,
            analytical_storage_ttl: TimeToLive::Forever,
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

    pub fn with_default_ttl(mut self, default_ttl: impl Into<TimeToLive>) -> Self {
        self.default_ttl = default_ttl.into();
        self
    }

    pub fn with_analytical_storage_ttl(
        mut self,
        analytical_storage_ttl: impl Into<TimeToLive>,
    ) -> Self {
        self.analytical_storage_ttl = analytical_storage_ttl.into();
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

    use super::TimeToLive;
    use crate::models::ContainerProperties;

    #[derive(Debug, Deserialize, Serialize)]
    struct TtlHolder {
        #[serde(default)]
        #[serde(skip_serializing_if = "TimeToLive::is_forever")]
        pub ttl: TimeToLive,
    }

    #[test]
    fn serialize_ttl_seconds() {
        let value = TtlHolder {
            ttl: TimeToLive::Seconds(4200),
        };
        let json = serde_json::to_string(&value).unwrap();
        assert_eq!(r#"{"ttl":4200}"#, json);
    }

    #[test]
    fn serialize_ttl_forever() {
        let value = TtlHolder {
            ttl: TimeToLive::Forever,
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
        assert_eq!(TimeToLive::Seconds(4200), value.ttl);
    }

    #[test]
    fn deserialize_ttl_missing() {
        let value: TtlHolder = serde_json::from_str(r#"{}"#).unwrap();
        assert_eq!(TimeToLive::Forever, value.ttl);
    }

    #[test]
    fn deserialize_ttl_null() {
        let value: TtlHolder = serde_json::from_str(r#"{"ttl":null}"#).unwrap();
        assert_eq!(TimeToLive::Forever, value.ttl);
    }

    #[test]
    fn deserialize_ttl_negative_one() {
        let value: TtlHolder = serde_json::from_str(r#"{"ttl":-1}"#).unwrap();
        assert_eq!(TimeToLive::NoDefault, value.ttl);
    }

    #[test]
    fn deserialize_ttl_zero() {
        let result = serde_json::from_str::<TtlHolder>(r#"{"ttl":0}"#);
        assert!(result.is_err());
    }

    #[test]
    fn deserialize_ttl_invalid_negative() {
        let result = serde_json::from_str::<TtlHolder>(r#"{"ttl":-2}"#);
        assert!(result.is_err());
    }

    #[test]
    fn deserialize_ttl_overflow() {
        let result = serde_json::from_str::<TtlHolder>(r#"{"ttl":2147483648}"#);
        assert!(result.is_err());
    }

    #[test]
    fn serialize_ttl_seconds_value() {
        let json = serde_json::to_string(&TimeToLive::Seconds(86400)).unwrap();
        assert_eq!("86400", json);
    }

    #[test]
    fn serialize_ttl_no_default_value() {
        let json = serde_json::to_string(&TimeToLive::NoDefault).unwrap();
        assert_eq!("-1", json);
    }

    #[test]
    fn serialize_ttl_forever_value() {
        let json = serde_json::to_string(&TimeToLive::Forever).unwrap();
        assert_eq!("null", json);
    }

    #[test]
    fn deserialize_container_properties_with_ttl_negative_one() {
        let json = r#"{
            "id": "MyContainer",
            "partitionKey": {"paths": ["/pk"], "kind": "Hash", "version": 2},
            "defaultTtl": -1
        }"#;
        let props: ContainerProperties = serde_json::from_str(json).unwrap();
        assert_eq!(TimeToLive::NoDefault, props.default_ttl);
        assert_eq!(TimeToLive::Forever, props.analytical_storage_ttl);
    }

    #[test]
    fn deserialize_container_properties_with_ttl_seconds() {
        let json = r#"{
            "id": "MyContainer",
            "partitionKey": {"paths": ["/pk"], "kind": "Hash", "version": 2},
            "defaultTtl": 3600,
            "analyticalStorageTtl": -1
        }"#;
        let props: ContainerProperties = serde_json::from_str(json).unwrap();
        assert_eq!(TimeToLive::Seconds(3600), props.default_ttl);
        assert_eq!(TimeToLive::NoDefault, props.analytical_storage_ttl);
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

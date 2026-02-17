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
/// # Required fields
///
/// * `id` — The unique identifier for the container.
/// * `partition_key` — The partition key definition.
///
/// Use [`ContainerProperties::new()`] to construct an instance:
///
/// ```rust
/// # use azure_data_cosmos::models::ContainerProperties;
/// let properties = ContainerProperties::new("NewContainer", "/partitionKey");
/// ```
#[non_exhaustive]
#[derive(Clone, SafeDebug, Deserialize, Serialize, PartialEq, Eq)]
#[safe(true)]
#[serde(rename_all = "camelCase")]
pub struct ContainerProperties {
    /// The ID of the container.
    id: Cow<'static, str>,

    /// The definition of the partition key for the container.
    partition_key: PartitionKeyDefinition,

    /// The indexing policy for the container.
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    indexing_policy: Option<IndexingPolicy>,

    /// The unique key policy for the container.
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    unique_key_policy: Option<UniqueKeyPolicy>,

    /// The conflict resolution policy for the container.
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    conflict_resolution_policy: Option<ConflictResolutionPolicy>,

    /// The vector embedding policy for the container.
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    vector_embedding_policy: Option<VectorEmbeddingPolicy>,

    /// The time-to-live for items in the container.
    ///
    /// For more information see <https://learn.microsoft.com/azure/cosmos-db/time-to-live#time-to-live-configurations>
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(deserialize_with = "deserialize_ttl")]
    #[serde(serialize_with = "serialize_ttl")]
    default_ttl: Option<Duration>,

    /// The time-to-live for the analytical store in the container.
    ///
    /// For more information see <https://learn.microsoft.com/azure/cosmos-db/analytical-store-introduction#analytical-ttl>
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(deserialize_with = "deserialize_ttl")]
    #[serde(serialize_with = "serialize_ttl")]
    analytical_storage_ttl: Option<Duration>,

    /// A [`SystemProperties`] object containing common system properties for the container.
    #[serde(flatten)]
    #[serde(default)]
    system_properties: SystemProperties,
}

impl ContainerProperties {
    /// Creates a new [`ContainerProperties`] with the required `id` and `partition_key` fields.
    ///
    /// Optional fields can be set via `with_*` builder methods:
    ///
    /// ```rust
    /// # use azure_data_cosmos::models::ContainerProperties;
    /// # use std::time::Duration;
    /// let properties = ContainerProperties::new("MyContainer", "/partitionKey")
    ///     .with_default_ttl(Duration::from_secs(3600));
    /// ```
    pub fn new(
        id: impl Into<Cow<'static, str>>,
        partition_key: impl Into<PartitionKeyDefinition>,
    ) -> Self {
        Self {
            id: id.into(),
            partition_key: partition_key.into(),
            indexing_policy: None,
            unique_key_policy: None,
            conflict_resolution_policy: None,
            vector_embedding_policy: None,
            default_ttl: None,
            analytical_storage_ttl: None,
            system_properties: SystemProperties::default(),
        }
    }

    /// Gets the ID of the container.
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Gets the partition key definition for the container.
    pub fn partition_key(&self) -> &PartitionKeyDefinition {
        &self.partition_key
    }

    /// Gets the indexing policy for the container.
    pub fn indexing_policy(&self) -> Option<&IndexingPolicy> {
        self.indexing_policy.as_ref()
    }

    /// Sets the indexing policy.
    pub fn with_indexing_policy(
        mut self,
        indexing_policy: impl Into<Option<IndexingPolicy>>,
    ) -> Self {
        self.indexing_policy = indexing_policy.into();
        self
    }

    /// Gets the unique key policy for the container.
    pub fn unique_key_policy(&self) -> Option<&UniqueKeyPolicy> {
        self.unique_key_policy.as_ref()
    }

    /// Sets the unique key policy.
    pub fn with_unique_key_policy(
        mut self,
        unique_key_policy: impl Into<Option<UniqueKeyPolicy>>,
    ) -> Self {
        self.unique_key_policy = unique_key_policy.into();
        self
    }

    /// Gets the conflict resolution policy for the container.
    pub fn conflict_resolution_policy(&self) -> Option<&ConflictResolutionPolicy> {
        self.conflict_resolution_policy.as_ref()
    }

    /// Sets the conflict resolution policy.
    pub fn with_conflict_resolution_policy(
        mut self,
        conflict_resolution_policy: impl Into<Option<ConflictResolutionPolicy>>,
    ) -> Self {
        self.conflict_resolution_policy = conflict_resolution_policy.into();
        self
    }

    /// Gets the vector embedding policy for the container.
    pub fn vector_embedding_policy(&self) -> Option<&VectorEmbeddingPolicy> {
        self.vector_embedding_policy.as_ref()
    }

    /// Sets the vector embedding policy.
    pub fn with_vector_embedding_policy(
        mut self,
        vector_embedding_policy: impl Into<Option<VectorEmbeddingPolicy>>,
    ) -> Self {
        self.vector_embedding_policy = vector_embedding_policy.into();
        self
    }

    /// Gets the default time-to-live for items in the container.
    pub fn default_ttl(&self) -> Option<Duration> {
        self.default_ttl
    }

    /// Sets the default time-to-live.
    pub fn with_default_ttl(mut self, default_ttl: impl Into<Option<Duration>>) -> Self {
        self.default_ttl = default_ttl.into();
        self
    }

    /// Gets the analytical store time-to-live.
    pub fn analytical_storage_ttl(&self) -> Option<Duration> {
        self.analytical_storage_ttl
    }

    /// Sets the analytical store time-to-live.
    pub fn with_analytical_storage_ttl(
        mut self,
        analytical_storage_ttl: impl Into<Option<Duration>>,
    ) -> Self {
        self.analytical_storage_ttl = analytical_storage_ttl.into();
        self
    }

    /// Gets the system properties for the container.
    pub fn system_properties(&self) -> &SystemProperties {
        &self.system_properties
    }
}

/// Represents the vector embedding policy for a container.
#[non_exhaustive]
#[derive(Clone, Default, SafeDebug, Deserialize, Serialize, PartialEq, Eq)]
#[safe(true)]
#[serde(rename_all = "camelCase")]
pub struct VectorEmbeddingPolicy {
    /// The [`VectorEmbedding`]s that describe the vector embeddings of items in the container.
    #[serde(rename = "vectorEmbeddings")]
    embeddings: Vec<VectorEmbedding>,
}

impl VectorEmbeddingPolicy {
    /// Gets the vector embeddings.
    pub fn embeddings(&self) -> &[VectorEmbedding] {
        &self.embeddings
    }

    /// Sets the vector embeddings.
    pub fn with_embeddings(mut self, embeddings: Vec<VectorEmbedding>) -> Self {
        self.embeddings = embeddings;
        self
    }
}

/// Represents a single vector embedding definition for a container.
///
/// # Required fields
///
/// * `path` — The path to the property containing the vector.
/// * `dimensions` — The number of dimensions in the vector.
///
/// Use [`VectorEmbedding::new()`] to construct an instance:
///
/// ```rust
/// # use azure_data_cosmos::models::VectorEmbedding;
/// let embedding = VectorEmbedding::new("/vector", 1536);
/// ```
#[non_exhaustive]
#[derive(Clone, SafeDebug, Deserialize, Serialize, PartialEq, Eq)]
#[safe(true)]
#[serde(rename_all = "camelCase")]
pub struct VectorEmbedding {
    /// The path to the property containing the vector.
    path: String,

    /// The data type of the elements stored in the vector.
    #[serde(default)]
    data_type: VectorDataType,

    /// The number of dimensions in the vector.
    dimensions: u32,

    /// The [`VectorDistanceFunction`] used to calculate the distance between vectors.
    #[serde(default)]
    distance_function: VectorDistanceFunction,
}

impl VectorEmbedding {
    /// Creates a new [`VectorEmbedding`] with the required `path` and `dimensions` fields.
    ///
    /// The `data_type` defaults to [`VectorDataType::Float32`] and the `distance_function` defaults
    /// to [`VectorDistanceFunction::Cosine`]. Use the `with_*` builder methods to override them.
    pub fn new(path: impl Into<String>, dimensions: u32) -> Self {
        Self {
            path: path.into(),
            data_type: VectorDataType::default(),
            dimensions,
            distance_function: VectorDistanceFunction::default(),
        }
    }

    /// Gets the path to the property containing the vector.
    pub fn path(&self) -> &str {
        &self.path
    }

    /// Gets the data type of the vector elements.
    pub fn data_type(&self) -> &VectorDataType {
        &self.data_type
    }

    /// Sets the data type.
    pub fn with_data_type(mut self, data_type: VectorDataType) -> Self {
        self.data_type = data_type;
        self
    }

    /// Gets the number of dimensions in the vector.
    pub fn dimensions(&self) -> u32 {
        self.dimensions
    }

    /// Gets the distance function used for this vector.
    pub fn distance_function(&self) -> &VectorDistanceFunction {
        &self.distance_function
    }

    /// Sets the distance function.
    pub fn with_distance_function(mut self, distance_function: VectorDistanceFunction) -> Self {
        self.distance_function = distance_function;
        self
    }
}

/// Defines the data types of the elements of a vector.
#[derive(Clone, Default, SafeDebug, Deserialize, Serialize, PartialEq, Eq)]
#[safe(true)]
#[serde(rename_all = "camelCase")]
pub enum VectorDataType {
    /// Represents the `float16` data type.
    Float16,

    /// Represents the `float32` data type.
    #[default]
    Float32,

    /// Represents the `uint8` data type.
    Uint8,

    /// Represents the `int8` data type.
    Int8,
}

/// Defines the distance functions that can be used to calculate the distance between vectors.
#[derive(Clone, Default, SafeDebug, Deserialize, Serialize, PartialEq, Eq)]
#[safe(true)]
#[serde(rename_all = "camelCase")]
pub enum VectorDistanceFunction {
    /// Represents the `euclidian` distance function.
    Euclidean,

    /// Represents the `cosine` distance function.
    #[default]
    Cosine,

    /// Represents the `dotproduct` distance function.
    #[serde(rename = "dotproduct")]
    DotProduct,
}

/// Represents a unique key policy for a container.
///
/// For more information see <https://learn.microsoft.com/azure/cosmos-db/unique-keys>
#[non_exhaustive]
#[derive(Clone, Default, SafeDebug, Deserialize, Serialize, PartialEq, Eq)]
#[safe(true)]
#[serde(rename_all = "camelCase")]
pub struct UniqueKeyPolicy {
    /// The keys defined in this policy.
    unique_keys: Vec<UniqueKey>,
}

impl UniqueKeyPolicy {
    /// Gets the unique keys defined in this policy.
    pub fn unique_keys(&self) -> &[UniqueKey] {
        &self.unique_keys
    }

    /// Sets the unique keys.
    pub fn with_unique_keys(mut self, unique_keys: Vec<UniqueKey>) -> Self {
        self.unique_keys = unique_keys;
        self
    }
}

/// Represents a single unique key for a container.
///
/// # Required fields
///
/// * `paths` — The set of paths which must be unique for each item.
///
/// Use [`UniqueKey::new()`] to construct an instance:
///
/// ```rust
/// # use azure_data_cosmos::models::UniqueKey;
/// let key = UniqueKey::new(vec!["/email".to_string()]);
/// ```
#[non_exhaustive]
#[derive(Clone, SafeDebug, Deserialize, Serialize, PartialEq, Eq)]
#[safe(true)]
#[serde(rename_all = "camelCase")]
pub struct UniqueKey {
    /// The set of paths which must be unique for each item.
    paths: Vec<String>,
}

impl UniqueKey {
    /// Creates a new [`UniqueKey`] with the required `paths` field.
    pub fn new(paths: Vec<String>) -> Self {
        Self { paths }
    }

    /// Gets the set of paths which must be unique for each item.
    pub fn paths(&self) -> &[String] {
        &self.paths
    }
}

/// Represents a conflict resolution policy for a container.
///
/// For more information, see <https://learn.microsoft.com/en-us/azure/cosmos-db/conflict-resolution-policies>
#[non_exhaustive]
#[derive(Clone, Default, SafeDebug, Deserialize, Serialize, PartialEq, Eq)]
#[safe(true)]
#[serde(rename_all = "camelCase")]
pub struct ConflictResolutionPolicy {
    /// The conflict resolution mode.
    mode: ConflictResolutionMode,

    /// The path within the item to use to perform [`LastWriterWins`](ConflictResolutionMode::LastWriterWins) conflict resolution.
    #[serde(rename = "conflictResolutionPath")]
    resolution_path: String,

    /// The stored procedure path to use to perform [`Custom`](ConflictResolutionMode::Custom) conflict resolution.
    #[serde(rename = "conflictResolutionProcedure")]
    resolution_procedure: String,
}

impl ConflictResolutionPolicy {
    /// Gets the conflict resolution mode.
    pub fn mode(&self) -> &ConflictResolutionMode {
        &self.mode
    }

    /// Sets the conflict resolution mode.
    pub fn with_mode(mut self, mode: ConflictResolutionMode) -> Self {
        self.mode = mode;
        self
    }

    /// Gets the resolution path.
    pub fn resolution_path(&self) -> &str {
        &self.resolution_path
    }

    /// Sets the resolution path.
    pub fn with_resolution_path(mut self, resolution_path: impl Into<String>) -> Self {
        self.resolution_path = resolution_path.into();
        self
    }

    /// Gets the resolution procedure.
    pub fn resolution_procedure(&self) -> &str {
        &self.resolution_procedure
    }

    /// Sets the resolution procedure.
    pub fn with_resolution_procedure(mut self, resolution_procedure: impl Into<String>) -> Self {
        self.resolution_procedure = resolution_procedure.into();
        self
    }
}

/// Defines conflict resolution types available in Azure Cosmos DB.
#[derive(Clone, Default, SafeDebug, Deserialize, Serialize, PartialEq, Eq)]
#[safe(true)]
#[serde(rename_all = "PascalCase")]
pub enum ConflictResolutionMode {
    /// Conflict resolution will be performed by using the highest value of the property specified by [`ConflictResolutionPolicy::resolution_path()`].
    #[default]
    LastWriterWins,

    /// Conflict resolution will be performed by executing the stored procedure specified by [`ConflictResolutionPolicy::resolution_procedure()`].
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
        let properties = ContainerProperties::new("MyContainer", "/partitionKey");
        let json = serde_json::to_string(&properties).unwrap();

        assert_eq!(
            "{\"id\":\"MyContainer\",\"partitionKey\":{\"paths\":[\"/partitionKey\"],\"kind\":\"Hash\",\"version\":2}}",
            json
        );
    }
}

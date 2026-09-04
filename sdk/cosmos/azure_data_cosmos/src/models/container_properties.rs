// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use std::{borrow::Cow, collections::BTreeMap, time::Duration};

use azure_core::fmt::SafeDebug;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::{
    models::PartitionKeyDefinition,
    models::{IndexingPolicy, SystemProperties},
};

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

    /// The full text policy for the container.
    ///
    /// Declares which paths hold full text content and how that text is
    /// analyzed. Paths listed in
    /// [`IndexingPolicy::full_text_indexes`](crate::models::IndexingPolicy::full_text_indexes)
    /// must also appear here.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub full_text_policy: Option<FullTextPolicy>,

    /// The change feed policy for the container.
    ///
    /// Configure a retention duration here to enable the
    /// [`AllVersionsAndDeletes`](crate::options::ChangeFeedMode::AllVersionsAndDeletes)
    /// change feed mode. Without it, only the default
    /// [`LatestVersion`](crate::options::ChangeFeedMode::LatestVersion) mode is
    /// available.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub change_feed_policy: Option<ChangeFeedPolicy>,

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

    /// Properties returned by the service that this version of the SDK does not model.
    ///
    /// Captured on deserialization and written back on serialization so a
    /// read-modify-replace round trip does not silently drop server-side
    /// configuration the SDK doesn't know about yet.
    #[safe(false)]
    #[serde(flatten)]
    extra: BTreeMap<String, serde_json::Value>,
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
            full_text_policy: None,
            change_feed_policy: None,
            default_ttl: TimeToLive::Forever,
            analytical_storage_ttl: TimeToLive::Forever,
            system_properties: SystemProperties::default(),
            extra: BTreeMap::new(),
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

    /// Sets the full text policy for the container.
    pub fn with_full_text_policy(mut self, full_text_policy: FullTextPolicy) -> Self {
        self.full_text_policy = Some(full_text_policy);
        self
    }

    /// Sets the change feed policy for the container.
    ///
    /// Use [`ChangeFeedPolicy::with_retention_duration`] to enable the
    /// [`AllVersionsAndDeletes`](crate::options::ChangeFeedMode::AllVersionsAndDeletes)
    /// change feed mode with a retention window.
    pub fn with_change_feed_policy(mut self, change_feed_policy: ChangeFeedPolicy) -> Self {
        self.change_feed_policy = Some(change_feed_policy);
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
#[derive(Clone, Default, SafeDebug, Deserialize, Serialize, PartialEq, Eq)]
#[safe(true)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct VectorEmbeddingPolicy {
    /// The [`VectorEmbedding`]s that describe the vector embeddings of items in the container.
    #[serde(rename = "vectorEmbeddings")]
    pub embeddings: Vec<VectorEmbedding>,

    /// Properties returned by the service that this version of the SDK does not model.
    ///
    /// Captured on deserialization and written back on serialization so a
    /// read-modify-replace round trip does not silently drop server-side
    /// configuration the SDK doesn't know about yet.
    #[safe(false)]
    #[serde(flatten)]
    extra: BTreeMap<String, serde_json::Value>,
}

impl VectorEmbeddingPolicy {
    /// Appends `embedding` to the policy's list of embeddings.
    pub fn with_embedding(mut self, embedding: VectorEmbedding) -> Self {
        self.embeddings.push(embedding);
        self
    }
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

    /// Properties returned by the service that this version of the SDK does not model.
    ///
    /// Captured on deserialization and written back on serialization so a
    /// read-modify-replace round trip does not silently drop server-side
    /// configuration the SDK doesn't know about yet.
    #[safe(false)]
    #[serde(flatten)]
    extra: BTreeMap<String, serde_json::Value>,
}

impl VectorEmbedding {
    /// Creates a new [`VectorEmbedding`] with the given path, data type, dimensions, and distance function.
    pub fn new(
        path: impl Into<String>,
        data_type: VectorDataType,
        dimensions: u32,
        distance_function: VectorDistanceFunction,
    ) -> Self {
        Self {
            path: path.into(),
            data_type,
            dimensions,
            distance_function,
            extra: BTreeMap::new(),
        }
    }

    /// Sets the path of this embedding.
    pub fn with_path(mut self, path: impl Into<String>) -> Self {
        self.path = path.into();
        self
    }

    /// Sets the data type of this embedding.
    pub fn with_data_type(mut self, data_type: VectorDataType) -> Self {
        self.data_type = data_type;
        self
    }

    /// Sets the number of dimensions of this embedding.
    pub fn with_dimensions(mut self, dimensions: u32) -> Self {
        self.dimensions = dimensions;
        self
    }

    /// Sets the distance function used by this embedding.
    pub fn with_distance_function(mut self, distance_function: VectorDistanceFunction) -> Self {
        self.distance_function = distance_function;
        self
    }
}

/// Defines the data types of the elements of a vector.
#[derive(Clone, SafeDebug, Deserialize, Serialize, PartialEq, Eq)]
#[safe(true)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
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
#[non_exhaustive]
pub enum VectorDistanceFunction {
    /// Represents the `euclidian` distance function.
    Euclidean,

    /// Represents the `cosine` distance function.
    Cosine,

    /// Represents the `dotproduct` distance function.
    #[serde(rename = "dotproduct")]
    DotProduct,
}

/// Represents the full text policy for a container.
///
/// Declares which paths hold full text content and which language each one is
/// analyzed as. A path must be declared here before it can be indexed via
/// [`IndexingPolicy::full_text_indexes`](crate::models::IndexingPolicy::full_text_indexes)
/// or queried with the full text system functions.
///
/// For more information, see <https://learn.microsoft.com/azure/cosmos-db/gen-ai/full-text-search>
#[derive(Clone, Default, SafeDebug, Deserialize, Serialize, PartialEq, Eq)]
#[safe(true)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct FullTextPolicy {
    /// The optional language used to analyze any [`FullTextPath`] that does not specify its own.
    ///
    /// This is a language tag such as `en-US`. The service validates the value
    /// and rejects unsupported languages; this type does not enforce the
    /// supported set, leaving the service as the source of truth.
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_language: Option<String>,

    /// The paths holding full text content in items in the container.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub full_text_paths: Vec<FullTextPath>,

    /// Properties returned by the service that this version of the SDK does not model.
    ///
    /// Captured on deserialization and written back on serialization so a
    /// read-modify-replace round trip does not silently drop server-side
    /// configuration the SDK doesn't know about yet.
    #[safe(false)]
    #[serde(flatten)]
    extra: BTreeMap<String, serde_json::Value>,
}

impl FullTextPolicy {
    /// Creates a new [`FullTextPolicy`] with the given default language and no paths.
    pub fn new(default_language: impl Into<String>) -> Self {
        Self {
            default_language: Some(default_language.into()),
            full_text_paths: Vec::new(),
            extra: BTreeMap::new(),
        }
    }

    /// Sets the language used to analyze any [`FullTextPath`] that does not specify its own.
    pub fn with_default_language(mut self, default_language: impl Into<String>) -> Self {
        self.default_language = Some(default_language.into());
        self
    }

    /// Appends `full_text_path` to the policy's list of full text paths.
    pub fn with_full_text_path(mut self, full_text_path: impl Into<FullTextPath>) -> Self {
        self.full_text_paths.push(full_text_path.into());
        self
    }
}

/// Describes a single path holding full text content.
#[derive(Clone, Default, SafeDebug, Deserialize, Serialize, PartialEq, Eq)]
#[safe(true)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct FullTextPath {
    /// The path to the property containing the text.
    pub path: String,

    /// The language this path is analyzed as.
    ///
    /// When unset, the containing policy's
    /// [`default_language`](FullTextPolicy::default_language) is used.
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,

    /// Properties returned by the service that this version of the SDK does not model.
    ///
    /// Captured on deserialization and written back on serialization so a
    /// read-modify-replace round trip does not silently drop server-side
    /// configuration the SDK doesn't know about yet.
    #[safe(false)]
    #[serde(flatten)]
    extra: BTreeMap<String, serde_json::Value>,
}

impl FullTextPath {
    /// Creates a new [`FullTextPath`] over the given path, inheriting the policy's default language.
    pub fn new(path: impl Into<String>) -> Self {
        Self {
            path: path.into(),
            language: None,
            extra: BTreeMap::new(),
        }
    }

    /// Sets the path of this full text path.
    pub fn with_path(mut self, path: impl Into<String>) -> Self {
        self.path = path.into();
        self
    }

    /// Sets the language this path is analyzed as, overriding the policy's default.
    pub fn with_language(mut self, language: impl Into<String>) -> Self {
        self.language = Some(language.into());
        self
    }
}

impl<T: Into<String>> From<T> for FullTextPath {
    fn from(value: T) -> Self {
        FullTextPath::new(value)
    }
}

/// Represents a unique key policy for a container.
///
/// For more information see <https://learn.microsoft.com/azure/cosmos-db/unique-keys>
#[derive(Clone, Default, SafeDebug, Deserialize, Serialize, PartialEq, Eq)]
#[safe(true)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct UniqueKeyPolicy {
    /// The keys defined in this policy.
    pub unique_keys: Vec<UniqueKey>,
}

impl UniqueKeyPolicy {
    /// Appends `unique_key` to the policy's list of unique keys.
    pub fn with_unique_key(mut self, unique_key: UniqueKey) -> Self {
        self.unique_keys.push(unique_key);
        self
    }
}

/// Represents a single unique key for a container.
#[derive(Clone, Default, SafeDebug, Deserialize, Serialize, PartialEq, Eq)]
#[safe(true)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct UniqueKey {
    /// The set of paths which must be unique for each item.
    pub paths: Vec<String>,
}

impl UniqueKey {
    /// Appends `path` to the unique key's list of paths.
    pub fn with_path(mut self, path: impl Into<String>) -> Self {
        self.paths.push(path.into());
        self
    }
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

impl ConflictResolutionPolicy {
    /// Creates a new [`ConflictResolutionPolicy`] with the given conflict resolution mode.
    ///
    /// `resolution_path` and `resolution_procedure` are initialized to empty strings; set
    /// the field appropriate for the chosen mode via [`with_resolution_path`](Self::with_resolution_path)
    /// or [`with_resolution_procedure`](Self::with_resolution_procedure).
    pub fn new(mode: ConflictResolutionMode) -> Self {
        Self {
            mode,
            resolution_path: String::new(),
            resolution_procedure: String::new(),
        }
    }

    /// Sets the path within the item used to resolve [`LastWriterWins`](ConflictResolutionMode::LastWriterWins) conflicts.
    pub fn with_resolution_path(mut self, resolution_path: impl Into<String>) -> Self {
        self.resolution_path = resolution_path.into();
        self
    }

    /// Sets the stored procedure path used to resolve [`Custom`](ConflictResolutionMode::Custom) conflicts.
    pub fn with_resolution_procedure(mut self, resolution_procedure: impl Into<String>) -> Self {
        self.resolution_procedure = resolution_procedure.into();
        self
    }
}

/// Defines conflict resolution types available in Azure Cosmos DB
#[derive(Clone, SafeDebug, Deserialize, Serialize, PartialEq, Eq)]
#[safe(true)]
#[serde(rename_all = "PascalCase")]
#[non_exhaustive]
pub enum ConflictResolutionMode {
    /// Conflict resolution will be performed by using the highest value of the property specified by [`ConflictResolutionPolicy::resolution_path`].
    LastWriterWins,

    /// Conflict resolution will be performed by executing the stored procedure specified by [`ConflictResolutionPolicy::resolution_procedure`].
    Custom,
}

/// The change feed policy for a container.
///
/// Configuring a retention duration enables the
/// [`AllVersionsAndDeletes`](crate::options::ChangeFeedMode::AllVersionsAndDeletes)
/// change feed mode: intermediate versions and deletes are
/// retained for the configured window so they can be read back. Without a
/// retention duration only the default
/// [`LatestVersion`](crate::options::ChangeFeedMode::LatestVersion) mode is
/// available.
///
/// The retention window has minute granularity. On the service it must fall
/// within the supported range (currently 1 hour to 30 days) when this mode
/// is enabled; this type does not enforce that range, leaving the service as the
/// source of truth.
///
/// For more information see <https://learn.microsoft.com/azure/cosmos-db/nosql/change-feed-modes>
#[derive(Clone, Default, SafeDebug, Deserialize, Serialize, PartialEq, Eq)]
#[safe(true)]
#[non_exhaustive]
pub struct ChangeFeedPolicy {
    /// The all versions and deletes retention window.
    ///
    /// `None` (the default) disables all versions and deletes, leaving only
    /// `LatestVersion` reads available. On the wire this maps to a whole number
    /// of minutes, where `0` means "no retention"; that conversion happens in
    /// [`serialize_retention_minutes`] and [`deserialize_retention_minutes`].
    #[serde(
        rename = "retentionDuration",
        default,
        serialize_with = "serialize_retention_minutes",
        deserialize_with = "deserialize_retention_minutes"
    )]
    retention_duration: Option<Duration>,
}

/// Serializes the retention window as the whole number of minutes the service
/// expects, where `0` means "no retention".
///
/// A `None` window serializes to `0`. Any non-zero [`Duration`] is rounded
/// **up** to the next whole minute so a sub-minute window never truncates to
/// `0` (which the service reads as "disabled").
fn serialize_retention_minutes<S>(
    value: &Option<Duration>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let minutes = value
        .map(|retention| retention.as_secs().div_ceil(60).min(i32::MAX as u64) as i32)
        .unwrap_or(0);
    serializer.serialize_i32(minutes)
}

/// Deserializes the retention window from the whole number of minutes on the
/// wire, where `0` (or any non-positive value from a malformed payload) means
/// "no retention" and maps to `None`.
fn deserialize_retention_minutes<'de, D>(deserializer: D) -> Result<Option<Duration>, D::Error>
where
    D: Deserializer<'de>,
{
    let minutes = i32::deserialize(deserializer)?;
    Ok((minutes > 0).then(|| Duration::from_secs(minutes as u64 * 60)))
}

impl ChangeFeedPolicy {
    /// Sets the all versions and deletes retention window, enabling the
    /// [`AllVersionsAndDeletes`](crate::options::ChangeFeedMode::AllVersionsAndDeletes)
    /// change feed mode.
    ///
    /// The window has minute granularity on the service; any non-zero
    /// `retention` is rounded **up** to the next whole minute when serialized so
    /// a sub-minute request never truncates to `0` (which would disable the
    /// mode).
    ///
    /// # Continuous backup accounts
    ///
    /// Accounts running in **continuous backup** mode derive the full-fidelity
    /// retention window from the backup retention, and reject container
    /// requests that also set a retention duration with HTTP 400
    /// (`"The retention duration in the Change Feed policy should not be set
    /// when continuous backup mode is enabled for the database account"`), so
    /// leave the change feed policy unset on those accounts.
    ///
    /// Continuous backup on its own does not make
    /// [`AllVersionsAndDeletes`](crate::options::ChangeFeedMode::AllVersionsAndDeletes)
    /// available: the account additionally needs the account-level
    /// full-fidelity change feed opt-in, which cannot be set when the account is
    /// created. Without it the account still answers reads in that mode with
    /// HTTP 400 (`"Change Feed 'All Versions and Deletes' mode must be
    /// enabled"`). See <https://aka.ms/ChangeFeed-AllVersionsAndDeletes>.
    pub fn with_retention_duration(mut self, retention: Duration) -> Self {
        self.retention_duration = Some(retention);
        self
    }

    /// Returns the all versions and deletes retention window, or `None` when the
    /// mode is disabled.
    pub fn retention_duration(&self) -> Option<Duration> {
        self.retention_duration
    }
}

#[cfg(test)]
mod tests {
    use serde::{Deserialize, Serialize};
    use serde_json::json;
    use std::time::Duration;

    use super::{ChangeFeedPolicy, TimeToLive};
    use crate::models::{ContainerProperties, FullTextPath, FullTextPolicy};

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

    #[test]
    fn change_feed_policy_serializes_retention_as_minutes() {
        let policy =
            ChangeFeedPolicy::default().with_retention_duration(Duration::from_secs(5 * 60));
        assert_eq!(
            r#"{"retentionDuration":5}"#,
            serde_json::to_string(&policy).unwrap()
        );
    }

    #[test]
    fn change_feed_policy_serializes_sub_minute_retention_rounded_up() {
        // A sub-minute window must round up to 1 minute, not truncate to 0
        // (which would disable the mode).
        let policy = ChangeFeedPolicy::default().with_retention_duration(Duration::from_secs(30));
        assert_eq!(
            r#"{"retentionDuration":1}"#,
            serde_json::to_string(&policy).unwrap()
        );

        // A non-whole-minute window rounds up to the next whole minute.
        let policy = ChangeFeedPolicy::default().with_retention_duration(Duration::from_secs(90));
        assert_eq!(
            r#"{"retentionDuration":2}"#,
            serde_json::to_string(&policy).unwrap()
        );
    }

    #[test]
    fn change_feed_policy_serializes_default_as_zero() {
        let policy = ChangeFeedPolicy::default();
        assert_eq!(
            r#"{"retentionDuration":0}"#,
            serde_json::to_string(&policy).unwrap()
        );
    }

    #[test]
    fn change_feed_policy_deserializes_retention_from_minutes() {
        let policy: ChangeFeedPolicy = serde_json::from_str(r#"{"retentionDuration":10}"#).unwrap();
        assert_eq!(
            Some(Duration::from_secs(10 * 60)),
            policy.retention_duration()
        );
    }

    #[test]
    fn change_feed_policy_deserializes_zero_retention_as_none() {
        let policy: ChangeFeedPolicy = serde_json::from_str(r#"{"retentionDuration":0}"#).unwrap();
        assert_eq!(None, policy.retention_duration());
    }

    #[test]
    fn change_feed_policy_deserializes_negative_retention_as_none() {
        // A negative window can only arrive from a malformed payload; it maps to
        // `None` so it can never be sent back to the service as a negative value.
        let policy: ChangeFeedPolicy = serde_json::from_str(r#"{"retentionDuration":-5}"#).unwrap();
        assert_eq!(None, policy.retention_duration());
    }

    #[test]
    fn change_feed_policy_deserializes_missing_retention_as_none() {
        let policy: ChangeFeedPolicy = serde_json::from_str("{}").unwrap();
        assert_eq!(None, policy.retention_duration());
    }

    #[test]
    fn container_properties_serializes_change_feed_policy() {
        let properties = ContainerProperties::new("MyContainer", "/partitionKey".into())
            .with_change_feed_policy(
                ChangeFeedPolicy::default().with_retention_duration(Duration::from_secs(60 * 60)),
            );
        assert_eq!(
            "{\"id\":\"MyContainer\",\"partitionKey\":{\"paths\":[\"/partitionKey\"],\"kind\":\"Hash\",\"version\":2},\"changeFeedPolicy\":{\"retentionDuration\":60}}",
            serde_json::to_string(&properties).unwrap()
        );
    }

    #[test]
    fn container_properties_deserializes_change_feed_policy() {
        let json = r#"{
            "id": "MyContainer",
            "partitionKey": {"paths": ["/partitionKey"], "kind": "Hash", "version": 2},
            "changeFeedPolicy": {"retentionDuration": 60}
        }"#;
        let properties: ContainerProperties = serde_json::from_str(json).unwrap();
        assert_eq!(
            Some(Duration::from_secs(60 * 60)),
            properties
                .change_feed_policy
                .and_then(|policy| policy.retention_duration())
        );
    }

    #[test]
    fn container_properties_serializes_full_text_policy() {
        let properties = ContainerProperties::new("MyContainer", "/partitionKey".into())
            .with_full_text_policy(
                FullTextPolicy::new("en-US")
                    .with_full_text_path("/title")
                    .with_full_text_path(FullTextPath::new("/abstract").with_language("fr-FR")),
            );

        assert_eq!(
            "{\"id\":\"MyContainer\",\"partitionKey\":{\"paths\":[\"/partitionKey\"],\"kind\":\"Hash\",\"version\":2},\"fullTextPolicy\":{\"defaultLanguage\":\"en-US\",\"fullTextPaths\":[{\"path\":\"/title\"},{\"path\":\"/abstract\",\"language\":\"fr-FR\"}]}}",
            serde_json::to_string(&properties).unwrap()
        );
    }

    #[test]
    fn container_properties_deserializes_full_text_policy() {
        let json = r#"{
            "id": "MyContainer",
            "partitionKey": {"paths": ["/partitionKey"], "kind": "Hash", "version": 2},
            "fullTextPolicy": {
                "defaultLanguage": "en-US",
                "fullTextPaths": [
                    {"path": "/title"},
                    {"path": "/abstract", "language": "fr-FR"}
                ]
            }
        }"#;

        let properties: ContainerProperties = serde_json::from_str(json).unwrap();

        assert_eq!(
            Some(
                FullTextPolicy::new("en-US")
                    .with_full_text_path("/title")
                    .with_full_text_path(FullTextPath::new("/abstract").with_language("fr-FR"))
            ),
            properties.full_text_policy
        );
    }

    #[test]
    fn container_properties_preserves_absent_full_text_default_language() {
        let json = r#"{
            "id": "MyContainer",
            "partitionKey": {"paths": ["/partitionKey"], "kind": "Hash", "version": 2},
            "fullTextPolicy": {
                "fullTextPaths": [
                    {"path": "/title", "language": "en-US"},
                    {"path": "/abstract", "language": "fr-FR"}
                ]
            }
        }"#;

        let properties: ContainerProperties = serde_json::from_str(json).unwrap();
        let policy = properties
            .full_text_policy
            .as_ref()
            .expect("full text policy should be present");

        assert_eq!(None, policy.default_language);
        assert_eq!(Some("en-US"), policy.full_text_paths[0].language.as_deref());

        let round_tripped = serde_json::to_value(&properties).unwrap();
        assert!(
            round_tripped["fullTextPolicy"]
                .get("defaultLanguage")
                .is_none(),
            "an absent default language must not be replaced with an empty string"
        );
    }

    #[test]
    fn container_properties_preserves_unknown_fields_through_round_trip() {
        // A container configured by a newer SDK, another language SDK, or the
        // portal must survive a read-modify-replace round trip through this SDK
        // without losing server-side configuration this version doesn't model.
        let json = r#"{
            "id": "MyContainer",
            "partitionKey": {"paths": ["/partitionKey"], "kind": "Hash", "version": 2},
            "_rid": "rid-value",
            "_etag": "\"etag-value\"",
            "_ts": 1729036800,
            "geospatialConfig": {"type": "Geography"},
            "clientEncryptionPolicy": {"policyFormatVersion": 2},
            "vectorEmbeddingPolicy": {
                "someFutureVectorPolicyKnob": true,
                "vectorEmbeddings": [
                    {
                        "path": "/vector",
                        "dataType": "float32",
                        "dimensions": 8,
                        "distanceFunction": "cosine",
                        "embeddingSource": {"modelName": "text-embedding-3-small"}
                    }
                ]
            },
            "fullTextPolicy": {
                "defaultLanguage": "en-US",
                "fullTextPaths": [{"path": "/abstract", "tokenizer": "word"}],
                "package": "standard"
            }
        }"#;

        let properties: ContainerProperties = serde_json::from_str(json).unwrap();
        let round_tripped: serde_json::Value =
            serde_json::from_str(&serde_json::to_string(&properties).unwrap()).unwrap();

        // Whole policies the SDK does not model at all.
        assert_eq!(
            json!({"type": "Geography"}),
            round_tripped["geospatialConfig"]
        );
        assert_eq!(
            json!({"policyFormatVersion": 2}),
            round_tripped["clientEncryptionPolicy"]
        );

        // Fields nested inside policies the SDK *does* model.
        assert_eq!(
            json!({"modelName": "text-embedding-3-small"}),
            round_tripped["vectorEmbeddingPolicy"]["vectorEmbeddings"][0]["embeddingSource"]
        );
        assert_eq!(
            json!(true),
            round_tripped["vectorEmbeddingPolicy"]["someFutureVectorPolicyKnob"]
        );
        assert_eq!(
            json!("standard"),
            round_tripped["fullTextPolicy"]["package"]
        );
        assert_eq!(
            json!("word"),
            round_tripped["fullTextPolicy"]["fullTextPaths"][0]["tokenizer"]
        );

        // System properties must keep their existing treatment: `_rid` round
        // trips, while `_etag` and `_ts` stay read-only and are not sent back.
        assert_eq!(json!("rid-value"), round_tripped["_rid"]);
        assert!(round_tripped.get("_etag").is_none());
        assert!(round_tripped.get("_ts").is_none());

        // Modelled fields remain readable alongside the unknown ones.
        assert_eq!("MyContainer", properties.id);
        assert_eq!(
            Some("en-US"),
            properties
                .full_text_policy
                .as_ref()
                .and_then(|policy| policy.default_language.as_deref())
        );
    }
}

use std::time::Duration;

use azure_core::Model;
use serde::{Deserialize, Deserializer};

use crate::models::SystemProperties;

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
#[derive(Model, Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContainerProperties {
    /// The ID of the container.
    pub id: String,

    /// The time-to-live for items in the container.
    ///
    /// For more information see https://docs.microsoft.com/azure/cosmos-db/time-to-live#time-to-live-configurations
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_ttl")]
    pub default_ttl: Option<Duration>,

    /// The time-to-live for the analytical store in the container.
    ///
    /// For more information see https://docs.microsoft.com/azure/cosmos-db/analytical-store-introduction#analytical-ttl
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

    /// A [`SystemProperties`] object containing common system properties for the container.
    #[serde(flatten)]
    pub system_properties: SystemProperties,
}

/// Represents the partition key definition for a container.
#[non_exhaustive]
#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PartitionKeyDefinition {
    /// The list of partition keys paths.
    pub paths: Vec<String>,

    /// The version of the partition key hash in use.
    pub version: Option<usize>,
}

/// Represents the indexing policy for a container.
///
/// For more information see https://docs.microsoft.com/azure/cosmos-db/index-policy
#[non_exhaustive]
#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IndexingPolicy {
    /// Indicates that the indexing policy is automatic.
    pub automatic: bool,

    /// The indexing mode in use.
    pub indexing_mode: IndexingMode,

    /// The paths to be indexed.
    #[serde(default)]
    pub included_paths: Vec<PropertyPath>,

    /// The paths to be excluded.
    #[serde(default)]
    pub excluded_paths: Vec<PropertyPath>,

    /// A list of spatial indexes in the container.
    #[serde(default)]
    pub spatial_indexes: Vec<SpatialIndex>,

    /// A list of composite indexes in the container
    #[serde(default)]
    pub composite_indexes: Vec<CompositeIndex>,
}

/// Defines the indexing modes supported by Azure Cosmos DB.
#[non_exhaustive]
#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum IndexingMode {
    Consistent,
    None,
}

/// Represents a JSON path.
#[non_exhaustive]
#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PropertyPath {
    // The path to the property referenced in this index.
    pub path: String,
}

/// Represents a spatial index
#[non_exhaustive]
#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpatialIndex {
    /// The path to the property referenced in this index.
    pub path: String,

    /// The spatial types used in this index
    pub types: Vec<SpatialType>,
}

/// Defines the types of spatial data that can be indexed.
#[non_exhaustive]
#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum SpatialType {
    Point,
    Polygon,
    LineString,
    MultiPolygon,
}

/// Represents a composite index
#[non_exhaustive]
#[derive(Clone, Debug, Deserialize)]
#[serde(transparent)]
pub struct CompositeIndex {
    /// The properties in this composite index
    pub members: Vec<CompositeIndexProperty>,
}

/// Describes a single property in a composite index.
#[non_exhaustive]
#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CompositeIndexProperty {
    /// The path to the property referenced in this index.
    pub path: String,

    /// The order of the composite index.
    ///
    /// For example, if you want to run the query "SELECT * FROM c ORDER BY c.age asc, c.height desc",
    /// then you'd specify the order for "/asc" to be *ascending* and the order for "/height" to be *descending*.
    pub order: CompositeIndexOrder,
}

/// Ordering values available for composite indexes.
#[non_exhaustive]
#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum CompositeIndexOrder {
    Ascending,
    Descending,
}

/// Represents a unique key policy for a container.
///
/// For more information see https://docs.microsoft.com/azure/cosmos-db/unique-keys
#[non_exhaustive]
#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UniqueKeyPolicy {
    /// The keys defined in this policy.
    pub unique_keys: Vec<UniqueKey>,
}

/// Represents a single unique key for a container.
#[non_exhaustive]
#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UniqueKey {
    /// The set of paths which must be unique for each item.
    pub paths: Vec<String>,
}

/// Represents a conflict resolution policy for a container
///
/// For more information, see https://learn.microsoft.com/en-us/azure/cosmos-db/conflict-resolution-policies
#[non_exhaustive]
#[derive(Clone, Debug, Deserialize)]
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
#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum ConflictResolutionMode {
    /// Conflict resolution will be performed by using the highest value of the property specified by [`ConflictResolutionPolicy::resolution_path`].
    LastWriterWins,

    /// Conflict resolution will be performed by executing the stored procedure specified by [`ConflictResolutionPolicy::resolution_procedure`].
    Custom,
}

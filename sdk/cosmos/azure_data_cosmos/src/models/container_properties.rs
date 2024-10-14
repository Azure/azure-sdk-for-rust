use std::time::Duration;

use azure_core::Model;
use serde::{Deserialize, Deserializer};

use crate::models::SystemProperties;

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

/// Represents the partition key definition for a container.
#[non_exhaustive]
#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct PartitionKeyDefinition {
    /// The list of partition keys paths.
    pub paths: Vec<String>,

    /// The version of the partition key hash in use.
    #[serde(default)]
    pub version: i32,
}

/// Represents the indexing policy for a container.
///
/// For more information see <https://docs.microsoft.com/azure/cosmos-db/index-policy>
#[non_exhaustive]
#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct IndexingPolicy {
    /// Indicates that the indexing policy is automatic.
    #[serde(default)]
    pub automatic: bool,

    /// The indexing mode in use.
    #[serde(default)]
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

    /// A list of vector indexes in the container
    #[serde(default)]
    pub vector_indexes: Vec<VectorIndex>,
}

/// Defines the indexing modes supported by Azure Cosmos DB.
#[non_exhaustive]
#[derive(Clone, Default, Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum IndexingMode {
    Consistent,

    #[default]
    None,
}

/// Represents a JSON path.
#[non_exhaustive]
#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct PropertyPath {
    // The path to the property referenced in this index.
    pub path: String,
}

/// Represents a spatial index
#[non_exhaustive]
#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct SpatialIndex {
    /// The path to the property referenced in this index.
    pub path: String,

    /// The spatial types used in this index
    pub types: Vec<SpatialType>,
}

/// Defines the types of spatial data that can be indexed.
#[non_exhaustive]
#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "PascalCase")]
pub enum SpatialType {
    Point,
    Polygon,
    LineString,
    MultiPolygon,
}

/// Represents a composite index
#[non_exhaustive]
#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
#[serde(transparent)]
pub struct CompositeIndex {
    /// The properties in this composite index
    pub properties: Vec<CompositeIndexProperty>,
}

/// Describes a single property in a composite index.
#[non_exhaustive]
#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
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
#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum CompositeIndexOrder {
    Ascending,
    Descending,
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

/// Represents a vector index
///
/// For more information, see <https://learn.microsoft.com/en-us/azure/cosmos-db/index-policy#vector-indexes>
#[non_exhaustive]
#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct VectorIndex {
    /// The path to the property referenced in this index.
    pub path: String,

    /// The type of the vector index.
    #[serde(rename = "type")] // "type" is a reserved word in Rust.
    pub index_type: VectorIndexType,
}

/// Types of vector indexes supported by Cosmos DB
#[non_exhaustive]
#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum VectorIndexType {
    Flat,
    QuantizedFlat,
    DiskANN,
}

#[cfg(test)]
mod tests {
    use crate::models::{
        CompositeIndex, CompositeIndexOrder, CompositeIndexProperty, IndexingMode, PropertyPath,
        SpatialIndex, SpatialType, VectorIndex, VectorIndexType,
    };

    use super::IndexingPolicy;

    #[test]
    pub fn deserialize_indexing_policy() {
        // A fairly complete deserialization test that covers most of the indexing policies described in our docs.
        let policy = r#"
            {
                "indexingMode": "consistent",
                "includedPaths": [
                    {
                        "path": "/*"
                    }
                ],
                "excludedPaths": [
                    {
                        "path": "/path/to/single/excluded/property/?"
                    },
                    {
                        "path": "/path/to/root/of/multiple/excluded/properties/*"
                    }
                ],
                "spatialIndexes": [
                    {
                        "path": "/path/to/geojson/property/?",
                        "types": [
                            "Point",
                            "Polygon",
                            "MultiPolygon",
                            "LineString"
                        ]
                    }
                ],
                "vectorIndexes": [
                    {
                        "path": "/vector1",
                        "type": "quantizedFlat"
                    },
                    {
                        "path": "/vector2",
                        "type": "diskANN"
                    }
                ],
                "compositeIndexes":[
                    [
                        {
                            "path":"/name",
                            "order":"ascending"
                        },
                        {
                            "path":"/age",
                            "order":"descending"
                        }
                    ],
                    [
                        {
                            "path":"/name2",
                            "order":"descending"
                        },
                        {
                            "path":"/age2",
                            "order":"ascending"
                        }
                    ]
                ],
                "extraValueNotCurrentlyPresentInModel": {
                    "this": "should not fail"
                }
            }
        "#;

        let policy: IndexingPolicy = serde_json::from_str(policy).unwrap();

        assert_eq!(
            IndexingPolicy {
                automatic: false,
                indexing_mode: IndexingMode::Consistent,
                included_paths: vec![PropertyPath {
                    path: "/*".to_string(),
                }],
                excluded_paths: vec![
                    PropertyPath {
                        path: "/path/to/single/excluded/property/?".to_string()
                    },
                    PropertyPath {
                        path: "/path/to/root/of/multiple/excluded/properties/*".to_string()
                    },
                ],
                spatial_indexes: vec![SpatialIndex {
                    path: "/path/to/geojson/property/?".to_string(),
                    types: vec![
                        SpatialType::Point,
                        SpatialType::Polygon,
                        SpatialType::MultiPolygon,
                        SpatialType::LineString,
                    ]
                }],
                composite_indexes: vec![
                    CompositeIndex {
                        properties: vec![
                            CompositeIndexProperty {
                                path: "/name".to_string(),
                                order: CompositeIndexOrder::Ascending,
                            },
                            CompositeIndexProperty {
                                path: "/age".to_string(),
                                order: CompositeIndexOrder::Descending,
                            },
                        ]
                    },
                    CompositeIndex {
                        properties: vec![
                            CompositeIndexProperty {
                                path: "/name2".to_string(),
                                order: CompositeIndexOrder::Descending,
                            },
                            CompositeIndexProperty {
                                path: "/age2".to_string(),
                                order: CompositeIndexOrder::Ascending,
                            },
                        ]
                    },
                ],
                vector_indexes: vec![
                    VectorIndex {
                        path: "/vector1".to_string(),
                        index_type: VectorIndexType::QuantizedFlat,
                    },
                    VectorIndex {
                        path: "/vector2".to_string(),
                        index_type: VectorIndexType::DiskANN,
                    }
                ]
            },
            policy
        );
    }
}

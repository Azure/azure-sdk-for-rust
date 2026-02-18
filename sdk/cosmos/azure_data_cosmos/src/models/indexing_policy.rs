// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use azure_core::fmt::SafeDebug;
use serde::{Deserialize, Serialize};

/// Represents the indexing policy for a container.
///
/// For more information see <https://learn.microsoft.com/azure/cosmos-db/index-policy>
#[derive(Clone, Default, SafeDebug, Deserialize, Serialize, PartialEq, Eq)]
#[safe(true)]
#[serde(rename_all = "camelCase")]
pub struct IndexingPolicy {
    /// Indicates that the indexing policy is automatic.
    #[serde(default)]
    pub automatic: bool,

    /// The indexing mode in use.
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub indexing_mode: Option<IndexingMode>,

    /// The paths to be indexed.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub included_paths: Vec<PropertyPath>,

    /// The paths to be excluded.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub excluded_paths: Vec<PropertyPath>,

    /// A list of spatial indexes in the container.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub spatial_indexes: Vec<SpatialIndex>,

    /// A list of composite indexes in the container
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub composite_indexes: Vec<CompositeIndex>,

    /// A list of vector indexes in the container
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub vector_indexes: Vec<VectorIndex>,
}

/// Defines the indexing modes supported by Azure Cosmos DB.
#[derive(Clone, SafeDebug, Deserialize, Serialize, PartialEq, Eq)]
#[safe(true)]
#[serde(rename_all = "camelCase")]
pub enum IndexingMode {
    Consistent,
    None,
}

/// Represents a JSON path.
#[derive(Clone, SafeDebug, Deserialize, Serialize, PartialEq, Eq)]
#[safe(true)]
#[serde(rename_all = "camelCase")]
pub struct PropertyPath {
    // The path to the property referenced in this index.
    pub path: String,
}

impl<T: Into<String>> From<T> for PropertyPath {
    fn from(value: T) -> Self {
        PropertyPath { path: value.into() }
    }
}

/// Represents a spatial index
#[derive(Clone, SafeDebug, Deserialize, Serialize, PartialEq, Eq)]
#[safe(true)]
#[serde(rename_all = "camelCase")]
pub struct SpatialIndex {
    /// The path to the property referenced in this index.
    pub path: String,

    /// The spatial types used in this index
    pub types: Vec<SpatialType>,
}

/// Defines the types of spatial data that can be indexed.
#[derive(Clone, SafeDebug, Deserialize, Serialize, PartialEq, Eq)]
#[safe(true)]
#[serde(rename_all = "PascalCase")]
pub enum SpatialType {
    Point,
    Polygon,
    LineString,
    MultiPolygon,
}

/// Represents a composite index
#[derive(Clone, SafeDebug, Deserialize, Serialize, PartialEq, Eq)]
#[safe(true)]
#[serde(transparent)]
pub struct CompositeIndex {
    /// The properties in this composite index
    pub properties: Vec<CompositeIndexProperty>,
}

/// Describes a single property in a composite index.
#[derive(Clone, SafeDebug, Deserialize, Serialize, PartialEq, Eq)]
#[safe(true)]
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
#[derive(Clone, SafeDebug, Deserialize, Serialize, PartialEq, Eq)]
#[safe(true)]
#[serde(rename_all = "camelCase")]
pub enum CompositeIndexOrder {
    Ascending,
    Descending,
}

/// Represents a vector index
///
/// For more information, see <https://learn.microsoft.com/en-us/azure/cosmos-db/index-policy#vector-indexes>
#[derive(Clone, SafeDebug, Deserialize, Serialize, PartialEq, Eq)]
#[safe(true)]
#[serde(rename_all = "camelCase")]
pub struct VectorIndex {
    /// The path to the property referenced in this index.
    pub path: String,

    /// The type of the vector index.
    #[serde(rename = "type")] // "type" is a reserved word in Rust.
    pub index_type: VectorIndexType,
}

/// Types of vector indexes supported by Cosmos DB
#[derive(Clone, SafeDebug, Deserialize, Serialize, PartialEq, Eq)]
#[safe(true)]
#[serde(rename_all = "camelCase")]
pub enum VectorIndexType {
    /// Represents the `flat` vector index type.
    Flat,

    /// Represents the `quantizedFlat` vector index type.
    QuantizedFlat,

    /// Represents the `diskANN` vector index type.
    DiskANN,
}

#[cfg(test)]
mod tests {
    use crate::models::{
        CompositeIndex, CompositeIndexOrder, CompositeIndexProperty, IndexingMode, IndexingPolicy,
        PropertyPath, SpatialIndex, SpatialType, VectorIndex, VectorIndexType,
    };

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
                indexing_mode: Some(IndexingMode::Consistent),
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

    #[test]
    pub fn serialize_indexing_policy() {
        let policy = IndexingPolicy {
            automatic: true,
            indexing_mode: None,
            included_paths: vec![PropertyPath {
                path: "/*".to_string(),
            }],
            excluded_paths: vec![
                PropertyPath {
                    path: "/path/to/single/excluded/property/?".to_string(),
                },
                PropertyPath {
                    path: "/path/to/root/of/multiple/excluded/properties/*".to_string(),
                },
            ],
            spatial_indexes: vec![
                SpatialIndex {
                    path: "/path/to/geojson/property/?".to_string(),
                    types: vec![
                        SpatialType::Point,
                        SpatialType::Polygon,
                        SpatialType::MultiPolygon,
                        SpatialType::LineString,
                    ],
                },
                SpatialIndex {
                    path: "/path/to/geojson/property2/?".to_string(),
                    types: vec![],
                },
            ],
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
                    ],
                },
                CompositeIndex { properties: vec![] },
            ],
            vector_indexes: vec![
                VectorIndex {
                    path: "/vector1".to_string(),
                    index_type: VectorIndexType::QuantizedFlat,
                },
                VectorIndex {
                    path: "/vector2".to_string(),
                    index_type: VectorIndexType::DiskANN,
                },
            ],
        };
        let json = serde_json::to_string(&policy).unwrap();

        assert_eq!(
            "{\"automatic\":true,\"includedPaths\":[{\"path\":\"/*\"}],\"excludedPaths\":[{\"path\":\"/path/to/single/excluded/property/?\"},{\"path\":\"/path/to/root/of/multiple/excluded/properties/*\"}],\"spatialIndexes\":[{\"path\":\"/path/to/geojson/property/?\",\"types\":[\"Point\",\"Polygon\",\"MultiPolygon\",\"LineString\"]},{\"path\":\"/path/to/geojson/property2/?\",\"types\":[]}],\"compositeIndexes\":[[{\"path\":\"/name\",\"order\":\"ascending\"},{\"path\":\"/age\",\"order\":\"descending\"}],[]],\"vectorIndexes\":[{\"path\":\"/vector1\",\"type\":\"quantizedFlat\"},{\"path\":\"/vector2\",\"type\":\"diskANN\"}]}",
            json
        );
    }
}

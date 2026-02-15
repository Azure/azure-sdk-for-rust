// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use azure_core::fmt::SafeDebug;
use serde::{Deserialize, Serialize};

/// Represents the indexing policy for a container.
///
/// For more information see <https://learn.microsoft.com/azure/cosmos-db/index-policy>
#[non_exhaustive]
#[derive(Clone, Default, SafeDebug, Deserialize, Serialize, PartialEq, Eq)]
#[safe(true)]
#[serde(rename_all = "camelCase")]
pub struct IndexingPolicy {
    /// Indicates that the indexing policy is automatic.
    #[serde(default)]
    automatic: bool,

    /// The indexing mode in use.
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    indexing_mode: Option<IndexingMode>,

    /// The paths to be indexed.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    included_paths: Vec<PropertyPath>,

    /// The paths to be excluded.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    excluded_paths: Vec<PropertyPath>,

    /// A list of spatial indexes in the container.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    spatial_indexes: Vec<SpatialIndex>,

    /// A list of composite indexes in the container.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    composite_indexes: Vec<CompositeIndex>,

    /// A list of vector indexes in the container.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    vector_indexes: Vec<VectorIndex>,
}

impl IndexingPolicy {
    /// Gets whether the indexing policy is automatic.
    pub fn automatic(&self) -> bool {
        self.automatic
    }

    /// Sets whether the indexing policy is automatic.
    pub fn with_automatic(mut self, automatic: bool) -> Self {
        self.automatic = automatic;
        self
    }

    /// Gets the indexing mode in use.
    pub fn indexing_mode(&self) -> Option<&IndexingMode> {
        self.indexing_mode.as_ref()
    }

    /// Sets the indexing mode.
    pub fn with_indexing_mode(mut self, indexing_mode: impl Into<Option<IndexingMode>>) -> Self {
        self.indexing_mode = indexing_mode.into();
        self
    }

    /// Gets the paths to be indexed.
    pub fn included_paths(&self) -> &[PropertyPath] {
        &self.included_paths
    }

    /// Sets the paths to be indexed.
    pub fn with_included_paths(mut self, included_paths: Vec<PropertyPath>) -> Self {
        self.included_paths = included_paths;
        self
    }

    /// Gets the paths to be excluded.
    pub fn excluded_paths(&self) -> &[PropertyPath] {
        &self.excluded_paths
    }

    /// Sets the paths to be excluded.
    pub fn with_excluded_paths(mut self, excluded_paths: Vec<PropertyPath>) -> Self {
        self.excluded_paths = excluded_paths;
        self
    }

    /// Gets the spatial indexes in the container.
    pub fn spatial_indexes(&self) -> &[SpatialIndex] {
        &self.spatial_indexes
    }

    /// Sets the spatial indexes.
    pub fn with_spatial_indexes(mut self, spatial_indexes: Vec<SpatialIndex>) -> Self {
        self.spatial_indexes = spatial_indexes;
        self
    }

    /// Gets the composite indexes in the container.
    pub fn composite_indexes(&self) -> &[CompositeIndex] {
        &self.composite_indexes
    }

    /// Sets the composite indexes.
    pub fn with_composite_indexes(mut self, composite_indexes: Vec<CompositeIndex>) -> Self {
        self.composite_indexes = composite_indexes;
        self
    }

    /// Gets the vector indexes in the container.
    pub fn vector_indexes(&self) -> &[VectorIndex] {
        &self.vector_indexes
    }

    /// Sets the vector indexes.
    pub fn with_vector_indexes(mut self, vector_indexes: Vec<VectorIndex>) -> Self {
        self.vector_indexes = vector_indexes;
        self
    }
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
#[non_exhaustive]
#[derive(Clone, SafeDebug, Deserialize, Serialize, PartialEq, Eq)]
#[safe(true)]
#[serde(rename_all = "camelCase")]
pub struct PropertyPath {
    // The path to the property referenced in this index.
    path: String,
}

impl PropertyPath {
    /// Creates a new [`PropertyPath`] with the required `path` field.
    pub fn new(path: impl Into<String>) -> Self {
        Self { path: path.into() }
    }

    /// Gets the path to the property.
    pub fn path(&self) -> &str {
        &self.path
    }
}

impl<T: Into<String>> From<T> for PropertyPath {
    fn from(value: T) -> Self {
        PropertyPath { path: value.into() }
    }
}

/// Represents a spatial index
///
/// # Required fields
///
/// * `path` — The path to the property.
/// * `types` — The spatial types used in this index.
#[non_exhaustive]
#[derive(Clone, SafeDebug, Deserialize, Serialize, PartialEq, Eq)]
#[safe(true)]
#[serde(rename_all = "camelCase")]
pub struct SpatialIndex {
    /// The path to the property referenced in this index.
    path: String,

    /// The spatial types used in this index.
    types: Vec<SpatialType>,
}

impl SpatialIndex {
    /// Creates a new [`SpatialIndex`] with the required `path` and `types` fields.
    pub fn new(path: impl Into<String>, types: Vec<SpatialType>) -> Self {
        Self {
            path: path.into(),
            types,
        }
    }

    /// Gets the path to the property referenced in this index.
    pub fn path(&self) -> &str {
        &self.path
    }

    /// Gets the spatial types used in this index.
    pub fn types(&self) -> &[SpatialType] {
        &self.types
    }
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
///
/// # Required fields
///
/// * `properties` — The properties in this composite index.
#[non_exhaustive]
#[derive(Clone, SafeDebug, Deserialize, Serialize, PartialEq, Eq)]
#[safe(true)]
#[serde(transparent)]
pub struct CompositeIndex {
    /// The properties in this composite index.
    properties: Vec<CompositeIndexProperty>,
}

impl CompositeIndex {
    /// Creates a new [`CompositeIndex`] with the required `properties` field.
    pub fn new(properties: Vec<CompositeIndexProperty>) -> Self {
        Self { properties }
    }

    /// Gets the properties in this composite index.
    pub fn properties(&self) -> &[CompositeIndexProperty] {
        &self.properties
    }
}

/// Describes a single property in a composite index.
///
/// # Required fields
///
/// * `path` — The path to the property.
///
/// The `order` defaults to [`CompositeIndexOrder::Ascending`].
#[non_exhaustive]
#[derive(Clone, SafeDebug, Deserialize, Serialize, PartialEq, Eq)]
#[safe(true)]
#[serde(rename_all = "camelCase")]
pub struct CompositeIndexProperty {
    /// The path to the property referenced in this index.
    path: String,

    /// The order of the composite index.
    ///
    /// For example, if you want to run the query "SELECT * FROM c ORDER BY c.age asc, c.height desc",
    /// then you'd specify the order for "/asc" to be *ascending* and the order for "/height" to be *descending*.
    #[serde(default)]
    order: CompositeIndexOrder,
}

impl CompositeIndexProperty {
    /// Creates a new [`CompositeIndexProperty`] with the required `path` field.
    ///
    /// The `order` defaults to [`CompositeIndexOrder::Ascending`].
    pub fn new(path: impl Into<String>) -> Self {
        Self {
            path: path.into(),
            order: CompositeIndexOrder::default(),
        }
    }

    /// Gets the path to the property referenced in this index.
    pub fn path(&self) -> &str {
        &self.path
    }

    /// Gets the order of the composite index.
    pub fn order(&self) -> &CompositeIndexOrder {
        &self.order
    }

    /// Sets the order.
    pub fn with_order(mut self, order: CompositeIndexOrder) -> Self {
        self.order = order;
        self
    }
}

/// Ordering values available for composite indexes.
#[derive(Clone, Default, SafeDebug, Deserialize, Serialize, PartialEq, Eq)]
#[safe(true)]
#[serde(rename_all = "camelCase")]
pub enum CompositeIndexOrder {
    #[default]
    Ascending,
    Descending,
}

/// Represents a vector index
///
/// # Required fields
///
/// * `path` — The path to the property.
///
/// The `index_type` defaults to [`VectorIndexType::Flat`].
///
/// For more information, see <https://learn.microsoft.com/en-us/azure/cosmos-db/index-policy#vector-indexes>
#[non_exhaustive]
#[derive(Clone, SafeDebug, Deserialize, Serialize, PartialEq, Eq)]
#[safe(true)]
#[serde(rename_all = "camelCase")]
pub struct VectorIndex {
    /// The path to the property referenced in this index.
    path: String,

    /// The type of the vector index.
    #[serde(rename = "type")] // "type" is a reserved word in Rust.
    #[serde(default)]
    index_type: VectorIndexType,
}

impl VectorIndex {
    /// Creates a new [`VectorIndex`] with the required `path` field.
    ///
    /// The `index_type` defaults to [`VectorIndexType::Flat`].
    pub fn new(path: impl Into<String>) -> Self {
        Self {
            path: path.into(),
            index_type: VectorIndexType::default(),
        }
    }

    /// Gets the path to the property referenced in this index.
    pub fn path(&self) -> &str {
        &self.path
    }

    /// Gets the type of the vector index.
    pub fn index_type(&self) -> &VectorIndexType {
        &self.index_type
    }

    /// Sets the index type.
    pub fn with_index_type(mut self, index_type: VectorIndexType) -> Self {
        self.index_type = index_type;
        self
    }
}

/// Types of vector indexes supported by Cosmos DB
#[derive(Clone, Default, SafeDebug, Deserialize, Serialize, PartialEq, Eq)]
#[safe(true)]
#[serde(rename_all = "camelCase")]
pub enum VectorIndexType {
    /// Represents the `flat` vector index type.
    #[default]
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
            IndexingPolicy::default()
                .with_indexing_mode(IndexingMode::Consistent)
                .with_included_paths(vec![PropertyPath::from("/*")])
                .with_excluded_paths(vec![
                    PropertyPath::from("/path/to/single/excluded/property/?"),
                    PropertyPath::from("/path/to/root/of/multiple/excluded/properties/*"),
                ])
                .with_spatial_indexes(vec![SpatialIndex::new(
                    "/path/to/geojson/property/?",
                    vec![
                        SpatialType::Point,
                        SpatialType::Polygon,
                        SpatialType::MultiPolygon,
                        SpatialType::LineString,
                    ],
                )])
                .with_composite_indexes(vec![
                    CompositeIndex::new(vec![
                        CompositeIndexProperty::new("/name")
                            .with_order(CompositeIndexOrder::Ascending),
                        CompositeIndexProperty::new("/age")
                            .with_order(CompositeIndexOrder::Descending),
                    ]),
                    CompositeIndex::new(vec![
                        CompositeIndexProperty::new("/name2")
                            .with_order(CompositeIndexOrder::Descending),
                        CompositeIndexProperty::new("/age2")
                            .with_order(CompositeIndexOrder::Ascending),
                    ]),
                ])
                .with_vector_indexes(vec![
                    VectorIndex::new("/vector1").with_index_type(VectorIndexType::QuantizedFlat),
                    VectorIndex::new("/vector2").with_index_type(VectorIndexType::DiskANN),
                ]),
            policy
        );
    }

    #[test]
    pub fn serialize_indexing_policy() {
        let policy = IndexingPolicy::default()
            .with_automatic(true)
            .with_included_paths(vec![PropertyPath::from("/*")])
            .with_excluded_paths(vec![
                PropertyPath::from("/path/to/single/excluded/property/?"),
                PropertyPath::from("/path/to/root/of/multiple/excluded/properties/*"),
            ])
            .with_spatial_indexes(vec![
                SpatialIndex::new(
                    "/path/to/geojson/property/?",
                    vec![
                        SpatialType::Point,
                        SpatialType::Polygon,
                        SpatialType::MultiPolygon,
                        SpatialType::LineString,
                    ],
                ),
                SpatialIndex::new("/path/to/geojson/property2/?", vec![]),
            ])
            .with_composite_indexes(vec![
                CompositeIndex::new(vec![
                    CompositeIndexProperty::new("/name").with_order(CompositeIndexOrder::Ascending),
                    CompositeIndexProperty::new("/age").with_order(CompositeIndexOrder::Descending),
                ]),
                CompositeIndex::new(vec![]),
            ])
            .with_vector_indexes(vec![
                VectorIndex::new("/vector1").with_index_type(VectorIndexType::QuantizedFlat),
                VectorIndex::new("/vector2").with_index_type(VectorIndexType::DiskANN),
            ]);
        let json = serde_json::to_string(&policy).unwrap();

        assert_eq!(
            "{\"automatic\":true,\"includedPaths\":[{\"path\":\"/*\"}],\"excludedPaths\":[{\"path\":\"/path/to/single/excluded/property/?\"},{\"path\":\"/path/to/root/of/multiple/excluded/properties/*\"}],\"spatialIndexes\":[{\"path\":\"/path/to/geojson/property/?\",\"types\":[\"Point\",\"Polygon\",\"MultiPolygon\",\"LineString\"]},{\"path\":\"/path/to/geojson/property2/?\",\"types\":[]}],\"compositeIndexes\":[[{\"path\":\"/name\",\"order\":\"ascending\"},{\"path\":\"/age\",\"order\":\"descending\"}],[]],\"vectorIndexes\":[{\"path\":\"/vector1\",\"type\":\"quantizedFlat\"},{\"path\":\"/vector2\",\"type\":\"diskANN\"}]}",
            json
        );
    }
}

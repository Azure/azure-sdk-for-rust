// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use std::collections::BTreeMap;

use azure_core::fmt::SafeDebug;
use serde::{Deserialize, Serialize};

/// Represents the indexing policy for a container.
///
/// For more information see <https://learn.microsoft.com/azure/cosmos-db/index-policy>
#[derive(Clone, Default, SafeDebug, Deserialize, Serialize, PartialEq, Eq)]
#[safe(true)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
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

    /// A list of full text indexes in the container.
    ///
    /// Every path indexed here must also appear in the container's
    /// [`FullTextPolicy`](crate::models::FullTextPolicy).
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub full_text_indexes: Vec<FullTextIndex>,

    /// Properties returned by the service that this version of the SDK does not model.
    ///
    /// Captured on deserialization and written back on serialization so a
    /// read-modify-replace round trip does not silently drop server-side
    /// configuration the SDK doesn't know about yet.
    #[serde(flatten)]
    extra: BTreeMap<String, serde_json::Value>,
}

impl IndexingPolicy {
    pub fn with_indexing_mode(mut self, indexing_mode: IndexingMode) -> Self {
        self.indexing_mode = Some(indexing_mode);
        self
    }

    pub fn with_included_path(mut self, included_path: impl Into<PropertyPath>) -> Self {
        self.included_paths.push(included_path.into());
        self
    }

    pub fn with_excluded_path(mut self, excluded_path: impl Into<PropertyPath>) -> Self {
        self.excluded_paths.push(excluded_path.into());
        self
    }

    pub fn with_spatial_index(mut self, spatial_index: SpatialIndex) -> Self {
        self.spatial_indexes.push(spatial_index);
        self
    }

    pub fn with_composite_index(mut self, composite_index: CompositeIndex) -> Self {
        self.composite_indexes.push(composite_index);
        self
    }

    pub fn with_vector_index(mut self, vector_index: VectorIndex) -> Self {
        self.vector_indexes.push(vector_index);
        self
    }

    /// Appends `full_text_index` to the policy's list of full text indexes.
    pub fn with_full_text_index(mut self, full_text_index: impl Into<FullTextIndex>) -> Self {
        self.full_text_indexes.push(full_text_index.into());
        self
    }
}

/// Defines the indexing modes supported by Azure Cosmos DB.
#[derive(Clone, SafeDebug, Deserialize, Serialize, PartialEq, Eq)]
#[safe(true)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub enum IndexingMode {
    Consistent,
    None,
}

/// Represents a JSON path.
#[derive(Clone, Default, SafeDebug, Deserialize, Serialize, PartialEq, Eq)]
#[safe(true)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct PropertyPath {
    // The path to the property referenced in this index.
    pub path: String,
}

impl PropertyPath {
    /// Sets the path of this `PropertyPath`.
    pub fn with_path(mut self, path: impl Into<String>) -> Self {
        self.path = path.into();
        self
    }
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
#[non_exhaustive]
pub struct SpatialIndex {
    /// The path to the property referenced in this index.
    pub path: String,

    /// The spatial types used in this index
    pub types: Vec<SpatialType>,
}

impl SpatialIndex {
    /// Creates a new [`SpatialIndex`] over the given path with no spatial types.
    pub fn new(path: impl Into<String>) -> Self {
        Self {
            path: path.into(),
            types: Vec::new(),
        }
    }

    /// Appends `spatial_type` to the index's list of spatial types.
    pub fn with_type(mut self, spatial_type: SpatialType) -> Self {
        self.types.push(spatial_type);
        self
    }
}

/// Defines the types of spatial data that can be indexed.
#[derive(Clone, SafeDebug, Deserialize, Serialize, PartialEq, Eq)]
#[safe(true)]
#[serde(rename_all = "PascalCase")]
#[non_exhaustive]
pub enum SpatialType {
    Point,
    Polygon,
    LineString,
    MultiPolygon,
}

/// Represents a composite index
#[derive(Clone, Default, SafeDebug, Deserialize, Serialize, PartialEq, Eq)]
#[safe(true)]
#[serde(transparent)]
#[non_exhaustive]
pub struct CompositeIndex {
    /// The properties in this composite index
    pub properties: Vec<CompositeIndexProperty>,
}

impl CompositeIndex {
    /// Appends `property` to the composite index.
    pub fn with_property(mut self, property: CompositeIndexProperty) -> Self {
        self.properties.push(property);
        self
    }
}

/// Describes a single property in a composite index.
#[derive(Clone, SafeDebug, Deserialize, Serialize, PartialEq, Eq)]
#[safe(true)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct CompositeIndexProperty {
    /// The path to the property referenced in this index.
    pub path: String,

    /// The order of the composite index.
    ///
    /// For example, if you want to run the query "SELECT * FROM c ORDER BY c.age asc, c.height desc",
    /// then you'd specify the order for "/asc" to be *ascending* and the order for "/height" to be *descending*.
    pub order: CompositeIndexOrder,
}

impl CompositeIndexProperty {
    /// Creates a new [`CompositeIndexProperty`] with the given path and order.
    pub fn new(path: impl Into<String>, order: CompositeIndexOrder) -> Self {
        Self {
            path: path.into(),
            order,
        }
    }

    /// Sets the path of this composite index property.
    pub fn with_path(mut self, path: impl Into<String>) -> Self {
        self.path = path.into();
        self
    }

    /// Sets the order of this composite index property.
    pub fn with_order(mut self, order: CompositeIndexOrder) -> Self {
        self.order = order;
        self
    }
}

/// Ordering values available for composite indexes.
#[derive(Clone, SafeDebug, Deserialize, Serialize, PartialEq, Eq)]
#[safe(true)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
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
#[non_exhaustive]
pub struct VectorIndex {
    /// The path to the property referenced in this index.
    pub path: String,

    /// The type of the vector index.
    #[serde(rename = "type")] // "type" is a reserved word in Rust.
    pub index_type: VectorIndexType,

    /// The quantization technique used to compress vectors in this index.
    ///
    /// Only applies to the [`QuantizedFlat`](VectorIndexType::QuantizedFlat) and
    /// [`DiskANN`](VectorIndexType::DiskANN) index types. When unset, the service
    /// chooses a default.
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantizer_type: Option<QuantizerType>,

    /// The number of bytes each vector is compressed to.
    ///
    /// Only applies to the [`QuantizedFlat`](VectorIndexType::QuantizedFlat) and
    /// [`DiskANN`](VectorIndexType::DiskANN) index types. The service constrains
    /// this to the range `1..=min(dimensions, 512)`; this type does not enforce
    /// that range, leaving the service as the source of truth.
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantization_byte_size: Option<u32>,

    /// The size of the candidate list used while building the index.
    ///
    /// Only applies to the [`DiskANN`](VectorIndexType::DiskANN) index type.
    /// Larger values improve recall at the cost of a slower build. The service
    /// constrains this to the range `25..=500`; this type does not enforce that
    /// range, leaving the service as the source of truth.
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub indexing_search_list_size: Option<u32>,

    /// The paths used to shard the vector index.
    ///
    /// Only applies to the [`QuantizedFlat`](VectorIndexType::QuantizedFlat) and
    /// [`DiskANN`](VectorIndexType::DiskANN) index types. Sharding restricts a
    /// vector search to the documents sharing the same shard key values, which
    /// improves latency for queries that always filter on those paths.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub vector_index_shard_key: Vec<String>,

    /// Properties returned by the service that this version of the SDK does not model.
    ///
    /// Captured on deserialization and written back on serialization so a
    /// read-modify-replace round trip does not silently drop server-side
    /// configuration the SDK doesn't know about yet.
    #[serde(flatten)]
    extra: BTreeMap<String, serde_json::Value>,
}

impl VectorIndex {
    /// Creates a new [`VectorIndex`] with the given path and index type.
    pub fn new(path: impl Into<String>, index_type: VectorIndexType) -> Self {
        Self {
            path: path.into(),
            index_type,
            quantizer_type: None,
            quantization_byte_size: None,
            indexing_search_list_size: None,
            vector_index_shard_key: Vec::new(),
            extra: BTreeMap::new(),
        }
    }

    /// Sets the path of this vector index.
    pub fn with_path(mut self, path: impl Into<String>) -> Self {
        self.path = path.into();
        self
    }

    /// Sets the type of this vector index.
    pub fn with_index_type(mut self, index_type: VectorIndexType) -> Self {
        self.index_type = index_type;
        self
    }

    /// Sets the quantization technique used to compress vectors in this index.
    pub fn with_quantizer_type(mut self, quantizer_type: QuantizerType) -> Self {
        self.quantizer_type = Some(quantizer_type);
        self
    }

    /// Sets the number of bytes each vector is compressed to.
    pub fn with_quantization_byte_size(mut self, quantization_byte_size: u32) -> Self {
        self.quantization_byte_size = Some(quantization_byte_size);
        self
    }

    /// Sets the size of the candidate list used while building the index.
    pub fn with_indexing_search_list_size(mut self, indexing_search_list_size: u32) -> Self {
        self.indexing_search_list_size = Some(indexing_search_list_size);
        self
    }

    /// Appends `path` to the index's list of shard key paths.
    pub fn with_shard_key_path(mut self, path: impl Into<String>) -> Self {
        self.vector_index_shard_key.push(path.into());
        self
    }
}

/// Types of vector indexes supported by Cosmos DB
#[derive(Clone, SafeDebug, Deserialize, Serialize, PartialEq, Eq)]
#[safe(true)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub enum VectorIndexType {
    /// Represents the `flat` vector index type.
    Flat,

    /// Represents the `quantizedFlat` vector index type.
    QuantizedFlat,

    /// Represents the `diskANN` vector index type.
    DiskANN,
}

/// Quantization techniques available for compressing vectors in a [`VectorIndex`].
///
/// For more information, see <https://learn.microsoft.com/azure/cosmos-db/index-policy#vector-indexes>
#[derive(Clone, SafeDebug, Deserialize, Serialize, PartialEq, Eq)]
#[safe(true)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub enum QuantizerType {
    /// Represents the `product` quantization technique.
    Product,

    /// Represents the `spherical` quantization technique.
    Spherical,
}

/// Represents a full text index.
///
/// Every path indexed here must also be declared in the container's
/// [`FullTextPolicy`](crate::models::FullTextPolicy), which controls how the
/// text at that path is analyzed.
///
/// For more information, see <https://learn.microsoft.com/azure/cosmos-db/gen-ai/full-text-search>
#[derive(Clone, Default, SafeDebug, Deserialize, Serialize, PartialEq, Eq)]
#[safe(true)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct FullTextIndex {
    /// The path to the property referenced in this index.
    pub path: String,

    /// Properties returned by the service that this version of the SDK does not model.
    ///
    /// Captured on deserialization and written back on serialization so a
    /// read-modify-replace round trip does not silently drop server-side
    /// configuration the SDK doesn't know about yet.
    #[serde(flatten)]
    extra: BTreeMap<String, serde_json::Value>,
}

impl FullTextIndex {
    /// Creates a new [`FullTextIndex`] over the given path.
    pub fn new(path: impl Into<String>) -> Self {
        Self {
            path: path.into(),
            extra: BTreeMap::new(),
        }
    }

    /// Sets the path of this full text index.
    pub fn with_path(mut self, path: impl Into<String>) -> Self {
        self.path = path.into();
        self
    }
}

impl<T: Into<String>> From<T> for FullTextIndex {
    fn from(value: T) -> Self {
        FullTextIndex::new(value)
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use crate::models::{
        CompositeIndex, CompositeIndexOrder, CompositeIndexProperty, FullTextIndex, IndexingMode,
        IndexingPolicy, QuantizerType, SpatialIndex, SpatialType, VectorIndex, VectorIndexType,
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
                "fullTextIndexes": [
                    {
                        "path": "/abstract"
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
                ]
            }
        "#;

        let policy: IndexingPolicy = serde_json::from_str(policy).unwrap();

        let expected = IndexingPolicy::default()
            .with_indexing_mode(IndexingMode::Consistent)
            .with_included_path("/*")
            .with_excluded_path("/path/to/single/excluded/property/?")
            .with_excluded_path("/path/to/root/of/multiple/excluded/properties/*")
            .with_spatial_index(
                SpatialIndex::new("/path/to/geojson/property/?")
                    .with_type(SpatialType::Point)
                    .with_type(SpatialType::Polygon)
                    .with_type(SpatialType::MultiPolygon)
                    .with_type(SpatialType::LineString),
            )
            .with_composite_index(
                CompositeIndex::default()
                    .with_property(CompositeIndexProperty::new(
                        "/name",
                        CompositeIndexOrder::Ascending,
                    ))
                    .with_property(CompositeIndexProperty::new(
                        "/age",
                        CompositeIndexOrder::Descending,
                    )),
            )
            .with_composite_index(
                CompositeIndex::default()
                    .with_property(CompositeIndexProperty::new(
                        "/name2",
                        CompositeIndexOrder::Descending,
                    ))
                    .with_property(CompositeIndexProperty::new(
                        "/age2",
                        CompositeIndexOrder::Ascending,
                    )),
            )
            .with_vector_index(VectorIndex::new("/vector1", VectorIndexType::QuantizedFlat))
            .with_vector_index(VectorIndex::new("/vector2", VectorIndexType::DiskANN))
            .with_full_text_index("/abstract");

        assert_eq!(expected, policy);
    }

    #[test]
    pub fn serialize_indexing_policy() {
        let mut policy = IndexingPolicy::default()
            .with_included_path("/*")
            .with_excluded_path("/path/to/single/excluded/property/?")
            .with_excluded_path("/path/to/root/of/multiple/excluded/properties/*")
            .with_spatial_index(
                SpatialIndex::new("/path/to/geojson/property/?")
                    .with_type(SpatialType::Point)
                    .with_type(SpatialType::Polygon)
                    .with_type(SpatialType::MultiPolygon)
                    .with_type(SpatialType::LineString),
            )
            .with_spatial_index(SpatialIndex::new("/path/to/geojson/property2/?"))
            .with_composite_index(
                CompositeIndex::default()
                    .with_property(CompositeIndexProperty::new(
                        "/name",
                        CompositeIndexOrder::Ascending,
                    ))
                    .with_property(CompositeIndexProperty::new(
                        "/age",
                        CompositeIndexOrder::Descending,
                    )),
            )
            .with_composite_index(CompositeIndex::default())
            .with_vector_index(VectorIndex::new("/vector1", VectorIndexType::QuantizedFlat))
            .with_vector_index(VectorIndex::new("/vector2", VectorIndexType::DiskANN));
        policy.automatic = true;

        let json = serde_json::to_string(&policy).unwrap();

        assert_eq!(
            "{\"automatic\":true,\"includedPaths\":[{\"path\":\"/*\"}],\"excludedPaths\":[{\"path\":\"/path/to/single/excluded/property/?\"},{\"path\":\"/path/to/root/of/multiple/excluded/properties/*\"}],\"spatialIndexes\":[{\"path\":\"/path/to/geojson/property/?\",\"types\":[\"Point\",\"Polygon\",\"MultiPolygon\",\"LineString\"]},{\"path\":\"/path/to/geojson/property2/?\",\"types\":[]}],\"compositeIndexes\":[[{\"path\":\"/name\",\"order\":\"ascending\"},{\"path\":\"/age\",\"order\":\"descending\"}],[]],\"vectorIndexes\":[{\"path\":\"/vector1\",\"type\":\"quantizedFlat\"},{\"path\":\"/vector2\",\"type\":\"diskANN\"}]}",
            json
        );
    }

    #[test]
    fn round_trips_vector_index_tuning_options() {
        let json = r#"
            {
                "vectorIndexes": [
                    {
                        "path": "/vector",
                        "type": "diskANN",
                        "quantizerType": "product",
                        "quantizationByteSize": 8,
                        "indexingSearchListSize": 50,
                        "vectorIndexShardKey": ["/city", "/country"]
                    }
                ]
            }
        "#;

        let policy: IndexingPolicy = serde_json::from_str(json).unwrap();

        assert_eq!(
            vec![VectorIndex::new("/vector", VectorIndexType::DiskANN)
                .with_quantizer_type(QuantizerType::Product)
                .with_quantization_byte_size(8)
                .with_indexing_search_list_size(50)
                .with_shard_key_path("/city")
                .with_shard_key_path("/country")],
            policy.vector_indexes
        );

        assert_eq!(
            r#"{"automatic":false,"vectorIndexes":[{"path":"/vector","type":"diskANN","quantizerType":"product","quantizationByteSize":8,"indexingSearchListSize":50,"vectorIndexShardKey":["/city","/country"]}]}"#,
            serde_json::to_string(&policy).unwrap()
        );
    }

    #[test]
    fn omits_unset_vector_index_tuning_options() {
        let index = VectorIndex::new("/vector", VectorIndexType::Flat);

        assert_eq!(
            r#"{"path":"/vector","type":"flat"}"#,
            serde_json::to_string(&index).unwrap()
        );
    }

    #[test]
    fn serializes_spherical_quantizer_type() {
        let index = VectorIndex::new("/vector", VectorIndexType::DiskANN)
            .with_quantizer_type(QuantizerType::Spherical);

        assert_eq!(
            r#"{"path":"/vector","type":"diskANN","quantizerType":"spherical"}"#,
            serde_json::to_string(&index).unwrap()
        );
    }

    #[test]
    fn round_trips_full_text_indexes() {
        let policy = IndexingPolicy::default()
            .with_full_text_index("/abstract")
            .with_full_text_index(FullTextIndex::new("/title"));

        let json = serde_json::to_string(&policy).unwrap();

        assert_eq!(
            r#"{"automatic":false,"fullTextIndexes":[{"path":"/abstract"},{"path":"/title"}]}"#,
            json
        );
        assert_eq!(policy, serde_json::from_str(&json).unwrap());
    }

    #[test]
    fn preserves_unknown_fields_through_round_trip() {
        // A policy configured by a newer SDK (or the portal) must survive a
        // read-modify-replace round trip through this SDK untouched.
        let json = r#"
            {
                "indexingMode": "consistent",
                "vectorIndexes": [
                    {
                        "path": "/vector",
                        "type": "diskANN",
                        "someFutureVectorKnob": 7
                    }
                ],
                "fullTextIndexes": [
                    {
                        "path": "/abstract",
                        "someFutureTextKnob": "on"
                    }
                ],
                "someFuturePolicyKnob": {
                    "nested": [1, 2, 3]
                }
            }
        "#;

        let policy: IndexingPolicy = serde_json::from_str(json).unwrap();
        let round_tripped: serde_json::Value =
            serde_json::from_str(&serde_json::to_string(&policy).unwrap()).unwrap();

        assert_eq!(
            json!({"nested": [1, 2, 3]}),
            round_tripped["someFuturePolicyKnob"]
        );
        assert_eq!(
            json!(7),
            round_tripped["vectorIndexes"][0]["someFutureVectorKnob"]
        );
        assert_eq!(
            json!("on"),
            round_tripped["fullTextIndexes"][0]["someFutureTextKnob"]
        );

        // The modelled fields must still be readable alongside the unknown ones.
        assert_eq!(Some(IndexingMode::Consistent), policy.indexing_mode);
        assert_eq!("/vector", policy.vector_indexes[0].path);
        assert_eq!("/abstract", policy.full_text_indexes[0].path);
    }
}

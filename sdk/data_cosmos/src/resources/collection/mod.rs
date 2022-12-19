//! Utilities for interacting with [`Collection`]s.

mod offer;

use super::Resource;
pub use offer::Offer;

/// A container of JSON documents and associated JavaScript application logic.
///
/// You can learn more about Collections [here](https://docs.microsoft.com/rest/api/cosmos-db/collections).
#[derive(Serialize, Deserialize, Clone, Debug, PartialOrd, PartialEq, Eq)]
pub struct Collection {
    /// The collection id
    pub id: String,
    /// The indexing policy
    #[serde(rename = "indexingPolicy")]
    pub indexing_policy: IndexingPolicy,
    /// The partition key
    #[serde(rename = "partitionKey")]
    pub parition_key: PartitionKey,
    /// The resource id
    #[serde(rename = "_rid")]
    pub rid: String,
    /// The last updated timestamp
    #[serde(rename = "_ts")]
    pub ts: u64,
    /// The resource's uri
    #[serde(rename = "_self")]
    pub _self: String,
    /// The resource's etag used for concurrency control
    #[serde(rename = "_etag")]
    pub etag: String,
    /// the addressable path of the documents resource
    #[serde(rename = "_docs")]
    pub docs: String,
    /// the addressable path of the stored procedures resource
    #[serde(rename = "_sprocs")]
    pub sprocs: String,
    /// the addressable path of the triggers resource
    #[serde(rename = "_triggers")]
    pub triggers: String,
    /// the addressable path of the user-defined functions (udfs) resource
    #[serde(rename = "_udfs")]
    pub udfs: String,
    /// the addressable path of the conflicts resource
    #[serde(rename = "_conflicts")]
    pub conflicts: String,
}

impl Resource for Collection {
    fn uri(&self) -> &str {
        &self._self
    }
}

impl Resource for &Collection {
    fn uri(&self) -> &str {
        &self._self
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialOrd, PartialEq, Eq)]
/// The type of index
pub enum KeyKind {
    /// useful for equality comparisons
    Hash,
    /// seful for equality, range comparisons and sorting
    Range,
    /// useful for spatial queries
    Spatial,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialOrd, PartialEq, Eq)]
/// The datatype for which the indexing behavior is applied to
#[allow(missing_docs)]
pub enum DataType {
    String,
    Number,
    Point,
    Polygon,
    LineString,
}

/// The indexing mode
#[derive(Serialize, Deserialize, Clone, Debug, PartialOrd, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum IndexingMode {
    /// indexing occurs synchronously during insertion, replacment or deletion of documents
    Consistent,
    /// indexing occurs asynchronously during insertion, replacment or deletion of documents
    Lazy,
}

/// Path to be indexed
#[allow(missing_docs)]
#[derive(Serialize, Deserialize, Clone, Debug, PartialOrd, PartialEq, Eq)]
pub struct IncludedPath {
    pub path: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "indexes")]
    pub indexes: Option<Vec<IncludedPathIndex>>,
}

/// An indexed description
#[derive(Serialize, Deserialize, Clone, Debug, PartialOrd, PartialEq, Eq)]
pub struct IncludedPathIndex {
    /// The datatype for which the indexing behavior is applied to
    #[serde(rename = "dataType")]
    pub data_type: DataType,
    /// The precision of the index
    #[serde(skip_serializing_if = "Option::is_none")]
    pub precision: Option<i8>,
    /// The type of index
    pub kind: KeyKind,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialOrd, PartialEq, Eq)]
/// Path that is excluded from indexing
pub struct ExcludedPath {
    #[allow(missing_docs)]
    pub path: String,
}

impl From<String> for ExcludedPath {
    fn from(s: String) -> Self {
        Self { path: s }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialOrd, PartialEq, Eq)]
/// The partitioning configuration settings for collection
pub struct PartitionKey {
    /// An array of paths using which data within the collection can be partitioned
    pub paths: Vec<String>,
    /// The algorithm used for partitioning
    pub kind: KeyKind,
}

impl std::default::Default for PartitionKey {
    fn default() -> Self {
        Self {
            paths: vec![],
            kind: KeyKind::Hash,
        }
    }
}

impl<T> From<T> for PartitionKey
where
    T: AsRef<str>,
{
    fn from(t: T) -> Self {
        Self {
            paths: vec![t.as_ref().to_owned()],
            kind: KeyKind::Hash,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialOrd, PartialEq, Eq)]
/// The indexing policy for a collection
#[serde(rename_all = "camelCase")]
pub struct IndexingPolicy {
    /// Whether automatic indexing is on or off
    pub automatic: bool,
    /// The mode of indexing
    pub indexing_mode: IndexingMode,
    /// Array containing document paths to be indexed
    pub included_paths: Vec<IncludedPath>,
    /// Array containing document paths to be excluded from indexing
    pub excluded_paths: Vec<ExcludedPath>,
}

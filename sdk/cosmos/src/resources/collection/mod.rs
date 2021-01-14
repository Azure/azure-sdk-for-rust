//! Utilities for interacting with [`Collection`]s.

mod offer;

use super::Resource;
pub use offer::Offer;

/// A container of JSON documents and associated JavaScript application logic.
///
/// You can learn more about Collections [here](https://docs.microsoft.com/en-us/rest/api/cosmos-db/collections).
#[derive(Serialize, Deserialize, Clone, Debug, PartialOrd, PartialEq)]
pub struct Collection {
    pub id: String,
    #[serde(rename = "indexingPolicy")]
    pub indexing_policy: IndexingPolicy,
    #[serde(rename = "partitionKey")]
    pub parition_key: PartitionKey,
    #[serde(rename = "_rid")]
    pub rid: String,
    #[serde(rename = "_ts")]
    pub ts: u64,
    #[serde(rename = "_self")]
    pub _self: String,
    #[serde(rename = "_etag")]
    pub etag: String,
    #[serde(rename = "_docs")]
    pub docs: String,
    #[serde(rename = "_sprocs")]
    pub sprocs: String,
    #[serde(rename = "_triggers")]
    pub triggers: String,
    #[serde(rename = "_udfs")]
    pub udfs: String,
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

#[derive(Serialize, Deserialize, Clone, Debug, PartialOrd, PartialEq)]
pub enum KeyKind {
    Hash,
    Range,
    Spatial,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialOrd, PartialEq)]
pub enum DataType {
    String,
    Number,
    Point,
    Polygon,
    LineString,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialOrd, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum IndexingMode {
    Consistent,
    Lazy,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialOrd, PartialEq)]
pub struct IncludedPath {
    pub path: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "indexes")]
    pub indexes: Option<Vec<IncludedPathIndex>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialOrd, PartialEq)]
pub struct IncludedPathIndex {
    #[serde(rename = "dataType")]
    pub data_type: DataType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub precision: Option<i8>,
    pub kind: KeyKind,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialOrd, PartialEq)]
pub struct ExcludedPath {
    pub path: String,
}

impl From<String> for ExcludedPath {
    fn from(s: String) -> Self {
        Self { path: s }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialOrd, PartialEq)]
pub struct PartitionKey {
    pub paths: Vec<String>,
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

#[derive(Serialize, Deserialize, Clone, Debug, PartialOrd, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct IndexingPolicy {
    pub automatic: bool,
    pub indexing_mode: IndexingMode,
    pub included_paths: Vec<IncludedPath>,
    pub excluded_paths: Vec<ExcludedPath>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum KeyKind {
    Hash,
    Range,
    Spatial,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum DataType {
    String,
    Number,
    Point,
    Polygon,
    LineString,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct IncludedPath {
    #[serde(rename = "path")]
    path: String,
    #[serde(rename = "indexes")]
    indexes: Vec<IncludedPathIndex>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct IncludedPathIndex {
    #[serde(rename = "kind")]
    kind: KeyKind,
    #[serde(rename = "dataType")]
    data_type: DataType,
    #[serde(rename = "precision")]
    precision: Option<i8>,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct ExcludedPath {
    #[serde(rename = "path")]
    path: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PartitionKey {
    paths: Vec<String>,
    kind: KeyKind,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct IndexingPolicy {
    #[serde(rename = "automatic")]
    automatic: bool,
    #[serde(rename = "indexingMode")]
    indexing_mode: String,
    #[serde(rename = "includedPaths")]
    included_paths: Vec<IncludedPath>,
    #[serde(rename = "excludedPaths")]
    excluded_paths: Vec<ExcludedPath>,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct Collection {
    id: String,
    #[serde(rename = "indexingPolicy")]
    indexing_policy: IndexingPolicy,
    #[serde(rename = "partitionKey")]
    parition_key: PartitionKey,
    #[serde(rename = "_rid")]
    rid: String,
    #[serde(rename = "_ts")]
    ts: String,
    #[serde(rename = "_self")]
    _self: String,
    #[serde(rename = "_etag")]
    etag: String,
    #[serde(rename = "_docs")]
    docs: String,
    #[serde(rename = "_sprocs")]
    sprocs: String,
    #[serde(rename = "_triggers")]
    triggers: String,
    #[serde(rename = "_udfs")]
    udfs: String,
    #[serde(rename = "_conflicts")]
    conflicts: String,
}

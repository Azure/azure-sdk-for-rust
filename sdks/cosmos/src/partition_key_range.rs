#[derive(Debug, PartialEq, PartialOrd, Clone, Deserialize)]
pub struct PartitionKeyRange {
    #[serde(rename = "_rid")]
    pub rid: String,
    pub id: String,
    #[serde(rename = "_etag")]
    pub etag: String,
    #[serde(rename = "minInclusive")]
    pub min_exclusive: String,
    #[serde(rename = "maxExclusive")]
    pub max_exclusive: String,
    #[serde(rename = "ridPrefix")]
    pub rid_prefix: u64,
    pub _self: String,
    #[serde(rename = "throughputFraction")]
    pub throughput_fraction: u64,
    pub status: String,
    // TODO: parents
    #[serde(rename = "_ts")]
    pub ts: u64,
}

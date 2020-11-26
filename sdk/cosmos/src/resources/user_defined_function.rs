#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UserDefinedFunction {
    pub id: String,
    #[serde(rename = "_rid")]
    pub rid: String,
    #[serde(rename = "_ts")]
    pub ts: u64,
    #[serde(rename = "_self")]
    pub _self: String,
    #[serde(rename = "_etag")]
    pub etag: String,
    pub body: String,
}

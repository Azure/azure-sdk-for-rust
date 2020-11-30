/// An extension of the core of the Cosmos DB query language.
///
/// You can learn more about user defined functions [here](https://docs.microsoft.com/en-us/rest/api/cosmos-db/user-defined-functions).
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

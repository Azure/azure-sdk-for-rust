/// An extension of the core of the Cosmos DB query language.
///
/// You can learn more about user defined functions [here](https://docs.microsoft.com/rest/api/cosmos-db/user-defined-functions).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserDefinedFunction {
    /// The function id
    pub id: String,
    /// The resource id
    #[serde(rename = "_rid")]
    pub rid: String,
    /// The last updated timestamp
    #[serde(rename = "_ts")]
    pub ts: u64,
    /// The unique uri for this resource
    #[serde(rename = "_self")]
    pub _self: String,
    #[serde(rename = "_etag")]
    /// The function's etag used for concurrency control
    pub etag: String,
    /// The function's body
    pub body: String,
}

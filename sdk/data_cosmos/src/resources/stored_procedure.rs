//! Utilities for interacting with [`StoredProcedure`]s.

/// Stored procedure's parameters
pub type Parameters = crate::to_json_vector::ToJsonVector;

/// A piece of application logic that is registered and executed against a collection as a single transaction
///
/// You can learn more about stored procedures [here](https://docs.microsoft.com/rest/api/cosmos-db/stored-procedures).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StoredProcedure {
    /// The procedure id
    pub id: String,
    #[serde(rename = "_rid")]
    /// The resource id
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
    /// The body
    pub body: String,
}

impl StoredProcedure {
    /// The name of the stored procedure
    #[must_use]
    pub fn name(&self) -> &str {
        &self.id
    }
}

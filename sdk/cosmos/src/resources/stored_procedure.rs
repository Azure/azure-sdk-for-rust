//! Utilities for interacting with [`StoredProcedure`]s.

pub type Parameters = crate::to_json_vector::ToJsonVector;

/// A piece of application logic that is registered and executed against a collection as a single transaction
///
/// You can learn more about stored procedures [here](https://docs.microsoft.com/en-us/rest/api/cosmos-db/stored-procedures).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StoredProcedure {
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

impl StoredProcedureName for StoredProcedure {
    fn name(&self) -> &str {
        &self.id
    }
}
pub trait StoredProcedureName: std::fmt::Debug {
    fn name(&self) -> &str;
}

impl StoredProcedureName for &str {
    fn name(&self) -> &str {
        self
    }
}

impl StoredProcedureName for String {
    fn name(&self) -> &str {
        self.as_ref()
    }
}

//! Utilities for interacting with [`Database`]s.

use super::Resource;

/// A logical namespace for collections, users, and permissions.
///
/// You can learn more about Databases [here](https://docs.microsoft.com/en-us/rest/api/cosmos-db/databases).
#[derive(Serialize, Clone, PartialEq, PartialOrd, Deserialize, Debug)]
pub struct Database {
    pub id: String,
    #[serde(rename = "_rid")]
    pub rid: String,
    #[serde(rename = "_ts")]
    pub ts: u64,
    #[serde(rename = "_self")]
    pub _self: String,
    #[serde(rename = "_etag")]
    pub etag: String,
    #[serde(rename = "_colls")]
    pub colls: String,
    #[serde(rename = "_users")]
    pub users: String,
}

impl Database {
    /// The name of the database
    pub fn name(&self) -> &str {
        &self.id
    }
}

impl Resource for Database {
    fn uri(&self) -> &str {
        &self._self
    }
}

impl Resource for &Database {
    fn uri(&self) -> &str {
        &self._self
    }
}

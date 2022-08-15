//! Utilities for interacting with [`Database`]s.

use super::Resource;

/// A logical namespace for collections, users, and permissions.
///
/// You can learn more about Databases [here](https://docs.microsoft.com/rest/api/cosmos-db/databases).
#[derive(Serialize, Clone, PartialEq, Eq, PartialOrd, Deserialize, Debug)]
pub struct Database {
    /// The database id
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
    /// The path to the database collections resource
    #[serde(rename = "_colls")]
    pub colls: String,
    /// The path to the database users resource
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

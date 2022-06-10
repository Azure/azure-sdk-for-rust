use azure_core::Etag;
use bytes::Bytes;
use chrono::{DateTime, Utc};
use std::convert::TryFrom;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FileSystem {
    pub name: String,
    #[serde(with = "azure_core::parsing::rfc2822_time_format")]
    pub last_modified: DateTime<Utc>,
    pub etag: Etag,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct FileSystemList {
    #[serde(rename = "filesystems")]
    pub file_systems: Vec<FileSystem>,
}

impl TryFrom<Bytes> for FileSystemList {
    type Error = crate::Error;

    fn try_from(response: Bytes) -> Result<Self, Self::Error> {
        Ok(serde_json::from_slice::<FileSystemList>(response.as_ref())?)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Path {
    pub content_length: i64,
    pub etag: Etag,
    pub group: String,
    pub is_directory: bool,
    #[serde(with = "azure_core::parsing::rfc2822_time_format")]
    pub last_modified: DateTime<Utc>,
    pub name: String,
    pub owner: String,
    pub permissions: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct PathList {
    pub paths: Vec<Path>,
}

impl TryFrom<Bytes> for PathList {
    type Error = azure_core::error::Error;

    fn try_from(response: Bytes) -> Result<Self, Self::Error> {
        Ok(serde_json::from_slice::<PathList>(response.as_ref())?)
    }
}

use azure_core::errors::AzureError;
use azure_core::prelude::Etag;
use bytes::Bytes;
use chrono::{DateTime, Utc};
use http::Response;
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

impl TryFrom<&Response<Bytes>> for FileSystemList {
    type Error = AzureError;

    fn try_from(response: &Response<Bytes>) -> Result<Self, Self::Error> {
        trace!("{}", std::str::from_utf8(response.body())?);

        let file_system_list: FileSystemList = serde_json::from_slice(response.body())?;
        Ok(file_system_list)
    }
}

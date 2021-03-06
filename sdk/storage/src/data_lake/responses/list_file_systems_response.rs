use crate::data_lake::file_system::FileSystemList;
use crate::data_lake::FileSystem;
use azure_core::{errors::AzureError, headers::CommonStorageResponseHeaders, prelude::NextMarker};
use bytes::Bytes;
use http::Response;
use std::convert::{TryFrom, TryInto};

#[derive(Debug, Clone)]
pub struct ListFileSystemsResponse {
    pub common_storage_response_headers: CommonStorageResponseHeaders,
    pub file_systems: Vec<FileSystem>,
    pub next_marker: Option<NextMarker>,
}

impl TryFrom<&Response<Bytes>> for ListFileSystemsResponse {
    type Error = AzureError;

    fn try_from(response: &Response<Bytes>) -> Result<Self, Self::Error> {
        trace!("{}", std::str::from_utf8(response.body())?);
        trace!("{:?}", response.headers());

        let file_system_list: FileSystemList = response.try_into()?;

        Ok(ListFileSystemsResponse {
            common_storage_response_headers: response.headers().try_into()?,
            file_systems: file_system_list.file_systems,
            next_marker: NextMarker::from_header_optional(response.headers())?,
        })
    }
}

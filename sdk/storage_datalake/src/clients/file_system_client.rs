use super::{DataLakeClient, DirectoryClient, FileClient};
use crate::{operations::*, Properties};
use azure_core::Url;

#[derive(Debug, Clone)]
pub struct FileSystemClient {
    data_lake_client: DataLakeClient,
    name: String,
}

impl FileSystemClient {
    pub(crate) fn new(data_lake_client: DataLakeClient, name: String) -> Self {
        Self {
            data_lake_client,
            name,
        }
    }

    pub(crate) fn url(&self) -> azure_core::Result<Url> {
        Ok(self.data_lake_client.url()?.join(&self.name)?)
    }

    pub fn get_directory_client<P>(&self, path: P) -> DirectoryClient
    where
        P: Into<String>,
    {
        DirectoryClient::new(self.clone(), path.into())
    }

    pub fn into_directory_client<P>(self, path: P) -> DirectoryClient
    where
        P: Into<String>,
    {
        DirectoryClient::new(self, path.into())
    }

    pub fn get_file_client<P>(&self, path: P) -> FileClient
    where
        P: Into<String>,
    {
        FileClient::new(self.clone(), path.into())
    }

    pub fn into_file_client<P>(self, path: P) -> FileClient
    where
        P: Into<String>,
    {
        FileClient::new(self, path.into())
    }

    pub fn list_paths(&self) -> ListPathsBuilder {
        ListPathsBuilder::new(self.clone()).recursive(true)
    }

    pub fn create(&self) -> CreateFileSystemBuilder {
        CreateFileSystemBuilder::new(self.clone())
    }

    pub fn delete(&self) -> DeleteFileSystemBuilder {
        DeleteFileSystemBuilder::new(self.clone())
    }

    pub fn get_properties(&self) -> GetFileSystemPropertiesBuilder {
        GetFileSystemPropertiesBuilder::new(self.clone())
    }

    pub fn set_properties(&self, properties: Properties) -> SetFileSystemPropertiesBuilder {
        SetFileSystemPropertiesBuilder::new(self.clone(), properties)
    }

    pub(crate) async fn send(
        &self,
        ctx: &mut azure_core::Context,
        request: &mut azure_core::Request,
    ) -> azure_core::Result<azure_core::Response> {
        self.data_lake_client.send(ctx, request).await
    }
}

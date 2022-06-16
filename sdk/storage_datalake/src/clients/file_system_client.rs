use super::{DataLakeClient, DirectoryClient, FileClient};
use crate::operations::*;
use crate::Properties;
use azure_core::{error::Result, ClientOptions, Context, Pipeline};
use azure_storage::core::storage_shared_key_credential::StorageSharedKeyCredential;
use url::Url;

#[derive(Debug, Clone)]
pub struct FileSystemClient {
    data_lake_client: DataLakeClient,
    name: String,
    pub(crate) context: Context,
}

impl FileSystemClient {
    pub(crate) fn new(data_lake_client: DataLakeClient, name: String) -> Self {
        let context = data_lake_client.context.clone();

        Self {
            data_lake_client,
            name,
            context,
        }
    }

    pub fn new_with_options<FS>(
        credential: StorageSharedKeyCredential,
        custom_dns_suffix: Option<String>,
        options: ClientOptions,
        file_system_name: FS,
    ) -> Self
    where
        FS: Into<String>,
    {
        DataLakeClient::new_with_shared_key(credential, custom_dns_suffix, options)
            .into_file_system_client(file_system_name.into())
    }

    pub(crate) fn url(&self) -> Result<Url> {
        Ok(url::Url::parse(self.data_lake_client.url())?.join(&self.name)?)
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
        ListPathsBuilder::new(self.clone(), self.context.clone()).recursive(true)
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
        SetFileSystemPropertiesBuilder::new(self.clone(), Some(properties))
    }

    pub(crate) fn prepare_request(
        &self,
        uri: &str,
        http_method: http::Method,
    ) -> azure_core::Request {
        self.data_lake_client.prepare_request(uri, http_method)
    }

    pub(crate) fn pipeline(&self) -> &Pipeline {
        self.data_lake_client.pipeline()
    }
}

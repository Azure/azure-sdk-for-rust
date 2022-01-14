use super::{DataLakeClient, DirectoryClient, FileClient};
use crate::operations::*;
use crate::{Properties, Result};
use azure_core::{ClientOptions, Context, Pipeline};
use azure_storage::core::storage_shared_key_credential::StorageSharedKeyCredential;
use bytes::Bytes;
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
        DataLakeClient::new_with_options(credential, custom_dns_suffix, options)
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

    pub async fn append_to_file(
        &self,
        ctx: Context,
        file_path: &str,
        bytes: Bytes,
        position: i64,
        options: FileAppendOptions,
    ) -> Result<FileAppendResponse> {
        let mut request = self.prepare_file_append_request(file_path, position);

        options.decorate_request(&mut request, bytes)?;
        let response = self.pipeline().send(&mut ctx.clone(), &mut request).await?;

        Ok(FileAppendResponse::try_from(response).await?)
    }

    pub async fn flush_file(
        &self,
        ctx: Context,
        file_path: &str,
        position: i64,
        close: bool,
        options: FileFlushOptions,
    ) -> Result<FileFlushResponse> {
        let mut request = self.prepare_file_flush_request(file_path, position, close);
        options.decorate_request(&mut request)?;
        let response = self.pipeline().send(&mut ctx.clone(), &mut request).await?;

        Ok(FileFlushResponse::try_from(response).await?)
    }

    pub(crate) fn prepare_request(
        &self,
        uri: &str,
        http_method: http::Method,
    ) -> azure_core::Request {
        self.data_lake_client
            .prepare_request_pipeline(uri, http_method)
    }

    pub(crate) fn prepare_file_append_request(
        &self,
        file_path: &str,
        position: i64,
    ) -> azure_core::Request {
        let uri = format!(
            "{}/{}?action=append&position={}",
            self.url().unwrap(),
            file_path,
            position
        );
        http::request::Request::patch(uri)
            .body(bytes::Bytes::new()) // Request builder requires a body here
            .unwrap()
            .into()
    }

    pub(crate) fn prepare_file_flush_request(
        &self,
        file_path: &str,
        position: i64,
        close: bool,
    ) -> azure_core::Request {
        let uri = format!(
            "{}/{}?action=flush&position={}&close={}",
            self.url().unwrap(),
            file_path,
            position,
            close,
        );
        http::request::Request::patch(uri)
            .body(bytes::Bytes::new()) // Request builder requires a body here
            .unwrap()
            .into()
    }

    pub(crate) fn pipeline(&self) -> &Pipeline {
        self.data_lake_client.pipeline()
    }
}

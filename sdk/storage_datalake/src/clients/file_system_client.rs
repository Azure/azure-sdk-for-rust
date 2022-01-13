use super::DirectoryClient;
use crate::operations::*;
use crate::{clients::DataLakeClient, Properties, Result};
use azure_core::prelude::IfMatchCondition;
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

    pub fn create(&self) -> CreateFileSystemBuilder {
        CreateFileSystemBuilder::new(self.clone())
    }

    pub fn delete(&self) -> DeleteFileSystemBuilder {
        DeleteFileSystemBuilder::new(self.clone())
    }

    pub fn get_properties(&self) -> GetFileSystemPropertiesBuilder {
        GetFileSystemPropertiesBuilder::new(self.clone())
    }

    pub fn set_properties(&self, properties: Option<Properties>) -> SetFileSystemPropertiesBuilder {
        SetFileSystemPropertiesBuilder::new(self.clone(), properties)
    }

    pub async fn create_file(
        &self,
        ctx: Context,
        file_path: &str,
        options: FileCreateOptions,
    ) -> Result<FileCreateResponse> {
        let mut request = self.prepare_file_create_request(file_path);

        options.decorate_request(&mut request)?;
        let response = self.pipeline().send(&mut ctx.clone(), &mut request).await?;

        Ok(FileCreateResponse::try_from(response).await?)
    }

    pub async fn create_file_if_not_exists(
        &self,
        ctx: Context,
        file_path: &str,
    ) -> Result<FileCreateResponse> {
        let options = FileCreateOptions::new()
            .if_match_condition(IfMatchCondition::NotMatch("*".to_string()));

        let mut request = self.prepare_file_create_request(file_path);

        options.decorate_request(&mut request)?;
        let response = self.pipeline().send(&mut ctx.clone(), &mut request).await?;

        Ok(FileCreateResponse::try_from(response).await?)
    }

    pub async fn delete_file(
        &self,
        ctx: Context,
        file_path: &str,
        options: FileDeleteOptions,
    ) -> Result<FileDeleteResponse> {
        let mut request = self.prepare_file_delete_request(file_path);

        options.decorate_request(&mut request)?;
        let response = self.pipeline().send(&mut ctx.clone(), &mut request).await?;

        Ok(FileDeleteResponse::try_from(response).await?)
    }

    pub async fn rename_file(
        &self,
        ctx: Context,
        source_file_path: &str,
        destination_file_path: &str,
        options: FileRenameOptions,
    ) -> Result<FileRenameResponse> {
        let mut request = self.prepare_file_rename_request(destination_file_path);

        let rename_source = format!("/{}/{}", &self.name, source_file_path);
        options.decorate_request(&mut request, rename_source.as_str())?;
        let response = self.pipeline().send(&mut ctx.clone(), &mut request).await?;

        Ok(FileRenameResponse::try_from(response).await?)
    }

    pub async fn rename_file_if_not_exists(
        &self,
        ctx: Context,
        source_file_path: &str,
        destination_file_path: &str,
    ) -> Result<FileRenameResponse> {
        let options = FileRenameOptions::new()
            .if_match_condition(IfMatchCondition::NotMatch("*".to_string()));

        let mut request = self.prepare_file_rename_request(destination_file_path);

        let rename_source = format!("/{}/{}", &self.name, source_file_path);
        options.decorate_request(&mut request, rename_source.as_str())?;
        let response = self.pipeline().send(&mut ctx.clone(), &mut request).await?;

        Ok(FileRenameResponse::try_from(response).await?)
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

    pub(crate) fn prepare_request_pipeline(
        &self,
        uri: &str,
        http_method: http::Method,
    ) -> azure_core::Request {
        self.data_lake_client
            .prepare_request_pipeline(uri, http_method)
    }

    pub(crate) fn prepare_file_create_request(&self, file_path: &str) -> azure_core::Request {
        let uri = format!("{}/{}?resource=file", self.url().unwrap(), file_path);
        http::request::Request::put(uri)
            .body(bytes::Bytes::new()) // Request builder requires a body here
            .unwrap()
            .into()
    }

    pub(crate) fn prepare_file_delete_request(&self, file_path: &str) -> azure_core::Request {
        let uri = format!("{}/{}", self.url().unwrap(), file_path);
        http::request::Request::delete(uri)
            .body(bytes::Bytes::new()) // Request builder requires a body here
            .unwrap()
            .into()
    }

    pub(crate) fn prepare_file_rename_request(
        &self,
        destination_file_path: &str,
    ) -> azure_core::Request {
        let uri = format!(
            "{}/{}?mode=legacy",
            self.url().unwrap(),
            destination_file_path
        );
        http::request::Request::put(uri)
            .body(bytes::Bytes::new()) // Request builder requires a body here
            .unwrap()
            .into()
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

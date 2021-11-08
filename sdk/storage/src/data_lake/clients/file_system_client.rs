use crate::data_lake::authorization_policy::DataLakeContext;
use crate::data_lake::operations::*;
use crate::data_lake::requests::*;
use crate::{data_lake::clients::DataLakeClient, Properties};
use azure_core::pipeline::Pipeline;
use azure_core::prelude::IfMatchCondition;
use azure_core::{Context, HttpClient, PipelineContext};
use bytes::Bytes;
use std::sync::Arc;
use url::Url;

pub trait AsFileSystemClient<A: Into<String>> {
    fn as_file_system_client(&self, name: A) -> Result<Arc<FileSystemClient>, url::ParseError>;
}

impl<A: Into<String>> AsFileSystemClient<A> for Arc<DataLakeClient> {
    fn as_file_system_client(&self, name: A) -> Result<Arc<FileSystemClient>, url::ParseError> {
        FileSystemClient::new(self.clone(), name.into())
    }
}

#[derive(Debug, Clone)]
pub struct FileSystemClient {
    data_lake_client: Arc<DataLakeClient>,
    name: String,
    url: Url,
}

impl FileSystemClient {
    pub(crate) fn new(
        data_lake_client: Arc<DataLakeClient>,
        name: String,
    ) -> Result<Arc<Self>, url::ParseError> {
        let mut url = data_lake_client.url().to_owned();
        url.path_segments_mut()
            .map_err(|_| url::ParseError::SetHostOnCannotBeABaseUrl)?
            .push(&name);

        Ok(Arc::new(Self {
            data_lake_client,
            name,
            url,
        }))
    }

    pub fn create(&self) -> CreateFileSystemBuilder {
        CreateFileSystemBuilder::new(self)
    }

    pub fn delete(&self) -> DeleteFileSystemBuilder {
        DeleteFileSystemBuilder::new(self)
    }

    pub fn get_properties(&self) -> GetFileSystemPropertiesBuilder {
        GetFileSystemPropertiesBuilder::new(self)
    }

    pub fn set_properties<'a>(
        &'a self,
        properties: Option<&'a Properties<'a, 'a>>,
    ) -> SetFileSystemPropertiesBuilder {
        SetFileSystemPropertiesBuilder::new(self, properties)
    }

    pub async fn create_file(
        &self,
        ctx: Context,
        file_path: &str,
        options: FileCreateOptions<'_>,
    ) -> Result<FileCreateResponse, crate::Error> {
        let mut request = self.prepare_file_create_request(file_path);
        let contents = DataLakeContext {};
        let mut pipeline_context = PipelineContext::new(ctx, contents);

        options.decorate_request(&mut request)?;
        let response = self
            .pipeline()
            .send(&mut pipeline_context, &mut request)
            .await?;

        Ok(FileCreateResponse::try_from(response).await?)
    }

    pub async fn create_file_if_not_exists(
        &self,
        ctx: Context,
        file_path: &str,
    ) -> Result<FileCreateResponse, crate::Error> {
        let options = FileCreateOptions::new().if_match_condition(IfMatchCondition::NotMatch("*"));

        let mut request = self.prepare_file_create_request(file_path);
        let contents = DataLakeContext {};
        let mut pipeline_context = PipelineContext::new(ctx, contents);

        options.decorate_request(&mut request)?;
        let response = self
            .pipeline()
            .send(&mut pipeline_context, &mut request)
            .await?;

        Ok(FileCreateResponse::try_from(response).await?)
    }

    pub async fn rename_file(
        &self,
        ctx: Context,
        source_file_path: &str,
        destination_file_path: &str,
        options: FileRenameOptions<'_>,
    ) -> Result<FileRenameResponse, crate::Error> {
        let mut request = self.prepare_file_rename_request(destination_file_path);
        let contents = DataLakeContext {};
        let mut pipeline_context = PipelineContext::new(ctx, contents);

        let rename_source = format!("/{}/{}", &self.name, source_file_path);
        options.decorate_request(&mut request, rename_source.as_str())?;
        let response = self
            .pipeline()
            .send(&mut pipeline_context, &mut request)
            .await?;

        Ok(FileRenameResponse::try_from(response).await?)
    }

    pub async fn append_to_file(
        &self,
        ctx: Context,
        file_path: &str,
        bytes: Bytes,
        position: i64,
        options: FileAppendOptions<'_>,
    ) -> Result<FileAppendResponse, crate::Error> {
        let mut request = self.prepare_file_append_request(file_path, position);
        let contents = DataLakeContext {};
        let mut pipeline_context = PipelineContext::new(ctx, contents);

        options.decorate_request(&mut request, bytes)?;
        let response = self
            .pipeline()
            .send(&mut pipeline_context, &mut request)
            .await?;

        Ok(FileAppendResponse::try_from(response).await?)
    }

    pub async fn flush_file(
        &self,
        ctx: Context,
        file_path: &str,
        position: i64,
        options: FileFlushOptions<'_>,
    ) -> Result<FileFlushResponse, crate::Error> {
        let mut request = self.prepare_file_flush_request(file_path, position);
        let contents = DataLakeContext {};
        let mut pipeline_context = PipelineContext::new(ctx, contents);

        options.decorate_request(&mut request)?;
        let response = self
            .pipeline()
            .send(&mut pipeline_context, &mut request)
            .await?;

        Ok(FileFlushResponse::try_from(response).await?)
    }

    pub(crate) fn http_client(&self) -> &dyn HttpClient {
        self.data_lake_client.http_client()
    }

    pub(crate) fn url(&self) -> &Url {
        &self.url
    }

    /// Note: This is part of the old (non-pipeline) architecture. Eventually this method will disappear.
    pub(crate) fn prepare_request(
        &self,
        url: &str,
        method: &http::method::Method,
        http_header_adder: &dyn Fn(http::request::Builder) -> http::request::Builder,
        request_body: Option<Bytes>,
    ) -> crate::Result<(http::request::Request<Bytes>, url::Url)> {
        self.data_lake_client
            .prepare_request(url, method, http_header_adder, request_body)
    }

    pub(crate) fn prepare_file_create_request(&self, file_path: &str) -> azure_core::Request {
        let uri = format!("{}/{}?resource=file", self.url(), file_path);
        http::request::Request::put(uri)
            .body(bytes::Bytes::new()) // Request builder requires a body here
            .unwrap()
            .into()
    }

    pub(crate) fn prepare_file_rename_request(
        &self,
        destination_file_path: &str,
    ) -> azure_core::Request {
        let uri = format!("{}/{}?mode=legacy", self.url(), destination_file_path);
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
            self.url(),
            file_path,
            position
        );
        http::request::Request::patch(uri)
            .body(bytes::Bytes::new()) // Request builder requires a body here
            .unwrap()
            .into()
    }

    // TODO: take close as param
    pub(crate) fn prepare_file_flush_request(
        &self,
        file_path: &str,
        position: i64,
    ) -> azure_core::Request {
        let uri = format!(
            "{}/{}?action=flush&position={}&close=true",
            self.url(),
            file_path,
            position
        );
        http::request::Request::patch(uri)
            .body(bytes::Bytes::new()) // Request builder requires a body here
            .unwrap()
            .into()
    }

    pub(crate) fn pipeline(&self) -> &Pipeline<DataLakeContext> {
        self.data_lake_client.pipeline()
    }
}

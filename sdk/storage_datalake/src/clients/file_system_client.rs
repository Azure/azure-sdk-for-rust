use crate::operations::*;
use crate::Properties;
use azure_core::headers::*;
use azure_core::prelude::IfMatchCondition;
use azure_core::Context;
use azure_core::Pipeline;
use azure_storage::prelude::StorageAccountClient;
use bytes::Bytes;
use url::Url;

pub(crate) const HEADER_VERSION: &str = "x-ms-version";

pub(crate) const AZURE_VERSION: &str = "2019-12-12";

pub(crate) fn prepare_file_system_url(client: &StorageAccountClient, name: &str) -> Url {
    let mut url = client.filesystem_url().clone();
    url.path_segments_mut()
        .map_err(|_| url::ParseError::SetHostOnCannotBeABaseUrl)
        .unwrap()
        .push(name);
    url.query_pairs_mut().append_pair("resource", "filesystem");
    url
}

#[derive(Debug, Clone)]
pub struct FileSystemClient {
    client: StorageAccountClient,
    name: String,
    context: Context,
    url: Url,
}

impl FileSystemClient {
    pub(crate) fn new(client: StorageAccountClient, name: String, context: Context) -> Self {
        let mut url = client.filesystem_url().clone();
        url.path_segments_mut()
            .map_err(|_| url::ParseError::SetHostOnCannotBeABaseUrl)
            .unwrap()
            .push(&name);

        Self {
            client,
            name,
            context,
            url,
        }
    }

    pub fn create(&self) -> CreateFileSystemBuilder {
        CreateFileSystemBuilder::new(
            self.client.clone(),
            self.name.clone(),
            Some(self.context.clone()),
        )
    }

    pub fn delete(&self) -> DeleteFileSystemBuilder {
        DeleteFileSystemBuilder::new(
            self.client.clone(),
            self.name.clone(),
            Some(self.context.clone()),
        )
    }

    pub fn get_properties(&self) -> GetFileSystemPropertiesBuilder {
        GetFileSystemPropertiesBuilder::new(
            self.client.clone(),
            self.name.clone(),
            Some(self.context.clone()),
        )
    }

    pub fn set_properties(&self, properties: Option<Properties>) -> SetFileSystemPropertiesBuilder {
        SetFileSystemPropertiesBuilder::new(
            self.client.clone(),
            self.name.clone(),
            properties,
            Some(self.context.clone()),
        )
    }

    pub async fn create_file(
        &self,
        file_path: &str,
        options: FileCreateOptions<'_>,
    ) -> Result<FileCreateResponse, crate::Error> {
        let mut request = self.prepare_file_create_request(file_path);

        options.decorate_request(&mut request)?;
        let response = self
            .pipeline()
            .send(&mut self.context.clone(), &mut request)
            .await?;

        Ok(FileCreateResponse::try_from(response).await?)
    }

    pub async fn create_file_if_not_exists(
        &self,
        file_path: &str,
    ) -> Result<FileCreateResponse, crate::Error> {
        let options = FileCreateOptions::new().if_match_condition(IfMatchCondition::NotMatch("*"));

        let mut request = self.prepare_file_create_request(file_path);

        options.decorate_request(&mut request)?;
        let response = self
            .pipeline()
            .send(&mut self.context.clone(), &mut request)
            .await?;

        Ok(FileCreateResponse::try_from(response).await?)
    }

    pub async fn delete_file(
        &self,
        file_path: &str,
        options: FileDeleteOptions<'_>,
    ) -> Result<FileDeleteResponse, crate::Error> {
        let mut request = self.prepare_file_delete_request(file_path);

        options.decorate_request(&mut request)?;
        let response = self
            .pipeline()
            .send(&mut self.context.clone(), &mut request)
            .await?;

        Ok(FileDeleteResponse::try_from(response).await?)
    }

    pub async fn rename_file(
        &self,
        source_file_path: &str,
        destination_file_path: &str,
        options: FileRenameOptions<'_>,
    ) -> Result<FileRenameResponse, crate::Error> {
        let mut request = self.prepare_file_rename_request(destination_file_path);

        let rename_source = format!("/{}/{}", &self.name, source_file_path);
        options.decorate_request(&mut request, rename_source.as_str())?;
        let response = self
            .pipeline()
            .send(&mut self.context.clone(), &mut request)
            .await?;

        Ok(FileRenameResponse::try_from(response).await?)
    }

    pub async fn rename_file_if_not_exists(
        &self,
        source_file_path: &str,
        destination_file_path: &str,
    ) -> Result<FileRenameResponse, crate::Error> {
        let options = FileRenameOptions::new().if_match_condition(IfMatchCondition::NotMatch("*"));

        let mut request = self.prepare_file_rename_request(destination_file_path);

        let rename_source = format!("/{}/{}", &self.name, source_file_path);
        options.decorate_request(&mut request, rename_source.as_str())?;
        let response = self
            .pipeline()
            .send(&mut self.context.clone(), &mut request)
            .await?;

        Ok(FileRenameResponse::try_from(response).await?)
    }

    pub async fn append_to_file(
        &self,
        file_path: &str,
        bytes: Bytes,
        position: i64,
        options: FileAppendOptions<'_>,
    ) -> Result<FileAppendResponse, crate::Error> {
        let mut request = self.prepare_file_append_request(file_path, position);

        options.decorate_request(&mut request, bytes)?;
        let response = self
            .pipeline()
            .send(&mut self.context.clone(), &mut request)
            .await?;

        Ok(FileAppendResponse::try_from(response).await?)
    }

    pub async fn flush_file(
        &self,
        file_path: &str,
        position: i64,
        close: bool,
        options: FileFlushOptions<'_>,
    ) -> Result<FileFlushResponse, crate::Error> {
        let mut request = self.prepare_file_flush_request(file_path, position, close);
        options.decorate_request(&mut request)?;
        let response = self
            .pipeline()
            .send(&mut self.context.clone(), &mut request)
            .await?;

        Ok(FileFlushResponse::try_from(response).await?)
    }

    pub(crate) fn url(&self) -> &Url {
        &self.url
    }

    pub(crate) fn prepare_file_create_request(&self, file_path: &str) -> azure_core::Request {
        let uri = format!("{}/{}?resource=file", self.url(), file_path);
        let dt = chrono::Utc::now();
        let time = format!("{}", dt.format("%a, %d %h %Y %T GMT"));
        http::request::Request::put(uri)
            .header(MS_DATE, time)
            .header(HEADER_VERSION, AZURE_VERSION)
            .body(bytes::Bytes::new()) // Request builder requires a body here
            .unwrap()
            .into()
    }

    pub(crate) fn prepare_file_delete_request(&self, file_path: &str) -> azure_core::Request {
        let uri = format!("{}/{}", self.url(), file_path);
        let dt = chrono::Utc::now();
        let time = format!("{}", dt.format("%a, %d %h %Y %T GMT"));
        http::request::Request::delete(uri)
            .header(MS_DATE, time)
            .header(HEADER_VERSION, AZURE_VERSION)
            .body(bytes::Bytes::new()) // Request builder requires a body here
            .unwrap()
            .into()
    }

    pub(crate) fn prepare_file_rename_request(
        &self,
        destination_file_path: &str,
    ) -> azure_core::Request {
        let uri = format!("{}/{}?mode=legacy", self.url(), destination_file_path);
        let dt = chrono::Utc::now();
        let time = format!("{}", dt.format("%a, %d %h %Y %T GMT"));
        http::request::Request::put(uri)
            .header(MS_DATE, time)
            .header(HEADER_VERSION, AZURE_VERSION)
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
        let dt = chrono::Utc::now();
        let time = format!("{}", dt.format("%a, %d %h %Y %T GMT"));
        http::request::Request::patch(uri)
            .header(MS_DATE, time)
            .header(HEADER_VERSION, AZURE_VERSION)
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
            self.url(),
            file_path,
            position,
            close,
        );
        let dt = chrono::Utc::now();
        let time = format!("{}", dt.format("%a, %d %h %Y %T GMT"));
        http::request::Request::patch(uri)
            .header(MS_DATE, time)
            .header(HEADER_VERSION, AZURE_VERSION)
            .body(bytes::Bytes::new()) // Request builder requires a body here
            .unwrap()
            .into()
    }

    pub(crate) fn pipeline(&self) -> &Pipeline {
        self.client.pipeline()
    }
}

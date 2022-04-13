use super::{DataLakeClient, FileSystemClient, PathClient};
use crate::{operations::*, request_options::*, Properties, Result};
use azure_core::prelude::IfMatchCondition;
use azure_core::{ClientOptions, Context, Pipeline};
use azure_storage::core::storage_shared_key_credential::StorageSharedKeyCredential;
use bytes::Bytes;
use url::Url;

#[derive(Debug, Clone)]
pub struct FileClient {
    file_system_client: FileSystemClient,
    file_path: String,
}

impl PathClient for FileClient {
    fn url(&self) -> Result<Url> {
        let fs_url = self.file_system_client.url()?;
        let file_path = vec![fs_url.path(), &self.file_path].join("/");
        Ok(self.file_system_client.url()?.join(&file_path)?)
    }

    fn prepare_request(&self, uri: &str, http_method: http::Method) -> azure_core::Request {
        self.file_system_client.prepare_request(uri, http_method)
    }

    fn pipeline(&self) -> &Pipeline {
        self.file_system_client.pipeline()
    }

    fn context(&self) -> &Context {
        &self.file_system_client.context
    }
}

impl FileClient {
    pub(crate) fn new<P>(file_system_client: FileSystemClient, path: P) -> Self
    where
        P: Into<String>,
    {
        Self {
            file_system_client,
            file_path: path.into(),
        }
    }

    pub fn new_with_options<FS, D>(
        credential: StorageSharedKeyCredential,
        custom_dns_suffix: Option<String>,
        options: ClientOptions,
        file_system_name: FS,
        path: D,
    ) -> Self
    where
        FS: Into<String>,
        D: Into<String>,
    {
        DataLakeClient::new_with_shared_key(credential, custom_dns_suffix, options)
            .into_file_system_client(file_system_name.into())
            .into_file_client(path)
    }

    pub fn create(&self) -> PutPathBuilder<Self> {
        PutPathBuilder::new(self.clone(), self.file_system_client.context.clone())
            .resource(ResourceType::File)
    }

    pub fn create_if_not_exists(&self) -> PutPathBuilder<Self> {
        self.create()
            .if_match_condition(IfMatchCondition::NotMatch("*".to_string()))
    }

    pub fn append<B>(&self, position: i64, bytes: B) -> PatchPathBuilder<Self>
    where
        B: Into<Bytes>,
    {
        PatchPathBuilder::new(self.clone(), self.file_system_client.context.clone())
            .action(PathUpdateAction::Append)
            .position(position)
            .bytes(bytes.into())
    }

    pub fn flush(&self, position: i64) -> PatchPathBuilder<Self> {
        PatchPathBuilder::new(self.clone(), self.file_system_client.context.clone())
            .action(PathUpdateAction::Flush)
            .position(position)
    }

    pub fn read(&self) -> GetFileBuilder {
        GetFileBuilder::new(self.clone(), self.file_system_client.context.clone())
    }

    pub fn rename<P>(&self, destination_path: P) -> RenamePathBuilder<Self>
    where
        P: Into<String>,
    {
        let destination_client = self.file_system_client.get_file_client(destination_path);
        let fs_url = self.file_system_client.url().unwrap();
        // the path will contain a leading '/' as we extract if from the path component of the url
        let file_path = vec![fs_url.path(), &self.file_path].join("/");
        RenamePathBuilder::new(destination_client, self.file_system_client.context.clone())
            .mode(PathRenameMode::Legacy)
            .rename_source(file_path)
    }

    pub fn rename_if_not_exists<P>(&self, destination_path: P) -> RenamePathBuilder<Self>
    where
        P: Into<String>,
    {
        self.rename(destination_path)
            .if_match_condition(IfMatchCondition::NotMatch("*".to_string()))
    }

    pub fn delete(&self) -> DeletePathBuilder<Self> {
        DeletePathBuilder::new(self.clone(), None, self.file_system_client.context.clone())
    }

    pub fn get_properties(&self) -> HeadPathBuilder<Self> {
        HeadPathBuilder::new(self.clone(), self.file_system_client.context.clone())
    }

    pub fn get_status(&self) -> HeadPathBuilder<Self> {
        HeadPathBuilder::new(self.clone(), self.file_system_client.context.clone())
            .action(PathGetPropertiesAction::GetStatus)
    }

    pub fn get_access_control_list(&self) -> HeadPathBuilder<Self> {
        HeadPathBuilder::new(self.clone(), self.file_system_client.context.clone())
            .action(PathGetPropertiesAction::GetAccessControl)
    }

    pub fn set_properties(&self, properties: Properties) -> PatchPathBuilder<Self> {
        PatchPathBuilder::new(self.clone(), self.file_system_client.context.clone())
            .properties(properties)
            .action(PathUpdateAction::SetProperties)
    }
}

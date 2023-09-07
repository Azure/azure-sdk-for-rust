use super::{FileSystemClient, PathClient};
use crate::{operations::*, request_options::*, Properties};
use azure_core::prelude::IfMatchCondition;
use bytes::Bytes;
use url::Url;

#[derive(Debug, Clone)]
pub struct FileClient {
    file_system_client: FileSystemClient,
    file_path: String,
}

#[cfg_attr(target_arch = "wasm32", async_trait::async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait::async_trait)]
impl PathClient for FileClient {
    fn url(&self) -> azure_core::Result<Url> {
        let fs_url = self.file_system_client.url()?;
        let file_path = [fs_url.path(), &self.file_path].join("/");
        Ok(self.file_system_client.url()?.join(&file_path)?)
    }

    async fn send(
        &self,
        ctx: &mut azure_core::Context,
        request: &mut azure_core::Request,
    ) -> crate::Result<azure_core::Response> {
        self.file_system_client.send(ctx, request).await
    }
}

impl FileClient {
    pub(crate) fn new(file_system_client: FileSystemClient, path: String) -> Self {
        Self {
            file_system_client,
            file_path: path,
        }
    }

    pub fn create(&self) -> PutPathBuilder<Self> {
        PutPathBuilder::new(self.clone()).resource(ResourceType::File)
    }

    pub fn create_if_not_exists(&self) -> PutPathBuilder<Self> {
        self.create()
            .if_match_condition(IfMatchCondition::NotMatch("*".to_string()))
    }

    pub fn append<B>(&self, position: i64, bytes: B) -> PatchPathBuilder<Self>
    where
        B: Into<Bytes>,
    {
        PatchPathBuilder::new(self.clone(), PathUpdateAction::Append)
            .position(position)
            .bytes(bytes.into())
    }

    pub fn flush(&self, position: i64) -> PatchPathBuilder<Self> {
        PatchPathBuilder::new(self.clone(), PathUpdateAction::Flush).position(position)
    }

    pub fn read(&self) -> GetFileBuilder {
        GetFileBuilder::new(self.clone())
    }

    pub fn rename<P>(&self, destination_path: P) -> RenamePathBuilder<Self>
    where
        P: Into<String>,
    {
        let destination_client = self.file_system_client.get_file_client(destination_path);
        let fs_url = self.file_system_client.url().unwrap();
        // the path will contain a leading '/' as we extract if from the path component of the url
        let file_path = [fs_url.path(), &self.file_path].join("/");
        RenamePathBuilder::new(destination_client)
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
        DeletePathBuilder::new(self.clone())
    }

    pub fn get_properties(&self) -> HeadPathBuilder<Self> {
        HeadPathBuilder::new(self.clone())
    }

    pub fn get_status(&self) -> HeadPathBuilder<Self> {
        HeadPathBuilder::new(self.clone()).action(PathGetPropertiesAction::GetStatus)
    }

    pub fn get_access_control_list(&self) -> HeadPathBuilder<Self> {
        HeadPathBuilder::new(self.clone()).action(PathGetPropertiesAction::GetAccessControl)
    }

    pub fn set_properties(&self, properties: impl Into<Properties>) -> PatchPathBuilder<Self> {
        PatchPathBuilder::new(self.clone(), PathUpdateAction::SetProperties).properties(properties)
    }

    pub fn set_access_control_list(
        &self,
        acl: impl Into<AccessControlList>,
    ) -> PatchPathBuilder<Self> {
        PatchPathBuilder::new(self.clone(), PathUpdateAction::SetAccessControl).acl(acl)
    }
}

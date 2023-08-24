use crate::operations::*;
use crate::request_options::*;
use crate::{clients::FileSystemClient, prelude::PathClient, Properties};
use azure_core::prelude::IfMatchCondition;
use url::Url;

#[derive(Debug, Clone)]
pub struct DirectoryClient {
    file_system_client: FileSystemClient,
    dir_path: String,
}

#[cfg_attr(target_arch = "wasm32", async_trait::async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait::async_trait)]
impl PathClient for DirectoryClient {
    fn url(&self) -> azure_core::Result<Url> {
        let fs_url = self.file_system_client.url()?;
        let dir_path = [fs_url.path(), &self.dir_path].join("/");
        Ok(self.file_system_client.url()?.join(&dir_path)?)
    }

    async fn send(
        &self,
        ctx: &mut azure_core::Context,
        request: &mut azure_core::Request,
    ) -> crate::Result<azure_core::Response> {
        self.file_system_client.send(ctx, request).await
    }
}

impl DirectoryClient {
    pub(crate) fn new(file_system_client: FileSystemClient, path: String) -> Self {
        Self {
            file_system_client,
            dir_path: path,
        }
    }

    pub fn list_paths(&self) -> ListPathsBuilder {
        let fs_url = self.file_system_client.url().unwrap();
        // the path will contain a leading '/' as we extract if from the path component of the url
        let dir_path = [fs_url.path(), &self.dir_path].join("/");
        ListPathsBuilder::new(self.file_system_client.clone())
            .directory(dir_path)
            .recursive(true)
    }

    pub fn create(&self) -> PutPathBuilder<Self> {
        PutPathBuilder::new(self.clone()).resource(ResourceType::Directory)
    }

    pub fn create_if_not_exists(&self) -> PutPathBuilder<Self> {
        self.create()
            .if_match_condition(IfMatchCondition::NotMatch("*".to_string()))
    }

    pub fn rename<P>(&self, destination_path: P) -> RenamePathBuilder<Self>
    where
        P: Into<String>,
    {
        let destination_client = self
            .file_system_client
            .get_directory_client(destination_path);
        let fs_url = self.file_system_client.url().unwrap();
        // the path will contain a leading '/' as we extract if from the path component of the url
        let dir_path = [fs_url.path(), &self.dir_path].join("/");
        RenamePathBuilder::new(destination_client)
            .mode(PathRenameMode::Legacy)
            .rename_source(dir_path)
    }

    pub fn rename_if_not_exists<P>(&self, destination_path: P) -> RenamePathBuilder<Self>
    where
        P: Into<String>,
    {
        self.rename(destination_path)
            .if_match_condition(IfMatchCondition::NotMatch("*".to_string()))
    }

    pub fn delete<R>(&self, recursive: R) -> DeletePathBuilder<Self>
    where
        R: Into<Recursive>,
    {
        DeletePathBuilder::new(self.clone()).recursive(recursive.into())
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
        recursive: bool,
    ) -> PatchPathBuilder<Self> {
        let action = if recursive {
            PathUpdateAction::SetAccessControlRecursive
        } else {
            PathUpdateAction::SetAccessControl
        };

        PatchPathBuilder::new(self.clone(), action).acl(acl)
    }
}

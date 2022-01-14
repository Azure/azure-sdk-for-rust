use crate::operations::*;
use crate::request_options::*;
use crate::{
    clients::{DataLakeClient, FileSystemClient},
    prelude::PathClient,
    Properties, Result,
};
use azure_core::prelude::IfMatchCondition;
use azure_core::{ClientOptions, Context, Pipeline};
use azure_storage::core::storage_shared_key_credential::StorageSharedKeyCredential;
use url::Url;

#[derive(Debug, Clone)]
pub struct DirectoryClient {
    file_system_client: FileSystemClient,
    path: String,
}

impl PathClient for DirectoryClient {
    fn url(&self) -> Result<Url> {
        let fs_url = self.file_system_client.url()?;
        let dir_path = vec![fs_url.path(), &self.path].join("/");
        Ok(self.file_system_client.url()?.join(&dir_path)?)
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

impl DirectoryClient {
    pub(crate) fn new(file_system_client: FileSystemClient, path: String) -> Self {
        Self {
            file_system_client,
            path,
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
        DataLakeClient::new_with_options(credential, custom_dns_suffix, options)
            .into_file_system_client(file_system_name.into())
            .into_directory_client(path)
    }

    pub fn create(&self) -> PutPathBuilder<Self> {
        PutPathBuilder::new(self.clone(), self.file_system_client.context.clone())
            .resource(ResourceType::Directory)
    }

    pub fn create_if_not_exists(&self) -> PutPathBuilder<Self> {
        self.create()
            .if_match_condition(IfMatchCondition::NotMatch("*".to_string()))
    }

    // TODO rename seems to not delete source
    pub fn rename<P>(&self, destination_path: P) -> PutPathBuilder<Self>
    where
        P: Into<String>,
    {
        let destination_client = self
            .file_system_client
            .get_directory_client(destination_path);

        let fs_url = self.file_system_client.url().unwrap();
        let dir_path = vec![fs_url.path(), &self.path].join("/");
        println!("{}", dir_path);
        destination_client
            .create()
            .mode(PathRenameMode::Legacy)
            .rename_source(format!("{}/", dir_path))
    }

    pub fn rename_if_not_exists<P>(&self, destination_path: P) -> PutPathBuilder<Self>
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
        DeletePathBuilder::new(
            self.clone(),
            Some(recursive.into()),
            self.file_system_client.context.clone(),
        )
    }

    pub fn get_properties(&self) -> HeadPathBuilder<Self> {
        HeadPathBuilder::new(self.clone(), self.file_system_client.context.clone())
    }

    pub fn set_properties(&self, _properties: Properties) -> SetFileSystemPropertiesBuilder {
        todo!()
    }

    // pub async fn rename(
    //     &self,
    //     ctx: Context,
    //     source_file_path: &str,
    //     destination_file_path: &str,
    //     options: FileRenameOptions,
    // ) -> Result<FileRenameResponse, crate::Error> {
    //     let mut request = self.prepare_file_rename_request(destination_file_path);
    //
    //     let rename_source = format!("/{}/{}", &self.name, source_file_path);
    //     options.decorate_request(&mut request, rename_source.as_str())?;
    //     let response = self.pipeline().send(&mut ctx.clone(), &mut request).await?;
    //
    //     Ok(FileRenameResponse::try_from(response).await?)
    // }
    //
    // pub async fn rename_if_not_exists(
    //     &self,
    //     ctx: Context,
    //     source_file_path: &str,
    //     destination_file_path: &str,
    // ) -> Result<FileRenameResponse, crate::Error> {
    //     let options = FileRenameOptions::new()
    //         .if_match_condition(IfMatchCondition::NotMatch("*".to_string()));
    //
    //     let mut request = self.prepare_file_rename_request(destination_file_path);
    //
    //     let rename_source = format!("/{}/{}", &self.name, source_file_path);
    //     options.decorate_request(&mut request, rename_source.as_str())?;
    //     let response = self.pipeline().send(&mut ctx.clone(), &mut request).await?;
    //
    //     Ok(FileRenameResponse::try_from(response).await?)
    // }
}

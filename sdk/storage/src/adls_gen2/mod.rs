pub mod filesystem;
pub mod prelude;

use crate::core::{Client, No};

pub trait Filesystem<C>
where
    C: Client,
{
    fn create_filesystem<'a>(&'a self) -> filesystem::requests::CreateFilesystemBuilder<'a, C, No>;
    fn delete_filesystem<'a>(&'a self) -> filesystem::requests::DeleteFilesystemBuilder<'a, C, No>;
    fn get_filesystem_properties<'a>(
        &'a self,
    ) -> filesystem::requests::GetFilesystemPropertiesBuilder<'a, C, No>;
    fn list_filesystems<'a>(&'a self) -> filesystem::requests::ListFilesystemsBuilder<'a, C>;
    fn set_filesystem_properties<'a>(
        &'a self,
    ) -> filesystem::requests::SetFilesystemPropertiesBuilder<'a, C, No>;
}

impl<C> Filesystem<C> for C
where
    C: Client,
{
    fn create_filesystem<'a>(&'a self) -> filesystem::requests::CreateFilesystemBuilder<'a, C, No> {
        filesystem::requests::CreateFilesystemBuilder::new(self)
    }

    fn delete_filesystem<'a>(&'a self) -> filesystem::requests::DeleteFilesystemBuilder<'a, C, No> {
        filesystem::requests::DeleteFilesystemBuilder::new(self)
    }

    fn get_filesystem_properties<'a>(
        &'a self,
    ) -> filesystem::requests::GetFilesystemPropertiesBuilder<'a, C, No> {
        filesystem::requests::GetFilesystemPropertiesBuilder::new(self)
    }

    fn list_filesystems<'a>(&'a self) -> filesystem::requests::ListFilesystemsBuilder<'a, C> {
        filesystem::requests::ListFilesystemsBuilder::new(self)
    }

    fn set_filesystem_properties<'a>(
        &'a self,
    ) -> filesystem::requests::SetFilesystemPropertiesBuilder<'a, C, No> {
        filesystem::requests::SetFilesystemPropertiesBuilder::new(self)
    }
}

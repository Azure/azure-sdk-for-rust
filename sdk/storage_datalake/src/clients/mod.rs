mod data_lake_client;
mod directory_client;
mod file_client;
mod file_system_client;

use std::fmt::Debug;

pub use data_lake_client::DataLakeClient;
pub use directory_client::DirectoryClient;
pub use file_client::FileClient;
pub use file_system_client::FileSystemClient;

pub trait PathClient: Debug + Clone + Send + Sync {
    fn url(&self) -> azure_core::Result<url::Url>;
    fn pipeline(&self) -> &azure_core::Pipeline;
    fn context(&self) -> &azure_core::Context;
}

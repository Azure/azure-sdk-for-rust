mod data_lake_client;
mod directory_client;
mod file_client;
mod file_system_client;

use std::fmt::Debug;

pub use data_lake_client::DataLakeClient;
pub use directory_client::DirectoryClient;
pub use file_client::FileClient;
pub use file_system_client::FileSystemClient;
use futures::Future;

pub trait PathClient: Debug + Clone + Send + Sync {
    fn url(&self) -> crate::Result<url::Url>;
    fn prepare_request(&self, uri: &str, http_method: http::Method) -> azure_core::Request;
    fn pipeline(&self) -> &azure_core::Pipeline;
    fn context(&self) -> &azure_core::Context;
}

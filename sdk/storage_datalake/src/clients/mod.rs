mod data_lake_client;
mod directory_client;
mod file_client;
mod file_system_client;

pub use data_lake_client::{DataLakeClient, DataLakeClientBuilder};
pub use directory_client::DirectoryClient;
pub use file_client::FileClient;
pub use file_system_client::FileSystemClient;

use azure_core::{Context, Request, Response};
use std::fmt::Debug;

#[cfg_attr(target_arch = "wasm32", async_trait::async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait::async_trait)]
pub trait PathClient: Debug + Clone + Send + Sync {
    fn url(&self) -> azure_core::Result<url::Url>;
    async fn send(&self, ctx: &mut Context, request: &mut Request) -> crate::Result<Response>;
}

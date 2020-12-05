mod storage_account_client;
pub use storage_account_client::{ServiceType, StorageAccountClient, StorageCredentials};
mod storage_client;
pub use storage_client::{AsStorageClient, StorageClient};
mod container_client;
pub use container_client::{AsContainerClient, ContainerClient};

mod storage_account_client;
pub use storage_account_client::{
    ServiceType, StorageAccountClient, StorageAccountOptions, StorageCredentials,
};
mod storage_client;
pub use storage_client::{AsStorageClient, StorageClient};

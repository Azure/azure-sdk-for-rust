mod storage_account_client;
pub use storage_account_client::{ServiceType, StorageAccountClient, StorageCredentials};
mod blob_storage_account_client;
pub use blob_storage_account_client::{AsBlobStorageAccountClient, BlobStorageAccountClient};

use azure_storage::core::clients::{AsStorageClient, StorageAccountClient, StorageClient};
use std::sync::Arc;

pub trait AsBlobServiceClient {
    fn as_blob_service_client(&self) -> Arc<BlobServiceClient>;
}

impl AsBlobServiceClient for Arc<StorageClient> {
    fn as_blob_service_client(&self) -> Arc<BlobServiceClient> {
        BlobServiceClient::new(self.clone())
    }
}

impl AsBlobServiceClient for Arc<StorageAccountClient> {
    fn as_blob_service_client(&self) -> Arc<BlobServiceClient> {
        self.as_storage_client().as_blob_service_client()
    }
}

#[derive(Debug, Clone)]
pub struct BlobServiceClient {
    storage_client: Arc<StorageClient>,
}

impl BlobServiceClient {
    pub(crate) fn new(storage_client: Arc<StorageClient>) -> Arc<Self> {
        Arc::new(Self { storage_client })
    }

    pub fn list_containers(&self) -> crate::container::requests::ListContainersBuilder {
        crate::container::requests::ListContainersBuilder::new(&self.storage_client)
    }
}

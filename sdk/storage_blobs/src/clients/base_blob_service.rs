use azure_storage::core::clients::{AsStorageClient, StorageAccountClient, StorageClient};
use std::sync::Arc;

pub trait AsBaseBlobService {
    fn as_base_blob_service(&self) -> Arc<BaseBlobService>;
}

impl AsBaseBlobService for Arc<StorageClient> {
    fn as_base_blob_service(&self) -> Arc<BaseBlobService> {
        BaseBlobService::new(self.clone())
    }
}

impl AsBaseBlobService for Arc<StorageAccountClient> {
    fn as_base_blob_service(&self) -> Arc<BaseBlobService> {
        self.as_storage_client().as_base_blob_service()
    }
}

#[derive(Debug, Clone)]
pub struct BaseBlobService {
    storage_client: Arc<StorageClient>,
}

impl BaseBlobService {
    pub(crate) fn new(storage_client: Arc<StorageClient>) -> Arc<Self> {
        Arc::new(Self { storage_client })
    }

    pub fn list_containers(&self) -> crate::container::requests::ListContainersBuilder {
        crate::container::requests::ListContainersBuilder::new(&self.storage_client)
    }
}

use azure_core::{Context, Request, Response};
use azure_storage::core::clients::{
    AsStorageClient, ServiceType, StorageAccountClient, StorageClient,
};
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
    pub(crate) storage_client: Arc<StorageClient>,
}

impl BlobServiceClient {
    pub(crate) fn new(storage_client: Arc<StorageClient>) -> Arc<Self> {
        Arc::new(Self { storage_client })
    }

    pub fn list_containers(&self) -> crate::container::operations::ListContainersBuilder {
        crate::container::operations::ListContainersBuilder::new(self.clone())
    }

    pub(crate) async fn send(
        &self,
        context: &mut Context,
        request: &mut Request,
    ) -> azure_core::Result<Response> {
        self.storage_client
            .send(context, request, ServiceType::Blob)
            .await
    }
}

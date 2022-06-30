use crate::container::operations::ListContainersBuilder;
use azure_core::{Context, Request, Response};
use azure_storage::core::clients::{ServiceType, StorageClient};
use std::sync::Arc;

pub trait AsBlobServiceClient {
    fn blob_service_client(&self) -> Arc<BlobServiceClient>;
}

impl AsBlobServiceClient for Arc<StorageClient> {
    fn blob_service_client(&self) -> Arc<BlobServiceClient> {
        BlobServiceClient::new(self.clone())
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

    pub fn list_containers(&self) -> ListContainersBuilder {
        ListContainersBuilder::new(self.clone())
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

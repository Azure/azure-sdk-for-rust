use crate::service::operations::*;
use azure_core::{Context, Request, Response};
use azure_storage::core::clients::{ServiceType, StorageClient};

pub trait AsBlobServiceClient {
    fn blob_service_client(&self) -> BlobServiceClient;
}

impl AsBlobServiceClient for StorageClient {
    fn blob_service_client(&self) -> BlobServiceClient {
        BlobServiceClient::new(self.clone())
    }
}

#[derive(Debug, Clone)]
pub struct BlobServiceClient {
    pub(crate) storage_client: StorageClient,
}

impl BlobServiceClient {
    pub(crate) fn new(storage_client: StorageClient) -> Self {
        Self { storage_client }
    }

    pub fn find_blobs_by_tags(&self, expression: String) -> FindBlobsByTagsBuilder {
        FindBlobsByTagsBuilder::new(self.clone(), expression)
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

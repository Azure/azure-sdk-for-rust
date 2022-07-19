use crate::{blob::operations::*, prelude::*};
use azure_core::{headers::Headers, prelude::*, Body, Context, Method, Request, Response, Url};
use azure_storage::core::prelude::*;

#[derive(Debug, Clone)]
pub struct BlobLeaseClient {
    blob_client: BlobClient,
    lease_id: LeaseId,
}

impl BlobLeaseClient {
    pub(crate) fn new(blob_client: BlobClient, lease_id: LeaseId) -> Self {
        Self {
            blob_client,
            lease_id,
        }
    }

    pub fn change(&self, proposed_lease_id: ProposedLeaseId) -> ChangeLeaseBuilder {
        ChangeLeaseBuilder::new(self.clone(), proposed_lease_id)
    }

    pub fn release(&self) -> ReleaseLeaseBuilder {
        ReleaseLeaseBuilder::new(self.clone())
    }

    pub fn renew(&self) -> RenewLeaseBuilder {
        RenewLeaseBuilder::new(self.clone())
    }

    pub fn lease_id(&self) -> LeaseId {
        self.lease_id
    }

    pub fn storage_client(&self) -> &StorageClient {
        self.blob_client.storage_client()
    }

    pub fn container_client(&self) -> &ContainerClient {
        self.blob_client.container_client()
    }

    pub fn blob_client(&self) -> &BlobClient {
        &self.blob_client
    }

    pub(crate) fn url_with_segments<'a, I>(&'a self, segments: I) -> azure_core::Result<url::Url>
    where
        I: IntoIterator<Item = &'a str>,
    {
        self.blob_client.url_with_segments(segments)
    }

    pub(crate) fn finalize_request(
        &self,
        url: Url,
        method: Method,
        headers: Headers,
        request_body: Option<Body>,
    ) -> azure_core::Result<Request> {
        self.blob_client
            .finalize_request(url, method, headers, request_body)
    }

    pub(crate) async fn send(
        &self,
        context: &mut Context,
        request: &mut Request,
    ) -> azure_core::Result<Response> {
        self.blob_client.send(context, request).await
    }
}

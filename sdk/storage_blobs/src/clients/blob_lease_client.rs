use crate::{blob::operations::*, prelude::*};
use azure_core::{prelude::*, Context, Method, Request, Response};
use azure_storage::core::prelude::*;
use bytes::Bytes;
use std::sync::Arc;

pub trait AsBlobLeaseClient {
    fn blob_lease_client(&self, lease_id: LeaseId) -> Arc<BlobLeaseClient>;
}

impl AsBlobLeaseClient for Arc<BlobClient> {
    fn blob_lease_client(&self, lease_id: LeaseId) -> Arc<BlobLeaseClient> {
        BlobLeaseClient::new(self.clone(), lease_id)
    }
}

#[derive(Debug, Clone)]
pub struct BlobLeaseClient {
    blob_client: Arc<BlobClient>,
    lease_id: LeaseId,
}

impl BlobLeaseClient {
    pub(crate) fn new(blob_client: Arc<BlobClient>, lease_id: LeaseId) -> Arc<Self> {
        Arc::new(Self {
            blob_client,
            lease_id,
        })
    }

    pub fn lease_id(&self) -> &LeaseId {
        &self.lease_id
    }

    #[allow(dead_code)]
    pub(crate) fn storage_account_client(&self) -> &StorageAccountClient {
        self.blob_client.storage_account_client()
    }

    #[allow(dead_code)]
    pub(crate) fn container_client(&self) -> &ContainerClient {
        self.blob_client.container_client()
    }

    #[allow(dead_code)]
    pub(crate) fn blob_client(&self) -> &BlobClient {
        self.blob_client.as_ref()
    }

    pub(crate) fn url_with_segments<'a, I>(&'a self, segments: I) -> azure_core::Result<url::Url>
    where
        I: IntoIterator<Item = &'a str>,
    {
        self.blob_client.url_with_segments(segments)
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

    pub(crate) fn prepare_request(
        &self,
        url: &str,
        method: Method,
        request_body: Option<Bytes>,
    ) -> azure_core::Result<Request> {
        self.blob_client.prepare_request(url, method, request_body)
    }

    pub(crate) async fn send(
        &self,
        context: &mut Context,
        request: &mut Request,
    ) -> azure_core::Result<Response> {
        self.blob_client.send(context, request).await
    }
}

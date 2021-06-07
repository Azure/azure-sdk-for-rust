use crate::blob::blob::requests::*;
use crate::blob::prelude::*;
use crate::core::prelude::*;
use azure_core::prelude::*;
use azure_core::HttpClient;
use bytes::Bytes;
use http::method::Method;
use http::request::{Builder, Request};
use std::sync::Arc;

pub trait AsBlobLeaseClient {
    fn as_blob_lease_client(&self, lease_id: LeaseId) -> Arc<BlobLeaseClient>;
}

impl AsBlobLeaseClient for Arc<BlobClient> {
    fn as_blob_lease_client(&self, lease_id: LeaseId) -> Arc<BlobLeaseClient> {
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

    pub(crate) fn http_client(&self) -> &dyn HttpClient {
        self.blob_client.http_client()
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

    pub(crate) fn url_with_segments<'a, I>(
        &'a self,
        segments: I,
    ) -> Result<url::Url, url::ParseError>
    where
        I: IntoIterator<Item = &'a str>,
    {
        self.blob_client.url_with_segments(segments)
    }

    pub fn change<'a>(&'a self, proposed_lease_id: &'a ProposedLeaseId) -> ChangeLeaseBuilder<'a> {
        ChangeLeaseBuilder::new(self, proposed_lease_id)
    }

    pub fn release<'a>(&'a self) -> ReleaseLeaseBuilder {
        ReleaseLeaseBuilder::new(self)
    }

    pub fn renew<'a>(&'a self) -> RenewLeaseBuilder {
        RenewLeaseBuilder::new(self)
    }

    pub(crate) fn prepare_request(
        &self,
        url: &str,
        method: &Method,
        http_header_adder: &dyn Fn(Builder) -> Builder,
        request_body: Option<Bytes>,
    ) -> Result<(Request<Bytes>, url::Url), crate::Error> {
        self.blob_client
            .prepare_request(url, method, http_header_adder, request_body)
    }
}

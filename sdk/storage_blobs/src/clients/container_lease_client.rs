use crate::{container::operations::*, prelude::*};
use azure_core::{headers::Headers, prelude::*, Context, Method, Request, Response, Url};
use azure_storage::core::prelude::*;
use bytes::Bytes;
use std::sync::Arc;

pub trait AsContainerLeaseClient {
    fn container_lease_client(&self, lease_id: LeaseId) -> Arc<ContainerLeaseClient>;
}

impl AsContainerLeaseClient for Arc<ContainerClient> {
    fn container_lease_client(&self, lease_id: LeaseId) -> Arc<ContainerLeaseClient> {
        ContainerLeaseClient::new(self.clone(), lease_id)
    }
}

#[derive(Debug, Clone)]
pub struct ContainerLeaseClient {
    container_client: Arc<ContainerClient>,
    lease_id: LeaseId,
}

impl ContainerLeaseClient {
    pub(crate) fn new(container_client: Arc<ContainerClient>, lease_id: LeaseId) -> Arc<Self> {
        Arc::new(Self {
            container_client,
            lease_id,
        })
    }

    pub fn lease_id(&self) -> LeaseId {
        self.lease_id
    }

    #[allow(dead_code)]
    pub(crate) fn storage_client(&self) -> &StorageClient {
        self.container_client.storage_client()
    }

    #[allow(dead_code)]
    pub(crate) fn container_client(&self) -> &ContainerClient {
        self.container_client.as_ref()
    }

    pub(crate) fn url_with_segments<'a, I>(&'a self, segments: I) -> azure_core::Result<url::Url>
    where
        I: IntoIterator<Item = &'a str>,
    {
        self.container_client.url_with_segments(segments)
    }

    pub fn release(&self) -> ReleaseLeaseBuilder {
        ReleaseLeaseBuilder::new(self.clone())
    }

    pub fn renew(&self) -> RenewLeaseBuilder {
        RenewLeaseBuilder::new(self.clone())
    }

    pub(crate) fn finalize_request(
        &self,
        url: Url,
        method: Method,
        headers: Headers,
        request_body: Option<Bytes>,
    ) -> azure_core::Result<Request> {
        self.container_client
            .finalize_request(url, method, headers, request_body)
    }

    pub(crate) async fn send(
        &self,
        context: &mut Context,
        request: &mut Request,
    ) -> azure_core::Result<Response> {
        self.container_client.send(context, request).await
    }
}

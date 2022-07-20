use crate::{container::operations::*, prelude::*};
use azure_core::{headers::Headers, prelude::*, Body, Context, Method, Request, Response, Url};
use azure_storage::core::prelude::*;

#[derive(Debug, Clone)]
pub struct ContainerLeaseClient {
    container_client: ContainerClient,
    lease_id: LeaseId,
}

impl ContainerLeaseClient {
    pub(crate) fn new(container_client: ContainerClient, lease_id: LeaseId) -> Self {
        Self {
            container_client,
            lease_id,
        }
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
        request_body: Option<Body>,
    ) -> azure_core::Result<Request> {
        self.container_client
            .finalize_request(url, method, headers, request_body)
    }

    pub fn lease_id(&self) -> LeaseId {
        self.lease_id
    }

    pub fn storage_client(&self) -> &StorageClient {
        self.container_client.storage_client()
    }

    pub fn container_client(&self) -> &ContainerClient {
        &self.container_client
    }

    pub(crate) fn url(&self) -> azure_core::Result<url::Url> {
        self.container_client.url()
    }

    pub(crate) async fn send(
        &self,
        context: &mut Context,
        request: &mut Request,
    ) -> azure_core::Result<Response> {
        self.container_client.send(context, request).await
    }
}

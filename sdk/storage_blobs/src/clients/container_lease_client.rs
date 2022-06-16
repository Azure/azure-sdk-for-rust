use crate::{container::requests::*, prelude::*};
use azure_core::{error::Result, prelude::*, HttpClient};
use azure_storage::core::prelude::*;
use bytes::Bytes;
use http::{
    method::Method,
    request::{Builder, Request},
};
use std::sync::Arc;

pub trait AsContainerLeaseClient {
    fn as_container_lease_client(&self, lease_id: LeaseId) -> Arc<ContainerLeaseClient>;
}

impl AsContainerLeaseClient for Arc<ContainerClient> {
    fn as_container_lease_client(&self, lease_id: LeaseId) -> Arc<ContainerLeaseClient> {
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

    pub fn lease_id(&self) -> &LeaseId {
        &self.lease_id
    }

    pub(crate) fn http_client(&self) -> &dyn HttpClient {
        self.container_client.http_client()
    }

    #[allow(dead_code)]
    pub(crate) fn storage_account_client(&self) -> &StorageAccountClient {
        self.container_client.storage_account_client()
    }

    #[allow(dead_code)]
    pub(crate) fn container_client(&self) -> &ContainerClient {
        self.container_client.as_ref()
    }

    pub(crate) fn url_with_segments<'a, I>(&'a self, segments: I) -> Result<url::Url>
    where
        I: IntoIterator<Item = &'a str>,
    {
        self.container_client.url_with_segments(segments)
    }

    pub fn release(&self) -> ReleaseLeaseBuilder {
        ReleaseLeaseBuilder::new(self)
    }

    pub fn renew(&self) -> RenewLeaseBuilder {
        RenewLeaseBuilder::new(self)
    }

    pub(crate) fn prepare_request(
        &self,
        url: &str,
        method: &Method,
        http_header_adder: &dyn Fn(Builder) -> Builder,
        request_body: Option<Bytes>,
    ) -> crate::Result<(Request<Bytes>, url::Url)> {
        self.container_client
            .prepare_request(url, method, http_header_adder, request_body)
    }
}

use crate::blob::blob::generate_blob_uri;
use crate::blob::blob::responses::RenewBlobLeaseResponse;
use crate::core::prelude::*;
use azure_core::errors::AzureError;
use azure_core::headers::LEASE_ACTION;
use azure_core::lease::LeaseId;
use azure_core::prelude::*;
use azure_core::{No, ToAssign, Yes};
use hyper::{Method, StatusCode};
use std::marker::PhantomData;

#[derive(Debug, Clone)]
pub struct RenewBlobLeaseBuilder<'a, C, ContainerNameSet, BlobNameSet, LeaseIdSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    LeaseIdSet: ToAssign,
    C: Client,
{
    client: &'a C,
    p_container_name: PhantomData<ContainerNameSet>,
    p_blob_name: PhantomData<BlobNameSet>,
    p_lease_id: PhantomData<LeaseIdSet>,
    container_name: Option<&'a str>,
    blob_name: Option<&'a str>,
    lease_id: Option<&'a LeaseId>,
    timeout: Option<u64>,
    client_request_id: Option<&'a str>,
}

impl<'a, C> RenewBlobLeaseBuilder<'a, C, No, No, No>
where
    C: Client,
{
    #[inline]
    pub(crate) fn new(client: &'a C) -> RenewBlobLeaseBuilder<'a, C, No, No, No> {
        RenewBlobLeaseBuilder {
            client,
            p_container_name: PhantomData {},
            container_name: None,
            p_blob_name: PhantomData {},
            blob_name: None,
            p_lease_id: PhantomData {},
            lease_id: None,
            timeout: None,
            client_request_id: None,
        }
    }
}

impl<'a, C, ContainerNameSet, BlobNameSet, LeaseIdSet> ClientRequired<'a, C>
    for RenewBlobLeaseBuilder<'a, C, ContainerNameSet, BlobNameSet, LeaseIdSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    LeaseIdSet: ToAssign,
    C: Client,
{
    #[inline]
    fn client(&self) -> &'a C {
        self.client
    }
}

//get mandatory no traits methods

//set mandatory no traits methods
impl<'a, C, BlobNameSet, LeaseIdSet> ContainerNameRequired<'a>
    for RenewBlobLeaseBuilder<'a, C, Yes, BlobNameSet, LeaseIdSet>
where
    BlobNameSet: ToAssign,
    LeaseIdSet: ToAssign,
    C: Client,
{
    #[inline]
    fn container_name(&self) -> &'a str {
        self.container_name.unwrap()
    }
}

impl<'a, C, ContainerNameSet, LeaseIdSet> BlobNameRequired<'a>
    for RenewBlobLeaseBuilder<'a, C, ContainerNameSet, Yes, LeaseIdSet>
where
    ContainerNameSet: ToAssign,
    LeaseIdSet: ToAssign,
    C: Client,
{
    #[inline]
    fn blob_name(&self) -> &'a str {
        self.blob_name.unwrap()
    }
}

impl<'a, C, ContainerNameSet, BlobNameSet> LeaseIdRequired<'a>
    for RenewBlobLeaseBuilder<'a, C, ContainerNameSet, BlobNameSet, Yes>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    C: Client,
{
    #[inline]
    fn lease_id(&self) -> &'a LeaseId {
        self.lease_id.unwrap()
    }
}

impl<'a, C, ContainerNameSet, BlobNameSet, LeaseIdSet> TimeoutOption
    for RenewBlobLeaseBuilder<'a, C, ContainerNameSet, BlobNameSet, LeaseIdSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    LeaseIdSet: ToAssign,
    C: Client,
{
    #[inline]
    fn timeout(&self) -> Option<u64> {
        self.timeout
    }
}

impl<'a, C, ContainerNameSet, BlobNameSet, LeaseIdSet> ClientRequestIdOption<'a>
    for RenewBlobLeaseBuilder<'a, C, ContainerNameSet, BlobNameSet, LeaseIdSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    LeaseIdSet: ToAssign,
    C: Client,
{
    #[inline]
    fn client_request_id(&self) -> Option<&'a str> {
        self.client_request_id
    }
}

impl<'a, C, BlobNameSet, LeaseIdSet> ContainerNameSupport<'a>
    for RenewBlobLeaseBuilder<'a, C, No, BlobNameSet, LeaseIdSet>
where
    BlobNameSet: ToAssign,
    LeaseIdSet: ToAssign,
    C: Client,
{
    type O = RenewBlobLeaseBuilder<'a, C, Yes, BlobNameSet, LeaseIdSet>;

    #[inline]
    fn with_container_name(self, container_name: &'a str) -> Self::O {
        RenewBlobLeaseBuilder {
            client: self.client,
            p_container_name: PhantomData {},
            p_blob_name: PhantomData {},
            p_lease_id: PhantomData {},
            container_name: Some(container_name),
            blob_name: self.blob_name,
            lease_id: self.lease_id,
            timeout: self.timeout,
            client_request_id: self.client_request_id,
        }
    }
}

impl<'a, C, ContainerNameSet, LeaseIdSet> BlobNameSupport<'a>
    for RenewBlobLeaseBuilder<'a, C, ContainerNameSet, No, LeaseIdSet>
where
    ContainerNameSet: ToAssign,
    LeaseIdSet: ToAssign,
    C: Client,
{
    type O = RenewBlobLeaseBuilder<'a, C, ContainerNameSet, Yes, LeaseIdSet>;

    #[inline]
    fn with_blob_name(self, blob_name: &'a str) -> Self::O {
        RenewBlobLeaseBuilder {
            client: self.client,
            p_container_name: PhantomData {},
            p_blob_name: PhantomData {},
            p_lease_id: PhantomData {},
            container_name: self.container_name,
            blob_name: Some(blob_name),
            lease_id: self.lease_id,
            timeout: self.timeout,
            client_request_id: self.client_request_id,
        }
    }
}

impl<'a, C, ContainerNameSet, BlobNameSet> LeaseIdSupport<'a>
    for RenewBlobLeaseBuilder<'a, C, ContainerNameSet, BlobNameSet, No>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    C: Client,
{
    type O = RenewBlobLeaseBuilder<'a, C, ContainerNameSet, BlobNameSet, Yes>;

    #[inline]
    fn with_lease_id(self, lease_id: &'a LeaseId) -> Self::O {
        RenewBlobLeaseBuilder {
            client: self.client,
            p_container_name: PhantomData {},
            p_blob_name: PhantomData {},
            p_lease_id: PhantomData {},
            container_name: self.container_name,
            blob_name: self.blob_name,
            lease_id: Some(lease_id),
            timeout: self.timeout,
            client_request_id: self.client_request_id,
        }
    }
}

impl<'a, C, ContainerNameSet, BlobNameSet, LeaseIdSet> TimeoutSupport
    for RenewBlobLeaseBuilder<'a, C, ContainerNameSet, BlobNameSet, LeaseIdSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    LeaseIdSet: ToAssign,
    C: Client,
{
    type O = RenewBlobLeaseBuilder<'a, C, ContainerNameSet, BlobNameSet, LeaseIdSet>;

    #[inline]
    fn with_timeout(self, timeout: u64) -> Self::O {
        RenewBlobLeaseBuilder {
            client: self.client,
            p_container_name: PhantomData {},
            p_blob_name: PhantomData {},
            p_lease_id: PhantomData {},
            container_name: self.container_name,
            blob_name: self.blob_name,
            lease_id: self.lease_id,
            timeout: Some(timeout),
            client_request_id: self.client_request_id,
        }
    }
}

impl<'a, C, ContainerNameSet, BlobNameSet, LeaseIdSet> ClientRequestIdSupport<'a>
    for RenewBlobLeaseBuilder<'a, C, ContainerNameSet, BlobNameSet, LeaseIdSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    LeaseIdSet: ToAssign,
    C: Client,
{
    type O = RenewBlobLeaseBuilder<'a, C, ContainerNameSet, BlobNameSet, LeaseIdSet>;

    #[inline]
    fn with_client_request_id(self, client_request_id: &'a str) -> Self::O {
        RenewBlobLeaseBuilder {
            client: self.client,
            p_container_name: PhantomData {},
            p_blob_name: PhantomData {},
            p_lease_id: PhantomData {},
            container_name: self.container_name,
            blob_name: self.blob_name,
            lease_id: self.lease_id,
            timeout: self.timeout,
            client_request_id: Some(client_request_id),
        }
    }
}

// methods callable only when every mandatory field has been filled
impl<'a, C> RenewBlobLeaseBuilder<'a, C, Yes, Yes, Yes>
where
    C: Client,
{
    pub async fn finalize(self) -> Result<RenewBlobLeaseResponse, AzureError> {
        let mut uri = generate_blob_uri(
            self.client(),
            self.container_name(),
            self.blob_name(),
            Some("comp=lease"),
        );

        if let Some(nm) = TimeoutOption::to_uri_parameter(&self) {
            uri = format!("{}&{}", uri, nm);
        }

        let (headers, _) = self
            .client()
            .perform_request(
                &uri,
                &Method::PUT,
                &|mut request| {
                    request = LeaseIdRequired::add_header(&self, request);
                    request = request.header(LEASE_ACTION, "renew");
                    request = ClientRequestIdOption::add_header(&self, request);
                    request
                },
                None,
            )?
            .check_status_extract_headers_and_body(StatusCode::OK)
            .await?;
        RenewBlobLeaseResponse::from_headers(&headers)
    }
}

use crate::blob::generate_blob_uri;
use crate::blob::responses::RenewBlobLeaseResponse;
use azure_sdk_core::errors::{check_status_extract_headers_and_body, AzureError};
use azure_sdk_core::headers::LEASE_ACTION;
use azure_sdk_core::lease::LeaseId;
use azure_sdk_core::{
    BlobNameRequired, BlobNameSupport, ClientRequestIdOption, ClientRequestIdSupport,
    ContainerNameRequired, ContainerNameSupport, LeaseIdRequired, LeaseIdSupport, TimeoutOption,
    TimeoutSupport,
};
use azure_sdk_core::{No, ToAssign, Yes};
use azure_sdk_storage_core::client::Client;
use azure_sdk_storage_core::ClientRequired;
use hyper::{Method, StatusCode};
use std::marker::PhantomData;

#[derive(Debug, Clone)]
pub struct RenewBlobLeaseBuilder<'a, ContainerNameSet, BlobNameSet, LeaseIdSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    LeaseIdSet: ToAssign,
{
    client: &'a Client,
    p_container_name: PhantomData<ContainerNameSet>,
    p_blob_name: PhantomData<BlobNameSet>,
    p_lease_id: PhantomData<LeaseIdSet>,
    container_name: Option<&'a str>,
    blob_name: Option<&'a str>,
    lease_id: Option<&'a LeaseId>,
    timeout: Option<u64>,
    client_request_id: Option<&'a str>,
}

impl<'a> RenewBlobLeaseBuilder<'a, No, No, No> {
    #[inline]
    pub(crate) fn new(client: &'a Client) -> RenewBlobLeaseBuilder<'a, No, No, No> {
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

impl<'a, ContainerNameSet, BlobNameSet, LeaseIdSet> ClientRequired<'a>
    for RenewBlobLeaseBuilder<'a, ContainerNameSet, BlobNameSet, LeaseIdSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    LeaseIdSet: ToAssign,
{
    #[inline]
    fn client(&self) -> &'a Client {
        self.client
    }
}

impl<'a, BlobNameSet, LeaseIdSet> ContainerNameRequired<'a>
    for RenewBlobLeaseBuilder<'a, Yes, BlobNameSet, LeaseIdSet>
where
    BlobNameSet: ToAssign,
    LeaseIdSet: ToAssign,
{
    #[inline]
    fn container_name(&self) -> &'a str {
        self.container_name.unwrap()
    }
}

impl<'a, ContainerNameSet, LeaseIdSet> BlobNameRequired<'a>
    for RenewBlobLeaseBuilder<'a, ContainerNameSet, Yes, LeaseIdSet>
where
    ContainerNameSet: ToAssign,
    LeaseIdSet: ToAssign,
{
    #[inline]
    fn blob_name(&self) -> &'a str {
        self.blob_name.unwrap()
    }
}

impl<'a, ContainerNameSet, BlobNameSet> LeaseIdRequired<'a>
    for RenewBlobLeaseBuilder<'a, ContainerNameSet, BlobNameSet, Yes>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
{
    #[inline]
    fn lease_id(&self) -> &'a LeaseId {
        self.lease_id.unwrap()
    }
}

impl<'a, ContainerNameSet, BlobNameSet, LeaseIdSet> TimeoutOption
    for RenewBlobLeaseBuilder<'a, ContainerNameSet, BlobNameSet, LeaseIdSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    LeaseIdSet: ToAssign,
{
    #[inline]
    fn timeout(&self) -> Option<u64> {
        self.timeout
    }
}

impl<'a, ContainerNameSet, BlobNameSet, LeaseIdSet> ClientRequestIdOption<'a>
    for RenewBlobLeaseBuilder<'a, ContainerNameSet, BlobNameSet, LeaseIdSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    LeaseIdSet: ToAssign,
{
    #[inline]
    fn client_request_id(&self) -> Option<&'a str> {
        self.client_request_id
    }
}

impl<'a, ContainerNameSet, BlobNameSet, LeaseIdSet> ContainerNameSupport<'a>
    for RenewBlobLeaseBuilder<'a, ContainerNameSet, BlobNameSet, LeaseIdSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    LeaseIdSet: ToAssign,
{
    type O = RenewBlobLeaseBuilder<'a, Yes, BlobNameSet, LeaseIdSet>;

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

impl<'a, ContainerNameSet, BlobNameSet, LeaseIdSet> BlobNameSupport<'a>
    for RenewBlobLeaseBuilder<'a, ContainerNameSet, BlobNameSet, LeaseIdSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    LeaseIdSet: ToAssign,
{
    type O = RenewBlobLeaseBuilder<'a, ContainerNameSet, Yes, LeaseIdSet>;

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

impl<'a, ContainerNameSet, BlobNameSet, LeaseIdSet> LeaseIdSupport<'a>
    for RenewBlobLeaseBuilder<'a, ContainerNameSet, BlobNameSet, LeaseIdSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    LeaseIdSet: ToAssign,
{
    type O = RenewBlobLeaseBuilder<'a, ContainerNameSet, BlobNameSet, Yes>;

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

impl<'a, ContainerNameSet, BlobNameSet, LeaseIdSet> TimeoutSupport
    for RenewBlobLeaseBuilder<'a, ContainerNameSet, BlobNameSet, LeaseIdSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    LeaseIdSet: ToAssign,
{
    type O = RenewBlobLeaseBuilder<'a, ContainerNameSet, BlobNameSet, LeaseIdSet>;

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

impl<'a, ContainerNameSet, BlobNameSet, LeaseIdSet> ClientRequestIdSupport<'a>
    for RenewBlobLeaseBuilder<'a, ContainerNameSet, BlobNameSet, LeaseIdSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    LeaseIdSet: ToAssign,
{
    type O = RenewBlobLeaseBuilder<'a, ContainerNameSet, BlobNameSet, LeaseIdSet>;

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

// methods callable regardless
impl<'a, ContainerNameSet, BlobNameSet, LeaseIdSet>
    RenewBlobLeaseBuilder<'a, ContainerNameSet, BlobNameSet, LeaseIdSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    LeaseIdSet: ToAssign,
{
}

impl<'a> RenewBlobLeaseBuilder<'a, Yes, Yes, Yes> {
    pub async fn finalize(self) -> Result<RenewBlobLeaseResponse, AzureError> {
        let mut uri = generate_blob_uri(&self, Some("comp=lease"));

        if let Some(nm) = TimeoutOption::to_uri_parameter(&self) {
            uri = format!("{}&{}", uri, nm);
        }

        let future_response = self.client().perform_request(
            &uri,
            &Method::PUT,
            |mut request| {
                request = LeaseIdRequired::add_header(&self, request);
                request = request.header(LEASE_ACTION, "renew");
                request = ClientRequestIdOption::add_header(&self, request);
                request
            },
            None,
        )?;

        let (headers, _body) =
            check_status_extract_headers_and_body(future_response, StatusCode::OK).await?;
        RenewBlobLeaseResponse::from_headers(&headers)
    }
}

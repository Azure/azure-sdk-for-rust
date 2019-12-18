use crate::blob::generate_blob_uri;
use crate::blob::responses::AcquireBlobLeaseResponse;
use azure_sdk_core::errors::{check_status_extract_headers_and_body, AzureError};
use azure_sdk_core::headers::LEASE_ACTION;
use azure_sdk_core::lease::LeaseId;
use azure_sdk_core::{
    BlobNameRequired, BlobNameSupport, ClientRequestIdOption, ClientRequestIdSupport,
    ContainerNameRequired, ContainerNameSupport, LeaseDurationRequired, LeaseDurationSupport,
    ProposedLeaseIdOption, ProposedLeaseIdSupport, TimeoutOption, TimeoutSupport,
};
use azure_sdk_core::{No, ToAssign, Yes};
use azure_sdk_storage_core::client::Client;
use azure_sdk_storage_core::ClientRequired;
use hyper::{Method, StatusCode};
use std::marker::PhantomData;

#[derive(Debug, Clone)]
pub struct AcquireBlobLeaseBuilder<'a, ContainerNameSet, BlobNameSet, LeaseDurationSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    LeaseDurationSet: ToAssign,
{
    client: &'a Client,
    p_container_name: PhantomData<ContainerNameSet>,
    p_blob_name: PhantomData<BlobNameSet>,
    p_lease_duration: PhantomData<LeaseDurationSet>,
    container_name: Option<&'a str>,
    blob_name: Option<&'a str>,
    lease_duration: i8,
    proposed_lease_id: Option<&'a LeaseId>,
    timeout: Option<u64>,
    client_request_id: Option<&'a str>,
}

impl<'a> AcquireBlobLeaseBuilder<'a, No, No, No> {
    #[inline]
    pub(crate) fn new(client: &'a Client) -> AcquireBlobLeaseBuilder<'a, No, No, No> {
        AcquireBlobLeaseBuilder {
            client,
            p_container_name: PhantomData {},
            container_name: None,
            p_blob_name: PhantomData {},
            blob_name: None,
            p_lease_duration: PhantomData {},
            lease_duration: -1,
            proposed_lease_id: None,
            timeout: None,
            client_request_id: None,
        }
    }
}

impl<'a, ContainerNameSet, BlobNameSet, LeaseDurationSet> ClientRequired<'a>
    for AcquireBlobLeaseBuilder<'a, ContainerNameSet, BlobNameSet, LeaseDurationSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    LeaseDurationSet: ToAssign,
{
    #[inline]
    fn client(&self) -> &'a Client {
        self.client
    }
}

impl<'a, BlobNameSet, LeaseDurationSet> ContainerNameRequired<'a>
    for AcquireBlobLeaseBuilder<'a, Yes, BlobNameSet, LeaseDurationSet>
where
    BlobNameSet: ToAssign,
    LeaseDurationSet: ToAssign,
{
    #[inline]
    fn container_name(&self) -> &'a str {
        self.container_name.unwrap()
    }
}

impl<'a, ContainerNameSet, LeaseDurationSet> BlobNameRequired<'a>
    for AcquireBlobLeaseBuilder<'a, ContainerNameSet, Yes, LeaseDurationSet>
where
    ContainerNameSet: ToAssign,
    LeaseDurationSet: ToAssign,
{
    #[inline]
    fn blob_name(&self) -> &'a str {
        self.blob_name.unwrap()
    }
}

impl<'a, ContainerNameSet, BlobNameSet> LeaseDurationRequired
    for AcquireBlobLeaseBuilder<'a, ContainerNameSet, BlobNameSet, Yes>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
{
    #[inline]
    fn lease_duration(&self) -> i8 {
        self.lease_duration
    }
}

impl<'a, ContainerNameSet, BlobNameSet, LeaseDurationSet> ProposedLeaseIdOption<'a>
    for AcquireBlobLeaseBuilder<'a, ContainerNameSet, BlobNameSet, LeaseDurationSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    LeaseDurationSet: ToAssign,
{
    #[inline]
    fn proposed_lease_id(&self) -> Option<&'a LeaseId> {
        self.proposed_lease_id
    }
}

impl<'a, ContainerNameSet, BlobNameSet, LeaseDurationSet> TimeoutOption
    for AcquireBlobLeaseBuilder<'a, ContainerNameSet, BlobNameSet, LeaseDurationSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    LeaseDurationSet: ToAssign,
{
    #[inline]
    fn timeout(&self) -> Option<u64> {
        self.timeout
    }
}

impl<'a, ContainerNameSet, BlobNameSet, LeaseDurationSet> ClientRequestIdOption<'a>
    for AcquireBlobLeaseBuilder<'a, ContainerNameSet, BlobNameSet, LeaseDurationSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    LeaseDurationSet: ToAssign,
{
    #[inline]
    fn client_request_id(&self) -> Option<&'a str> {
        self.client_request_id
    }
}

impl<'a, ContainerNameSet, BlobNameSet, LeaseDurationSet> ContainerNameSupport<'a>
    for AcquireBlobLeaseBuilder<'a, ContainerNameSet, BlobNameSet, LeaseDurationSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    LeaseDurationSet: ToAssign,
{
    type O = AcquireBlobLeaseBuilder<'a, Yes, BlobNameSet, LeaseDurationSet>;

    #[inline]
    fn with_container_name(self, container_name: &'a str) -> Self::O {
        AcquireBlobLeaseBuilder {
            client: self.client,
            p_container_name: PhantomData {},
            p_blob_name: PhantomData {},
            p_lease_duration: PhantomData {},
            container_name: Some(container_name),
            blob_name: self.blob_name,
            lease_duration: self.lease_duration,
            proposed_lease_id: self.proposed_lease_id,
            timeout: self.timeout,
            client_request_id: self.client_request_id,
        }
    }
}

impl<'a, ContainerNameSet, BlobNameSet, LeaseDurationSet> BlobNameSupport<'a>
    for AcquireBlobLeaseBuilder<'a, ContainerNameSet, BlobNameSet, LeaseDurationSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    LeaseDurationSet: ToAssign,
{
    type O = AcquireBlobLeaseBuilder<'a, ContainerNameSet, Yes, LeaseDurationSet>;

    #[inline]
    fn with_blob_name(self, blob_name: &'a str) -> Self::O {
        AcquireBlobLeaseBuilder {
            client: self.client,
            p_container_name: PhantomData {},
            p_blob_name: PhantomData {},
            p_lease_duration: PhantomData {},
            container_name: self.container_name,
            blob_name: Some(blob_name),
            lease_duration: self.lease_duration,
            proposed_lease_id: self.proposed_lease_id,
            timeout: self.timeout,
            client_request_id: self.client_request_id,
        }
    }
}

impl<'a, ContainerNameSet, BlobNameSet, LeaseDurationSet> LeaseDurationSupport
    for AcquireBlobLeaseBuilder<'a, ContainerNameSet, BlobNameSet, LeaseDurationSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    LeaseDurationSet: ToAssign,
{
    type O = AcquireBlobLeaseBuilder<'a, ContainerNameSet, BlobNameSet, Yes>;

    #[inline]
    fn with_lease_duration(self, lease_duration: i8) -> Self::O {
        AcquireBlobLeaseBuilder {
            client: self.client,
            p_container_name: PhantomData {},
            p_blob_name: PhantomData {},
            p_lease_duration: PhantomData {},
            container_name: self.container_name,
            blob_name: self.blob_name,
            lease_duration,
            proposed_lease_id: self.proposed_lease_id,
            timeout: self.timeout,
            client_request_id: self.client_request_id,
        }
    }
}

impl<'a, ContainerNameSet, BlobNameSet, LeaseDurationSet> ProposedLeaseIdSupport<'a>
    for AcquireBlobLeaseBuilder<'a, ContainerNameSet, BlobNameSet, LeaseDurationSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    LeaseDurationSet: ToAssign,
{
    type O = AcquireBlobLeaseBuilder<'a, ContainerNameSet, BlobNameSet, LeaseDurationSet>;

    #[inline]
    fn with_proposed_lease_id(self, proposed_lease_id: &'a LeaseId) -> Self::O {
        AcquireBlobLeaseBuilder {
            client: self.client,
            p_container_name: PhantomData {},
            p_blob_name: PhantomData {},
            p_lease_duration: PhantomData {},
            container_name: self.container_name,
            blob_name: self.blob_name,
            lease_duration: self.lease_duration,
            proposed_lease_id: Some(proposed_lease_id),
            timeout: self.timeout,
            client_request_id: self.client_request_id,
        }
    }
}

impl<'a, ContainerNameSet, BlobNameSet, LeaseDurationSet> TimeoutSupport
    for AcquireBlobLeaseBuilder<'a, ContainerNameSet, BlobNameSet, LeaseDurationSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    LeaseDurationSet: ToAssign,
{
    type O = AcquireBlobLeaseBuilder<'a, ContainerNameSet, BlobNameSet, LeaseDurationSet>;

    #[inline]
    fn with_timeout(self, timeout: u64) -> Self::O {
        AcquireBlobLeaseBuilder {
            client: self.client,
            p_container_name: PhantomData {},
            p_blob_name: PhantomData {},
            p_lease_duration: PhantomData {},
            container_name: self.container_name,
            blob_name: self.blob_name,
            lease_duration: self.lease_duration,
            proposed_lease_id: self.proposed_lease_id,
            timeout: Some(timeout),
            client_request_id: self.client_request_id,
        }
    }
}

impl<'a, ContainerNameSet, BlobNameSet, LeaseDurationSet> ClientRequestIdSupport<'a>
    for AcquireBlobLeaseBuilder<'a, ContainerNameSet, BlobNameSet, LeaseDurationSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    LeaseDurationSet: ToAssign,
{
    type O = AcquireBlobLeaseBuilder<'a, ContainerNameSet, BlobNameSet, LeaseDurationSet>;

    #[inline]
    fn with_client_request_id(self, client_request_id: &'a str) -> Self::O {
        AcquireBlobLeaseBuilder {
            client: self.client,
            p_container_name: PhantomData {},
            p_blob_name: PhantomData {},
            p_lease_duration: PhantomData {},
            container_name: self.container_name,
            blob_name: self.blob_name,
            lease_duration: self.lease_duration,
            proposed_lease_id: self.proposed_lease_id,
            timeout: self.timeout,
            client_request_id: Some(client_request_id),
        }
    }
}

// methods callable regardless
impl<'a, ContainerNameSet, BlobNameSet, LeaseDurationSet>
    AcquireBlobLeaseBuilder<'a, ContainerNameSet, BlobNameSet, LeaseDurationSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    LeaseDurationSet: ToAssign,
{
}

impl<'a> AcquireBlobLeaseBuilder<'a, Yes, Yes, Yes> {
    pub async fn finalize(self) -> Result<AcquireBlobLeaseResponse, AzureError> {
        let mut uri = generate_blob_uri(&self, Some("comp=lease"));

        if let Some(nm) = TimeoutOption::to_uri_parameter(&self) {
            uri = format!("{}&{}", uri, nm);
        }

        let future_response = self.client().perform_request(
            &uri,
            &Method::PUT,
            |mut request| {
                request = request.header(LEASE_ACTION, "acquire");
                request = LeaseDurationRequired::add_header(&self, request);
                request = ProposedLeaseIdOption::add_header(&self, request);
                request = ClientRequestIdOption::add_header(&self, request);
                request
            },
            None,
        )?;

        let (headers, _body) =
            check_status_extract_headers_and_body(future_response, StatusCode::CREATED).await?;
        AcquireBlobLeaseResponse::from_headers(&headers)
    }
}

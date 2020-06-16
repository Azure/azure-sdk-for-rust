use crate::blob::generate_blob_uri;
use crate::blob::responses::AcquireBlobLeaseResponse;
use azure_sdk_core::errors::{check_status_extract_headers_and_body, AzureError};
use azure_sdk_core::headers::LEASE_ACTION;
use azure_sdk_core::lease::LeaseId;
use azure_sdk_core::prelude::*;
use azure_sdk_core::{No, ToAssign, Yes};
use azure_sdk_storage_core::prelude::*;
use hyper::{Method, StatusCode};
use std::marker::PhantomData;

#[derive(Debug, Clone)]
pub struct AcquireBlobLeaseBuilder<'a, C, ContainerNameSet, BlobNameSet, LeaseDurationSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    LeaseDurationSet: ToAssign,
    C: Client,
{
    client: &'a C,
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

impl<'a, C> AcquireBlobLeaseBuilder<'a, C, No, No, No>
where
    C: Client,
{
    #[inline]
    pub(crate) fn new(client: &'a C) -> AcquireBlobLeaseBuilder<'a, C, No, No, No> {
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

impl<'a, C, ContainerNameSet, BlobNameSet, LeaseDurationSet> ClientRequired<'a, C>
    for AcquireBlobLeaseBuilder<'a, C, ContainerNameSet, BlobNameSet, LeaseDurationSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    LeaseDurationSet: ToAssign,
    C: Client,
{
    #[inline]
    fn client(&self) -> &'a C {
        self.client
    }
}

//get mandatory no traits methods

//set mandatory no traits methods
impl<'a, C, BlobNameSet, LeaseDurationSet> ContainerNameRequired<'a>
    for AcquireBlobLeaseBuilder<'a, C, Yes, BlobNameSet, LeaseDurationSet>
where
    BlobNameSet: ToAssign,
    LeaseDurationSet: ToAssign,
    C: Client,
{
    #[inline]
    fn container_name(&self) -> &'a str {
        self.container_name.unwrap()
    }
}

impl<'a, C, ContainerNameSet, LeaseDurationSet> BlobNameRequired<'a>
    for AcquireBlobLeaseBuilder<'a, C, ContainerNameSet, Yes, LeaseDurationSet>
where
    ContainerNameSet: ToAssign,
    LeaseDurationSet: ToAssign,
    C: Client,
{
    #[inline]
    fn blob_name(&self) -> &'a str {
        self.blob_name.unwrap()
    }
}

impl<'a, C, ContainerNameSet, BlobNameSet> LeaseDurationRequired
    for AcquireBlobLeaseBuilder<'a, C, ContainerNameSet, BlobNameSet, Yes>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    C: Client,
{
    #[inline]
    fn lease_duration(&self) -> i8 {
        self.lease_duration
    }
}

impl<'a, C, ContainerNameSet, BlobNameSet, LeaseDurationSet> ProposedLeaseIdOption<'a>
    for AcquireBlobLeaseBuilder<'a, C, ContainerNameSet, BlobNameSet, LeaseDurationSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    LeaseDurationSet: ToAssign,
    C: Client,
{
    #[inline]
    fn proposed_lease_id(&self) -> Option<&'a LeaseId> {
        self.proposed_lease_id
    }
}

impl<'a, C, ContainerNameSet, BlobNameSet, LeaseDurationSet> TimeoutOption
    for AcquireBlobLeaseBuilder<'a, C, ContainerNameSet, BlobNameSet, LeaseDurationSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    LeaseDurationSet: ToAssign,
    C: Client,
{
    #[inline]
    fn timeout(&self) -> Option<u64> {
        self.timeout
    }
}

impl<'a, C, ContainerNameSet, BlobNameSet, LeaseDurationSet> ClientRequestIdOption<'a>
    for AcquireBlobLeaseBuilder<'a, C, ContainerNameSet, BlobNameSet, LeaseDurationSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    LeaseDurationSet: ToAssign,
    C: Client,
{
    #[inline]
    fn client_request_id(&self) -> Option<&'a str> {
        self.client_request_id
    }
}

impl<'a, C, BlobNameSet, LeaseDurationSet> ContainerNameSupport<'a>
    for AcquireBlobLeaseBuilder<'a, C, No, BlobNameSet, LeaseDurationSet>
where
    BlobNameSet: ToAssign,
    LeaseDurationSet: ToAssign,
    C: Client,
{
    type O = AcquireBlobLeaseBuilder<'a, C, Yes, BlobNameSet, LeaseDurationSet>;

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

impl<'a, C, ContainerNameSet, LeaseDurationSet> BlobNameSupport<'a>
    for AcquireBlobLeaseBuilder<'a, C, ContainerNameSet, No, LeaseDurationSet>
where
    ContainerNameSet: ToAssign,
    LeaseDurationSet: ToAssign,
    C: Client,
{
    type O = AcquireBlobLeaseBuilder<'a, C, ContainerNameSet, Yes, LeaseDurationSet>;

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

impl<'a, C, ContainerNameSet, BlobNameSet> LeaseDurationSupport
    for AcquireBlobLeaseBuilder<'a, C, ContainerNameSet, BlobNameSet, No>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    C: Client,
{
    type O = AcquireBlobLeaseBuilder<'a, C, ContainerNameSet, BlobNameSet, Yes>;

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

impl<'a, C, ContainerNameSet, BlobNameSet, LeaseDurationSet> ProposedLeaseIdSupport<'a>
    for AcquireBlobLeaseBuilder<'a, C, ContainerNameSet, BlobNameSet, LeaseDurationSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    LeaseDurationSet: ToAssign,
    C: Client,
{
    type O = AcquireBlobLeaseBuilder<'a, C, ContainerNameSet, BlobNameSet, LeaseDurationSet>;

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

impl<'a, C, ContainerNameSet, BlobNameSet, LeaseDurationSet> TimeoutSupport
    for AcquireBlobLeaseBuilder<'a, C, ContainerNameSet, BlobNameSet, LeaseDurationSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    LeaseDurationSet: ToAssign,
    C: Client,
{
    type O = AcquireBlobLeaseBuilder<'a, C, ContainerNameSet, BlobNameSet, LeaseDurationSet>;

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

impl<'a, C, ContainerNameSet, BlobNameSet, LeaseDurationSet> ClientRequestIdSupport<'a>
    for AcquireBlobLeaseBuilder<'a, C, ContainerNameSet, BlobNameSet, LeaseDurationSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    LeaseDurationSet: ToAssign,
    C: Client,
{
    type O = AcquireBlobLeaseBuilder<'a, C, ContainerNameSet, BlobNameSet, LeaseDurationSet>;

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

impl<'a, C> AcquireBlobLeaseBuilder<'a, C, Yes, Yes, Yes>
where
    C: Client,
{
    pub async fn finalize(self) -> Result<AcquireBlobLeaseResponse, AzureError> {
        let mut uri = generate_blob_uri(
            self.client(),
            self.container_name(),
            self.blob_name(),
            Some("comp=lease"),
        );

        if let Some(nm) = TimeoutOption::to_uri_parameter(&self) {
            uri = format!("{}&{}", uri, nm);
        }

        let future_response = self.client().perform_request(
            &uri,
            &Method::PUT,
            &|mut request| {
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

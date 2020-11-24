use crate::blob::blob::generate_blob_uri;
use crate::blob::blob::responses::BreakBlobLeaseResponse;
use crate::core::prelude::*;
use azure_core::errors::{check_status_extract_headers_and_body, AzureError};
use azure_core::headers::LEASE_ACTION;
use azure_core::prelude::*;
use azure_core::{No, ToAssign, Yes};
use hyper::{Method, StatusCode};
use std::marker::PhantomData;

#[derive(Debug, Clone)]
pub struct BreakBlobLeaseBuilder<'a, C, ContainerNameSet, BlobNameSet, BreakPeriodSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    BreakPeriodSet: ToAssign,
    C: Client,
{
    client: &'a C,
    p_container_name: PhantomData<ContainerNameSet>,
    p_blob_name: PhantomData<BlobNameSet>,
    p_lease_break_period: PhantomData<BreakPeriodSet>,
    container_name: Option<&'a str>,
    blob_name: Option<&'a str>,
    lease_break_period: u8,
    timeout: Option<u64>,
    client_request_id: Option<&'a str>,
}

impl<'a, C> BreakBlobLeaseBuilder<'a, C, No, No, No>
where
    C: Client,
{
    #[inline]
    pub(crate) fn new(client: &'a C) -> BreakBlobLeaseBuilder<'a, C, No, No, No> {
        BreakBlobLeaseBuilder {
            client,
            p_container_name: PhantomData {},
            container_name: None,
            p_blob_name: PhantomData {},
            blob_name: None,
            p_lease_break_period: PhantomData {},
            lease_break_period: 0,
            timeout: None,
            client_request_id: None,
        }
    }
}

impl<'a, C, ContainerNameSet, BlobNameSet, BreakPeriodSet> ClientRequired<'a, C>
    for BreakBlobLeaseBuilder<'a, C, ContainerNameSet, BlobNameSet, BreakPeriodSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    BreakPeriodSet: ToAssign,
    C: Client,
{
    #[inline]
    fn client(&self) -> &'a C {
        self.client
    }
}

//get mandatory no traits methods

//set mandatory no traits methods
impl<'a, C, BlobNameSet, BreakPeriodSet> ContainerNameRequired<'a>
    for BreakBlobLeaseBuilder<'a, C, Yes, BlobNameSet, BreakPeriodSet>
where
    BlobNameSet: ToAssign,
    BreakPeriodSet: ToAssign,
    C: Client,
{
    #[inline]
    fn container_name(&self) -> &'a str {
        self.container_name.unwrap()
    }
}

impl<'a, C, ContainerNameSet, BreakPeriodSet> BlobNameRequired<'a>
    for BreakBlobLeaseBuilder<'a, C, ContainerNameSet, Yes, BreakPeriodSet>
where
    ContainerNameSet: ToAssign,
    BreakPeriodSet: ToAssign,
    C: Client,
{
    #[inline]
    fn blob_name(&self) -> &'a str {
        self.blob_name.unwrap()
    }
}

impl<'a, C, ContainerNameSet, BlobNameSet> LeaseBreakPeriodRequired
    for BreakBlobLeaseBuilder<'a, C, ContainerNameSet, BlobNameSet, Yes>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    C: Client,
{
    #[inline]
    fn lease_break_period(&self) -> u8 {
        self.lease_break_period
    }
}

impl<'a, C, ContainerNameSet, BlobNameSet, BreakPeriodSet> TimeoutOption
    for BreakBlobLeaseBuilder<'a, C, ContainerNameSet, BlobNameSet, BreakPeriodSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    BreakPeriodSet: ToAssign,
    C: Client,
{
    #[inline]
    fn timeout(&self) -> Option<u64> {
        self.timeout
    }
}

impl<'a, C, ContainerNameSet, BlobNameSet, BreakPeriodSet> ClientRequestIdOption<'a>
    for BreakBlobLeaseBuilder<'a, C, ContainerNameSet, BlobNameSet, BreakPeriodSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    BreakPeriodSet: ToAssign,
    C: Client,
{
    #[inline]
    fn client_request_id(&self) -> Option<&'a str> {
        self.client_request_id
    }
}

impl<'a, C, BlobNameSet, BreakPeriodSet> ContainerNameSupport<'a>
    for BreakBlobLeaseBuilder<'a, C, No, BlobNameSet, BreakPeriodSet>
where
    BlobNameSet: ToAssign,
    BreakPeriodSet: ToAssign,
    C: Client,
{
    type O = BreakBlobLeaseBuilder<'a, C, Yes, BlobNameSet, BreakPeriodSet>;

    #[inline]
    fn with_container_name(self, container_name: &'a str) -> Self::O {
        BreakBlobLeaseBuilder {
            client: self.client,
            p_container_name: PhantomData {},
            p_blob_name: PhantomData {},
            p_lease_break_period: PhantomData {},
            container_name: Some(container_name),
            blob_name: self.blob_name,
            lease_break_period: self.lease_break_period,
            timeout: self.timeout,
            client_request_id: self.client_request_id,
        }
    }
}

impl<'a, C, ContainerNameSet, BreakPeriodSet> BlobNameSupport<'a>
    for BreakBlobLeaseBuilder<'a, C, ContainerNameSet, No, BreakPeriodSet>
where
    ContainerNameSet: ToAssign,
    BreakPeriodSet: ToAssign,
    C: Client,
{
    type O = BreakBlobLeaseBuilder<'a, C, ContainerNameSet, Yes, BreakPeriodSet>;

    #[inline]
    fn with_blob_name(self, blob_name: &'a str) -> Self::O {
        BreakBlobLeaseBuilder {
            client: self.client,
            p_container_name: PhantomData {},
            p_blob_name: PhantomData {},
            p_lease_break_period: PhantomData {},
            container_name: self.container_name,
            blob_name: Some(blob_name),
            lease_break_period: self.lease_break_period,
            timeout: self.timeout,
            client_request_id: self.client_request_id,
        }
    }
}

impl<'a, C, ContainerNameSet, BlobNameSet> LeaseBreakPeriodSupport
    for BreakBlobLeaseBuilder<'a, C, ContainerNameSet, BlobNameSet, No>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    C: Client,
{
    type O = BreakBlobLeaseBuilder<'a, C, ContainerNameSet, BlobNameSet, Yes>;

    #[inline]
    fn with_lease_break_period(self, lease_break_period: u8) -> Self::O {
        BreakBlobLeaseBuilder {
            client: self.client,
            p_container_name: PhantomData {},
            p_blob_name: PhantomData {},
            p_lease_break_period: PhantomData {},
            container_name: self.container_name,
            blob_name: self.blob_name,
            lease_break_period,
            timeout: self.timeout,
            client_request_id: self.client_request_id,
        }
    }
}

impl<'a, C, ContainerNameSet, BlobNameSet, BreakPeriodSet> TimeoutSupport
    for BreakBlobLeaseBuilder<'a, C, ContainerNameSet, BlobNameSet, BreakPeriodSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    BreakPeriodSet: ToAssign,
    C: Client,
{
    type O = BreakBlobLeaseBuilder<'a, C, ContainerNameSet, BlobNameSet, BreakPeriodSet>;

    #[inline]
    fn with_timeout(self, timeout: u64) -> Self::O {
        BreakBlobLeaseBuilder {
            client: self.client,
            p_container_name: PhantomData {},
            p_blob_name: PhantomData {},
            p_lease_break_period: PhantomData {},
            container_name: self.container_name,
            blob_name: self.blob_name,
            lease_break_period: self.lease_break_period,
            timeout: Some(timeout),
            client_request_id: self.client_request_id,
        }
    }
}

impl<'a, C, ContainerNameSet, BlobNameSet, BreakPeriodSet> ClientRequestIdSupport<'a>
    for BreakBlobLeaseBuilder<'a, C, ContainerNameSet, BlobNameSet, BreakPeriodSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    BreakPeriodSet: ToAssign,
    C: Client,
{
    type O = BreakBlobLeaseBuilder<'a, C, ContainerNameSet, BlobNameSet, BreakPeriodSet>;

    #[inline]
    fn with_client_request_id(self, client_request_id: &'a str) -> Self::O {
        BreakBlobLeaseBuilder {
            client: self.client,
            p_container_name: PhantomData {},
            p_blob_name: PhantomData {},
            p_lease_break_period: PhantomData {},
            container_name: self.container_name,
            blob_name: self.blob_name,
            lease_break_period: self.lease_break_period,
            timeout: self.timeout,
            client_request_id: Some(client_request_id),
        }
    }
}

// methods callable only when every mandatory field has been filled
impl<'a, C> BreakBlobLeaseBuilder<'a, C, Yes, Yes, Yes>
where
    C: Client,
{
    pub async fn finalize(self) -> Result<BreakBlobLeaseResponse, AzureError> {
        let mut uri = generate_blob_uri(
            self.client(),
            self.container_name(),
            self.blob_name(),
            Some("comp=lease"),
        );

        if let Some(nm) = TimeoutOption::to_uri_parameter(&self) {
            uri = format!("{}&{}", uri, nm);
        }

        let perform_request_response = self.client().perform_request(
            &uri,
            &Method::PUT,
            &|mut request| {
                request = request.header(LEASE_ACTION, "break");
                request = LeaseBreakPeriodRequired::add_header(&self, request);
                request = ClientRequestIdOption::add_header(&self, request);
                request
            },
            None,
        )?;

        let (headers, _body) = check_status_extract_headers_and_body(
            perform_request_response.response_future,
            StatusCode::ACCEPTED,
        )
        .await?;
        BreakBlobLeaseResponse::from_headers(&headers)
    }
}

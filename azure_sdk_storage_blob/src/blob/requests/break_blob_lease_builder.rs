use crate::blob::generate_blob_uri;
use crate::blob::responses::BreakBlobLeaseResponse;
use azure_sdk_core::errors::{check_status_extract_headers_and_body, AzureError};
use azure_sdk_core::headers::LEASE_ACTION;
use azure_sdk_core::{
    BlobNameRequired, BlobNameSupport, ClientRequestIdOption, ClientRequestIdSupport,
    ContainerNameRequired, ContainerNameSupport, LeaseBreakPeriodRequired, LeaseBreakPeriodSupport,
    TimeoutOption, TimeoutSupport,
};
use azure_sdk_core::{No, ToAssign, Yes};
use azure_sdk_storage_core::client::Client;
use azure_sdk_storage_core::ClientRequired;
use hyper::{Method, StatusCode};
use std::marker::PhantomData;

#[derive(Debug, Clone)]
pub struct BreakBlobLeaseBuilder<'a, ContainerNameSet, BlobNameSet, BreakPeriodSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    BreakPeriodSet: ToAssign,
{
    client: &'a Client,
    p_container_name: PhantomData<ContainerNameSet>,
    p_blob_name: PhantomData<BlobNameSet>,
    p_lease_break_period: PhantomData<BreakPeriodSet>,
    container_name: Option<&'a str>,
    blob_name: Option<&'a str>,
    lease_break_period: u8,
    timeout: Option<u64>,
    client_request_id: Option<&'a str>,
}

impl<'a> BreakBlobLeaseBuilder<'a, No, No, No> {
    #[inline]
    pub(crate) fn new(client: &'a Client) -> BreakBlobLeaseBuilder<'a, No, No, No> {
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

impl<'a, ContainerNameSet, BlobNameSet, BreakPeriodSet> ClientRequired<'a>
    for BreakBlobLeaseBuilder<'a, ContainerNameSet, BlobNameSet, BreakPeriodSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    BreakPeriodSet: ToAssign,
{
    #[inline]
    fn client(&self) -> &'a Client {
        self.client
    }
}

impl<'a, BlobNameSet, BreakPeriodSet> ContainerNameRequired<'a>
    for BreakBlobLeaseBuilder<'a, Yes, BlobNameSet, BreakPeriodSet>
where
    BlobNameSet: ToAssign,
    BreakPeriodSet: ToAssign,
{
    #[inline]
    fn container_name(&self) -> &'a str {
        self.container_name.unwrap()
    }
}

impl<'a, ContainerNameSet, BreakPeriodSet> BlobNameRequired<'a>
    for BreakBlobLeaseBuilder<'a, ContainerNameSet, Yes, BreakPeriodSet>
where
    ContainerNameSet: ToAssign,
    BreakPeriodSet: ToAssign,
{
    #[inline]
    fn blob_name(&self) -> &'a str {
        self.blob_name.unwrap()
    }
}

impl<'a, ContainerNameSet, BlobNameSet> LeaseBreakPeriodRequired
    for BreakBlobLeaseBuilder<'a, ContainerNameSet, BlobNameSet, Yes>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
{
    #[inline]
    fn lease_break_period(&self) -> u8 {
        self.lease_break_period
    }
}

impl<'a, ContainerNameSet, BlobNameSet, BreakPeriodSet> TimeoutOption
    for BreakBlobLeaseBuilder<'a, ContainerNameSet, BlobNameSet, BreakPeriodSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    BreakPeriodSet: ToAssign,
{
    #[inline]
    fn timeout(&self) -> Option<u64> {
        self.timeout
    }
}

impl<'a, ContainerNameSet, BlobNameSet, BreakPeriodSet> ClientRequestIdOption<'a>
    for BreakBlobLeaseBuilder<'a, ContainerNameSet, BlobNameSet, BreakPeriodSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    BreakPeriodSet: ToAssign,
{
    #[inline]
    fn client_request_id(&self) -> Option<&'a str> {
        self.client_request_id
    }
}

impl<'a, ContainerNameSet, BlobNameSet, BreakPeriodSet> ContainerNameSupport<'a>
    for BreakBlobLeaseBuilder<'a, ContainerNameSet, BlobNameSet, BreakPeriodSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    BreakPeriodSet: ToAssign,
{
    type O = BreakBlobLeaseBuilder<'a, Yes, BlobNameSet, BreakPeriodSet>;

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

impl<'a, ContainerNameSet, BlobNameSet, BreakPeriodSet> BlobNameSupport<'a>
    for BreakBlobLeaseBuilder<'a, ContainerNameSet, BlobNameSet, BreakPeriodSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    BreakPeriodSet: ToAssign,
{
    type O = BreakBlobLeaseBuilder<'a, ContainerNameSet, Yes, BreakPeriodSet>;

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

impl<'a, ContainerNameSet, BlobNameSet, BreakPeriodSet> LeaseBreakPeriodSupport
    for BreakBlobLeaseBuilder<'a, ContainerNameSet, BlobNameSet, BreakPeriodSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    BreakPeriodSet: ToAssign,
{
    type O = BreakBlobLeaseBuilder<'a, ContainerNameSet, BlobNameSet, Yes>;

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

impl<'a, ContainerNameSet, BlobNameSet, BreakPeriodSet> TimeoutSupport
    for BreakBlobLeaseBuilder<'a, ContainerNameSet, BlobNameSet, BreakPeriodSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    BreakPeriodSet: ToAssign,
{
    type O = BreakBlobLeaseBuilder<'a, ContainerNameSet, BlobNameSet, BreakPeriodSet>;

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

impl<'a, ContainerNameSet, BlobNameSet, BreakPeriodSet> ClientRequestIdSupport<'a>
    for BreakBlobLeaseBuilder<'a, ContainerNameSet, BlobNameSet, BreakPeriodSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    BreakPeriodSet: ToAssign,
{
    type O = BreakBlobLeaseBuilder<'a, ContainerNameSet, BlobNameSet, BreakPeriodSet>;

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

// methods callable regardless
impl<'a, ContainerNameSet, BlobNameSet, BreakPeriodSet>
    BreakBlobLeaseBuilder<'a, ContainerNameSet, BlobNameSet, BreakPeriodSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    BreakPeriodSet: ToAssign,
{
}

impl<'a> BreakBlobLeaseBuilder<'a, Yes, Yes, Yes> {
    pub async fn finalize(self) -> Result<BreakBlobLeaseResponse, AzureError> {
        let mut uri = generate_blob_uri(&self, Some("comp=lease"));

        if let Some(nm) = TimeoutOption::to_uri_parameter(&self) {
            uri = format!("{}&{}", uri, nm);
        }

        let future_response = self.client().perform_request(
            &uri,
            &Method::PUT,
            |mut request| {
                request = request.header(LEASE_ACTION, "break");
                request = LeaseBreakPeriodRequired::add_header(&self, request);
                request = ClientRequestIdOption::add_header(&self, request);
                request
            },
            None,
        )?;

        let (headers, _body) =
            check_status_extract_headers_and_body(future_response, StatusCode::ACCEPTED).await?;
        BreakBlobLeaseResponse::from_headers(&headers)
    }
}

use azure::core::errors::{check_status_extract_headers_and_body, AzureError};
use azure::core::headers::LEASE_ACTION;
use azure::core::lease::LeaseId;
use azure::core::{
    BlobNameRequired, BlobNameSupport, ClientRequestIdOption, ClientRequestIdSupport, ClientRequired, ContainerNameRequired,
    ContainerNameSupport, LeaseDurationRequired, LeaseDurationSupport, ProposedLeaseIdOption, ProposedLeaseIdSupport, TimeoutOption,
    TimeoutSupport, COMPLETE_ENCODE_SET,
};
use azure::core::{No, ToAssign, Yes};
use azure::storage::blob::responses::AcquireBlobLeaseResponse;
use azure::storage::client::Client;
use futures::future::{done, Future};
use hyper::{Method, StatusCode};
use std::marker::PhantomData;
use url::percent_encoding::utf8_percent_encode;

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

impl<'a, BlobNameSet, LeaseDurationSet> ContainerNameRequired<'a> for AcquireBlobLeaseBuilder<'a, Yes, BlobNameSet, LeaseDurationSet>
where
    BlobNameSet: ToAssign,
    LeaseDurationSet: ToAssign,
{
    #[inline]
    fn container_name(&self) -> &'a str {
        self.container_name.unwrap()
    }
}

impl<'a, ContainerNameSet, LeaseDurationSet> BlobNameRequired<'a> for AcquireBlobLeaseBuilder<'a, ContainerNameSet, Yes, LeaseDurationSet>
where
    ContainerNameSet: ToAssign,
    LeaseDurationSet: ToAssign,
{
    #[inline]
    fn blob_name(&self) -> &'a str {
        self.blob_name.unwrap()
    }
}

impl<'a, ContainerNameSet, BlobNameSet> LeaseDurationRequired for AcquireBlobLeaseBuilder<'a, ContainerNameSet, BlobNameSet, Yes>
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
impl<'a, ContainerNameSet, BlobNameSet, LeaseDurationSet> AcquireBlobLeaseBuilder<'a, ContainerNameSet, BlobNameSet, LeaseDurationSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    LeaseDurationSet: ToAssign,
{}

impl<'a> AcquireBlobLeaseBuilder<'a, Yes, Yes, Yes> {
    pub fn finalize(self) -> impl Future<Item = AcquireBlobLeaseResponse, Error = AzureError> {
        let mut uri = format!(
            "https://{}.blob.core.windows.net/{}/{}?comp=lease",
            self.client().account(),
            utf8_percent_encode(self.container_name(), COMPLETE_ENCODE_SET),
            utf8_percent_encode(self.blob_name(), COMPLETE_ENCODE_SET)
        );

        if let Some(nm) = TimeoutOption::to_uri_parameter(&self) {
            uri = format!("{}&{}", uri, nm);
        }

        let req = self.client().perform_request(
            &uri,
            Method::PUT,
            |ref mut request| {
                request.header(LEASE_ACTION, "acquire");
                LeaseDurationRequired::add_header(&self, request);
                ProposedLeaseIdOption::add_header(&self, request);
                ClientRequestIdOption::add_header(&self, request);
            },
            None,
        );

        done(req)
            .from_err()
            .and_then(move |future_response| check_status_extract_headers_and_body(future_response, StatusCode::CREATED))
            .and_then(|(headers, _body)| done(AcquireBlobLeaseResponse::from_headers(&headers)))
    }
}

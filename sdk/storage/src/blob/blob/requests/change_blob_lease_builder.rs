use crate::blob::blob::generate_blob_uri;
use crate::blob::blob::responses::ChangeBlobLeaseResponse;
use crate::core::prelude::*;
use azure_core::errors::AzureError;
use azure_core::headers::LEASE_ACTION;
use azure_core::lease::LeaseId;
use azure_core::prelude::*;
use azure_core::{No, ToAssign, Yes};
use hyper::{Method, StatusCode};
use std::marker::PhantomData;

#[derive(Debug, Clone)]
pub struct ChangeBlobLeaseBuilder<
    'a,
    C,
    ContainerNameSet,
    BlobNameSet,
    LeaseIdSet,
    ProposedLeaseIdSet,
> where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    LeaseIdSet: ToAssign,
    ProposedLeaseIdSet: ToAssign,
    C: Client,
{
    client: &'a C,
    p_container_name: PhantomData<ContainerNameSet>,
    p_blob_name: PhantomData<BlobNameSet>,
    p_lease_id: PhantomData<LeaseIdSet>,
    p_proposed_lease_id: PhantomData<ProposedLeaseIdSet>,
    container_name: Option<&'a str>,
    blob_name: Option<&'a str>,
    lease_id: Option<&'a LeaseId>,
    proposed_lease_id: Option<&'a LeaseId>,
    timeout: Option<u64>,
    client_request_id: Option<&'a str>,
}

impl<'a, C> ChangeBlobLeaseBuilder<'a, C, No, No, No, No>
where
    C: Client,
{
    #[inline]
    pub(crate) fn new(client: &'a C) -> ChangeBlobLeaseBuilder<'a, C, No, No, No, No> {
        ChangeBlobLeaseBuilder {
            client,
            p_container_name: PhantomData {},
            container_name: None,
            p_blob_name: PhantomData {},
            blob_name: None,
            p_lease_id: PhantomData {},
            lease_id: None,
            p_proposed_lease_id: PhantomData {},
            proposed_lease_id: None,
            timeout: None,
            client_request_id: None,
        }
    }
}

impl<'a, C, ContainerNameSet, BlobNameSet, LeaseIdSet, ProposedLeaseIdSet> ClientRequired<'a, C>
    for ChangeBlobLeaseBuilder<'a, C, ContainerNameSet, BlobNameSet, LeaseIdSet, ProposedLeaseIdSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    LeaseIdSet: ToAssign,
    ProposedLeaseIdSet: ToAssign,
    C: Client,
{
    #[inline]
    fn client(&self) -> &'a C {
        self.client
    }
}

//get mandatory no traits methods

//set mandatory no traits methods
impl<'a, C, BlobNameSet, LeaseIdSet, ProposedLeaseIdSet> ContainerNameRequired<'a>
    for ChangeBlobLeaseBuilder<'a, C, Yes, BlobNameSet, LeaseIdSet, ProposedLeaseIdSet>
where
    BlobNameSet: ToAssign,
    LeaseIdSet: ToAssign,
    ProposedLeaseIdSet: ToAssign,
    C: Client,
{
    #[inline]
    fn container_name(&self) -> &'a str {
        self.container_name.unwrap()
    }
}

impl<'a, C, ContainerNameSet, LeaseIdSet, ProposedLeaseIdSet> BlobNameRequired<'a>
    for ChangeBlobLeaseBuilder<'a, C, ContainerNameSet, Yes, LeaseIdSet, ProposedLeaseIdSet>
where
    ContainerNameSet: ToAssign,
    LeaseIdSet: ToAssign,
    ProposedLeaseIdSet: ToAssign,
    C: Client,
{
    #[inline]
    fn blob_name(&self) -> &'a str {
        self.blob_name.unwrap()
    }
}

impl<'a, C, ContainerNameSet, BlobNameSet, ProposedLeaseIdSet> LeaseIdRequired<'a>
    for ChangeBlobLeaseBuilder<'a, C, ContainerNameSet, BlobNameSet, Yes, ProposedLeaseIdSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    ProposedLeaseIdSet: ToAssign,
    C: Client,
{
    #[inline]
    fn lease_id(&self) -> &'a LeaseId {
        self.lease_id.unwrap()
    }
}

impl<'a, C, ContainerNameSet, BlobNameSet, LeaseIdSet> ProposedLeaseIdRequired<'a>
    for ChangeBlobLeaseBuilder<'a, C, ContainerNameSet, BlobNameSet, LeaseIdSet, Yes>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    LeaseIdSet: ToAssign,
    C: Client,
{
    #[inline]
    fn proposed_lease_id(&self) -> &'a LeaseId {
        self.proposed_lease_id.unwrap()
    }
}

impl<'a, C, ContainerNameSet, BlobNameSet, LeaseIdSet, ProposedLeaseIdSet> TimeoutOption
    for ChangeBlobLeaseBuilder<'a, C, ContainerNameSet, BlobNameSet, LeaseIdSet, ProposedLeaseIdSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    LeaseIdSet: ToAssign,
    ProposedLeaseIdSet: ToAssign,
    C: Client,
{
    #[inline]
    fn timeout(&self) -> Option<u64> {
        self.timeout
    }
}

impl<'a, C, ContainerNameSet, BlobNameSet, LeaseIdSet, ProposedLeaseIdSet> ClientRequestIdOption<'a>
    for ChangeBlobLeaseBuilder<'a, C, ContainerNameSet, BlobNameSet, LeaseIdSet, ProposedLeaseIdSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    LeaseIdSet: ToAssign,
    ProposedLeaseIdSet: ToAssign,
    C: Client,
{
    #[inline]
    fn client_request_id(&self) -> Option<&'a str> {
        self.client_request_id
    }
}

impl<'a, C, BlobNameSet, LeaseIdSet, ProposedLeaseIdSet> ContainerNameSupport<'a>
    for ChangeBlobLeaseBuilder<'a, C, No, BlobNameSet, LeaseIdSet, ProposedLeaseIdSet>
where
    BlobNameSet: ToAssign,
    LeaseIdSet: ToAssign,
    ProposedLeaseIdSet: ToAssign,
    C: Client,
{
    type O = ChangeBlobLeaseBuilder<'a, C, Yes, BlobNameSet, LeaseIdSet, ProposedLeaseIdSet>;

    #[inline]
    fn with_container_name(self, container_name: &'a str) -> Self::O {
        ChangeBlobLeaseBuilder {
            client: self.client,
            p_container_name: PhantomData {},
            p_blob_name: PhantomData {},
            p_lease_id: PhantomData {},
            p_proposed_lease_id: PhantomData {},
            container_name: Some(container_name),
            blob_name: self.blob_name,
            lease_id: self.lease_id,
            proposed_lease_id: self.proposed_lease_id,
            timeout: self.timeout,
            client_request_id: self.client_request_id,
        }
    }
}

impl<'a, C, ContainerNameSet, LeaseIdSet, ProposedLeaseIdSet> BlobNameSupport<'a>
    for ChangeBlobLeaseBuilder<'a, C, ContainerNameSet, No, LeaseIdSet, ProposedLeaseIdSet>
where
    ContainerNameSet: ToAssign,
    LeaseIdSet: ToAssign,
    ProposedLeaseIdSet: ToAssign,
    C: Client,
{
    type O = ChangeBlobLeaseBuilder<'a, C, ContainerNameSet, Yes, LeaseIdSet, ProposedLeaseIdSet>;

    #[inline]
    fn with_blob_name(self, blob_name: &'a str) -> Self::O {
        ChangeBlobLeaseBuilder {
            client: self.client,
            p_container_name: PhantomData {},
            p_blob_name: PhantomData {},
            p_lease_id: PhantomData {},
            p_proposed_lease_id: PhantomData {},
            container_name: self.container_name,
            blob_name: Some(blob_name),
            lease_id: self.lease_id,
            proposed_lease_id: self.proposed_lease_id,
            timeout: self.timeout,
            client_request_id: self.client_request_id,
        }
    }
}

impl<'a, C, ContainerNameSet, BlobNameSet, ProposedLeaseIdSet> LeaseIdSupport<'a>
    for ChangeBlobLeaseBuilder<'a, C, ContainerNameSet, BlobNameSet, No, ProposedLeaseIdSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    ProposedLeaseIdSet: ToAssign,
    C: Client,
{
    type O = ChangeBlobLeaseBuilder<'a, C, ContainerNameSet, BlobNameSet, Yes, ProposedLeaseIdSet>;

    #[inline]
    fn with_lease_id(self, lease_id: &'a LeaseId) -> Self::O {
        ChangeBlobLeaseBuilder {
            client: self.client,
            p_container_name: PhantomData {},
            p_blob_name: PhantomData {},
            p_lease_id: PhantomData {},
            p_proposed_lease_id: PhantomData {},
            container_name: self.container_name,
            blob_name: self.blob_name,
            lease_id: Some(lease_id),
            proposed_lease_id: self.proposed_lease_id,
            timeout: self.timeout,
            client_request_id: self.client_request_id,
        }
    }
}

impl<'a, C, ContainerNameSet, BlobNameSet, LeaseIdSet> ProposedLeaseIdSupport<'a>
    for ChangeBlobLeaseBuilder<'a, C, ContainerNameSet, BlobNameSet, LeaseIdSet, No>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    LeaseIdSet: ToAssign,
    C: Client,
{
    type O = ChangeBlobLeaseBuilder<'a, C, ContainerNameSet, BlobNameSet, LeaseIdSet, Yes>;

    #[inline]
    fn with_proposed_lease_id(self, proposed_lease_id: &'a LeaseId) -> Self::O {
        ChangeBlobLeaseBuilder {
            client: self.client,
            p_container_name: PhantomData {},
            p_blob_name: PhantomData {},
            p_lease_id: PhantomData {},
            p_proposed_lease_id: PhantomData {},
            container_name: self.container_name,
            blob_name: self.blob_name,
            lease_id: self.lease_id,
            proposed_lease_id: Some(proposed_lease_id),
            timeout: self.timeout,
            client_request_id: self.client_request_id,
        }
    }
}

impl<'a, C, ContainerNameSet, BlobNameSet, LeaseIdSet, ProposedLeaseIdSet> TimeoutSupport
    for ChangeBlobLeaseBuilder<'a, C, ContainerNameSet, BlobNameSet, LeaseIdSet, ProposedLeaseIdSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    LeaseIdSet: ToAssign,
    ProposedLeaseIdSet: ToAssign,
    C: Client,
{
    type O = ChangeBlobLeaseBuilder<
        'a,
        C,
        ContainerNameSet,
        BlobNameSet,
        LeaseIdSet,
        ProposedLeaseIdSet,
    >;

    #[inline]
    fn with_timeout(self, timeout: u64) -> Self::O {
        ChangeBlobLeaseBuilder {
            client: self.client,
            p_container_name: PhantomData {},
            p_blob_name: PhantomData {},
            p_lease_id: PhantomData {},
            p_proposed_lease_id: PhantomData {},
            container_name: self.container_name,
            blob_name: self.blob_name,
            lease_id: self.lease_id,
            proposed_lease_id: self.proposed_lease_id,
            timeout: Some(timeout),
            client_request_id: self.client_request_id,
        }
    }
}

impl<'a, C, ContainerNameSet, BlobNameSet, LeaseIdSet, ProposedLeaseIdSet>
    ClientRequestIdSupport<'a>
    for ChangeBlobLeaseBuilder<'a, C, ContainerNameSet, BlobNameSet, LeaseIdSet, ProposedLeaseIdSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    LeaseIdSet: ToAssign,
    ProposedLeaseIdSet: ToAssign,
    C: Client,
{
    type O = ChangeBlobLeaseBuilder<
        'a,
        C,
        ContainerNameSet,
        BlobNameSet,
        LeaseIdSet,
        ProposedLeaseIdSet,
    >;

    #[inline]
    fn with_client_request_id(self, client_request_id: &'a str) -> Self::O {
        ChangeBlobLeaseBuilder {
            client: self.client,
            p_container_name: PhantomData {},
            p_blob_name: PhantomData {},
            p_lease_id: PhantomData {},
            p_proposed_lease_id: PhantomData {},
            container_name: self.container_name,
            blob_name: self.blob_name,
            lease_id: self.lease_id,
            proposed_lease_id: self.proposed_lease_id,
            timeout: self.timeout,
            client_request_id: Some(client_request_id),
        }
    }
}

// methods callable only when every mandatory field has been filled
impl<'a, C> ChangeBlobLeaseBuilder<'a, C, Yes, Yes, Yes, Yes>
where
    C: Client,
{
    pub async fn finalize(self) -> Result<ChangeBlobLeaseResponse, AzureError> {
        let mut uri = generate_blob_uri(
            self.client(),
            self.container_name(),
            self.blob_name(),
            Some("comp=lease"),
        );

        if let Some(nm) = TimeoutOption::to_uri_parameter(&self) {
            uri = format!("{}&{}", uri, nm);
        }

        let (headers, _body) = self
            .client()
            .perform_request(
                &uri,
                &Method::PUT,
                &|mut request| {
                    request = LeaseIdRequired::add_optional_header(&self, request);
                    request = request.header(LEASE_ACTION, "change");
                    request = ProposedLeaseIdRequired::add_optional_header(&self, request);
                    request = ClientRequestIdOption::add_optional_header(&self, request);
                    request
                },
                None,
            )?
            .check_status_extract_headers_and_body(StatusCode::OK)
            .await?;
        ChangeBlobLeaseResponse::from_headers(&headers)
    }
}

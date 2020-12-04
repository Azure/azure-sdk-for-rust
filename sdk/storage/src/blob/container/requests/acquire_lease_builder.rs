use crate::container::responses::AcquireLeaseResponse;
use crate::core::prelude::*;
use azure_core::errors::AzureError;
use azure_core::headers::LEASE_ACTION;
use azure_core::lease::LeaseId;
use azure_core::prelude::*;
use azure_core::{No, ToAssign, Yes};
use hyper::{Method, StatusCode};
use std::marker::PhantomData;

#[derive(Debug, Clone)]
pub struct AcquireLeaseBuilder<'a, C, ContainerNameSet, LeaseDurationSet>
where
    ContainerNameSet: ToAssign,
    LeaseDurationSet: ToAssign,
    C: Client,
{
    client: &'a C,
    p_container_name: PhantomData<ContainerNameSet>,
    p_lease_duration: PhantomData<LeaseDurationSet>,
    container_name: Option<&'a str>,
    client_request_id: Option<&'a str>,
    timeout: Option<u64>,
    lease_id: Option<&'a LeaseId>,
    lease_duration: Option<i8>,
    proposed_lease_id: Option<&'a LeaseId>,
}

impl<'a, C> AcquireLeaseBuilder<'a, C, No, No>
where
    C: Client,
{
    #[inline]
    pub(crate) fn new(client: &'a C) -> AcquireLeaseBuilder<'a, C, No, No> {
        AcquireLeaseBuilder {
            client,
            p_container_name: PhantomData {},
            container_name: None,
            p_lease_duration: PhantomData {},
            lease_duration: None,
            client_request_id: None,
            timeout: None,
            lease_id: None,
            proposed_lease_id: None,
        }
    }
}

impl<'a, C, ContainerNameSet, LeaseDurationSet> ClientRequired<'a, C>
    for AcquireLeaseBuilder<'a, C, ContainerNameSet, LeaseDurationSet>
where
    ContainerNameSet: ToAssign,
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
impl<'a, C, LeaseDurationSet> ContainerNameRequired<'a>
    for AcquireLeaseBuilder<'a, C, Yes, LeaseDurationSet>
where
    LeaseDurationSet: ToAssign,
    C: Client,
{
    #[inline]
    fn container_name(&self) -> &'a str {
        self.container_name.unwrap()
    }
}

impl<'a, C, ContainerNameSet, LeaseDurationSet> ClientRequestIdOption<'a>
    for AcquireLeaseBuilder<'a, C, ContainerNameSet, LeaseDurationSet>
where
    ContainerNameSet: ToAssign,
    LeaseDurationSet: ToAssign,
    C: Client,
{
    #[inline]
    fn client_request_id(&self) -> Option<&'a str> {
        self.client_request_id
    }
}

impl<'a, C, ContainerNameSet, LeaseDurationSet> TimeoutOption
    for AcquireLeaseBuilder<'a, C, ContainerNameSet, LeaseDurationSet>
where
    ContainerNameSet: ToAssign,
    LeaseDurationSet: ToAssign,
    C: Client,
{
    #[inline]
    fn timeout(&self) -> Option<u64> {
        self.timeout
    }
}

impl<'a, C, ContainerNameSet, LeaseDurationSet> LeaseIdOption<'a>
    for AcquireLeaseBuilder<'a, C, ContainerNameSet, LeaseDurationSet>
where
    ContainerNameSet: ToAssign,
    LeaseDurationSet: ToAssign,
    C: Client,
{
    #[inline]
    fn lease_id(&self) -> Option<&'a LeaseId> {
        self.lease_id
    }
}

impl<'a, C, ContainerNameSet> LeaseDurationRequired
    for AcquireLeaseBuilder<'a, C, ContainerNameSet, Yes>
where
    ContainerNameSet: ToAssign,
    C: Client,
{
    #[inline]
    fn lease_duration(&self) -> i8 {
        self.lease_duration.unwrap()
    }
}

impl<'a, C, ContainerNameSet, LeaseDurationSet> ProposedLeaseIdOption<'a>
    for AcquireLeaseBuilder<'a, C, ContainerNameSet, LeaseDurationSet>
where
    ContainerNameSet: ToAssign,
    LeaseDurationSet: ToAssign,
    C: Client,
{
    #[inline]
    fn proposed_lease_id(&self) -> Option<&'a LeaseId> {
        self.proposed_lease_id
    }
}

impl<'a, C, LeaseDurationSet> ContainerNameSupport<'a>
    for AcquireLeaseBuilder<'a, C, No, LeaseDurationSet>
where
    LeaseDurationSet: ToAssign,
    C: Client,
{
    type O = AcquireLeaseBuilder<'a, C, Yes, LeaseDurationSet>;

    #[inline]
    fn with_container_name(self, container_name: &'a str) -> Self::O {
        AcquireLeaseBuilder {
            client: self.client,
            p_container_name: PhantomData {},
            p_lease_duration: PhantomData {},
            container_name: Some(container_name),
            client_request_id: self.client_request_id,
            timeout: self.timeout,
            lease_id: self.lease_id,
            lease_duration: self.lease_duration,
            proposed_lease_id: self.proposed_lease_id,
        }
    }
}

impl<'a, C, ContainerNameSet, LeaseDurationSet> ClientRequestIdSupport<'a>
    for AcquireLeaseBuilder<'a, C, ContainerNameSet, LeaseDurationSet>
where
    ContainerNameSet: ToAssign,
    LeaseDurationSet: ToAssign,
    C: Client,
{
    type O = AcquireLeaseBuilder<'a, C, ContainerNameSet, LeaseDurationSet>;

    #[inline]
    fn with_client_request_id(self, client_request_id: &'a str) -> Self::O {
        AcquireLeaseBuilder {
            client: self.client,
            p_container_name: PhantomData {},
            p_lease_duration: PhantomData {},
            container_name: self.container_name,
            client_request_id: Some(client_request_id),
            timeout: self.timeout,
            lease_id: self.lease_id,
            lease_duration: self.lease_duration,
            proposed_lease_id: self.proposed_lease_id,
        }
    }
}

impl<'a, C, ContainerNameSet, LeaseDurationSet> TimeoutSupport
    for AcquireLeaseBuilder<'a, C, ContainerNameSet, LeaseDurationSet>
where
    ContainerNameSet: ToAssign,
    LeaseDurationSet: ToAssign,
    C: Client,
{
    type O = AcquireLeaseBuilder<'a, C, ContainerNameSet, LeaseDurationSet>;

    #[inline]
    fn with_timeout(self, timeout: u64) -> Self::O {
        AcquireLeaseBuilder {
            client: self.client,
            p_container_name: PhantomData {},
            p_lease_duration: PhantomData {},
            container_name: self.container_name,
            client_request_id: self.client_request_id,
            timeout: Some(timeout),
            lease_id: self.lease_id,
            lease_duration: self.lease_duration,
            proposed_lease_id: self.proposed_lease_id,
        }
    }
}

impl<'a, C, ContainerNameSet, LeaseDurationSet> LeaseIdSupport<'a>
    for AcquireLeaseBuilder<'a, C, ContainerNameSet, LeaseDurationSet>
where
    ContainerNameSet: ToAssign,
    LeaseDurationSet: ToAssign,
    C: Client,
{
    type O = AcquireLeaseBuilder<'a, C, ContainerNameSet, LeaseDurationSet>;

    #[inline]
    fn with_lease_id(self, lease_id: &'a LeaseId) -> Self::O {
        AcquireLeaseBuilder {
            client: self.client,
            p_container_name: PhantomData {},
            p_lease_duration: PhantomData {},
            container_name: self.container_name,
            client_request_id: self.client_request_id,
            timeout: self.timeout,
            lease_id: Some(lease_id),
            lease_duration: self.lease_duration,
            proposed_lease_id: self.proposed_lease_id,
        }
    }
}

impl<'a, C, ContainerNameSet> LeaseDurationSupport
    for AcquireLeaseBuilder<'a, C, ContainerNameSet, No>
where
    ContainerNameSet: ToAssign,
    C: Client,
{
    type O = AcquireLeaseBuilder<'a, C, ContainerNameSet, Yes>;

    #[inline]
    fn with_lease_duration(self, lease_duration: i8) -> Self::O {
        AcquireLeaseBuilder {
            client: self.client,
            p_container_name: PhantomData {},
            p_lease_duration: PhantomData {},
            container_name: self.container_name,
            client_request_id: self.client_request_id,
            timeout: self.timeout,
            lease_id: self.lease_id,
            lease_duration: Some(lease_duration),
            proposed_lease_id: self.proposed_lease_id,
        }
    }
}

impl<'a, C, ContainerNameSet, LeaseDurationSet> ProposedLeaseIdSupport<'a>
    for AcquireLeaseBuilder<'a, C, ContainerNameSet, LeaseDurationSet>
where
    ContainerNameSet: ToAssign,
    LeaseDurationSet: ToAssign,
    C: Client,
{
    type O = AcquireLeaseBuilder<'a, C, ContainerNameSet, LeaseDurationSet>;

    #[inline]
    fn with_proposed_lease_id(self, proposed_lease_id: &'a LeaseId) -> Self::O {
        AcquireLeaseBuilder {
            client: self.client,
            p_container_name: PhantomData {},
            p_lease_duration: PhantomData {},
            container_name: self.container_name,
            client_request_id: self.client_request_id,
            timeout: self.timeout,
            lease_id: self.lease_id,
            lease_duration: self.lease_duration,
            proposed_lease_id: Some(proposed_lease_id),
        }
    }
}

// methods callable only when every mandatory field has been filled
impl<'a, C> AcquireLeaseBuilder<'a, C, Yes, Yes>
where
    C: Client,
{
    pub async fn finalize(self) -> Result<AcquireLeaseResponse, AzureError> {
        let mut uri = format!(
            "{}/{}?comp=lease&restype=container",
            self.client().blob_uri(),
            self.container_name()
        );

        if let Some(nm) = TimeoutOption::to_uri_parameter(&self) {
            uri = format!("{}&{}", uri, nm);
        }

        let perform_request_response = self.client().perform_request(
            &uri,
            &Method::PUT,
            &|mut request| {
                request = ClientRequestIdOption::add_optional_header(&self, request);
                request = LeaseIdOption::add_optional_header(&self, request);
                request = request.header(LEASE_ACTION, "acquire");
                request = LeaseDurationRequired::add_mandatory_header(&self, request);
                request = ProposedLeaseIdOption::add_optional_header(&self, request);
                request
            },
            Some(&[]),
        )?;

        let (headers, _body) = perform_request_response
            .check_status_extract_headers_and_body(StatusCode::CREATED)
            .await?;
        AcquireLeaseResponse::from_headers(&headers)
    }
}

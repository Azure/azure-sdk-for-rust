use crate::container::responses::RenewLeaseResponse;
use crate::core::prelude::*;
use azure_core::errors::AzureError;
use azure_core::headers::LEASE_ACTION;
use azure_core::lease::LeaseId;
use azure_core::prelude::*;
use azure_core::{No, ToAssign, Yes};
use hyper::{Method, StatusCode};
use std::marker::PhantomData;

#[derive(Debug, Clone)]
pub struct RenewLeaseBuilder<'a, C, ContainerNameSet, LeaseIdSet>
where
    ContainerNameSet: ToAssign,
    LeaseIdSet: ToAssign,
    C: Client,
{
    client: &'a C,
    p_container_name: PhantomData<ContainerNameSet>,
    p_lease_id: PhantomData<LeaseIdSet>,
    container_name: Option<&'a str>,
    client_request_id: Option<&'a str>,
    timeout: Option<u64>,
    lease_id: Option<&'a LeaseId>,
}

impl<'a, C> RenewLeaseBuilder<'a, C, No, No>
where
    C: Client,
{
    #[inline]
    pub(crate) fn new(client: &'a C) -> RenewLeaseBuilder<'a, C, No, No> {
        RenewLeaseBuilder {
            client,
            p_container_name: PhantomData {},
            container_name: None,
            p_lease_id: PhantomData {},
            lease_id: None,
            client_request_id: None,
            timeout: None,
        }
    }
}

impl<'a, C, ContainerNameSet, LeaseIdSet> ClientRequired<'a, C>
    for RenewLeaseBuilder<'a, C, ContainerNameSet, LeaseIdSet>
where
    ContainerNameSet: ToAssign,
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
impl<'a, C, LeaseIdSet> ContainerNameRequired<'a> for RenewLeaseBuilder<'a, C, Yes, LeaseIdSet>
where
    LeaseIdSet: ToAssign,
    C: Client,
{
    #[inline]
    fn container_name(&self) -> &'a str {
        self.container_name.unwrap()
    }
}

impl<'a, C, ContainerNameSet, LeaseIdSet> ClientRequestIdOption<'a>
    for RenewLeaseBuilder<'a, C, ContainerNameSet, LeaseIdSet>
where
    ContainerNameSet: ToAssign,
    LeaseIdSet: ToAssign,
    C: Client,
{
    #[inline]
    fn client_request_id(&self) -> Option<&'a str> {
        self.client_request_id
    }
}

impl<'a, C, ContainerNameSet, LeaseIdSet> TimeoutOption
    for RenewLeaseBuilder<'a, C, ContainerNameSet, LeaseIdSet>
where
    ContainerNameSet: ToAssign,
    LeaseIdSet: ToAssign,
    C: Client,
{
    #[inline]
    fn timeout(&self) -> Option<u64> {
        self.timeout
    }
}

impl<'a, C, ContainerNameSet> LeaseIdRequired<'a>
    for RenewLeaseBuilder<'a, C, ContainerNameSet, Yes>
where
    ContainerNameSet: ToAssign,
    C: Client,
{
    #[inline]
    fn lease_id(&self) -> &'a LeaseId {
        self.lease_id.unwrap()
    }
}

impl<'a, C, LeaseIdSet> ContainerNameSupport<'a> for RenewLeaseBuilder<'a, C, No, LeaseIdSet>
where
    LeaseIdSet: ToAssign,
    C: Client,
{
    type O = RenewLeaseBuilder<'a, C, Yes, LeaseIdSet>;

    #[inline]
    fn with_container_name(self, container_name: &'a str) -> Self::O {
        RenewLeaseBuilder {
            client: self.client,
            p_container_name: PhantomData {},
            p_lease_id: PhantomData {},
            container_name: Some(container_name),
            client_request_id: self.client_request_id,
            timeout: self.timeout,
            lease_id: self.lease_id,
        }
    }
}

impl<'a, C, ContainerNameSet, LeaseIdSet> ClientRequestIdSupport<'a>
    for RenewLeaseBuilder<'a, C, ContainerNameSet, LeaseIdSet>
where
    ContainerNameSet: ToAssign,
    LeaseIdSet: ToAssign,
    C: Client,
{
    type O = RenewLeaseBuilder<'a, C, ContainerNameSet, LeaseIdSet>;

    #[inline]
    fn with_client_request_id(self, client_request_id: &'a str) -> Self::O {
        RenewLeaseBuilder {
            client: self.client,
            p_container_name: PhantomData {},
            p_lease_id: PhantomData {},
            container_name: self.container_name,
            client_request_id: Some(client_request_id),
            timeout: self.timeout,
            lease_id: self.lease_id,
        }
    }
}

impl<'a, C, ContainerNameSet, LeaseIdSet> TimeoutSupport
    for RenewLeaseBuilder<'a, C, ContainerNameSet, LeaseIdSet>
where
    ContainerNameSet: ToAssign,
    LeaseIdSet: ToAssign,
    C: Client,
{
    type O = RenewLeaseBuilder<'a, C, ContainerNameSet, LeaseIdSet>;

    #[inline]
    fn with_timeout(self, timeout: u64) -> Self::O {
        RenewLeaseBuilder {
            client: self.client,
            p_container_name: PhantomData {},
            p_lease_id: PhantomData {},
            container_name: self.container_name,
            client_request_id: self.client_request_id,
            timeout: Some(timeout),
            lease_id: self.lease_id,
        }
    }
}

impl<'a, C, ContainerNameSet> LeaseIdSupport<'a> for RenewLeaseBuilder<'a, C, ContainerNameSet, No>
where
    ContainerNameSet: ToAssign,
    C: Client,
{
    type O = RenewLeaseBuilder<'a, C, ContainerNameSet, Yes>;

    #[inline]
    fn with_lease_id(self, lease_id: &'a LeaseId) -> Self::O {
        RenewLeaseBuilder {
            client: self.client,
            p_container_name: PhantomData {},
            p_lease_id: PhantomData {},
            container_name: self.container_name,
            client_request_id: self.client_request_id,
            timeout: self.timeout,
            lease_id: Some(lease_id),
        }
    }
}

// methods callable only when every mandatory field has been filled
impl<'a, C> RenewLeaseBuilder<'a, C, Yes, Yes>
where
    C: Client,
{
    pub async fn finalize(self) -> Result<RenewLeaseResponse, AzureError> {
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
                request = ClientRequestIdOption::add_header(&self, request);
                request = LeaseIdRequired::add_header(&self, request);
                request = request.header(LEASE_ACTION, "renew");
                request
            },
            Some(&[]),
        )?;

        let (headers, _body) = perform_request_response
            .check_status_extract_headers_and_body(StatusCode::OK)
            .await?;
        RenewLeaseResponse::from_headers(&headers)
    }
}

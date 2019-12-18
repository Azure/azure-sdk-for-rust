use crate::container::responses::BreakLeaseResponse;
use azure_sdk_core::errors::{check_status_extract_headers_and_body, AzureError};
use azure_sdk_core::headers::LEASE_ACTION;
use azure_sdk_core::lease::LeaseId;
use azure_sdk_core::{
    ClientRequestIdOption, ClientRequestIdSupport, ContainerNameRequired, ContainerNameSupport,
    LeaseBreakPeriodOption, LeaseBreakPeriodSupport, LeaseIdOption, LeaseIdSupport, TimeoutOption,
    TimeoutSupport,
};
use azure_sdk_core::{No, ToAssign, Yes};
use azure_sdk_storage_core::client::Client;
use azure_sdk_storage_core::ClientRequired;
use hyper::{Method, StatusCode};
use std::marker::PhantomData;

#[derive(Debug, Clone)]
pub struct BreakLeaseBuilder<'a, ContainerNameSet>
where
    ContainerNameSet: ToAssign,
{
    client: &'a Client,
    p_container_name: PhantomData<ContainerNameSet>,
    container_name: Option<&'a str>,
    client_request_id: Option<&'a str>,
    timeout: Option<u64>,
    lease_break_period: Option<u8>,
    lease_id: Option<&'a LeaseId>,
}

impl<'a> BreakLeaseBuilder<'a, No> {
    pub(crate) fn new(client: &'a Client) -> BreakLeaseBuilder<'a, No> {
        BreakLeaseBuilder {
            client,
            p_container_name: PhantomData {},
            container_name: None,
            client_request_id: None,
            timeout: None,
            lease_break_period: None,
            lease_id: None,
        }
    }
}

impl<'a, ContainerNameSet> ClientRequired<'a> for BreakLeaseBuilder<'a, ContainerNameSet>
where
    ContainerNameSet: ToAssign,
{
    fn client(&self) -> &'a Client {
        self.client
    }
}

impl<'a> ContainerNameRequired<'a> for BreakLeaseBuilder<'a, Yes> {
    fn container_name(&self) -> &'a str {
        self.container_name.unwrap()
    }
}

impl<'a, ContainerNameSet> ClientRequestIdOption<'a> for BreakLeaseBuilder<'a, ContainerNameSet>
where
    ContainerNameSet: ToAssign,
{
    fn client_request_id(&self) -> Option<&'a str> {
        self.client_request_id
    }
}

impl<'a, ContainerNameSet> TimeoutOption for BreakLeaseBuilder<'a, ContainerNameSet>
where
    ContainerNameSet: ToAssign,
{
    fn timeout(&self) -> Option<u64> {
        self.timeout
    }
}

impl<'a, ContainerNameSet> LeaseBreakPeriodOption for BreakLeaseBuilder<'a, ContainerNameSet>
where
    ContainerNameSet: ToAssign,
{
    fn lease_break_period(&self) -> Option<u8> {
        self.lease_break_period
    }
}

impl<'a, ContainerNameSet> LeaseIdOption<'a> for BreakLeaseBuilder<'a, ContainerNameSet>
where
    ContainerNameSet: ToAssign,
{
    fn lease_id(&self) -> Option<&'a LeaseId> {
        self.lease_id
    }
}

impl<'a, ContainerNameSet> ContainerNameSupport<'a> for BreakLeaseBuilder<'a, ContainerNameSet>
where
    ContainerNameSet: ToAssign,
{
    type O = BreakLeaseBuilder<'a, Yes>;

    fn with_container_name(self, container_name: &'a str) -> Self::O {
        BreakLeaseBuilder {
            client: self.client,
            p_container_name: PhantomData {},
            container_name: Some(container_name),
            client_request_id: self.client_request_id,
            timeout: self.timeout,
            lease_break_period: self.lease_break_period,
            lease_id: self.lease_id,
        }
    }
}

impl<'a, ContainerNameSet> ClientRequestIdSupport<'a> for BreakLeaseBuilder<'a, ContainerNameSet>
where
    ContainerNameSet: ToAssign,
{
    type O = BreakLeaseBuilder<'a, ContainerNameSet>;

    fn with_client_request_id(self, client_request_id: &'a str) -> Self::O {
        BreakLeaseBuilder {
            client: self.client,
            p_container_name: PhantomData {},
            container_name: self.container_name,
            client_request_id: Some(client_request_id),
            timeout: self.timeout,
            lease_break_period: self.lease_break_period,
            lease_id: self.lease_id,
        }
    }
}

impl<'a, ContainerNameSet> TimeoutSupport for BreakLeaseBuilder<'a, ContainerNameSet>
where
    ContainerNameSet: ToAssign,
{
    type O = BreakLeaseBuilder<'a, ContainerNameSet>;

    fn with_timeout(self, timeout: u64) -> Self::O {
        BreakLeaseBuilder {
            client: self.client,
            p_container_name: PhantomData {},
            container_name: self.container_name,
            client_request_id: self.client_request_id,
            timeout: Some(timeout),
            lease_break_period: self.lease_break_period,
            lease_id: self.lease_id,
        }
    }
}

impl<'a, ContainerNameSet> LeaseBreakPeriodSupport for BreakLeaseBuilder<'a, ContainerNameSet>
where
    ContainerNameSet: ToAssign,
{
    type O = BreakLeaseBuilder<'a, ContainerNameSet>;

    fn with_lease_break_period(self, lease_break_period: u8) -> Self::O {
        BreakLeaseBuilder {
            client: self.client,
            p_container_name: PhantomData {},
            container_name: self.container_name,
            client_request_id: self.client_request_id,
            timeout: self.timeout,
            lease_break_period: Some(lease_break_period),
            lease_id: self.lease_id,
        }
    }
}

impl<'a, ContainerNameSet> LeaseIdSupport<'a> for BreakLeaseBuilder<'a, ContainerNameSet>
where
    ContainerNameSet: ToAssign,
{
    type O = BreakLeaseBuilder<'a, ContainerNameSet>;

    fn with_lease_id(self, lease_id: &'a LeaseId) -> Self::O {
        BreakLeaseBuilder {
            client: self.client,
            p_container_name: PhantomData {},
            container_name: self.container_name,
            client_request_id: self.client_request_id,
            timeout: self.timeout,
            lease_break_period: self.lease_break_period,
            lease_id: Some(lease_id),
        }
    }
}

// methods callable regardless
impl<'a, ContainerNameSet> BreakLeaseBuilder<'a, ContainerNameSet> where ContainerNameSet: ToAssign {}

impl<'a> BreakLeaseBuilder<'a, Yes> {
    pub async fn finalize(self) -> Result<BreakLeaseResponse, AzureError> {
        let mut uri = format!(
            "{}/{}?comp=lease&restype=container",
            self.client().blob_uri(),
            self.container_name()
        );

        if let Some(nm) = TimeoutOption::to_uri_parameter(&self) {
            uri = format!("{}&{}", uri, nm);
        }

        let future_response = self.client().perform_request(
            &uri,
            &Method::PUT,
            |mut request| {
                request = ClientRequestIdOption::add_header(&self, request);
                request = LeaseIdOption::add_header(&self, request);
                request = request.header(LEASE_ACTION, "break");
                request = LeaseBreakPeriodOption::add_header(&self, request);
                request
            },
            Some(&[]),
        )?;

        let (headers, _body) =
            check_status_extract_headers_and_body(future_response, StatusCode::ACCEPTED).await?;
        BreakLeaseResponse::from_headers(&headers)
    }
}

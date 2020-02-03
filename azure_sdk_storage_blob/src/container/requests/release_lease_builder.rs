use crate::container::responses::ReleaseLeaseResponse;
use azure_sdk_core::errors::{check_status_extract_headers_and_body, AzureError};
use azure_sdk_core::headers::LEASE_ACTION;
use azure_sdk_core::lease::LeaseId;
use azure_sdk_core::{
    ClientRequestIdOption, ClientRequestIdSupport, ContainerNameRequired, ContainerNameSupport,
    LeaseIdRequired, LeaseIdSupport, TimeoutOption, TimeoutSupport,
};
use azure_sdk_core::{No, ToAssign, Yes};
use azure_sdk_storage_core::client::Client;
use azure_sdk_storage_core::ClientRequired;
use hyper::{Method, StatusCode};
use std::marker::PhantomData;

#[derive(Debug, Clone)]
pub struct ReleaseLeaseBuilder<'a, ContainerNameSet, LeaseIdSet>
where
    ContainerNameSet: ToAssign,
    LeaseIdSet: ToAssign,
{
    client: &'a Client,
    p_container_name: PhantomData<ContainerNameSet>,
    p_lease_id: PhantomData<LeaseIdSet>,
    container_name: Option<&'a str>,
    client_request_id: Option<&'a str>,
    timeout: Option<u64>,
    lease_id: Option<&'a LeaseId>,
}

impl<'a> ReleaseLeaseBuilder<'a, No, No> {
    pub(crate) fn new(client: &'a Client) -> ReleaseLeaseBuilder<'a, No, No> {
        ReleaseLeaseBuilder {
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

impl<'a, ContainerNameSet, LeaseIdSet> ClientRequired<'a>
    for ReleaseLeaseBuilder<'a, ContainerNameSet, LeaseIdSet>
where
    ContainerNameSet: ToAssign,
    LeaseIdSet: ToAssign,
{
    fn client(&self) -> &'a Client {
        self.client
    }
}

impl<'a, LeaseIdSet> ContainerNameRequired<'a> for ReleaseLeaseBuilder<'a, Yes, LeaseIdSet>
where
    LeaseIdSet: ToAssign,
{
    fn container_name(&self) -> &'a str {
        self.container_name.unwrap()
    }
}

impl<'a, ContainerNameSet, LeaseIdSet> ClientRequestIdOption<'a>
    for ReleaseLeaseBuilder<'a, ContainerNameSet, LeaseIdSet>
where
    ContainerNameSet: ToAssign,
    LeaseIdSet: ToAssign,
{
    fn client_request_id(&self) -> Option<&'a str> {
        self.client_request_id
    }
}

impl<'a, ContainerNameSet, LeaseIdSet> TimeoutOption
    for ReleaseLeaseBuilder<'a, ContainerNameSet, LeaseIdSet>
where
    ContainerNameSet: ToAssign,
    LeaseIdSet: ToAssign,
{
    fn timeout(&self) -> Option<u64> {
        self.timeout
    }
}

impl<'a, ContainerNameSet> LeaseIdRequired<'a> for ReleaseLeaseBuilder<'a, ContainerNameSet, Yes>
where
    ContainerNameSet: ToAssign,
{
    fn lease_id(&self) -> &'a LeaseId {
        self.lease_id.unwrap()
    }
}

impl<'a, ContainerNameSet, LeaseIdSet> ContainerNameSupport<'a>
    for ReleaseLeaseBuilder<'a, ContainerNameSet, LeaseIdSet>
where
    ContainerNameSet: ToAssign,
    LeaseIdSet: ToAssign,
{
    type O = ReleaseLeaseBuilder<'a, Yes, LeaseIdSet>;

    fn with_container_name(self, container_name: &'a str) -> Self::O {
        ReleaseLeaseBuilder {
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

impl<'a, ContainerNameSet, LeaseIdSet> ClientRequestIdSupport<'a>
    for ReleaseLeaseBuilder<'a, ContainerNameSet, LeaseIdSet>
where
    ContainerNameSet: ToAssign,
    LeaseIdSet: ToAssign,
{
    type O = ReleaseLeaseBuilder<'a, ContainerNameSet, LeaseIdSet>;

    fn with_client_request_id(self, client_request_id: &'a str) -> Self::O {
        ReleaseLeaseBuilder {
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

impl<'a, ContainerNameSet, LeaseIdSet> TimeoutSupport
    for ReleaseLeaseBuilder<'a, ContainerNameSet, LeaseIdSet>
where
    ContainerNameSet: ToAssign,
    LeaseIdSet: ToAssign,
{
    type O = ReleaseLeaseBuilder<'a, ContainerNameSet, LeaseIdSet>;

    fn with_timeout(self, timeout: u64) -> Self::O {
        ReleaseLeaseBuilder {
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

impl<'a, ContainerNameSet, LeaseIdSet> LeaseIdSupport<'a>
    for ReleaseLeaseBuilder<'a, ContainerNameSet, LeaseIdSet>
where
    ContainerNameSet: ToAssign,
    LeaseIdSet: ToAssign,
{
    type O = ReleaseLeaseBuilder<'a, ContainerNameSet, Yes>;

    fn with_lease_id(self, lease_id: &'a LeaseId) -> Self::O {
        ReleaseLeaseBuilder {
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

// methods callable regardless
impl<'a, ContainerNameSet, LeaseIdSet> ReleaseLeaseBuilder<'a, ContainerNameSet, LeaseIdSet>
where
    ContainerNameSet: ToAssign,
    LeaseIdSet: ToAssign,
{
}

impl<'a> ReleaseLeaseBuilder<'a, Yes, Yes> {
    pub async fn finalize(self) -> Result<ReleaseLeaseResponse, AzureError> {
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
                request = LeaseIdRequired::add_header(&self, request);
                request = request.header(LEASE_ACTION, "release");
                request
            },
            Some(&[]),
        )?;

        let (headers, _body) =
            check_status_extract_headers_and_body(future_response, StatusCode::OK).await?;
        ReleaseLeaseResponse::from_headers(&headers)
    }
}

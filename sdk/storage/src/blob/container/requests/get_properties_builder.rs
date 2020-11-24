use crate::container::responses::GetPropertiesResponse;
use crate::core::prelude::*;
use azure_core::errors::AzureError;
use azure_core::lease::LeaseId;
use azure_core::prelude::*;
use azure_core::{No, ToAssign, Yes};
use hyper::{Method, StatusCode};
use std::marker::PhantomData;

#[derive(Debug, Clone)]
pub struct GetPropertiesBuilder<'a, C, ContainerNameSet>
where
    ContainerNameSet: ToAssign,
    C: Client,
{
    client: &'a C,
    p_container_name: PhantomData<ContainerNameSet>,
    container_name: Option<&'a str>,
    client_request_id: Option<&'a str>,
    timeout: Option<u64>,
    lease_id: Option<&'a LeaseId>,
}

impl<'a, C> GetPropertiesBuilder<'a, C, No>
where
    C: Client,
{
    #[inline]
    pub(crate) fn new(client: &'a C) -> GetPropertiesBuilder<'a, C, No> {
        GetPropertiesBuilder {
            client,
            p_container_name: PhantomData {},
            container_name: None,
            client_request_id: None,
            timeout: None,
            lease_id: None,
        }
    }
}

impl<'a, C, ContainerNameSet> ClientRequired<'a, C>
    for GetPropertiesBuilder<'a, C, ContainerNameSet>
where
    ContainerNameSet: ToAssign,
    C: Client,
{
    #[inline]
    fn client(&self) -> &'a C {
        self.client
    }
}

//get mandatory no traits methods

//set mandatory no traits methods
impl<'a, C> ContainerNameRequired<'a> for GetPropertiesBuilder<'a, C, Yes>
where
    C: Client,
{
    #[inline]
    fn container_name(&self) -> &'a str {
        self.container_name.unwrap()
    }
}

impl<'a, C, ContainerNameSet> ClientRequestIdOption<'a>
    for GetPropertiesBuilder<'a, C, ContainerNameSet>
where
    ContainerNameSet: ToAssign,
    C: Client,
{
    #[inline]
    fn client_request_id(&self) -> Option<&'a str> {
        self.client_request_id
    }
}

impl<'a, C, ContainerNameSet> TimeoutOption for GetPropertiesBuilder<'a, C, ContainerNameSet>
where
    ContainerNameSet: ToAssign,
    C: Client,
{
    #[inline]
    fn timeout(&self) -> Option<u64> {
        self.timeout
    }
}

impl<'a, C, ContainerNameSet> LeaseIdOption<'a> for GetPropertiesBuilder<'a, C, ContainerNameSet>
where
    ContainerNameSet: ToAssign,
    C: Client,
{
    #[inline]
    fn lease_id(&self) -> Option<&'a LeaseId> {
        self.lease_id
    }
}

impl<'a, C> ContainerNameSupport<'a> for GetPropertiesBuilder<'a, C, No>
where
    C: Client,
{
    type O = GetPropertiesBuilder<'a, C, Yes>;

    #[inline]
    fn with_container_name(self, container_name: &'a str) -> Self::O {
        GetPropertiesBuilder {
            client: self.client,
            p_container_name: PhantomData {},
            container_name: Some(container_name),
            client_request_id: self.client_request_id,
            timeout: self.timeout,
            lease_id: self.lease_id,
        }
    }
}

impl<'a, C, ContainerNameSet> ClientRequestIdSupport<'a>
    for GetPropertiesBuilder<'a, C, ContainerNameSet>
where
    ContainerNameSet: ToAssign,
    C: Client,
{
    type O = GetPropertiesBuilder<'a, C, ContainerNameSet>;

    #[inline]
    fn with_client_request_id(self, client_request_id: &'a str) -> Self::O {
        GetPropertiesBuilder {
            client: self.client,
            p_container_name: PhantomData {},
            container_name: self.container_name,
            client_request_id: Some(client_request_id),
            timeout: self.timeout,
            lease_id: self.lease_id,
        }
    }
}

impl<'a, C, ContainerNameSet> TimeoutSupport for GetPropertiesBuilder<'a, C, ContainerNameSet>
where
    ContainerNameSet: ToAssign,
    C: Client,
{
    type O = GetPropertiesBuilder<'a, C, ContainerNameSet>;

    #[inline]
    fn with_timeout(self, timeout: u64) -> Self::O {
        GetPropertiesBuilder {
            client: self.client,
            p_container_name: PhantomData {},
            container_name: self.container_name,
            client_request_id: self.client_request_id,
            timeout: Some(timeout),
            lease_id: self.lease_id,
        }
    }
}

impl<'a, C, ContainerNameSet> LeaseIdSupport<'a> for GetPropertiesBuilder<'a, C, ContainerNameSet>
where
    ContainerNameSet: ToAssign,
    C: Client,
{
    type O = GetPropertiesBuilder<'a, C, ContainerNameSet>;

    #[inline]
    fn with_lease_id(self, lease_id: &'a LeaseId) -> Self::O {
        GetPropertiesBuilder {
            client: self.client,
            p_container_name: PhantomData {},
            container_name: self.container_name,
            client_request_id: self.client_request_id,
            timeout: self.timeout,
            lease_id: Some(lease_id),
        }
    }
}

// methods callable only when every mandatory field has been filled
impl<'a, C> GetPropertiesBuilder<'a, C, Yes>
where
    C: Client,
{
    pub async fn finalize(self) -> Result<GetPropertiesResponse, AzureError> {
        let mut uri = format!(
            "{}/{}?restype=container",
            self.client().blob_uri(),
            self.container_name()
        );

        if let Some(nm) = TimeoutOption::to_uri_parameter(&self) {
            uri = format!("{}&{}", uri, nm);
        }

        let perform_request_response = self.client().perform_request(
            &uri,
            &Method::HEAD,
            &|mut request| {
                request = ClientRequestIdOption::add_header(&self, request);
                request = LeaseIdOption::add_header(&self, request);
                request
            },
            None,
        )?;

        let (headers, _) = perform_request_response
            .check_status_extract_headers_and_body(StatusCode::OK)
            .await?;
        GetPropertiesResponse::from_response(self.container_name().to_owned(), &headers)
    }
}

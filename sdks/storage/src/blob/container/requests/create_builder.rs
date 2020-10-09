use crate::container::{PublicAccess, PublicAccessRequired, PublicAccessSupport};
use crate::core::prelude::*;
use azure_sdk_core::errors::{check_status_extract_body, AzureError};
use azure_sdk_core::prelude::*;
use azure_sdk_core::{No, ToAssign, Yes};
use hyper::{Method, StatusCode};
use std::collections::HashMap;
use std::marker::PhantomData;

#[derive(Debug, Clone)]
pub struct CreateBuilder<'a, C, ContainerNameSet, PublicAccessSet>
where
    ContainerNameSet: ToAssign,
    PublicAccessSet: ToAssign,
    C: Client,
{
    client: &'a C,
    p_container_name: PhantomData<ContainerNameSet>,
    p_public_access: PhantomData<PublicAccessSet>,
    container_name: Option<&'a str>,
    public_access: PublicAccess,
    metadata: Option<&'a HashMap<&'a str, &'a str>>,
    client_request_id: Option<&'a str>,
    timeout: Option<u64>,
}

impl<'a, C> CreateBuilder<'a, C, No, No>
where
    C: Client,
{
    #[inline]
    pub(crate) fn new(client: &'a C) -> CreateBuilder<'a, C, No, No> {
        CreateBuilder {
            client,
            p_container_name: PhantomData {},
            container_name: None,
            p_public_access: PhantomData {},
            public_access: PublicAccess::None,
            metadata: None,
            client_request_id: None,
            timeout: None,
        }
    }
}

impl<'a, C, ContainerNameSet, PublicAccessSet> ClientRequired<'a, C>
    for CreateBuilder<'a, C, ContainerNameSet, PublicAccessSet>
where
    ContainerNameSet: ToAssign,
    PublicAccessSet: ToAssign,
    C: Client,
{
    #[inline]
    fn client(&self) -> &'a C {
        self.client
    }
}

//get mandatory no traits methods

//set mandatory no traits methods
impl<'a, C, PublicAccessSet> ContainerNameRequired<'a>
    for CreateBuilder<'a, C, Yes, PublicAccessSet>
where
    PublicAccessSet: ToAssign,
    C: Client,
{
    #[inline]
    fn container_name(&self) -> &'a str {
        self.container_name.unwrap()
    }
}

impl<'a, C, ContainerNameSet> PublicAccessRequired for CreateBuilder<'a, C, ContainerNameSet, Yes>
where
    ContainerNameSet: ToAssign,
    C: Client,
{
    #[inline]
    fn public_access(&self) -> PublicAccess {
        self.public_access
    }
}

impl<'a, C, ContainerNameSet, PublicAccessSet> MetadataOption<'a>
    for CreateBuilder<'a, C, ContainerNameSet, PublicAccessSet>
where
    ContainerNameSet: ToAssign,
    PublicAccessSet: ToAssign,
    C: Client,
{
    #[inline]
    fn metadata(&self) -> Option<&'a HashMap<&'a str, &'a str>> {
        self.metadata
    }
}

impl<'a, C, ContainerNameSet, PublicAccessSet> ClientRequestIdOption<'a>
    for CreateBuilder<'a, C, ContainerNameSet, PublicAccessSet>
where
    ContainerNameSet: ToAssign,
    PublicAccessSet: ToAssign,
    C: Client,
{
    #[inline]
    fn client_request_id(&self) -> Option<&'a str> {
        self.client_request_id
    }
}

impl<'a, C, ContainerNameSet, PublicAccessSet> TimeoutOption
    for CreateBuilder<'a, C, ContainerNameSet, PublicAccessSet>
where
    ContainerNameSet: ToAssign,
    PublicAccessSet: ToAssign,
    C: Client,
{
    #[inline]
    fn timeout(&self) -> Option<u64> {
        self.timeout
    }
}

impl<'a, C, PublicAccessSet> ContainerNameSupport<'a> for CreateBuilder<'a, C, No, PublicAccessSet>
where
    PublicAccessSet: ToAssign,
    C: Client,
{
    type O = CreateBuilder<'a, C, Yes, PublicAccessSet>;

    #[inline]
    fn with_container_name(self, container_name: &'a str) -> Self::O {
        CreateBuilder {
            client: self.client,
            p_container_name: PhantomData {},
            p_public_access: PhantomData {},
            container_name: Some(container_name),
            public_access: self.public_access,
            metadata: self.metadata,
            client_request_id: self.client_request_id,
            timeout: self.timeout,
        }
    }
}

impl<'a, C, ContainerNameSet> PublicAccessSupport for CreateBuilder<'a, C, ContainerNameSet, No>
where
    ContainerNameSet: ToAssign,
    C: Client,
{
    type O = CreateBuilder<'a, C, ContainerNameSet, Yes>;

    #[inline]
    fn with_public_access(self, public_access: PublicAccess) -> Self::O {
        CreateBuilder {
            client: self.client,
            p_container_name: PhantomData {},
            p_public_access: PhantomData {},
            container_name: self.container_name,
            public_access,
            metadata: self.metadata,
            client_request_id: self.client_request_id,
            timeout: self.timeout,
        }
    }
}

impl<'a, C, ContainerNameSet, PublicAccessSet> MetadataSupport<'a>
    for CreateBuilder<'a, C, ContainerNameSet, PublicAccessSet>
where
    ContainerNameSet: ToAssign,
    PublicAccessSet: ToAssign,
    C: Client,
{
    type O = CreateBuilder<'a, C, ContainerNameSet, PublicAccessSet>;

    #[inline]
    fn with_metadata(self, metadata: &'a HashMap<&'a str, &'a str>) -> Self::O {
        CreateBuilder {
            client: self.client,
            p_container_name: PhantomData {},
            p_public_access: PhantomData {},
            container_name: self.container_name,
            public_access: self.public_access,
            metadata: Some(metadata),
            client_request_id: self.client_request_id,
            timeout: self.timeout,
        }
    }
}

impl<'a, C, ContainerNameSet, PublicAccessSet> ClientRequestIdSupport<'a>
    for CreateBuilder<'a, C, ContainerNameSet, PublicAccessSet>
where
    ContainerNameSet: ToAssign,
    PublicAccessSet: ToAssign,
    C: Client,
{
    type O = CreateBuilder<'a, C, ContainerNameSet, PublicAccessSet>;

    #[inline]
    fn with_client_request_id(self, client_request_id: &'a str) -> Self::O {
        CreateBuilder {
            client: self.client,
            p_container_name: PhantomData {},
            p_public_access: PhantomData {},
            container_name: self.container_name,
            public_access: self.public_access,
            metadata: self.metadata,
            client_request_id: Some(client_request_id),
            timeout: self.timeout,
        }
    }
}

impl<'a, C, ContainerNameSet, PublicAccessSet> TimeoutSupport
    for CreateBuilder<'a, C, ContainerNameSet, PublicAccessSet>
where
    ContainerNameSet: ToAssign,
    PublicAccessSet: ToAssign,
    C: Client,
{
    type O = CreateBuilder<'a, C, ContainerNameSet, PublicAccessSet>;

    #[inline]
    fn with_timeout(self, timeout: u64) -> Self::O {
        CreateBuilder {
            client: self.client,
            p_container_name: PhantomData {},
            p_public_access: PhantomData {},
            container_name: self.container_name,
            public_access: self.public_access,
            metadata: self.metadata,
            client_request_id: self.client_request_id,
            timeout: Some(timeout),
        }
    }
}

// methods callable regardless
impl<'a, C, ContainerNameSet, PublicAccessSet>
    CreateBuilder<'a, C, ContainerNameSet, PublicAccessSet>
where
    ContainerNameSet: ToAssign,
    PublicAccessSet: ToAssign,
    C: Client,
{
}

// methods callable only when every mandatory field has been filled
impl<'a, C> CreateBuilder<'a, C, Yes, Yes>
where
    C: Client,
{
    pub async fn finalize(self) -> Result<(), AzureError> {
        let mut uri = format!(
            "{}/{}?restype=container",
            self.client().blob_uri(),
            self.container_name()
        );

        if let Some(nm) = TimeoutOption::to_uri_parameter(&self) {
            uri = format!("{}&{}", uri, nm);
        }

        let future_response = self.client().perform_request(
            &uri,
            &Method::PUT,
            &|mut request| {
                request = PublicAccessRequired::add_header(&self, request);
                request = ClientRequestIdOption::add_header(&self, request);
                request = MetadataOption::add_header(&self, request);
                request
            },
            Some(&[]),
        )?;

        check_status_extract_body(future_response, StatusCode::CREATED).await?;
        Ok(())
    }
}

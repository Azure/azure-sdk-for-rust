use crate::container::{PublicAccess, PublicAccessRequired, PublicAccessSupport};
use azure_sdk_core::errors::{check_status_extract_body, AzureError};
use azure_sdk_core::{
    ClientRequestIdOption, ClientRequestIdSupport, ContainerNameRequired, ContainerNameSupport,
    MetadataOption, MetadataSupport, TimeoutOption, TimeoutSupport,
};
use azure_sdk_core::{No, ToAssign, Yes};
use azure_sdk_storage_core::client::Client;
use azure_sdk_storage_core::ClientRequired;
use hyper::{Method, StatusCode};
use std::collections::HashMap;
use std::marker::PhantomData;

#[derive(Debug, Clone)]
pub struct CreateBuilder<'a, ContainerNameSet, PublicAccessSet>
where
    ContainerNameSet: ToAssign,
    PublicAccessSet: ToAssign,
{
    p_container_name: PhantomData<ContainerNameSet>,
    p_public_access: PhantomData<PublicAccessSet>,
    client: &'a Client,
    container_name: Option<&'a str>,
    public_access: PublicAccess,
    timeout: Option<u64>,
    client_request_id: Option<&'a str>,
    metadata: Option<&'a HashMap<&'a str, &'a str>>,
}

impl<'a, ContainerNameSet, PublicAccessSet> ClientRequired<'a>
    for CreateBuilder<'a, ContainerNameSet, PublicAccessSet>
where
    ContainerNameSet: ToAssign,
    PublicAccessSet: ToAssign,
{
    fn client(&self) -> &'a Client {
        self.client
    }
}

impl<'a, ContainerNameSet, PublicAccessSet> MetadataOption<'a>
    for CreateBuilder<'a, ContainerNameSet, PublicAccessSet>
where
    ContainerNameSet: ToAssign,
    PublicAccessSet: ToAssign,
{
    fn metadata(&self) -> Option<&'a HashMap<&'a str, &'a str>> {
        self.metadata
    }
}

impl<'a, ContainerNameSet, PublicAccessSet> MetadataSupport<'a>
    for CreateBuilder<'a, ContainerNameSet, PublicAccessSet>
where
    ContainerNameSet: ToAssign,
    PublicAccessSet: ToAssign,
{
    type O = CreateBuilder<'a, ContainerNameSet, PublicAccessSet>;

    fn with_metadata(self, metadata: &'a HashMap<&'a str, &'a str>) -> Self::O {
        CreateBuilder {
            p_container_name: PhantomData {},
            p_public_access: PhantomData {},
            client: self.client,
            container_name: self.container_name,
            public_access: self.public_access,
            timeout: self.timeout,
            client_request_id: self.client_request_id,
            metadata: Some(metadata),
        }
    }
}

impl<'a, ContainerNameSet, PublicAccessSet> PublicAccessSupport
    for CreateBuilder<'a, ContainerNameSet, PublicAccessSet>
where
    ContainerNameSet: ToAssign,
    PublicAccessSet: ToAssign,
{
    type O = CreateBuilder<'a, ContainerNameSet, Yes>;

    fn with_public_access(
        self,
        public_access: PublicAccess,
    ) -> CreateBuilder<'a, ContainerNameSet, Yes> {
        CreateBuilder {
            p_container_name: PhantomData {},
            p_public_access: PhantomData {},
            client: self.client,
            container_name: self.container_name,
            public_access,
            timeout: self.timeout,
            client_request_id: self.client_request_id,
            metadata: self.metadata,
        }
    }
}

impl<'a, ContainerNameSet> PublicAccessRequired for CreateBuilder<'a, ContainerNameSet, Yes>
where
    ContainerNameSet: ToAssign,
{
    fn public_access(&self) -> PublicAccess {
        self.public_access
    }
}

impl<'a, ContainerNameSet, PublicAccessSet> ClientRequestIdOption<'a>
    for CreateBuilder<'a, ContainerNameSet, PublicAccessSet>
where
    ContainerNameSet: ToAssign,
    PublicAccessSet: ToAssign,
{
    fn client_request_id(&self) -> Option<&'a str> {
        self.client_request_id
    }
}

impl<'a, ContainerNameSet, PublicAccessSet> ClientRequestIdSupport<'a>
    for CreateBuilder<'a, ContainerNameSet, PublicAccessSet>
where
    ContainerNameSet: ToAssign,
    PublicAccessSet: ToAssign,
{
    type O = CreateBuilder<'a, ContainerNameSet, PublicAccessSet>;

    fn with_client_request_id(self, client_request_id: &'a str) -> Self::O {
        CreateBuilder {
            p_container_name: PhantomData {},
            p_public_access: PhantomData {},
            client: self.client,
            container_name: self.container_name,
            public_access: self.public_access,
            timeout: self.timeout,
            client_request_id: Some(client_request_id),
            metadata: self.metadata,
        }
    }
}

impl<'a, ContainerNameSet, PublicAccessSet> TimeoutSupport
    for CreateBuilder<'a, ContainerNameSet, PublicAccessSet>
where
    ContainerNameSet: ToAssign,
    PublicAccessSet: ToAssign,
{
    type O = CreateBuilder<'a, ContainerNameSet, PublicAccessSet>;

    fn with_timeout(self, timeout: u64) -> Self::O {
        CreateBuilder {
            p_container_name: PhantomData {},
            p_public_access: PhantomData {},
            client: self.client,
            container_name: self.container_name,
            public_access: self.public_access,
            timeout: Some(timeout),
            client_request_id: self.client_request_id,
            metadata: self.metadata,
        }
    }
}

impl<'a, ContainerNameSet, PublicAccessSet> TimeoutOption
    for CreateBuilder<'a, ContainerNameSet, PublicAccessSet>
where
    ContainerNameSet: ToAssign,
    PublicAccessSet: ToAssign,
{
    fn timeout(&self) -> Option<u64> {
        self.timeout
    }
}

// methods callable regardless
impl<'a, ContainerNameSet, PublicAccessSet> CreateBuilder<'a, ContainerNameSet, PublicAccessSet>
where
    ContainerNameSet: ToAssign,
    PublicAccessSet: ToAssign,
{
}

impl<'a, ContainerNameSet, PublicAccessSet> ContainerNameSupport<'a>
    for CreateBuilder<'a, ContainerNameSet, PublicAccessSet>
where
    ContainerNameSet: ToAssign,
    PublicAccessSet: ToAssign,
{
    type O = CreateBuilder<'a, Yes, PublicAccessSet>;

    fn with_container_name(self, t: &'a str) -> Self::O {
        CreateBuilder {
            p_container_name: PhantomData {},
            p_public_access: PhantomData {},
            client: self.client,
            container_name: Some(t),
            public_access: self.public_access,
            timeout: self.timeout,
            client_request_id: self.client_request_id,
            metadata: self.metadata,
        }
    }
}

impl<'a, PublicAccessSet> ContainerNameRequired<'a> for CreateBuilder<'a, Yes, PublicAccessSet>
where
    PublicAccessSet: ToAssign,
{
    fn container_name(&self) -> &'a str {
        self.container_name.unwrap()
    }
}

impl<'a> CreateBuilder<'a, No, No> {
    pub fn new(client: &'a Client) -> CreateBuilder<'a, No, No> {
        CreateBuilder {
            p_container_name: PhantomData {},
            p_public_access: PhantomData {},
            client,
            container_name: None,
            public_access: PublicAccess::None,
            timeout: None,
            client_request_id: None,
            metadata: None,
        }
    }
}

impl<'a> CreateBuilder<'a, Yes, Yes> {
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
            |mut request| {
                request = ClientRequestIdOption::add_header(&self, request);
                request = PublicAccessRequired::add_header(&self, request);
                request = MetadataOption::add_header(&self, request);
                request
            },
            Some(&[]),
        )?;

        check_status_extract_body(future_response, StatusCode::CREATED).await?;
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn alloc() {
        let client = Client::new("a", "b").unwrap();
        let create = CreateBuilder::new(&client)
            .with_container_name("ciccio")
            .with_public_access(PublicAccess::None);
        println!("container_name == {}", create.container_name());
        create.public_access();
    }
}

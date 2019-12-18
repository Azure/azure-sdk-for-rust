use crate::container::{
    public_access_from_header, PublicAccess, PublicAccessRequired, PublicAccessSupport,
};
use azure_sdk_core::errors::{check_status_extract_headers_and_body, AzureError};
use azure_sdk_core::lease::LeaseId;
use azure_sdk_core::{
    ClientRequestIdOption, ClientRequestIdSupport, ContainerNameRequired, ContainerNameSupport,
    LeaseIdOption, LeaseIdSupport, TimeoutOption, TimeoutSupport,
};
use azure_sdk_core::{No, StoredAccessPolicyList, ToAssign, Yes};
use azure_sdk_storage_core::client::Client;
use azure_sdk_storage_core::ClientRequired;
use hyper::{Method, StatusCode};
use std::marker::PhantomData;

#[derive(Debug, Clone)]
pub struct SetACLBuilder<'a, ContainerNameSet, PublicAccessSet>
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
    lease_id: Option<&'a LeaseId>,
    stored_access_policy_list: Option<&'a StoredAccessPolicyList>,
}

impl<'a, ContainerNameSet, PublicAccessSet> ClientRequired<'a>
    for SetACLBuilder<'a, ContainerNameSet, PublicAccessSet>
where
    ContainerNameSet: ToAssign,
    PublicAccessSet: ToAssign,
{
    fn client(&self) -> &'a Client {
        self.client
    }
}

impl<'a> SetACLBuilder<'a, No, No> {
    pub(crate) fn new(client: &'a Client) -> SetACLBuilder<'a, No, No> {
        SetACLBuilder {
            p_container_name: PhantomData {},
            p_public_access: PhantomData {},
            client,
            container_name: None,
            public_access: PublicAccess::None,
            timeout: None,
            client_request_id: None,
            lease_id: None,
            stored_access_policy_list: None,
        }
    }
}

impl<'a, PublicAccessSet> ContainerNameRequired<'a> for SetACLBuilder<'a, Yes, PublicAccessSet>
where
    PublicAccessSet: ToAssign,
{
    fn container_name(&self) -> &'a str {
        self.container_name.unwrap()
    }
}

impl<'a, ContainerNameSet, PublicAccessSet> ContainerNameSupport<'a>
    for SetACLBuilder<'a, ContainerNameSet, PublicAccessSet>
where
    ContainerNameSet: ToAssign,
    PublicAccessSet: ToAssign,
{
    type O = SetACLBuilder<'a, Yes, PublicAccessSet>;

    fn with_container_name(self, container_name: &'a str) -> Self::O {
        SetACLBuilder {
            p_container_name: PhantomData {},
            p_public_access: PhantomData {},
            client: self.client,
            container_name: Some(container_name),
            public_access: self.public_access,
            timeout: self.timeout,
            client_request_id: self.client_request_id,
            lease_id: self.lease_id,
            stored_access_policy_list: self.stored_access_policy_list,
        }
    }
}

// method callable regardless
impl<'a, ContainerNameSet, PublicAccessSet> SetACLBuilder<'a, ContainerNameSet, PublicAccessSet>
where
    ContainerNameSet: ToAssign,
    PublicAccessSet: ToAssign,
{
    pub fn stored_access_policy_list(&self) -> Option<&'a StoredAccessPolicyList> {
        self.stored_access_policy_list
    }

    pub fn with_stored_access_policy_list(
        self,
        sapl: &'a StoredAccessPolicyList,
    ) -> SetACLBuilder<'a, ContainerNameSet, PublicAccessSet> {
        SetACLBuilder {
            p_container_name: PhantomData {},
            p_public_access: PhantomData {},
            client: self.client,
            container_name: self.container_name,
            public_access: self.public_access,
            timeout: self.timeout,
            client_request_id: self.client_request_id,
            lease_id: self.lease_id,
            stored_access_policy_list: Some(sapl),
        }
    }
}

impl<'a> SetACLBuilder<'a, Yes, Yes> {
    pub async fn finalize(self) -> Result<PublicAccess, AzureError> {
        let mut uri = format!(
            "{}/{}?restype=container&comp=acl",
            self.client().blob_uri(),
            self.container_name()
        );

        if let Some(nm) = TimeoutOption::to_uri_parameter(&self) {
            uri = format!("{}&{}", uri, nm);
        }

        let xml = if let Some(sapl) = self.stored_access_policy_list {
            let xml = sapl.to_xml();
            Some(xml)
        } else {
            None
        };

        let future_response = self.client().perform_request(
            &uri,
            &Method::PUT,
            |mut request| {
                request = ClientRequestIdOption::add_header(&self, request);
                request = LeaseIdOption::add_header(&self, request);
                request = PublicAccessRequired::add_header(&self, request);
                request
            },
            match xml {
                Some(ref x) => Some(x.as_bytes()),
                None => Some(&[]),
            },
        )?;

        let (headers, _body) =
            check_status_extract_headers_and_body(future_response, StatusCode::OK).await?;

        public_access_from_header(&headers)
    }
}

impl<'a, ContainerNameSet, PublicAccessSet> TimeoutOption
    for SetACLBuilder<'a, ContainerNameSet, PublicAccessSet>
where
    ContainerNameSet: ToAssign,
    PublicAccessSet: ToAssign,
{
    fn timeout(&self) -> Option<u64> {
        self.timeout
    }
}

impl<'a, ContainerNameSet, PublicAccessSet> TimeoutSupport
    for SetACLBuilder<'a, ContainerNameSet, PublicAccessSet>
where
    ContainerNameSet: ToAssign,
    PublicAccessSet: ToAssign,
{
    type O = SetACLBuilder<'a, ContainerNameSet, PublicAccessSet>;

    fn with_timeout(self, timeout: u64) -> Self::O {
        SetACLBuilder {
            p_container_name: PhantomData {},
            p_public_access: PhantomData {},
            client: self.client,
            container_name: self.container_name,
            public_access: self.public_access,
            timeout: Some(timeout),
            client_request_id: self.client_request_id,
            lease_id: self.lease_id,
            stored_access_policy_list: self.stored_access_policy_list,
        }
    }
}

impl<'a, ContainerNameSet, PublicAccessSet> ClientRequestIdOption<'a>
    for SetACLBuilder<'a, ContainerNameSet, PublicAccessSet>
where
    ContainerNameSet: ToAssign,
    PublicAccessSet: ToAssign,
{
    fn client_request_id(&self) -> Option<&'a str> {
        self.client_request_id
    }
}

impl<'a, ContainerNameSet, PublicAccessSet> ClientRequestIdSupport<'a>
    for SetACLBuilder<'a, ContainerNameSet, PublicAccessSet>
where
    ContainerNameSet: ToAssign,
    PublicAccessSet: ToAssign,
{
    type O = SetACLBuilder<'a, ContainerNameSet, PublicAccessSet>;

    fn with_client_request_id(self, client_request_id: &'a str) -> Self::O {
        SetACLBuilder {
            p_container_name: PhantomData {},
            p_public_access: PhantomData {},
            client: self.client,
            container_name: self.container_name,
            public_access: self.public_access,
            timeout: self.timeout,
            client_request_id: Some(client_request_id),
            lease_id: self.lease_id,
            stored_access_policy_list: self.stored_access_policy_list,
        }
    }
}

impl<'a, ContainerNameSet, PublicAccessSet> LeaseIdOption<'a>
    for SetACLBuilder<'a, ContainerNameSet, PublicAccessSet>
where
    ContainerNameSet: ToAssign,
    PublicAccessSet: ToAssign,
{
    fn lease_id(&self) -> Option<&'a LeaseId> {
        self.lease_id
    }
}

impl<'a, ContainerNameSet, PublicAccessSet> LeaseIdSupport<'a>
    for SetACLBuilder<'a, ContainerNameSet, PublicAccessSet>
where
    ContainerNameSet: ToAssign,
    PublicAccessSet: ToAssign,
{
    type O = SetACLBuilder<'a, ContainerNameSet, PublicAccessSet>;

    fn with_lease_id(self, lease_id: &'a LeaseId) -> Self::O {
        SetACLBuilder {
            p_container_name: PhantomData {},
            p_public_access: PhantomData {},
            client: self.client,
            container_name: self.container_name,
            public_access: self.public_access,
            timeout: self.timeout,
            client_request_id: self.client_request_id,
            lease_id: Some(lease_id),
            stored_access_policy_list: self.stored_access_policy_list,
        }
    }
}

impl<'a, ContainerNameSet, PublicAccessSet> PublicAccessSupport
    for SetACLBuilder<'a, ContainerNameSet, PublicAccessSet>
where
    ContainerNameSet: ToAssign,
    PublicAccessSet: ToAssign,
{
    type O = SetACLBuilder<'a, ContainerNameSet, Yes>;

    fn with_public_access(self, public_access: PublicAccess) -> Self::O {
        SetACLBuilder {
            p_container_name: PhantomData {},
            p_public_access: PhantomData {},
            client: self.client,
            container_name: self.container_name,
            public_access,
            timeout: self.timeout,
            client_request_id: self.client_request_id,
            lease_id: self.lease_id,
            stored_access_policy_list: self.stored_access_policy_list,
        }
    }
}

impl<'a, ContainerNameSet> PublicAccessRequired for SetACLBuilder<'a, ContainerNameSet, Yes>
where
    ContainerNameSet: ToAssign,
{
    fn public_access(&self) -> PublicAccess {
        self.public_access
    }
}

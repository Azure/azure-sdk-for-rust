use crate::container::public_access_from_header;
use crate::prelude::*;
use azure_sdk_core::errors::{check_status_extract_headers_and_body, AzureError};
use azure_sdk_core::lease::LeaseId;
use azure_sdk_core::prelude::*;
use azure_sdk_core::{No, StoredAccessPolicyList, ToAssign, Yes};
use azure_sdk_storage_core::prelude::*;
use hyper::{Method, StatusCode};
use std::marker::PhantomData;

#[derive(Debug, Clone)]
pub struct SetACLBuilder<'a, C, ContainerNameSet, PublicAccessSet>
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
    stored_access_policy_list: Option<&'a StoredAccessPolicyList>,
    client_request_id: Option<&'a str>,
    timeout: Option<u64>,
    lease_id: Option<&'a LeaseId>,
}

impl<'a, C> SetACLBuilder<'a, C, No, No>
where
    C: Client,
{
    #[inline]
    pub(crate) fn new(client: &'a C) -> SetACLBuilder<'a, C, No, No> {
        SetACLBuilder {
            client,
            p_container_name: PhantomData {},
            container_name: None,
            p_public_access: PhantomData {},
            public_access: PublicAccess::None,
            stored_access_policy_list: None,
            client_request_id: None,
            timeout: None,
            lease_id: None,
        }
    }
}

impl<'a, C, ContainerNameSet, PublicAccessSet> ClientRequired<'a, C>
    for SetACLBuilder<'a, C, ContainerNameSet, PublicAccessSet>
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
    for SetACLBuilder<'a, C, Yes, PublicAccessSet>
where
    PublicAccessSet: ToAssign,
    C: Client,
{
    #[inline]
    fn container_name(&self) -> &'a str {
        self.container_name.unwrap()
    }
}

impl<'a, C, ContainerNameSet> PublicAccessRequired for SetACLBuilder<'a, C, ContainerNameSet, Yes>
where
    ContainerNameSet: ToAssign,
    C: Client,
{
    #[inline]
    fn public_access(&self) -> PublicAccess {
        self.public_access
    }
}

impl<'a, C, ContainerNameSet, PublicAccessSet> StoredAccessPolicyListOption<'a>
    for SetACLBuilder<'a, C, ContainerNameSet, PublicAccessSet>
where
    ContainerNameSet: ToAssign,
    PublicAccessSet: ToAssign,
    C: Client,
{
    #[inline]
    fn stored_access_policy_list(&self) -> Option<&'a StoredAccessPolicyList> {
        self.stored_access_policy_list
    }
}

impl<'a, C, ContainerNameSet, PublicAccessSet> ClientRequestIdOption<'a>
    for SetACLBuilder<'a, C, ContainerNameSet, PublicAccessSet>
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
    for SetACLBuilder<'a, C, ContainerNameSet, PublicAccessSet>
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

impl<'a, C, ContainerNameSet, PublicAccessSet> LeaseIdOption<'a>
    for SetACLBuilder<'a, C, ContainerNameSet, PublicAccessSet>
where
    ContainerNameSet: ToAssign,
    PublicAccessSet: ToAssign,
    C: Client,
{
    #[inline]
    fn lease_id(&self) -> Option<&'a LeaseId> {
        self.lease_id
    }
}

impl<'a, C, PublicAccessSet> ContainerNameSupport<'a> for SetACLBuilder<'a, C, No, PublicAccessSet>
where
    PublicAccessSet: ToAssign,
    C: Client,
{
    type O = SetACLBuilder<'a, C, Yes, PublicAccessSet>;

    #[inline]
    fn with_container_name(self, container_name: &'a str) -> Self::O {
        SetACLBuilder {
            client: self.client,
            p_container_name: PhantomData {},
            p_public_access: PhantomData {},
            container_name: Some(container_name),
            public_access: self.public_access,
            stored_access_policy_list: self.stored_access_policy_list,
            client_request_id: self.client_request_id,
            timeout: self.timeout,
            lease_id: self.lease_id,
        }
    }
}

impl<'a, C, ContainerNameSet> PublicAccessSupport for SetACLBuilder<'a, C, ContainerNameSet, No>
where
    ContainerNameSet: ToAssign,
    C: Client,
{
    type O = SetACLBuilder<'a, C, ContainerNameSet, Yes>;

    #[inline]
    fn with_public_access(self, public_access: PublicAccess) -> Self::O {
        SetACLBuilder {
            client: self.client,
            p_container_name: PhantomData {},
            p_public_access: PhantomData {},
            container_name: self.container_name,
            public_access,
            stored_access_policy_list: self.stored_access_policy_list,
            client_request_id: self.client_request_id,
            timeout: self.timeout,
            lease_id: self.lease_id,
        }
    }
}

impl<'a, C, ContainerNameSet, PublicAccessSet> StoredAccessPolicyListSupport<'a>
    for SetACLBuilder<'a, C, ContainerNameSet, PublicAccessSet>
where
    ContainerNameSet: ToAssign,
    PublicAccessSet: ToAssign,
    C: Client,
{
    type O = SetACLBuilder<'a, C, ContainerNameSet, PublicAccessSet>;

    #[inline]
    fn with_stored_access_policy_list(
        self,
        stored_access_policy_list: &'a StoredAccessPolicyList,
    ) -> Self::O {
        SetACLBuilder {
            client: self.client,
            p_container_name: PhantomData {},
            p_public_access: PhantomData {},
            container_name: self.container_name,
            public_access: self.public_access,
            stored_access_policy_list: Some(stored_access_policy_list),
            client_request_id: self.client_request_id,
            timeout: self.timeout,
            lease_id: self.lease_id,
        }
    }
}

impl<'a, C, ContainerNameSet, PublicAccessSet> ClientRequestIdSupport<'a>
    for SetACLBuilder<'a, C, ContainerNameSet, PublicAccessSet>
where
    ContainerNameSet: ToAssign,
    PublicAccessSet: ToAssign,
    C: Client,
{
    type O = SetACLBuilder<'a, C, ContainerNameSet, PublicAccessSet>;

    #[inline]
    fn with_client_request_id(self, client_request_id: &'a str) -> Self::O {
        SetACLBuilder {
            client: self.client,
            p_container_name: PhantomData {},
            p_public_access: PhantomData {},
            container_name: self.container_name,
            public_access: self.public_access,
            stored_access_policy_list: self.stored_access_policy_list,
            client_request_id: Some(client_request_id),
            timeout: self.timeout,
            lease_id: self.lease_id,
        }
    }
}

impl<'a, C, ContainerNameSet, PublicAccessSet> TimeoutSupport
    for SetACLBuilder<'a, C, ContainerNameSet, PublicAccessSet>
where
    ContainerNameSet: ToAssign,
    PublicAccessSet: ToAssign,
    C: Client,
{
    type O = SetACLBuilder<'a, C, ContainerNameSet, PublicAccessSet>;

    #[inline]
    fn with_timeout(self, timeout: u64) -> Self::O {
        SetACLBuilder {
            client: self.client,
            p_container_name: PhantomData {},
            p_public_access: PhantomData {},
            container_name: self.container_name,
            public_access: self.public_access,
            stored_access_policy_list: self.stored_access_policy_list,
            client_request_id: self.client_request_id,
            timeout: Some(timeout),
            lease_id: self.lease_id,
        }
    }
}

impl<'a, C, ContainerNameSet, PublicAccessSet> LeaseIdSupport<'a>
    for SetACLBuilder<'a, C, ContainerNameSet, PublicAccessSet>
where
    ContainerNameSet: ToAssign,
    PublicAccessSet: ToAssign,
    C: Client,
{
    type O = SetACLBuilder<'a, C, ContainerNameSet, PublicAccessSet>;

    #[inline]
    fn with_lease_id(self, lease_id: &'a LeaseId) -> Self::O {
        SetACLBuilder {
            client: self.client,
            p_container_name: PhantomData {},
            p_public_access: PhantomData {},
            container_name: self.container_name,
            public_access: self.public_access,
            stored_access_policy_list: self.stored_access_policy_list,
            client_request_id: self.client_request_id,
            timeout: self.timeout,
            lease_id: Some(lease_id),
        }
    }
}

// methods callable only when every mandatory field has been filled
impl<'a, C> SetACLBuilder<'a, C, Yes, Yes>
where
    C: Client,
{
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
            &|mut request| {
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

use crate::{container::public_access_from_header, prelude::*};
use azure_core::{
    error::Result,
    headers::{add_optional_header, add_optional_header_ref, AsHeaders},
    prelude::*,
};
use azure_storage::core::StoredAccessPolicyList;
use bytes::Bytes;
use http::{method::Method, status::StatusCode};

#[derive(Debug, Clone)]
pub struct SetACLBuilder<'a> {
    container_client: &'a ContainerClient,
    public_access: PublicAccess,
    stored_access_policy_list: Option<&'a StoredAccessPolicyList>,
    client_request_id: Option<ClientRequestId>,
    timeout: Option<Timeout>,
    lease_id: Option<&'a LeaseId>,
}

impl<'a> SetACLBuilder<'a> {
    pub(crate) fn new(container_client: &'a ContainerClient, public_access: PublicAccess) -> Self {
        Self {
            container_client,
            public_access,
            stored_access_policy_list: None,
            client_request_id: None,
            timeout: None,
            lease_id: None,
        }
    }

    setters! {
        lease_id: &'a LeaseId => Some(lease_id),
        client_request_id: ClientRequestId => Some(client_request_id),
        timeout: Timeout => Some(timeout),
        stored_access_policy_list: &'a StoredAccessPolicyList => Some(stored_access_policy_list),
    }

    pub async fn execute(&self) -> Result<PublicAccess> {
        let mut url = self.container_client.url_with_segments(None)?;

        url.query_pairs_mut().append_pair("restype", "container");
        url.query_pairs_mut().append_pair("comp", "acl");

        self.timeout.append_to_url_query(&mut url);

        let xml = self.stored_access_policy_list.map(|xml| xml.to_xml());

        let request = self.container_client.prepare_request(
            url.as_str(),
            &Method::PUT,
            &|mut request| {
                for (name, value) in self.public_access.as_headers() {
                    request = request.header(name.as_str(), value.as_str())
                }
                request = add_optional_header(&self.client_request_id, request);
                request = add_optional_header_ref(&self.lease_id, request);
                request
            },
            xml.map(Bytes::from),
        )?;

        let response = self
            .container_client
            .storage_client()
            .storage_account_client()
            .http_client()
            .execute_request_check_status(request.0, StatusCode::OK)
            .await?;

        public_access_from_header(response.headers())
    }
}

use crate::blob_storage::prelude::*;
use crate::container::responses::GetACLResponse;
use azure_core::headers::{add_optional_header, add_optional_header_ref};
use azure_core::prelude::*;
use http::method::Method;
use http::status::StatusCode;
use std::convert::TryInto;

#[derive(Debug, Clone)]
pub struct GetACLBuilder<'a> {
    container_client: &'a ContainerClient,
    client_request_id: Option<ClientRequestId<'a>>,
    timeout: Option<Timeout>,
    lease_id: Option<&'a LeaseId>,
}

impl<'a> GetACLBuilder<'a> {
    pub(crate) fn new(container_client: &'a ContainerClient) -> Self {
        Self {
            container_client,
            client_request_id: None,
            timeout: None,
            lease_id: None,
        }
    }

    setters! {
        client_request_id: ClientRequestId<'a> => Some(client_request_id),
        timeout: Timeout => Some(timeout),
        lease_id: &'a LeaseId => Some(lease_id),
    }

    pub async fn execute(self) -> Result<GetACLResponse, Box<dyn std::error::Error + Sync + Send>> {
        let mut url = self
            .container_client
            .storage_client()
            .storage_account_client()
            .blob_storage_url()
            .join(self.container_client.container_name())?;

        url.query_pairs_mut().append_pair("restype", "container");
        url.query_pairs_mut().append_pair("comp", "acl");

        self.timeout.append_to_url_query(&mut url);

        let request = self.container_client.prepare_request(
            url.as_str(),
            &Method::GET,
            &|mut request| {
                request = add_optional_header(&self.client_request_id, request);
                request = add_optional_header_ref(&self.lease_id, request);
                request
            },
            None,
        )?;

        let response = self
            .container_client
            .storage_client()
            .storage_account_client()
            .http_client()
            .execute_request_check_status(request.0, StatusCode::OK)
            .await?;

        // todo: parse SAS policies
        Ok((std::str::from_utf8(response.body())?, response.headers()).try_into()?)
    }
}

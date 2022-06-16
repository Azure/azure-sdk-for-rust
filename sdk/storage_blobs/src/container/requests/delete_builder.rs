use crate::prelude::*;
use azure_core::{
    error::Result,
    headers::{add_optional_header, add_optional_header_ref},
    prelude::*,
};
use http::{method::Method, status::StatusCode};

#[derive(Debug, Clone)]
pub struct DeleteBuilder<'a> {
    container_client: &'a ContainerClient,
    lease_id: Option<&'a LeaseId>,
    client_request_id: Option<ClientRequestId>,
    #[allow(unused)]
    timeout: Option<Timeout>,
}

impl<'a> DeleteBuilder<'a> {
    pub(crate) fn new(container_client: &'a ContainerClient) -> Self {
        DeleteBuilder {
            container_client,
            lease_id: None,
            client_request_id: None,
            timeout: None,
        }
    }

    setters! {
        lease_id: &'a LeaseId => Some(lease_id),
        client_request_id: ClientRequestId => Some(client_request_id),
        timeout: Timeout => Some(timeout),
    }

    pub async fn execute(self) -> Result<()> {
        let mut url = self.container_client.url_with_segments(None)?;

        url.query_pairs_mut().append_pair("restype", "container");

        let request = self.container_client.prepare_request(
            url.as_str(),
            &Method::DELETE,
            &|mut request| {
                request = add_optional_header(&self.client_request_id, request);
                request = add_optional_header_ref(&self.lease_id, request);
                request
            },
            None,
        )?;

        let _response = self
            .container_client
            .storage_client()
            .storage_account_client()
            .http_client()
            .execute_request_check_status(request.0, StatusCode::ACCEPTED)
            .await?;

        // TODO: Capture and return the response headers
        Ok(())
    }
}

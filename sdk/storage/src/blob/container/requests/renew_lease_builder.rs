use crate::clients::ContainerClient;
use crate::container::responses::RenewLeaseResponse;
use azure_core::headers::{add_mandatory_header, add_optional_header, LEASE_ACTION};
use azure_core::lease::LeaseId;
use azure_core::prelude::*;
use http::method::Method;
use http::status::StatusCode;

#[derive(Debug, Clone)]
pub struct RenewLeaseBuilder<'a> {
    container_client: &'a ContainerClient,
    lease_id: &'a LeaseId,
    client_request_id: Option<ClientRequestId<'a>>,
    timeout: Option<Timeout>,
}

impl<'a> RenewLeaseBuilder<'a> {
    pub(crate) fn new(container_client: &'a ContainerClient, lease_id: &'a LeaseId) -> Self {
        Self {
            container_client,
            lease_id,
            client_request_id: None,
            timeout: None,
        }
    }

    pub async fn execute(
        &self,
    ) -> Result<RenewLeaseResponse, Box<dyn std::error::Error + Sync + Send>> {
        let mut url = self
            .container_client
            .storage_client()
            .storage_account_client()
            .blob_storage_url()
            .join(self.container_client.container_name())?;

        url.query_pairs_mut().append_pair("restype", "container");
        url.query_pairs_mut().append_pair("comp", "lease");

        self.timeout.append_to_url_query(&mut url);

        let request = self.container_client.prepare_request(
            url.as_str(),
            &Method::PUT,
            &|mut request| {
                request = request.header(LEASE_ACTION, "renew");
                request = add_mandatory_header(self.lease_id, request);
                request = add_optional_header(&self.client_request_id, request);
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

        Ok(RenewLeaseResponse::from_headers(response.headers())?)
    }
}

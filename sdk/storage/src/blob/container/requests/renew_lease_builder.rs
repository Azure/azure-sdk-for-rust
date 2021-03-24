use crate::blob::prelude::*;
use crate::container::responses::RenewLeaseResponse;
use azure_core::headers::{add_mandatory_header, add_optional_header, LEASE_ACTION};
use azure_core::prelude::*;
use http::method::Method;
use http::status::StatusCode;

#[derive(Debug, Clone)]
pub struct RenewLeaseBuilder<'a> {
    container_lease_client: &'a ContainerLeaseClient,
    client_request_id: Option<ClientRequestId<'a>>,
    timeout: Option<Timeout>,
}

impl<'a> RenewLeaseBuilder<'a> {
    pub(crate) fn new(container_lease_client: &'a ContainerLeaseClient) -> Self {
        Self {
            container_lease_client,
            client_request_id: None,
            timeout: None,
        }
    }

    pub async fn execute(
        &self,
    ) -> Result<RenewLeaseResponse, Box<dyn std::error::Error + Sync + Send>> {
        let mut url = self
            .container_lease_client
            .storage_account_client()
            .blob_storage_url()
            .to_owned();
        url.path_segments_mut()
            .map_err(|_| "Invalid blob URL")?
            .push(
                self.container_lease_client
                    .container_client()
                    .container_name(),
            );

        url.query_pairs_mut().append_pair("restype", "container");
        url.query_pairs_mut().append_pair("comp", "lease");

        self.timeout.append_to_url_query(&mut url);

        let request = self.container_lease_client.prepare_request(
            url.as_str(),
            &Method::PUT,
            &|mut request| {
                request = request.header(LEASE_ACTION, "renew");
                request = add_mandatory_header(self.container_lease_client.lease_id(), request);
                request = add_optional_header(&self.client_request_id, request);
                request
            },
            None,
        )?;

        let response = self
            .container_lease_client
            .http_client()
            .execute_request_check_status(request.0, StatusCode::OK)
            .await?;

        Ok(RenewLeaseResponse::from_headers(response.headers())?)
    }
}

use crate::clients::ContainerClient;
use crate::container::responses::BreakLeaseResponse;
use azure_core::headers::{add_optional_header, add_optional_header_ref, LEASE_ACTION};
use azure_core::prelude::*;
use http::method::Method;
use http::status::StatusCode;

#[derive(Debug, Clone)]
pub struct BreakLeaseBuilder<'a> {
    container_client: &'a ContainerClient,
    client_request_id: Option<ClientRequestId<'a>>,
    timeout: Option<Timeout>,
    lease_break_period: Option<LeaseBreakPeriod>,
    lease_id: Option<&'a LeaseId>,
}

impl<'a> BreakLeaseBuilder<'a> {
    pub(crate) fn new(container_client: &'a ContainerClient) -> BreakLeaseBuilder<'a> {
        Self {
            container_client,
            client_request_id: None,
            timeout: None,
            lease_break_period: None,
            lease_id: None,
        }
    }

    setters! {
        lease_id: &'a LeaseId => Some(lease_id),
        lease_break_period: LeaseBreakPeriod => Some(lease_break_period),
        client_request_id: ClientRequestId<'a> => Some(client_request_id),
        timeout: Timeout => Some(timeout),
    }

    pub async fn execute(
        self,
    ) -> Result<BreakLeaseResponse, Box<dyn std::error::Error + Sync + Send>> {
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
                request = request.header(LEASE_ACTION, "break");
                request = add_optional_header(&self.client_request_id, request);
                request = add_optional_header_ref(&self.lease_id, request);
                request = add_optional_header(&self.lease_break_period, request);
                request
            },
            None,
        )?;

        let response = self
            .container_client
            .storage_client()
            .storage_account_client()
            .http_client()
            .execute_request_check_status(request.0, StatusCode::ACCEPTED)
            .await?;

        Ok(BreakLeaseResponse::from_headers(response.headers())?)
    }
}

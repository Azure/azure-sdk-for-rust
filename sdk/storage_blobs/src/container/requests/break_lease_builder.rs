use crate::{container::responses::BreakLeaseResponse, prelude::*};
use azure_core::{headers::LEASE_ACTION, prelude::*};
use http::method::Method;

#[derive(Debug, Clone)]
pub struct BreakLeaseBuilder<'a> {
    container_client: &'a ContainerClient,
    client_request_id: Option<ClientRequestId>,
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
        client_request_id: ClientRequestId => Some(client_request_id),
        timeout: Timeout => Some(timeout),
    }

    pub async fn execute(self) -> azure_core::Result<BreakLeaseResponse> {
        let mut url = self.container_client.url_with_segments(None)?;

        url.query_pairs_mut().append_pair("restype", "container");
        url.query_pairs_mut().append_pair("comp", "lease");

        self.timeout.append_to_url_query(&mut url);

        let mut request = self
            .container_client
            .prepare_request(url.as_str(), Method::PUT, None)?;
        request.insert_header(LEASE_ACTION, "break");
        request.add_optional_header(self.client_request_id.as_ref());
        request.add_optional_header(self.lease_id);
        request.add_optional_header(self.lease_break_period.as_ref());

        let response = self
            .container_client
            .storage_client()
            .storage_account_client()
            .execute_request_check_status(&request)
            .await?;

        BreakLeaseResponse::from_headers(response.headers())
    }
}

use crate::{container::responses::AcquireLeaseResponse, prelude::*};
use azure_core::{headers::LEASE_ACTION, prelude::*};
use http::method::Method;

#[derive(Debug, Clone)]
pub struct AcquireLeaseBuilder<'a> {
    container_client: &'a ContainerClient,
    lease_duration: LeaseDuration,
    client_request_id: Option<ClientRequestId>,
    timeout: Option<Timeout>,
    lease_id: Option<&'a LeaseId>,
    proposed_lease_id: Option<&'a ProposedLeaseId>,
}

impl<'a> AcquireLeaseBuilder<'a> {
    pub(crate) fn new(
        container_client: &'a ContainerClient,
        lease_duration: LeaseDuration,
    ) -> Self {
        AcquireLeaseBuilder {
            container_client,
            lease_duration,
            client_request_id: None,
            timeout: None,
            lease_id: None,
            proposed_lease_id: None,
        }
    }

    setters! {
        lease_id: &'a LeaseId => Some(lease_id),
        proposed_lease_id: &'a ProposedLeaseId => Some(proposed_lease_id),
        client_request_id: ClientRequestId => Some(client_request_id),
        timeout: Timeout => Some(timeout),
    }

    pub async fn execute(&self) -> azure_core::Result<AcquireLeaseResponse> {
        let mut url = self.container_client.url_with_segments(None)?;

        url.query_pairs_mut().append_pair("restype", "container");
        url.query_pairs_mut().append_pair("comp", "lease");

        self.timeout.append_to_url_query(&mut url);

        let mut request = self
            .container_client
            .prepare_request(url.as_str(), Method::PUT, None)?;
        request.insert_header(LEASE_ACTION, "acquire");
        request.add_mandatory_header(&self.lease_duration);
        request.add_optional_header(self.client_request_id);
        request.add_optional_header(self.lease_id);
        request.add_optional_header(self.proposed_lease_id);

        let response = self
            .container_client
            .storage_client()
            .storage_account_client()
            .execute_request_check_status(&request)
            .await?;

        AcquireLeaseResponse::from_headers(response.headers())
    }
}

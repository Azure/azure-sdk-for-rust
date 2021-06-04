use crate::blob::prelude::*;
use crate::container::responses::AcquireLeaseResponse;
use azure_core::headers::{
    add_mandatory_header, add_optional_header, add_optional_header_ref, LEASE_ACTION,
};
use azure_core::prelude::*;
use http::method::Method;
use http::status::StatusCode;

#[derive(Debug, Clone)]
pub struct AcquireLeaseBuilder<'a> {
    container_client: &'a ContainerClient,
    lease_duration: LeaseDuration,
    client_request_id: Option<ClientRequestId<'a>>,
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
        client_request_id: ClientRequestId<'a> => Some(client_request_id),
        timeout: Timeout => Some(timeout),
    }

    pub async fn execute(
        &self,
    ) -> Result<AcquireLeaseResponse, Box<dyn std::error::Error + Sync + Send>> {
        let mut url = self.container_client.url_with_segments(None)?;

        url.query_pairs_mut().append_pair("restype", "container");
        url.query_pairs_mut().append_pair("comp", "lease");

        self.timeout.append_to_url_query(&mut url);

        let request = self.container_client.prepare_request(
            url.as_str(),
            &Method::PUT,
            &|mut request| {
                request = request.header(LEASE_ACTION, "acquire");
                request = add_mandatory_header(&self.lease_duration, request);
                request = add_optional_header(&self.client_request_id, request);
                request = add_optional_header_ref(&self.lease_id, request);
                request = add_optional_header_ref(&self.proposed_lease_id, request);
                request
            },
            None,
        )?;

        let response = self
            .container_client
            .storage_client()
            .storage_account_client()
            .http_client()
            .execute_request_check_status(request.0, StatusCode::CREATED)
            .await?;

        Ok(AcquireLeaseResponse::from_headers(response.headers())?)
    }
}

use crate::{blob::responses::AcquireBlobLeaseResponse, prelude::*};
use azure_core::{headers::LEASE_ACTION, prelude::*};

#[derive(Debug, Clone)]
pub struct AcquireLeaseBuilder<'a> {
    blob_client: &'a BlobClient,
    lease_duration: LeaseDuration,
    lease_id: Option<&'a LeaseId>,
    proposed_lease_id: Option<&'a ProposedLeaseId>,
    client_request_id: Option<ClientRequestId>,
    timeout: Option<Timeout>,
}

impl<'a> AcquireLeaseBuilder<'a> {
    pub(crate) fn new(blob_client: &'a BlobClient, lease_duration: LeaseDuration) -> Self {
        Self {
            blob_client,
            lease_duration,
            lease_id: None,
            proposed_lease_id: None,
            timeout: None,
            client_request_id: None,
        }
    }

    setters! {
        lease_id: &'a LeaseId => Some(lease_id),
        proposed_lease_id: &'a ProposedLeaseId => Some(proposed_lease_id),
        timeout: Timeout => Some(timeout),
        client_request_id: ClientRequestId => Some(client_request_id),
    }

    pub async fn execute(self) -> azure_core::Result<AcquireBlobLeaseResponse> {
        let mut url = self.blob_client.url_with_segments(None)?;

        url.query_pairs_mut().append_pair("comp", "lease");
        self.timeout.append_to_url_query(&mut url);

        let mut request =
            self.blob_client
                .prepare_request(url.as_str(), http::Method::PUT, None)?;
        request.insert_header(LEASE_ACTION, "acquire");
        request.add_mandatory_header(&self.lease_duration);
        request.add_optional_header_ref(&self.proposed_lease_id);
        request.add_optional_header(&self.client_request_id);
        request.add_optional_header_ref(&self.lease_id);

        let response = self
            .blob_client
            .http_client()
            .execute_request_check_status(&request)
            .await?;

        AcquireBlobLeaseResponse::from_headers(response.headers())
    }
}

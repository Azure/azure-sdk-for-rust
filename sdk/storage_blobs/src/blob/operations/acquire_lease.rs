use crate::prelude::*;
use azure_core::{
    headers::{
        date_from_headers, etag_from_headers, last_modified_from_headers, lease_id_from_headers,
        request_id_from_headers, LEASE_ACTION,
    },
    prelude::*,
    RequestId,
};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct AcquireLeaseBuilder {
    blob_client: BlobClient,
    lease_duration: LeaseDuration,
    lease_id: Option<LeaseId>,
    proposed_lease_id: Option<ProposedLeaseId>,
    client_request_id: Option<ClientRequestId>,
    timeout: Option<Timeout>,
}

impl AcquireLeaseBuilder {
    pub(crate) fn new(blob_client: BlobClient, lease_duration: LeaseDuration) -> Self {
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
        lease_id: LeaseId => Some(lease_id),
        proposed_lease_id: ProposedLeaseId => Some(proposed_lease_id),
        timeout: Timeout => Some(timeout),
        client_request_id: ClientRequestId => Some(client_request_id),
    }

    pub fn into_future(self) -> Response {
        Box::pin(async move {
            let mut url = self.blob_client.url_with_segments(None)?;

            url.query_pairs_mut().append_pair("comp", "lease");
            self.timeout.append_to_url_query(&mut url);

            let mut request =
                self.blob_client
                    .prepare_request(url.as_str(), http::Method::PUT, None)?;
            request.insert_header(LEASE_ACTION, "acquire");
            request.add_mandatory_header(&self.lease_duration);
            request.add_optional_header(&self.proposed_lease_id);
            request.add_optional_header(&self.client_request_id);
            request.add_optional_header(&self.lease_id);

            let response = self
                .blob_client
                .http_client()
                .execute_request_check_status(&request)
                .await?;

            AcquireLeaseResponse::from_headers(response.headers())
        })
    }
}

azure_storage::response_from_headers!(AcquireLeaseResponse,
               etag_from_headers => etag: String,
               last_modified_from_headers => last_modified: DateTime<Utc>,
                       lease_id_from_headers => lease_id: LeaseId,
               request_id_from_headers => request_id: RequestId,
               date_from_headers => date: DateTime<Utc>
);

/// The future returned by calling `into_future` on the builder.
pub type Response = futures::future::BoxFuture<'static, azure_core::Result<AcquireLeaseResponse>>;

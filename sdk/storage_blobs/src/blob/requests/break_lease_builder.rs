use crate::{blob::responses::BreakBlobLeaseResponse, prelude::*};
use azure_core::{
    error::Result,
    headers::{add_optional_header, add_optional_header_ref, LEASE_ACTION},
    prelude::*,
};

#[derive(Debug, Clone)]
pub struct BreakLeaseBuilder<'a> {
    blob_client: &'a BlobClient,
    lease_break_period: Option<LeaseBreakPeriod>,
    lease_id: Option<&'a LeaseId>,
    client_request_id: Option<ClientRequestId>,
    timeout: Option<Timeout>,
}

impl<'a> BreakLeaseBuilder<'a> {
    pub(crate) fn new(blob_client: &'a BlobClient) -> Self {
        Self {
            blob_client,
            lease_break_period: None,
            lease_id: None,
            client_request_id: None,
            timeout: None,
        }
    }

    setters! {
        lease_break_period: LeaseBreakPeriod => Some(lease_break_period),
        lease_id: &'a LeaseId => Some(lease_id),
        client_request_id: ClientRequestId => Some(client_request_id),
        timeout: Timeout => Some(timeout),
    }

    pub async fn execute(&self) -> Result<BreakBlobLeaseResponse> {
        let mut url = self.blob_client.url_with_segments(None)?;

        url.query_pairs_mut().append_pair("comp", "lease");
        self.timeout.append_to_url_query(&mut url);

        trace!("url == {:?}", url);

        let (request, _url) = self.blob_client.prepare_request(
            url.as_str(),
            &http::Method::PUT,
            &|mut request| {
                request = request.header(LEASE_ACTION, "break");
                request = add_optional_header(&self.lease_break_period, request);
                request = add_optional_header_ref(&self.lease_id, request);
                request = add_optional_header(&self.client_request_id, request);
                request
            },
            None,
        )?;

        let response = self
            .blob_client
            .http_client()
            .execute_request_check_status(request, http::StatusCode::ACCEPTED)
            .await?;

        BreakBlobLeaseResponse::from_headers(response.headers())
    }
}

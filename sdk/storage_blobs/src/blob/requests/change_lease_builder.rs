use crate::{blob::responses::ChangeBlobLeaseResponse, prelude::*};
use azure_core::{
    error::Result,
    headers::{add_mandatory_header, add_optional_header, LEASE_ACTION},
    prelude::*,
};

#[derive(Debug, Clone)]
pub struct ChangeLeaseBuilder<'a> {
    blob_lease_client: &'a BlobLeaseClient,
    proposed_lease_id: &'a ProposedLeaseId,
    client_request_id: Option<ClientRequestId>,
    timeout: Option<Timeout>,
}

impl<'a> ChangeLeaseBuilder<'a> {
    pub(crate) fn new(
        blob_lease_client: &'a BlobLeaseClient,
        proposed_lease_id: &'a ProposedLeaseId,
    ) -> Self {
        Self {
            blob_lease_client,
            proposed_lease_id,
            client_request_id: None,
            timeout: None,
        }
    }

    setters! {
        client_request_id: ClientRequestId => Some(client_request_id),
        timeout: Timeout => Some(timeout),
    }

    pub async fn execute(&self) -> Result<ChangeBlobLeaseResponse> {
        let mut url = self.blob_lease_client.url_with_segments(None)?;

        url.query_pairs_mut().append_pair("comp", "lease");
        self.timeout.append_to_url_query(&mut url);

        trace!("url == {:?}", url);

        let (request, _url) = self.blob_lease_client.prepare_request(
            url.as_str(),
            &http::Method::PUT,
            &|mut request| {
                request = request.header(LEASE_ACTION, "change");
                request = add_mandatory_header(self.blob_lease_client.lease_id(), request);
                request = add_mandatory_header(self.proposed_lease_id, request);
                request = add_optional_header(&self.client_request_id, request);
                request
            },
            None,
        )?;

        let response = self
            .blob_lease_client
            .http_client()
            .execute_request_check_status(request, http::StatusCode::OK)
            .await?;

        ChangeBlobLeaseResponse::from_headers(response.headers())
    }
}

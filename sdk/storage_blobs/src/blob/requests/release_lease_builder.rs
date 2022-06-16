use crate::{blob::responses::ReleaseBlobLeaseResponse, prelude::*};
use azure_core::{
    error::Result,
    headers::{add_mandatory_header, add_optional_header, LEASE_ACTION},
    prelude::*,
};

#[derive(Debug, Clone)]
pub struct ReleaseLeaseBuilder<'a> {
    blob_lease_client: &'a BlobLeaseClient,
    client_request_id: Option<ClientRequestId>,
    timeout: Option<Timeout>,
}

impl<'a> ReleaseLeaseBuilder<'a> {
    pub(crate) fn new(blob_lease_client: &'a BlobLeaseClient) -> Self {
        Self {
            blob_lease_client,
            client_request_id: None,
            timeout: None,
        }
    }

    setters! {
        client_request_id: ClientRequestId => Some(client_request_id),
        timeout: Timeout => Some(timeout),
    }

    pub async fn execute(&self) -> Result<ReleaseBlobLeaseResponse> {
        let mut url = self.blob_lease_client.url_with_segments(None)?;

        url.query_pairs_mut().append_pair("comp", "lease");
        self.timeout.append_to_url_query(&mut url);

        trace!("url == {:?}", url);

        let (request, _url) = self.blob_lease_client.prepare_request(
            url.as_str(),
            &http::Method::PUT,
            &|mut request| {
                request = request.header(LEASE_ACTION, "release");
                request = add_mandatory_header(self.blob_lease_client.lease_id(), request);
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

        Ok(ReleaseBlobLeaseResponse::from_headers(response.headers())?)
    }
}

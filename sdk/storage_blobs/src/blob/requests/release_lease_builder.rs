use crate::{blob::responses::ReleaseBlobLeaseResponse, prelude::*};
use azure_core::{headers::LEASE_ACTION, prelude::*};

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

    pub async fn execute(&self) -> azure_core::Result<ReleaseBlobLeaseResponse> {
        let mut url = self.blob_lease_client.url_with_segments(None)?;

        url.query_pairs_mut().append_pair("comp", "lease");
        self.timeout.append_to_url_query(&mut url);

        let mut request =
            self.blob_lease_client
                .prepare_request(url.as_str(), http::Method::PUT, None)?;
        request.insert_header(LEASE_ACTION, "release");
        request.add_mandatory_header(self.blob_lease_client.lease_id());
        request.add_optional_header(self.client_request_id.as_ref());

        let response = self
            .blob_lease_client
            .execute_request_check_status(&request)
            .await?;

        ReleaseBlobLeaseResponse::from_headers(response.headers())
    }
}

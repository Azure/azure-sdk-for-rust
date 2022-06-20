use crate::{blob::responses::DeleteBlobResponse, prelude::*};
use azure_core::prelude::*;

#[derive(Debug, Clone)]
pub struct DeleteBlobSnapshotBuilder<'a> {
    blob_client: &'a BlobClient,
    snapshot: Snapshot,
    permanent: bool,
    timeout: Option<Timeout>,
    lease_id: Option<&'a LeaseId>,
    client_request_id: Option<ClientRequestId>,
}

impl<'a> DeleteBlobSnapshotBuilder<'a> {
    pub(crate) fn new(blob_client: &'a BlobClient, snapshot: Snapshot) -> Self {
        Self {
            blob_client,
            snapshot,
            permanent: false,
            timeout: None,
            lease_id: None,
            client_request_id: None,
        }
    }

    setters! {
        permanent: bool => permanent,
        timeout: Timeout => Some(timeout),
        lease_id: &'a LeaseId => Some(lease_id),
        client_request_id: ClientRequestId => Some(client_request_id),
    }

    pub async fn execute(&self) -> azure_core::Result<DeleteBlobResponse> {
        let mut url = self.blob_client.url_with_segments(None)?;

        self.timeout.append_to_url_query(&mut url);
        (&self.snapshot).append_to_url_query(&mut url);
        if self.permanent {
            url.query_pairs_mut().append_pair("deletetype", "permanent");
        }

        let mut request =
            self.blob_client
                .prepare_request(url.as_str(), http::Method::DELETE, None)?;
        request.add_optional_header_ref(&self.lease_id);
        request.add_optional_header(&self.client_request_id);

        let response = self
            .blob_client
            .http_client()
            .execute_request_check_status(&request)
            .await?;

        DeleteBlobResponse::from_headers(response.headers())
    }
}

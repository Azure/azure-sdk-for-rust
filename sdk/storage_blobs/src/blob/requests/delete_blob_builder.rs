use crate::{blob::responses::DeleteBlobResponse, prelude::*};
use azure_core::prelude::*;

#[derive(Debug, Clone)]
pub struct DeleteBlobBuilder<'a> {
    blob_client: &'a BlobClient,
    delete_snapshots_method: DeleteSnapshotsMethod,
    timeout: Option<Timeout>,
    lease_id: Option<&'a LeaseId>,
    client_request_id: Option<ClientRequestId>,
}

impl<'a> DeleteBlobBuilder<'a> {
    pub(crate) fn new(blob_client: &'a BlobClient) -> Self {
        Self {
            blob_client,
            delete_snapshots_method: DeleteSnapshotsMethod::Include,
            timeout: None,
            lease_id: None,
            client_request_id: None,
        }
    }

    setters! {
        delete_snapshots_method: DeleteSnapshotsMethod => delete_snapshots_method,
        timeout: Timeout => Some(timeout),
        lease_id: &'a LeaseId => Some(lease_id),
        client_request_id: ClientRequestId => Some(client_request_id),
    }

    pub async fn execute(&self) -> azure_core::Result<DeleteBlobResponse> {
        let mut url = self.blob_client.url_with_segments(None)?;

        self.timeout.append_to_url_query(&mut url);

        let mut request =
            self.blob_client
                .prepare_request(url.as_str(), http::Method::DELETE, None)?;
        request.add_optional_header_ref(&self.lease_id);
        request.add_optional_header(&self.client_request_id);
        request.add_mandatory_header(&self.delete_snapshots_method);

        let response = self
            .blob_client
            .http_client()
            .execute_request_check_status(&request)
            .await?;

        DeleteBlobResponse::from_headers(response.headers())
    }
}

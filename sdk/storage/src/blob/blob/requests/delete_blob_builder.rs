use crate::blob::blob::responses::DeleteBlobResponse;
use crate::blob::prelude::*;
use azure_core::headers::{add_mandatory_header, add_optional_header, add_optional_header_ref};
use azure_core::prelude::*;

#[derive(Debug, Clone)]
pub struct DeleteBlobBuilder<'a> {
    blob_client: &'a BlobClient,
    delete_snapshots_method: DeleteSnapshotsMethod,
    timeout: Option<Timeout>,
    lease_id: Option<&'a LeaseId>,
    client_request_id: Option<ClientRequestId<'a>>,
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
        client_request_id: ClientRequestId<'a> => Some(client_request_id),
    }

    pub async fn execute(
        &self,
    ) -> Result<DeleteBlobResponse, Box<dyn std::error::Error + Send + Sync>> {
        let mut url = self
            .blob_client
            .storage_account_client()
            .blob_storage_url()
            .to_owned();
        url.path_segments_mut()
            .map_err(|_| "Invalid blob URL")?
            .push(self.blob_client.container_client().container_name())
            .push(self.blob_client.blob_name());

        self.timeout.append_to_url_query(&mut url);

        trace!("delete_blob url == {:?}", url);

        let (request, _url) = self.blob_client.prepare_request(
            url.as_str(),
            &http::Method::DELETE,
            &|mut request| {
                request = add_optional_header_ref(&self.lease_id, request);
                request = add_optional_header(&self.client_request_id, request);
                request = add_mandatory_header(&self.delete_snapshots_method, request);
                request
            },
            None,
        )?;

        let response = self
            .blob_client
            .http_client()
            .execute_request_check_status(request, http::StatusCode::ACCEPTED)
            .await?;

        debug!("response.headers() == {:#?}", response.headers());

        Ok(DeleteBlobResponse::from_headers(response.headers())?)
    }
}

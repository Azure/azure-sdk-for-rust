use crate::{blob::responses::GetBlobPropertiesResponse, prelude::*};
use azure_core::prelude::*;

#[derive(Debug, Clone)]
pub struct GetBlobPropertiesBuilder<'a> {
    blob_client: &'a BlobClient,
    blob_versioning: Option<&'a BlobVersioning>,
    timeout: Option<Timeout>,
    lease_id: Option<&'a LeaseId>,
    client_request_id: Option<ClientRequestId>,
}

impl<'a> GetBlobPropertiesBuilder<'a> {
    pub(crate) fn new(blob_client: &'a BlobClient) -> Self {
        Self {
            blob_client,
            blob_versioning: None,
            timeout: None,
            lease_id: None,
            client_request_id: None,
        }
    }

    setters! {
        blob_versioning: &'a BlobVersioning => Some(blob_versioning),
        timeout: Timeout => Some(timeout),
        lease_id: &'a LeaseId => Some(lease_id),
        client_request_id: ClientRequestId => Some(client_request_id),
    }

    pub async fn execute(&self) -> azure_core::Result<GetBlobPropertiesResponse> {
        let mut url = self.blob_client.url_with_segments(None)?;

        self.timeout.append_to_url_query(&mut url);
        self.blob_versioning.append_to_url_query(&mut url);

        trace!("url == {:?}", url);

        let mut request =
            self.blob_client
                .prepare_request(url.as_str(), http::Method::HEAD, None)?;
        request.add_optional_header_ref(&self.lease_id);
        request.add_optional_header(&self.client_request_id);

        let response = self
            .blob_client
            .execute_request_check_status(&request)
            .await?;

        // TODO: Fix this
        //let blob = Blob::from_headers(&blob_name, &container_name, snapshot_time, &headers)?;
        let blob = Blob::from_headers(self.blob_client.blob_name(), response.headers())?;
        GetBlobPropertiesResponse::from_response(response.headers(), blob)
    }
}

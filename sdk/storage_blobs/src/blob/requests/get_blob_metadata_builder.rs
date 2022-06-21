use crate::{blob::responses::GetBlobMetadataResponse, prelude::*};
use azure_core::prelude::*;
use std::convert::TryInto;

#[derive(Debug, Clone)]
pub struct GetBlobMetadataBuilder<'a> {
    blob_client: &'a BlobClient,
    blob_versioning: Option<&'a BlobVersioning>,
    lease_id: Option<&'a LeaseId>,
    client_request_id: Option<ClientRequestId>,
    timeout: Option<Timeout>,
}

impl<'a> GetBlobMetadataBuilder<'a> {
    pub(crate) fn new(blob_client: &'a BlobClient) -> Self {
        Self {
            blob_client,
            blob_versioning: None,
            lease_id: None,
            client_request_id: None,
            timeout: None,
        }
    }

    setters! {
        blob_versioning: &'a BlobVersioning => Some(blob_versioning),
        lease_id: &'a LeaseId => Some(lease_id),
        timeout: Timeout => Some(timeout),
        client_request_id: ClientRequestId => Some(client_request_id),
    }

    pub async fn execute(self) -> azure_core::Result<GetBlobMetadataResponse> {
        let mut url = self.blob_client.url_with_segments(None)?;

        url.query_pairs_mut().append_pair("comp", "metadata");
        self.blob_versioning.append_to_url_query(&mut url);
        self.timeout.append_to_url_query(&mut url);

        let mut request =
            self.blob_client
                .prepare_request(url.as_str(), http::Method::GET, None)?;
        request.add_optional_header(&self.client_request_id);
        request.add_optional_header_ref(&self.lease_id);

        let response = self
            .blob_client
            .http_client()
            .execute_request_check_status(&request)
            .await?;

        response.headers().try_into()
    }
}

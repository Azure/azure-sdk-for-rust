use crate::{blob::responses::GetPageRangesResponse, prelude::*};
use azure_core::prelude::*;

pub struct GetPageRangesBuilder<'a> {
    blob_client: &'a BlobClient,
    blob_versioning: Option<&'a BlobVersioning>,
    lease_id: Option<&'a LeaseId>,
    client_request_id: Option<ClientRequestId>,
    timeout: Option<Timeout>,
}

impl<'a> GetPageRangesBuilder<'a> {
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
        client_request_id: ClientRequestId => Some(client_request_id),
        timeout: Timeout => Some(timeout),
    }

    pub async fn execute(&self) -> azure_core::Result<GetPageRangesResponse> {
        let mut url = self.blob_client.url_with_segments(None)?;

        url.query_pairs_mut().append_pair("comp", "pagelist");
        self.blob_versioning.append_to_url_query(&mut url);
        self.timeout.append_to_url_query(&mut url);

        let mut request =
            self.blob_client
                .prepare_request(url.as_str(), http::Method::GET, None)?;
        request.add_optional_header(self.lease_id);
        request.add_optional_header(self.client_request_id.as_ref());

        let response = self
            .blob_client
            .execute_request_check_status(&request)
            .await?;

        GetPageRangesResponse::from_response(response.headers(), response.body())
    }
}

use crate::{
    blob::{responses::GetBlockListResponse, BlockListType},
    prelude::*,
};
use azure_core::prelude::*;

pub struct GetBlockListBuilder<'a> {
    blob_client: &'a BlobClient,
    block_list_type: BlockListType,
    blob_versioning: Option<&'a BlobVersioning>,
    lease_id: Option<&'a LeaseId>,
    client_request_id: Option<ClientRequestId>,
    timeout: Option<Timeout>,
}

impl<'a> GetBlockListBuilder<'a> {
    pub(crate) fn new(blob_client: &'a BlobClient) -> Self {
        Self {
            blob_client,
            block_list_type: BlockListType::Committed,
            blob_versioning: None,
            lease_id: None,
            client_request_id: None,
            timeout: None,
        }
    }

    setters! {
        block_list_type: BlockListType => block_list_type,
        blob_versioning: &'a BlobVersioning => Some(blob_versioning),
        lease_id: &'a LeaseId => Some(lease_id),
        client_request_id: ClientRequestId => Some(client_request_id),
        timeout: Timeout => Some(timeout),
    }

    pub async fn execute(&self) -> azure_core::Result<GetBlockListResponse> {
        let mut url = self.blob_client.url_with_segments(None)?;

        url.query_pairs_mut().append_pair("comp", "blocklist");
        self.blob_versioning.append_to_url_query(&mut url);
        self.block_list_type.append_to_url_query(&mut url);
        self.timeout.append_to_url_query(&mut url);

        let mut request =
            self.blob_client
                .prepare_request(url.as_str(), http::Method::GET, None)?;
        request.add_optional_header_ref(&self.lease_id);
        request.add_optional_header(&self.client_request_id);

        let response = self
            .blob_client
            .execute_request_check_status(&request)
            .await?;

        GetBlockListResponse::from_response(response.headers(), response.body())
    }
}

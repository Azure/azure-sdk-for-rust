use crate::{blob::responses::PutBlockResponse, prelude::*};
use azure_core::prelude::*;
use bytes::Bytes;

#[derive(Debug, Clone)]
pub struct PutBlockBuilder<'a> {
    blob_client: &'a BlobClient,
    block_id: BlockId,
    body: Bytes,
    #[allow(unused)]
    hash: Option<&'a Hash>,
    client_request_id: Option<ClientRequestId>,
    timeout: Option<Timeout>,
    lease_id: Option<&'a LeaseId>,
}

impl<'a> PutBlockBuilder<'a> {
    pub(crate) fn new(
        blob_client: &'a BlobClient,
        block_id: impl Into<BlockId>,
        body: impl Into<Bytes>,
    ) -> Self {
        Self {
            blob_client,
            block_id: block_id.into(),
            body: body.into(),
            hash: None,
            client_request_id: None,
            timeout: None,
            lease_id: None,
        }
    }

    setters! {
        hash: &'a Hash => Some(hash),
        client_request_id: ClientRequestId => Some(client_request_id),
        timeout: Timeout => Some(timeout),
        lease_id: &'a LeaseId => Some(lease_id),
    }

    pub async fn execute(&self) -> azure_core::Result<PutBlockResponse> {
        let mut url = self.blob_client.url_with_segments(None)?;

        self.timeout.append_to_url_query(&mut url);
        self.block_id.append_to_url_query(&mut url);
        url.query_pairs_mut().append_pair("comp", "block");

        let mut request = self.blob_client.prepare_request(
            url.as_str(),
            http::Method::PUT,
            Some(self.body.clone()),
        )?;
        request.add_optional_header(self.client_request_id.as_ref());
        request.add_optional_header(self.lease_id);

        let response = self
            .blob_client
            .execute_request_check_status(&request)
            .await?;

        PutBlockResponse::from_headers(response.headers())
    }
}

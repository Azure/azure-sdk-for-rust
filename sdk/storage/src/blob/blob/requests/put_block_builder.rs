use crate::blob::blob::responses::PutBlockResponse;
use crate::blob::prelude::*;
use azure_core::headers::{add_optional_header, add_optional_header_ref};
use azure_core::prelude::*;
use bytes::Bytes;

#[derive(Debug, Clone)]
pub struct PutBlockBuilder<'a> {
    blob_client: &'a BlobClient,
    block_id: BlockId,
    body: Bytes,
    #[allow(unused)]
    hash: Option<&'a Hash>,
    client_request_id: Option<ClientRequestId<'a>>,
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
        client_request_id: ClientRequestId<'a> => Some(client_request_id),
        timeout: Timeout => Some(timeout),
        lease_id: &'a LeaseId => Some(lease_id),
    }

    pub async fn execute(
        &self,
    ) -> Result<PutBlockResponse, Box<dyn std::error::Error + Send + Sync>> {
        let mut url = self.blob_client.url_with_segments(None)?;

        self.timeout.append_to_url_query(&mut url);
        self.block_id.append_to_url_query(&mut url);
        url.query_pairs_mut().append_pair("comp", "block");

        let (request, _url) = self.blob_client.prepare_request(
            url.as_str(),
            &http::Method::PUT,
            &|mut request| {
                request = add_optional_header(&self.client_request_id, request);
                request = add_optional_header_ref(&self.lease_id, request);
                request
            },
            Some(self.body.clone()),
        )?;

        trace!("request.headers() == {:#?}", request.headers());

        let response = self
            .blob_client
            .http_client()
            .execute_request_check_status(request, http::StatusCode::CREATED)
            .await?;

        debug!("response.headers() == {:#?}", response.headers());

        Ok(PutBlockResponse::from_headers(response.headers())?)
    }
}

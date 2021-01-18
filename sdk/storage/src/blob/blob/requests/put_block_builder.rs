use crate::blob::blob::responses::PutBlockResponse;
use crate::blob::prelude::*;
use crate::core::prelude::*;
use azure_core::headers::{add_optional_header, add_optional_header_ref};
use azure_core::prelude::*;
use bytes::Bytes;

#[derive(Debug, Clone)]
pub struct PutBlockBuilder<'a> {
    blob_client: &'a BlobClient,
    block_id: &'a BlockId,
    body: Bytes,
    hash: Option<&'a Hash>,
    client_request_id: Option<ClientRequestId<'a>>,
    timeout: Option<Timeout>,
    lease_id: Option<&'a LeaseId>,
}

impl<'a> PutBlockBuilder<'a> {
    pub(crate) fn new(
        blob_client: &'a BlobClient,
        block_id: &'a BlockId,
        body: impl Into<Bytes>,
    ) -> Self {
        Self {
            blob_client,
            block_id,
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

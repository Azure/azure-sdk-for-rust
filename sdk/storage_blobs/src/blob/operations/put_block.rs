use crate::prelude::*;
use azure_core::{headers::*, prelude::*, Body, RequestId};
use azure_storage::{headers::consistency_from_headers, ConsistencyCRC64, ConsistencyMD5};
use time::OffsetDateTime;

operation! {
    PutBlock,
    client: BlobClient,
    block_id: BlockId,
    body: Body,
    ?hash: Hash,
    ?lease_id: LeaseId
}

impl PutBlockBuilder {
    pub fn into_future(mut self) -> PutBlock {
        Box::pin(async move {
            let mut url = self.client.url()?;

            self.block_id.append_to_url_query(&mut url);
            url.query_pairs_mut().append_pair("comp", "block");

            let mut headers = Headers::new();
            headers.add(self.lease_id);

            let mut request = self.client.finalize_request(
                url,
                azure_core::Method::Put,
                headers,
                Some(self.body.clone()),
            )?;

            let response = self.client.send(&mut self.context, &mut request).await?;
            PutBlockResponse::from_headers(response.headers())
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct PutBlockResponse {
    pub content_md5: Option<ConsistencyMD5>,
    pub content_crc64: Option<ConsistencyCRC64>,
    pub request_id: RequestId,
    pub date: OffsetDateTime,
    pub request_server_encrypted: bool,
}

impl PutBlockResponse {
    pub(crate) fn from_headers(headers: &Headers) -> azure_core::Result<PutBlockResponse> {
        let (content_md5, content_crc64) = consistency_from_headers(headers)?;
        let request_id = request_id_from_headers(headers)?;
        let date = date_from_headers(headers)?;
        let request_server_encrypted = request_server_encrypted_from_headers(headers)?;

        Ok(PutBlockResponse {
            content_md5,
            content_crc64,
            request_id,
            date,
            request_server_encrypted,
        })
    }
}

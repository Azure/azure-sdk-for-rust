use time::OffsetDateTime;
use crate::prelude::*;
use azure_core::{headers::*, prelude::*, Body, RequestId};
use azure_storage::{ConsistencyCRC64, ConsistencyMD5};
use azure_storage::headers::consistency_from_headers;

operation! {
    AppendBlock,
    client: BlobClient,
    body: Body,
    ?hash: Hash,
    ?condition_max_size: ConditionMaxSize,
    ?condition_append_position: ConditionAppendPosition,
    ?if_modified_since: IfModifiedSinceCondition,
    ?if_match: IfMatchCondition,
    ?if_tags: IfTags,
    ?lease_id: LeaseId
}

impl AppendBlockBuilder {
    pub fn into_future(mut self) -> AppendBlock {
        Box::pin(async move {
            let mut url = self.client.url()?;

            url.query_pairs_mut().append_pair("comp", "appendblock");

            let mut headers = Headers::new();
            headers.add(self.hash);
            headers.add(self.condition_max_size);
            headers.add(self.condition_append_position);
            headers.add(self.if_modified_since);
            headers.add(self.if_match);
            headers.add(self.if_tags);
            headers.add(self.lease_id);

            let mut request = BlobClient::finalize_request(
                url,
                azure_core::Method::Put,
                headers,
                Some(self.body),
            )?;

            let response = self.client.send(&mut self.context, &mut request).await?;

            AppendBlockResponse::from_headers(response.headers())
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AppendBlockResponse {
    pub etag: String,
    pub content_md5: Option<ConsistencyMD5>,
    pub content_crc64: Option<ConsistencyCRC64>,
    pub request_id: RequestId,
    pub date: OffsetDateTime,
    pub request_server_encrypted: bool,
    pub blob_append_offset: u64,
    pub blob_committed_block_count: u64,
}

impl AppendBlockResponse {
    pub(crate) fn from_headers(headers: &Headers) -> azure_core::Result<AppendBlockResponse> {
        let etag = etag_from_headers(headers)?;
        let (content_md5, content_crc64) = consistency_from_headers(headers)?;
        let request_id = request_id_from_headers(headers)?;
        let date = date_from_headers(headers)?;
        let request_server_encrypted = request_server_encrypted_from_headers(headers)?;
        let blob_append_offset = headers.get_as(&BLOB_APPEND_OFFSET)?;
        let blob_committed_block_count = headers.get_as(&BLOB_COMMITTED_BLOCK_COUNT)?;

        Ok(AppendBlockResponse {
            etag,
            content_md5,
            content_crc64,
            request_id,
            date,
            request_server_encrypted,
            blob_append_offset,
            blob_committed_block_count
        })
    }
}

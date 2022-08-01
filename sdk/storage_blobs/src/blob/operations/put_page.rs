use crate::prelude::*;
use azure_core::{headers::*, prelude::*, Body, RequestId};
use azure_storage::{headers::content_md5_from_headers, ConsistencyMD5};
use time::OffsetDateTime;

operation! {
    PutPage,
    client: BlobClient,
    ba512_range: BA512Range,
    content: Body,
    ?hash: Hash,
    ?if_sequence_number: IfSequenceNumber,
    ?if_modified_since: IfModifiedSinceCondition,
    ?if_match: IfMatchCondition,
    ?if_tags: IfTags,
    ?lease_id: LeaseId
}

impl PutPageBuilder {
    pub fn into_future(mut self) -> PutPage {
        Box::pin(async move {
            let mut url = self.client.url()?;

            url.query_pairs_mut().append_pair("comp", "page");

            let mut headers = Headers::new();
            headers.insert(PAGE_WRITE, "update");
            headers.insert(BLOB_TYPE, "PageBlob");
            headers.add(self.ba512_range);
            headers.add(self.hash);
            headers.add(self.if_sequence_number);
            headers.add(self.if_modified_since);
            headers.add(self.if_match);
            headers.add(self.if_tags);
            headers.add(self.lease_id);

            let mut request = self.client.finalize_request(
                url,
                azure_core::Method::Put,
                headers,
                Some(self.content.clone()),
            )?;

            let response = self.client.send(&mut self.context, &mut request).await?;
            PutPageResponse::from_headers(response.headers())
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct PutPageResponse {
    pub etag: String,
    pub last_modified: OffsetDateTime,
    pub content_md5: ConsistencyMD5,
    pub sequence_number: u64,
    pub request_id: RequestId,
    pub date: OffsetDateTime,
    pub request_server_encrypted: bool,
}

impl PutPageResponse {
    pub(crate) fn from_headers(headers: &Headers) -> azure_core::Result<Self> {
        let etag = etag_from_headers(headers)?;
        let last_modified = last_modified_from_headers(headers)?;
        let content_md5 = content_md5_from_headers(headers)?;
        let sequence_number = sequence_number_from_headers(headers)?;
        let request_id = request_id_from_headers(headers)?;
        let date = date_from_headers(headers)?;
        let request_server_encrypted = request_server_encrypted_from_headers(headers)?;

        Ok(Self {
            etag,
            last_modified,
            content_md5,
            sequence_number,
            request_id,
            date,
            request_server_encrypted,
        })
    }
}

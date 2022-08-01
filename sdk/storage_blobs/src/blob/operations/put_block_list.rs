use crate::prelude::*;
use azure_core::{headers::*, prelude::*, RequestId};
use azure_storage::{headers::content_md5_from_headers, ConsistencyMD5};
use bytes::Bytes;
use time::OffsetDateTime;

operation! {
    PutBlockList,
    client: BlobClient,
    block_list: BlockList,
    ?content_type: ContentType,
    ?content_encoding: ContentEncoding,
    ?content_language: ContentLanguage,
    ?content_disposition: ContentDisposition,
    ?content_md5: BlobContentMD5,
    ?metadata: Metadata,
    ?access_tier: AccessTier,
    ?tags: Tags,
    ?lease_id: LeaseId
}

impl PutBlockListBuilder {
    pub fn into_future(mut self) -> PutBlockList {
        Box::pin(async move {
            let mut url = self.client.url()?;

            url.query_pairs_mut().append_pair("comp", "blocklist");

            let body = self.block_list.to_xml();
            let body_bytes = Bytes::from(body);

            // calculate the xml MD5. This can be made optional
            // if needed, but i think it's best to calculate it.
            let md5 = {
                let hash = md5::compute(body_bytes.clone());
                base64::encode(hash.0)
            };

            let mut headers = Headers::new();
            headers.insert(CONTENT_MD5, &md5);
            headers.add(self.content_type);
            headers.add(self.content_encoding);
            headers.add(self.content_language);
            headers.add(self.content_disposition);
            headers.add(self.content_md5);
            headers.add(self.tags);
            if let Some(metadata) = &self.metadata {
                for m in metadata.iter() {
                    headers.add(m);
                }
            }
            headers.add(self.access_tier);
            headers.add(self.lease_id);

            let mut request = self.client.finalize_request(
                url,
                azure_core::Method::Put,
                headers,
                Some(body_bytes.into()),
            )?;

            let response = self.client.send(&mut self.context, &mut request).await?;
            PutBlockListResponse::from_headers(response.headers())
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct PutBlockListResponse {
    pub etag: String,
    pub last_modified: OffsetDateTime,
    pub content_md5: ConsistencyMD5,
    pub request_id: RequestId,
    pub date: OffsetDateTime,
    pub request_server_encrypted: bool,
}

impl PutBlockListResponse {
    pub(crate) fn from_headers(headers: &Headers) -> azure_core::Result<PutBlockListResponse> {
        let etag = etag_from_headers(headers)?;
        let last_modified = last_modified_from_headers(headers)?;
        let content_md5 = content_md5_from_headers(headers)?;
        let request_id = request_id_from_headers(headers)?;
        let date = date_from_headers(headers)?;
        let request_server_encrypted = request_server_encrypted_from_headers(headers)?;

        Ok(PutBlockListResponse {
            etag,
            last_modified,
            content_md5,
            request_id,
            date,
            request_server_encrypted,
        })
    }
}

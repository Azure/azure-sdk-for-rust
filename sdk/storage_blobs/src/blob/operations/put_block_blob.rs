use crate::prelude::*;
use azure_core::{headers::*, prelude::*, Body, RequestId};
use azure_storage::{headers::consistency_from_headers, ConsistencyCRC64, ConsistencyMD5};
use time::OffsetDateTime;

operation! {
    PutBlockBlob,
    client: BlobClient,
    body: Body,
    ?hash: Hash,
    ?content_type: BlobContentType,
    ?content_encoding: BlobContentEncoding,
    ?content_language: BlobContentLanguage,
    ?content_disposition: BlobContentDisposition,
    ?metadata: Metadata,
    ?access_tier: AccessTier,
    ?tags: Tags,
    ?lease_id: LeaseId,
    ?encryption_scope: EncryptionScope,
    ?if_modified_since: IfModifiedSinceCondition,
    ?if_match: IfMatchCondition,
    ?if_tags: IfTags
}

impl PutBlockBlobBuilder {
    pub fn into_future(mut self) -> PutBlockBlob {
        Box::pin(async move {
            let url = self.client.url()?;

            let mut headers = Headers::new();
            headers.insert(BLOB_TYPE, "BlockBlob");
            headers.add(self.hash);
            headers.add(self.content_type);
            headers.add(self.content_encoding);
            headers.add(self.content_language);
            headers.add(self.content_disposition);
            headers.add(self.tags);
            if let Some(metadata) = &self.metadata {
                for m in metadata.iter() {
                    headers.add(m);
                }
            }
            headers.add(self.access_tier);
            headers.add(self.lease_id);
            headers.add(self.encryption_scope);
            headers.add(self.if_modified_since);
            headers.add(self.if_match);
            headers.add(self.if_tags);

            let mut request = self.client.finalize_request(
                url,
                azure_core::Method::Put,
                headers,
                Some(self.body),
            )?;

            let response = self.client.send(&mut self.context, &mut request).await?;
            PutBlockBlobResponse::from_headers(response.headers())
        })
    }
}

#[derive(Debug, Clone)]
pub struct PutBlockBlobResponse {
    pub etag: String,
    pub last_modified: OffsetDateTime,
    pub content_md5: Option<ConsistencyMD5>,
    pub content_crc64: Option<ConsistencyCRC64>,
    pub request_id: RequestId,
    pub date: OffsetDateTime,
    pub request_server_encrypted: bool,
}

impl PutBlockBlobResponse {
    pub fn from_headers(headers: &Headers) -> azure_core::Result<PutBlockBlobResponse> {
        let etag = etag_from_headers(headers)?;
        let last_modified = last_modified_from_headers(headers)?;
        let (content_md5, content_crc64) = consistency_from_headers(headers)?;
        let request_id = request_id_from_headers(headers)?;
        let date = date_from_headers(headers)?;
        let request_server_encrypted = request_server_encrypted_from_headers(headers)?;

        Ok(PutBlockBlobResponse {
            etag,
            last_modified,
            content_md5,
            content_crc64,
            request_id,
            date,
            request_server_encrypted,
        })
    }
}

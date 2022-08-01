use crate::prelude::*;
use azure_core::{headers::*, prelude::*, RequestId};
use time::OffsetDateTime;

operation! {
    PutAppendBlob,
    client: BlobClient,
    ?content_type: ContentType,
    ?content_encoding: ContentEncoding,
    ?content_language: ContentLanguage,
    ?content_disposition: ContentDisposition,
    ?metadata: Metadata,
    ?tags: Tags,
    ?lease_id: LeaseId
}

impl PutAppendBlobBuilder {
    pub fn into_future(mut self) -> PutAppendBlob {
        Box::pin(async move {
            let url = self.client.url()?;

            let mut headers = Headers::new();
            headers.insert(BLOB_TYPE, "AppendBlob");
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
            headers.add(self.lease_id);

            let mut request =
                self.client
                    .finalize_request(url, azure_core::Method::Put, headers, None)?;

            let response = self.client.send(&mut self.context, &mut request).await?;
            PutBlobResponse::from_headers(response.headers())
        })
    }
}

type PutAppendBlobResponse = PutBlobResponse;

#[derive(Debug, Clone)]
pub struct PutBlobResponse {
    pub etag: String,
    pub last_modified: OffsetDateTime,
    pub request_id: RequestId,
    pub date: OffsetDateTime,
    pub request_server_encrypted: bool,
}

impl PutBlobResponse {
    pub fn from_headers(headers: &Headers) -> azure_core::Result<PutBlobResponse> {
        let etag = etag_from_headers(headers)?;
        let last_modified = last_modified_from_headers(headers)?;
        let request_id = request_id_from_headers(headers)?;
        let date = date_from_headers(headers)?;
        let request_server_encrypted = request_server_encrypted_from_headers(headers)?;

        Ok(PutBlobResponse {
            etag,
            last_modified,
            request_id,
            date,
            request_server_encrypted,
        })
    }
}

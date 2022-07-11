use crate::prelude::*;
use azure_core::{headers::*, prelude::*, RequestId};
use azure_storage::{headers::consistency_from_headers, ConsistencyCRC64, ConsistencyMD5};
use bytes::Bytes;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct PutBlockBlobBuilder {
    blob_client: BlobClient,
    body: Bytes,
    hash: Option<Hash>,
    content_type: Option<ContentType>,
    content_encoding: Option<ContentEncoding>,
    content_language: Option<ContentLanguage>,
    content_disposition: Option<ContentDisposition>,
    metadata: Option<Metadata>,
    access_tier: Option<AccessTier>,
    // TODO: Support tags
    lease_id: Option<LeaseId>,
    context: Context,
}

impl PutBlockBlobBuilder {
    pub(crate) fn new(blob_client: BlobClient, body: Bytes) -> Self {
        Self {
            blob_client,
            body,
            hash: None,
            content_type: None,
            content_encoding: None,
            content_language: None,
            content_disposition: None,
            metadata: None,
            access_tier: None,
            lease_id: None,
            context: Context::new(),
        }
    }

    setters! {
        hash: Hash => Some(hash),
        content_type: ContentType => Some(content_type),
        content_encoding: ContentEncoding => Some(content_encoding),
        content_language: ContentLanguage => Some(content_language),
        content_disposition: ContentDisposition => Some(content_disposition),
        metadata: Metadata => Some(metadata),
        access_tier: AccessTier => Some(access_tier),
        lease_id: LeaseId => Some(lease_id),
    }

    pub fn into_future(mut self) -> Response {
        Box::pin(async move {
            let url = self.blob_client.url_with_segments(None)?;

            let mut headers = Headers::new();
            headers.insert(BLOB_TYPE, "BlockBlob");
            headers.add(self.hash);
            headers.add(self.content_type);
            headers.add(self.content_encoding);
            headers.add(self.content_language);
            headers.add(self.content_disposition);
            if let Some(metadata) = &self.metadata {
                for m in metadata.iter() {
                    headers.add(m);
                }
            }
            headers.add(self.access_tier);
            headers.add(self.lease_id);

            let mut request = self.blob_client.finalize_request(
                url,
                azure_core::Method::Put,
                headers,
                Some(self.body.clone()),
            )?;

            let response = self
                .blob_client
                .send(&mut self.context, &mut request)
                .await?;
            PutBlockBlobResponse::from_headers(response.headers())
        })
    }
}

#[derive(Debug, Clone)]
pub struct PutBlockBlobResponse {
    pub etag: String,
    pub last_modified: DateTime<Utc>,
    pub content_md5: Option<ConsistencyMD5>,
    pub content_crc64: Option<ConsistencyCRC64>,
    pub request_id: RequestId,
    pub date: DateTime<Utc>,
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

pub type Response = futures::future::BoxFuture<'static, azure_core::Result<PutBlockBlobResponse>>;

#[cfg(feature = "into_future")]
impl std::future::IntoFuture for PutBlockBlobBuilder {
    type IntoFuture = Response;
    type Output = <Response as std::future::Future>::Output;
    fn into_future(self) -> Self::IntoFuture {
        Self::into_future(self)
    }
}

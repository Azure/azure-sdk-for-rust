use crate::prelude::*;
use azure_core::{headers::*, prelude::*, RequestId};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct PutAppendBlobBuilder {
    blob_client: BlobClient,
    content_type: Option<ContentType>,
    content_encoding: Option<ContentEncoding>,
    content_language: Option<ContentLanguage>,
    content_disposition: Option<ContentDisposition>,
    metadata: Option<Metadata>,
    // TODO: Support tags
    lease_id: Option<LeaseId>,
    context: Context,
}

impl PutAppendBlobBuilder {
    pub(crate) fn new(blob_client: BlobClient) -> Self {
        Self {
            blob_client,
            content_type: None,
            content_encoding: None,
            content_language: None,
            content_disposition: None,
            metadata: None,
            lease_id: None,
            context: Context::new(),
        }
    }

    setters! {
        content_type: ContentType => Some(content_type),
        content_encoding: ContentEncoding => Some(content_encoding),
        content_language: ContentLanguage => Some(content_language),
        content_disposition: ContentDisposition => Some(content_disposition),
        metadata: Metadata => Some(metadata),
        lease_id: LeaseId => Some(lease_id),
    }

    pub fn into_future(mut self) -> Response {
        Box::pin(async move {
            let url = self.blob_client.url_with_segments(None)?;

            let mut headers = Headers::new();
            headers.insert(BLOB_TYPE, "AppendBlob");
            headers.add(self.content_type);
            headers.add(self.content_encoding);
            headers.add(self.content_language);
            headers.add(self.content_disposition);
            if let Some(metadata) = &self.metadata {
                for m in metadata.iter() {
                    headers.add(m);
                }
            }
            headers.add(self.lease_id);

            let mut request =
                self.blob_client
                    .finalize_request(url, azure_core::Method::Put, headers, None)?;

            let response = self
                .blob_client
                .send(&mut self.context, &mut request)
                .await?;
            PutBlobResponse::from_headers(response.headers())
        })
    }
}

#[derive(Debug, Clone)]
pub struct PutBlobResponse {
    pub etag: String,
    pub last_modified: DateTime<Utc>,
    pub request_id: RequestId,
    pub date: DateTime<Utc>,
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

pub type Response = futures::future::BoxFuture<'static, azure_core::Result<PutBlobResponse>>;

#[cfg(feature = "into_future")]
impl std::future::IntoFuture for PutAppendBlobBuilder {
    type IntoFuture = Response;
    type Output = <Response as std::future::Future>::Output;
    fn into_future(self) -> Self::IntoFuture {
        Self::into_future(self)
    }
}

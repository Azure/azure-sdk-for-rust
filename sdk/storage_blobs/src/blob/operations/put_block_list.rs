use crate::prelude::*;
use azure_core::{
    error::{ErrorKind, ResultExt},
    headers::*,
    prelude::*,
    RequestId,
};
use azure_storage::{headers::content_md5_from_headers, ConsistencyMD5};
use bytes::Bytes;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct PutBlockListBuilder {
    blob_client: BlobClient,
    block_list: BlockList,
    content_type: Option<ContentType>,
    content_encoding: Option<ContentEncoding>,
    content_language: Option<ContentLanguage>,
    content_disposition: Option<ContentDisposition>,
    content_md5: Option<BlobContentMD5>,
    metadata: Option<Metadata>,
    access_tier: Option<AccessTier>,
    // TODO: Support tags
    lease_id: Option<LeaseId>,
    timeout: Option<Timeout>,
    context: Context,
}

impl PutBlockListBuilder {
    pub(crate) fn new(blob_client: BlobClient, block_list: BlockList) -> Self {
        Self {
            blob_client,
            block_list,
            content_type: None,
            content_encoding: None,
            content_language: None,
            content_disposition: None,
            content_md5: None,
            metadata: None,
            access_tier: None,
            lease_id: None,
            context: Context::new(),
            timeout: None,
        }
    }

    setters! {
        content_type: ContentType => Some(content_type),
        content_encoding: ContentEncoding => Some(content_encoding),
        content_language: ContentLanguage => Some(content_language),
        content_disposition: ContentDisposition => Some(content_disposition),
        content_md5: BlobContentMD5 => Some(content_md5),
        metadata: Metadata => Some(metadata),
        access_tier: AccessTier => Some(access_tier),
        lease_id: LeaseId => Some(lease_id),
        timeout: Timeout => Some(timeout),
    }

    pub fn into_future(mut self) -> Response {
        Box::pin(async move {
            let mut url = self.blob_client.url_with_segments(None)?;

            url.query_pairs_mut().append_pair("comp", "blocklist");
            self.timeout.append_to_url_query(&mut url);

            let body = self.block_list.to_xml();
            let body_bytes = Bytes::from(body);

            // calculate the xml MD5. This can be made optional
            // if needed, but i think it's best to calculate it.
            let md5 = {
                let hash = md5::compute(body_bytes.clone());
                base64::encode(hash.0)
            };

            let mut request = self.blob_client.prepare_request(
                url.as_str(),
                http::Method::PUT,
                Some(body_bytes),
            )?;
            request.insert_header("Content-MD5", &md5);
            request.add_optional_header(&self.content_type);
            request.add_optional_header(&self.content_encoding);
            request.add_optional_header(&self.content_language);
            request.add_optional_header(&self.content_disposition);
            request.add_optional_header(&self.content_md5);
            if let Some(metadata) = &self.metadata {
                for m in metadata.iter() {
                    request.add_mandatory_header(&m);
                }
            }
            request.add_optional_header(&self.access_tier);
            request.add_optional_header(&self.lease_id);

            let response = self
                .blob_client
                .send(&mut self.context, &mut request)
                .await?;
            PutBlockListResponse::from_headers(response.headers())
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct PutBlockListResponse {
    pub etag: String,
    pub last_modified: DateTime<Utc>,
    pub content_md5: ConsistencyMD5,
    pub request_id: RequestId,
    pub date: DateTime<Utc>,
    pub request_server_encrypted: bool,
}

impl PutBlockListResponse {
    pub(crate) fn from_headers(headers: &Headers) -> azure_core::Result<PutBlockListResponse> {
        let etag = etag_from_headers(headers)?;
        let last_modified = last_modified_from_headers(headers)?;
        let content_md5 = content_md5_from_headers(headers).map_kind(ErrorKind::DataConversion)?;
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
pub type Response = futures::future::BoxFuture<'static, azure_core::Result<PutBlockListResponse>>;
#[cfg(feature = "into_future")]
impl std::future::IntoFuture for PutBlockListBuilder {
    type IntoFuture = Response;
    type Output = <Response as std::future::Future>::Output;
    fn into_future(self) -> Self::IntoFuture {
        Self::into_future(self)
    }
}

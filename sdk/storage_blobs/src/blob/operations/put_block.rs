use crate::prelude::*;
use azure_core::{headers::*, prelude::*, RequestId};
use azure_storage::{headers::consistency_from_headers, ConsistencyCRC64, ConsistencyMD5};
use bytes::Bytes;
use chrono::{DateTime, Utc};

pub struct PutBlockBuilder {
    blob_client: BlobClient,
    block_id: BlockId,
    body: Bytes,
    #[allow(unused)]
    hash: Option<Hash>,
    timeout: Option<Timeout>,
    lease_id: Option<LeaseId>,
    context: Context,
}

impl<'a> PutBlockBuilder {
    pub(crate) fn new(blob_client: BlobClient, block_id: BlockId, body: Bytes) -> Self {
        Self {
            blob_client,
            block_id,
            body,
            hash: None,
            context: Context::new(),
            timeout: None,
            lease_id: None,
        }
    }

    setters! {
        hash: Hash => Some(hash),
        timeout: Timeout => Some(timeout),
        lease_id: LeaseId => Some(lease_id),
    }

    pub fn into_future(mut self) -> Response {
        Box::pin(async move {
            let mut url = self.blob_client.url_with_segments(None)?;

            self.timeout.append_to_url_query(&mut url);
            self.block_id.append_to_url_query(&mut url);
            url.query_pairs_mut().append_pair("comp", "block");

            let mut request = self.blob_client.prepare_request(
                url,
                azure_core::Method::PUT,
                Some(self.body.clone()),
            )?;
            request.add_optional_header(&self.lease_id);

            let response = self
                .blob_client
                .send(&mut self.context, &mut request)
                .await?;
            PutBlockResponse::from_headers(response.headers())
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct PutBlockResponse {
    pub content_md5: Option<ConsistencyMD5>,
    pub content_crc64: Option<ConsistencyCRC64>,
    pub request_id: RequestId,
    pub date: DateTime<Utc>,
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
pub type Response = futures::future::BoxFuture<'static, azure_core::Result<PutBlockResponse>>;

#[cfg(feature = "into_future")]
impl std::future::IntoFuture for PutBlockBuilder {
    type IntoFuture = Response;
    type Output = <Response as std::future::Future>::Output;
    fn into_future(self) -> Self::IntoFuture {
        Self::into_future(self)
    }
}

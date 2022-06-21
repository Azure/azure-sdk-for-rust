use crate::prelude::*;
use azure_core::{
    error::{ErrorKind, ResultExt},
    headers::*,
    prelude::*,
    RequestId,
};
use azure_storage::{headers::consistency_from_headers, ConsistencyCRC64, ConsistencyMD5};
use bytes::Bytes;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct PutBlockBuilder<'a> {
    blob_client: BlobClient,
    block_id: BlockId,
    body: Bytes,
    #[allow(unused)]
    hash: Option<&'a Hash>,
    client_request_id: Option<ClientRequestId>,
    timeout: Option<Timeout>,
    lease_id: Option<LeaseId>,
}

impl<'a> PutBlockBuilder<'a> {
    pub(crate) fn new(
        blob_client: BlobClient,
        block_id: impl Into<BlockId>,
        body: impl Into<Bytes>,
    ) -> Self {
        Self {
            blob_client,
            block_id: block_id.into(),
            body: body.into(),
            hash: None,
            client_request_id: None,
            timeout: None,
            lease_id: None,
        }
    }

    setters! {
        hash: &'a Hash => Some(hash),
        client_request_id: ClientRequestId => Some(client_request_id),
        timeout: Timeout => Some(timeout),
        lease_id: LeaseId => Some(lease_id),
    }

    pub fn into_future(self) -> Response {
        Box::pin(async move {
            let mut url = self.blob_client.url_with_segments(None)?;

            self.timeout.append_to_url_query(&mut url);
            self.block_id.append_to_url_query(&mut url);
            url.query_pairs_mut().append_pair("comp", "block");

            let mut request = self.blob_client.prepare_request(
                url.as_str(),
                http::Method::PUT,
                Some(self.body.clone()),
            )?;
            request.add_optional_header(&self.client_request_id);
            request.add_optional_header(&self.lease_id);

            let response = self
                .blob_client
                .http_client()
                .execute_request_check_status(&request)
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
        debug!("{:#?}", headers);

        let (content_md5, content_crc64) =
            consistency_from_headers(headers).map_kind(ErrorKind::DataConversion)?;
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

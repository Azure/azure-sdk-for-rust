use crate::{prelude::*, BA512Range};
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
pub struct UpdatePageBuilder {
    blob_client: BlobClient,
    ba512_range: BA512Range,
    content: Bytes,
    hash: Option<Hash>,
    sequence_number_condition: Option<SequenceNumberCondition>,
    if_modified_since_condition: Option<IfModifiedSinceCondition>,
    if_match_condition: Option<IfMatchCondition>,
    client_request_id: Option<ClientRequestId>,
    timeout: Option<Timeout>,
    lease_id: Option<LeaseId>,
}

impl UpdatePageBuilder {
    pub(crate) fn new(
        blob_client: BlobClient,
        ba512_range: BA512Range,
        content: impl Into<Bytes>,
    ) -> Self {
        Self {
            blob_client,
            ba512_range,
            content: content.into(),
            hash: None,
            sequence_number_condition: None,
            if_modified_since_condition: None,
            if_match_condition: None,
            client_request_id: None,
            timeout: None,
            lease_id: None,
        }
    }

    setters! {
        hash: Hash => Some(hash),
        sequence_number_condition: SequenceNumberCondition => Some(sequence_number_condition),
        if_modified_since_condition: IfModifiedSinceCondition => Some(if_modified_since_condition),
        if_match_condition: IfMatchCondition => Some(if_match_condition),
        client_request_id: ClientRequestId => Some(client_request_id),
        timeout: Timeout => Some(timeout),
        lease_id: LeaseId => Some(lease_id),
    }

    pub fn into_future(self) -> Response {
        Box::pin(async move {
            let mut url = self.blob_client.url_with_segments(None)?;

            self.timeout.append_to_url_query(&mut url);
            url.query_pairs_mut().append_pair("comp", "page");

            let mut request = self.blob_client.prepare_request(
                url.as_str(),
                http::Method::PUT,
                Some(self.content.clone()),
            )?;
            request.insert_header(PAGE_WRITE, "update");
            request.insert_header(BLOB_TYPE, "PageBlob");
            request.add_mandatory_header(&self.ba512_range);
            request.add_optional_header(&self.sequence_number_condition);
            request.add_optional_header(&self.hash);
            request.add_optional_header(&self.if_modified_since_condition);
            request.add_optional_header(&self.if_match_condition);
            request.add_optional_header(&self.client_request_id);
            request.add_optional_header(&self.lease_id);

            let response = self
                .blob_client
                .http_client()
                .execute_request_check_status(&request)
                .await?;

            UpdatePageResponse::from_headers(response.headers())
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct UpdatePageResponse {
    pub etag: String,
    pub last_modified: DateTime<Utc>,
    pub content_md5: ConsistencyMD5,
    pub sequence_number: u64,
    pub request_id: RequestId,
    pub date: DateTime<Utc>,
    pub request_server_encrypted: bool,
}

impl UpdatePageResponse {
    pub(crate) fn from_headers(headers: &Headers) -> azure_core::Result<UpdatePageResponse> {
        let etag = etag_from_headers(headers)?;
        let last_modified = last_modified_from_headers(headers)?;
        let content_md5 = content_md5_from_headers(headers).map_kind(ErrorKind::DataConversion)?;
        let sequence_number = sequence_number_from_headers(headers)?;
        let request_id = request_id_from_headers(headers)?;
        let date = date_from_headers(headers)?;
        let request_server_encrypted = request_server_encrypted_from_headers(headers)?;

        Ok(UpdatePageResponse {
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

pub type Response = futures::future::BoxFuture<'static, azure_core::Result<UpdatePageResponse>>;

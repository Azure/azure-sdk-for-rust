use crate::{prelude::*, BA512Range};
use azure_core::{
    headers::{BLOB_TYPE, PAGE_WRITE, *},
    prelude::*,
    RequestId,
};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct ClearPageBuilder {
    blob_client: BlobClient,
    ba512_range: BA512Range,
    sequence_number_condition: Option<SequenceNumberCondition>,
    if_modified_since_condition: Option<IfModifiedSinceCondition>,
    if_match_condition: Option<IfMatchCondition>,
    timeout: Option<Timeout>,
    lease_id: Option<LeaseId>,
    context: Context,
}

impl ClearPageBuilder {
    pub(crate) fn new(blob_client: BlobClient, ba512_range: BA512Range) -> Self {
        Self {
            blob_client,
            ba512_range,
            sequence_number_condition: None,
            if_modified_since_condition: None,
            if_match_condition: None,
            timeout: None,
            lease_id: None,
            context: Context::new(),
        }
    }

    setters! {
        sequence_number_condition: SequenceNumberCondition => Some(sequence_number_condition),
        if_modified_since_condition: IfModifiedSinceCondition => Some(if_modified_since_condition),
        if_match_condition: IfMatchCondition => Some(if_match_condition),
        timeout: Timeout => Some(timeout),
        lease_id: LeaseId => Some(lease_id),
    }

    pub fn into_future(mut self) -> Response {
        Box::pin(async move {
            let mut url = self.blob_client.url_with_segments(None)?;

            self.timeout.append_to_url_query(&mut url);
            url.query_pairs_mut().append_pair("comp", "page");

            let mut request =
                self.blob_client
                    .prepare_request(url, azure_core::Method::Put, None)?;

            request.insert_header(PAGE_WRITE, "clear");
            request.insert_header(BLOB_TYPE, "PageBlob");
            request.add_mandatory_header(&self.ba512_range);
            request.add_optional_header(&self.sequence_number_condition);
            request.add_optional_header(&self.if_modified_since_condition);
            request.add_optional_header(&self.if_match_condition);
            request.add_optional_header(&self.lease_id);

            let response = self
                .blob_client
                .send(&mut self.context, &mut request)
                .await?;

            ClearPageResponse::from_headers(response.headers())
        })
    }
}

azure_storage::response_from_headers!(ClearPageResponse,
    etag_from_headers => etag: String,
    last_modified_from_headers => last_modified: DateTime<Utc>,
    sequence_number_from_headers => sequence_number: u64,
    request_id_from_headers => request_id: RequestId,
    date_from_headers => date: DateTime<Utc>
);
pub type Response = futures::future::BoxFuture<'static, azure_core::Result<ClearPageResponse>>;

#[cfg(feature = "into_future")]
impl std::future::IntoFuture for ClearPageBuilder {
    type IntoFuture = Response;
    type Output = <Response as std::future::Future>::Output;
    fn into_future(self) -> Self::IntoFuture {
        Self::into_future(self)
    }
}

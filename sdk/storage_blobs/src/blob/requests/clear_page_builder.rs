use crate::{blob::responses::ClearPageResponse, prelude::*, BA512Range};
use azure_core::{
    headers::{BLOB_TYPE, PAGE_WRITE},
    prelude::*,
};

#[derive(Debug, Clone)]
pub struct ClearPageBuilder<'a> {
    blob_client: &'a BlobClient,
    ba512_range: BA512Range,
    sequence_number_condition: Option<SequenceNumberCondition>,
    if_modified_since_condition: Option<IfModifiedSinceCondition>,
    if_match_condition: Option<IfMatchCondition>,
    client_request_id: Option<ClientRequestId>,
    timeout: Option<Timeout>,
    lease_id: Option<&'a LeaseId>,
}

impl<'a> ClearPageBuilder<'a> {
    pub(crate) fn new(blob_client: &'a BlobClient, ba512_range: BA512Range) -> Self {
        Self {
            blob_client,
            ba512_range,
            sequence_number_condition: None,
            if_modified_since_condition: None,
            if_match_condition: None,
            client_request_id: None,
            timeout: None,
            lease_id: None,
        }
    }

    setters! {
        sequence_number_condition: SequenceNumberCondition => Some(sequence_number_condition),
        if_modified_since_condition: IfModifiedSinceCondition => Some(if_modified_since_condition),
        if_match_condition: IfMatchCondition => Some(if_match_condition),
        client_request_id: ClientRequestId => Some(client_request_id),
        timeout: Timeout => Some(timeout),
        lease_id: &'a LeaseId => Some(lease_id),
    }

    pub async fn execute(&self) -> azure_core::Result<ClearPageResponse> {
        let mut url = self.blob_client.url_with_segments(None)?;

        self.timeout.append_to_url_query(&mut url);
        url.query_pairs_mut().append_pair("comp", "page");

        let mut request =
            self.blob_client
                .prepare_request(url.as_str(), http::Method::PUT, None)?;
        request.insert_header(PAGE_WRITE, "clear");
        request.insert_header(BLOB_TYPE, "PageBlob");
        request.add_mandatory_header(&self.ba512_range);
        request.add_optional_header(self.sequence_number_condition.as_ref());
        request.add_optional_header(self.if_modified_since_condition.as_ref());
        request.add_optional_header(self.if_match_condition.as_ref());
        request.add_optional_header(self.client_request_id.as_ref());
        request.add_optional_header(self.lease_id);

        let response = self
            .blob_client
            .execute_request_check_status(&request)
            .await?;

        ClearPageResponse::from_headers(response.headers())
    }
}

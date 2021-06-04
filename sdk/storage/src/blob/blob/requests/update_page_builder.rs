use crate::blob::blob::responses::UpdatePageResponse;
use crate::blob::prelude::*;
use azure_core::headers::{add_mandatory_header, add_optional_header, add_optional_header_ref};
use azure_core::headers::{BLOB_TYPE, PAGE_WRITE};
use azure_core::prelude::*;
use bytes::Bytes;

#[derive(Debug, Clone)]
pub struct UpdatePageBuilder<'a> {
    blob_client: &'a BlobClient,
    ba512_range: BA512Range,
    content: Bytes,
    hash: Option<&'a Hash>,
    sequence_number_condition: Option<SequenceNumberCondition>,
    if_modified_since_condition: Option<IfModifiedSinceCondition>,
    if_match_condition: Option<IfMatchCondition<'a>>,
    client_request_id: Option<ClientRequestId<'a>>,
    timeout: Option<Timeout>,
    lease_id: Option<&'a LeaseId>,
}

impl<'a> UpdatePageBuilder<'a> {
    pub(crate) fn new(
        blob_client: &'a BlobClient,
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
        hash: &'a Hash => Some(hash),
        sequence_number_condition: SequenceNumberCondition => Some(sequence_number_condition),
        if_modified_since_condition: IfModifiedSinceCondition => Some(if_modified_since_condition),
        if_match_condition: IfMatchCondition<'a> => Some(if_match_condition),
        client_request_id: ClientRequestId<'a> => Some(client_request_id),
        timeout: Timeout => Some(timeout),
        lease_id: &'a LeaseId => Some(lease_id),
    }

    pub async fn execute(
        &self,
    ) -> Result<UpdatePageResponse, Box<dyn std::error::Error + Send + Sync>> {
        let mut url = self.blob_client.url_with_segments(None)?;

        self.timeout.append_to_url_query(&mut url);
        url.query_pairs_mut().append_pair("comp", "page");

        trace!("url == {:?}", url);

        let (request, _url) = self.blob_client.prepare_request(
            url.as_str(),
            &http::Method::PUT,
            &|mut request| {
                request = request.header(PAGE_WRITE, "update");
                request = request.header(BLOB_TYPE, "PageBlob");
                request = add_mandatory_header(&self.ba512_range, request);
                request = add_optional_header(&self.sequence_number_condition, request);
                request = add_optional_header_ref(&self.hash, request);
                request = add_optional_header(&self.if_modified_since_condition, request);
                request = add_optional_header(&self.if_match_condition, request);
                request = add_optional_header(&self.client_request_id, request);
                request = add_optional_header_ref(&self.lease_id, request);
                request
            },
            Some(self.content.clone()),
        )?;

        trace!("request.headers() == {:#?}", request.headers());

        let response = self
            .blob_client
            .http_client()
            .execute_request_check_status(request, http::StatusCode::CREATED)
            .await?;

        debug!("response.headers() == {:#?}", response.headers());

        Ok(UpdatePageResponse::from_headers(response.headers())?)
    }
}

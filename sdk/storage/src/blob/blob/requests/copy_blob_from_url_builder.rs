use crate::blob::blob::responses::CopyBlobFromUrlResponse;
use crate::blob::prelude::*;
use azure_core::headers::{
    add_optional_header, add_optional_header_ref, COPY_SOURCE, REQUIRES_SYNC,
};
use azure_core::prelude::*;
use std::convert::TryInto;

pub struct CopyBlobFromUrlBuilder<'a> {
    blob_client: &'a BlobClient,
    source_url: &'a str,
    is_synchronous: bool,
    metadata: Option<&'a Metadata>,
    if_modified_since_condition: Option<IfModifiedSinceCondition>,
    if_match_condition: Option<IfMatchCondition<'a>>,
    timeout: Option<Timeout>,
    lease_id: Option<&'a LeaseId>,
    client_request_id: Option<ClientRequestId<'a>>,
    if_source_since_condition: Option<IfSourceModifiedSinceCondition>,
    if_source_match_condition: Option<IfSourceMatchCondition<'a>>,
    source_content_md5: Option<&'a SourceContentMD5>,
}

impl<'a> CopyBlobFromUrlBuilder<'a> {
    pub(crate) fn new(blob_client: &'a BlobClient, source_url: &'a str) -> Self {
        Self {
            blob_client,
            source_url,
            is_synchronous: false,
            metadata: None,
            if_modified_since_condition: None,
            if_match_condition: None,
            timeout: None,
            lease_id: None,
            client_request_id: None,
            if_source_since_condition: None,
            if_source_match_condition: None,
            source_content_md5: None,
        }
    }

    setters! {
        is_synchronous: bool => is_synchronous,
        metadata: &'a Metadata => Some(metadata),
        if_modified_since_condition: IfModifiedSinceCondition => Some(if_modified_since_condition),
        if_match_condition: IfMatchCondition<'a> => Some(if_match_condition),
        timeout: Timeout => Some(timeout),
        lease_id: &'a LeaseId => Some(lease_id),
        client_request_id: ClientRequestId<'a> => Some(client_request_id),
        if_source_since_condition: IfSourceModifiedSinceCondition => Some(if_source_since_condition),
        if_source_match_condition: IfSourceMatchCondition<'a> => Some(if_source_match_condition),
        source_content_md5: &'a SourceContentMD5 => Some(source_content_md5),
    }

    pub async fn execute(
        &self,
    ) -> Result<CopyBlobFromUrlResponse, Box<dyn std::error::Error + Send + Sync>> {
        let mut url = self.blob_client.url_with_segments(None)?;

        self.timeout.append_to_url_query(&mut url);

        trace!("url == {:?}", url);

        let (request, _url) = self.blob_client.prepare_request(
            url.as_str(),
            &http::Method::PUT,
            &|mut request| {
                request = request.header(COPY_SOURCE, self.source_url);
                request = request.header(REQUIRES_SYNC, format!("{}", self.is_synchronous));
                request = add_optional_header(&self.metadata, request);
                request = add_optional_header(&self.if_modified_since_condition, request);
                request = add_optional_header(&self.if_match_condition, request);
                request = add_optional_header_ref(&self.lease_id, request);
                request = add_optional_header(&self.client_request_id, request);
                request = add_optional_header(&self.if_source_since_condition, request);
                request = add_optional_header(&self.if_source_match_condition, request);
                request = add_optional_header_ref(&self.source_content_md5, request);
                request
            },
            None,
        )?;

        let response = self
            .blob_client
            .http_client()
            .execute_request_check_status(request, http::StatusCode::ACCEPTED)
            .await?;

        debug!("response.headers() == {:#?}", response.headers());

        Ok((response.headers()).try_into()?)
    }
}

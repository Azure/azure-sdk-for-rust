use crate::{blob::responses::CopyBlobResponse, prelude::*, RehydratePriority};
use azure_core::{headers::COPY_SOURCE, prelude::*};
use std::convert::TryInto;
use url::Url;

#[derive(Debug, Clone)]
pub struct CopyBlobBuilder<'a> {
    blob_client: &'a BlobClient,
    source_url: &'a Url,
    metadata: Option<&'a Metadata>,
    sequence_number_condition: Option<SequenceNumberCondition>,
    if_modified_since_condition: Option<IfModifiedSinceCondition>,
    if_match_condition: Option<IfMatchCondition>,
    access_tier: Option<AccessTier>,
    timeout: Option<Timeout>,
    lease_id: Option<&'a LeaseId>,
    client_request_id: Option<ClientRequestId>,
    if_source_since_condition: Option<IfSourceModifiedSinceCondition>,
    if_source_match_condition: Option<IfSourceMatchCondition>,
    source_lease_id: Option<&'a SourceLeaseId>,
    rehydrate_priority: RehydratePriority,
}

impl<'a> CopyBlobBuilder<'a> {
    pub(crate) fn new(blob_client: &'a BlobClient, source_url: &'a Url) -> Self {
        Self {
            blob_client,
            source_url,
            metadata: None,
            sequence_number_condition: None,
            if_modified_since_condition: None,
            if_match_condition: None,
            access_tier: None,
            timeout: None,
            lease_id: None,
            client_request_id: None,
            if_source_since_condition: None,
            if_source_match_condition: None,
            source_lease_id: None,
            rehydrate_priority: RehydratePriority::Standard,
        }
    }

    setters! {
        metadata: &'a Metadata => Some(metadata),
        sequence_number_condition: SequenceNumberCondition => Some(sequence_number_condition),
        if_modified_since_condition: IfModifiedSinceCondition => Some(if_modified_since_condition),
        if_match_condition: IfMatchCondition => Some(if_match_condition),
        access_tier: AccessTier => Some(access_tier),
        timeout: Timeout => Some(timeout),
        lease_id: &'a LeaseId => Some(lease_id),
        client_request_id: ClientRequestId => Some(client_request_id),
        if_source_since_condition: IfSourceModifiedSinceCondition => Some(if_source_since_condition),
        if_source_match_condition: IfSourceMatchCondition => Some(if_source_match_condition),
        source_lease_id: &'a SourceLeaseId => Some(source_lease_id),
        rehydrate_priority: RehydratePriority => rehydrate_priority,
    }

    pub async fn execute(&self) -> azure_core::Result<CopyBlobResponse> {
        let mut url = self.blob_client.url_with_segments(None)?;

        self.timeout.append_to_url_query(&mut url);

        let mut request =
            self.blob_client
                .prepare_request(url.as_str(), http::Method::PUT, None)?;
        request.insert_header(COPY_SOURCE, self.source_url.as_str().to_owned());
        if let Some(metadata) = &self.metadata {
            for m in metadata.iter() {
                request.add_mandatory_header(&m);
            }
        }
        request.add_optional_header(self.sequence_number_condition.as_ref());
        request.add_optional_header(self.if_modified_since_condition.as_ref());
        request.add_optional_header(self.if_match_condition.as_ref());
        request.add_optional_header(self.access_tier.as_ref());
        request.add_optional_header(self.lease_id);
        request.add_optional_header(self.client_request_id.as_ref());
        request.add_optional_header(self.if_source_since_condition.as_ref());
        request.add_optional_header(self.if_source_match_condition.as_ref());
        request.add_optional_header(self.source_lease_id);
        request.add_mandatory_header(&self.rehydrate_priority);

        let response = self
            .blob_client
            .execute_request_check_status(&request)
            .await?;

        (response.headers()).try_into()
    }
}

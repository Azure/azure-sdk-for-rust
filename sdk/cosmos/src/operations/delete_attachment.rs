use crate::headers::from_headers::*;
use crate::prelude::*;
use crate::ResourceQuota;
use azure_core::headers::session_token_from_headers;
use azure_core::prelude::*;
use azure_core::{
    collect_pinned_stream, Request as HttpRequest, Response as HttpResponse, SessionToken,
};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct DeleteAttachmentOptions<'a, 'b> {
    attachment_client: &'a AttachmentClient,
    if_match_condition: Option<IfMatchCondition<'b>>,
    activity_id: Option<ActivityId<'b>>,
    consistency_level: Option<ConsistencyLevel>,
}

impl<'a, 'b> DeleteAttachmentOptions<'a, 'b> {
    pub fn new(attachment_client: &'a AttachmentClient) -> Self {
        Self {
            attachment_client,
            if_match_condition: None,
            activity_id: None,
            consistency_level: None,
        }
    }

    setters! {
        activity_id: &'b str => Some(ActivityId::new(activity_id)),
        consistency_level: ConsistencyLevel => Some(consistency_level),
        if_match_condition: IfMatchCondition<'b> => Some(if_match_condition),
    }

    pub fn decorate_request(&self, request: &mut HttpRequest) -> Result<(), crate::Error> {
        azure_core::headers::add_optional_header2(&self.if_match_condition, request)?;
        azure_core::headers::add_optional_header2(&self.activity_id, request)?;
        azure_core::headers::add_optional_header2(&self.consistency_level, request)?;

        crate::cosmos_entity::add_as_partition_key_header_serialized2(
            self.attachment_client
                .document_client()
                .partition_key_serialized(),
            request,
        );

        request.set_body(bytes::Bytes::from_static(EMPTY_BODY).into());
        debug!("request == {:#?}", request);

        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct DeleteAttachmentResponse {
    pub max_media_storage_usage_mb: u64,
    pub media_storage_usage_mb: u64,
    pub last_change: DateTime<Utc>,
    pub resource_quota: Vec<ResourceQuota>,
    pub resource_usage: Vec<ResourceQuota>,
    pub lsn: u64,
    pub alt_content_path: String,
    pub content_path: String,
    pub quorum_acked_lsn: u64,
    pub current_write_quorum: u64,
    pub current_replica_set_size: u64,
    pub global_committed_lsn: u64,
    pub number_of_read_regions: u32,
    pub transport_request_id: u64,
    pub cosmos_llsn: u64,
    pub cosmos_quorum_acked_llsn: u64,
    pub session_token: SessionToken,
    pub request_charge: f64,
    pub service_version: String,
    pub activity_id: uuid::Uuid,
    pub gateway_version: String,
    pub date: DateTime<Utc>,
}

impl DeleteAttachmentResponse {
    pub async fn try_from(response: HttpResponse) -> Result<Self, crate::Error> {
        let (_status_code, headers, pinned_stream) = response.deconstruct();
        let body = collect_pinned_stream(pinned_stream).await?;

        debug!("headers == {:#?}", &headers);
        debug!("body == {:#?}", &body);

        Ok(Self {
            max_media_storage_usage_mb: max_media_storage_usage_mb_from_headers(&headers)?,
            media_storage_usage_mb: media_storage_usage_mb_from_headers(&headers)?,
            last_change: last_state_change_from_headers(&headers)?,
            resource_quota: resource_quota_from_headers(&headers)?,
            resource_usage: resource_usage_from_headers(&headers)?,
            lsn: lsn_from_headers(&headers)?,
            alt_content_path: alt_content_path_from_headers(&headers)?.to_owned(),
            content_path: content_path_from_headers(&headers)?.to_owned(),
            quorum_acked_lsn: quorum_acked_lsn_from_headers(&headers)?,
            current_write_quorum: current_write_quorum_from_headers(&headers)?,
            current_replica_set_size: current_replica_set_size_from_headers(&headers)?,
            global_committed_lsn: global_committed_lsn_from_headers(&headers)?,
            number_of_read_regions: number_of_read_regions_from_headers(&headers)?,
            transport_request_id: transport_request_id_from_headers(&headers)?,
            cosmos_llsn: cosmos_llsn_from_headers(&headers)?,
            cosmos_quorum_acked_llsn: cosmos_quorum_acked_llsn_from_headers(&headers)?,
            session_token: session_token_from_headers(&headers)?,
            request_charge: request_charge_from_headers(&headers)?,
            service_version: service_version_from_headers(&headers)?.to_owned(),
            activity_id: activity_id_from_headers(&headers)?,
            gateway_version: gateway_version_from_headers(&headers)?.to_owned(),
            date: date_from_headers(&headers)?,
        })
    }
}

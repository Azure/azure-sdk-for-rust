use crate::headers::from_headers::*;
use crate::prelude::*;
use crate::resources::Attachment;
use crate::ResourceQuota;
use azure_core::headers::{
    continuation_token_from_headers_optional, item_count_from_headers, session_token_from_headers,
};
use azure_core::prelude::*;
use azure_core::{
    collect_pinned_stream, Request as HttpRequest, Response as HttpResponse, SessionToken,
};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct ListAttachmentsOptions<'a> {
    if_match_condition: Option<IfMatchCondition<'a>>,
    activity_id: Option<ActivityId<'a>>,
    consistency_level: Option<ConsistencyLevel>,
    max_item_count: MaxItemCount,
    a_im: ChangeFeed,
}

impl<'a> ListAttachmentsOptions<'a> {
    pub fn new() -> Self {
        Self {
            if_match_condition: None,
            activity_id: None,
            consistency_level: None,
            max_item_count: MaxItemCount::new(-1),
            a_im: ChangeFeed::None,
        }
    }

    setters! {
        activity_id: &'a str => Some(ActivityId::new(activity_id)),
        consistency_level: ConsistencyLevel => Some(consistency_level),
        if_match_condition: IfMatchCondition<'a> => Some(if_match_condition),
        max_item_count: i32 => MaxItemCount::new(max_item_count),
        a_im: ChangeFeed,
    }

    pub(crate) fn decorate_request(
        &self,
        request: &mut HttpRequest,
        partition_key_serialized: &str,
    ) -> crate::Result<()> {
        // add trait headers
        azure_core::headers::add_optional_header2(&self.if_match_condition, request)?;
        azure_core::headers::add_optional_header2(&self.activity_id, request)?;
        azure_core::headers::add_optional_header2(&self.consistency_level, request)?;
        azure_core::headers::add_mandatory_header2(&self.max_item_count, request)?;
        azure_core::headers::add_mandatory_header2(&self.a_im, request)?;

        crate::cosmos_entity::add_as_partition_key_header_serialized2(
            partition_key_serialized,
            request,
        );
        request.set_body(bytes::Bytes::from_static(EMPTY_BODY).into());

        Ok(())
    }
}

#[derive(Debug, Clone, Deserialize)]
struct JsonListAttachmentResponse {
    #[serde(rename = "_rid")]
    pub rid: String,
    #[serde(rename = "_count")]
    pub count: u64,
    #[serde(rename = "Attachments")]
    pub attachments: Vec<Attachment>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ListAttachmentsResponse {
    pub rid: String,
    pub count: u64,
    pub attachments: Vec<Attachment>,

    pub max_media_storage_usage_mb: u64,
    pub media_storage_usage_mb: u64,
    pub last_change: DateTime<Utc>,
    pub resource_quota: Vec<ResourceQuota>,
    pub resource_usage: Vec<ResourceQuota>,
    pub lsn: u64,
    pub item_count: u32,
    pub alt_content_path: String,
    pub content_path: String,
    pub global_committed_lsn: u64,
    pub number_of_read_regions: u32,
    pub transport_request_id: u64,
    pub cosmos_llsn: u64,
    pub session_token: SessionToken,
    pub request_charge: f64,
    pub service_version: String,
    pub activity_id: uuid::Uuid,
    pub gateway_version: String,
    pub date: DateTime<Utc>,
    pub continuation_token: Option<String>,
}

impl ListAttachmentsResponse {
    pub async fn try_from(response: HttpResponse) -> crate::Result<Self> {
        let (_status_code, headers, pinned_stream) = response.deconstruct();
        let body = collect_pinned_stream(pinned_stream).await?;

        debug!("headers == {:#?}", headers);
        debug!("body == {:#?}", body);

        let json: JsonListAttachmentResponse = serde_json::from_slice(&body)?;

        Ok(Self {
            rid: json.rid,
            count: json.count,
            attachments: json.attachments,

            max_media_storage_usage_mb: max_media_storage_usage_mb_from_headers(&headers)?,
            media_storage_usage_mb: media_storage_usage_mb_from_headers(&headers)?,
            last_change: last_state_change_from_headers(&headers)?,
            resource_quota: resource_quota_from_headers(&headers)?,
            resource_usage: resource_usage_from_headers(&headers)?,
            lsn: lsn_from_headers(&headers)?,
            item_count: item_count_from_headers(&headers)?,
            alt_content_path: alt_content_path_from_headers(&headers)?.to_owned(),
            content_path: content_path_from_headers(&headers)?.to_owned(),
            global_committed_lsn: global_committed_lsn_from_headers(&headers)?,
            number_of_read_regions: number_of_read_regions_from_headers(&headers)?,
            transport_request_id: transport_request_id_from_headers(&headers)?,
            cosmos_llsn: cosmos_llsn_from_headers(&headers)?,
            session_token: session_token_from_headers(&headers)?,
            request_charge: request_charge_from_headers(&headers)?,
            service_version: service_version_from_headers(&headers)?.to_owned(),
            activity_id: activity_id_from_headers(&headers)?,
            gateway_version: gateway_version_from_headers(&headers)?.to_owned(),
            date: date_from_headers(&headers)?,
            continuation_token: continuation_token_from_headers_optional(&headers)?,
        })
    }
}

impl IntoIterator for ListAttachmentsResponse {
    type Item = Attachment;

    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.attachments.into_iter()
    }
}

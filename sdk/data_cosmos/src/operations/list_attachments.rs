use crate::headers::from_headers::*;
use crate::prelude::*;
use crate::resources::Attachment;
use crate::resources::ResourceType;
use crate::ResourceQuota;

use azure_core::headers::{
    continuation_token_from_headers_optional, item_count_from_headers, session_token_from_headers,
};
use azure_core::prelude::*;
use azure_core::{Pageable, Response as HttpResponse, SessionToken};
use time::OffsetDateTime;

operation! {
    #[stream]
    ListAttachments,
    client: DocumentClient,
    ?max_item_count: MaxItemCount,
    ?a_im: ChangeFeed,
    ?if_match_condition: IfMatchCondition,
    ?consistency_level: ConsistencyLevel
}

impl ListAttachmentsBuilder {
    pub fn into_stream(self) -> ListAttachments {
        let make_request = move |continuation: Option<Continuation>| {
            let this = self.clone();
            let ctx = self.context.clone();
            async move {
                let mut request = this.client.cosmos_client().request(
                    &format!(
                        "dbs/{}/colls/{}/docs/{}/attachments",
                        this.client.database_client().database_name(),
                        this.client.collection_client().collection_name(),
                        this.client.document_name()
                    ),
                    azure_core::Method::Get,
                );

                request.insert_headers(&this.if_match_condition);
                if let Some(cl) = &this.consistency_level {
                    request.insert_headers(cl);
                }
                request.insert_headers(&this.max_item_count.unwrap_or_default());
                request.insert_headers(&this.a_im.unwrap_or_default());
                crate::cosmos_entity::add_as_partition_key_header_serialized(
                    this.client.partition_key_serialized(),
                    &mut request,
                );

                request.insert_headers(&continuation);

                let response = this
                    .client
                    .cosmos_client()
                    .pipeline()
                    .send(ctx.clone().insert(ResourceType::Attachments), &mut request)
                    .await?;
                ListAttachmentsResponse::try_from(response).await
            }
        };

        Pageable::new(make_request)
    }
}

pub type ListAttachments = Pageable<ListAttachmentsResponse, azure_core::error::Error>;

#[derive(Debug, Clone, Deserialize)]
struct JsonListAttachmentResponse {
    #[serde(rename = "_rid")]
    pub rid: String,
    #[serde(rename = "_count")]
    pub count: u64,
    #[serde(rename = "Attachments")]
    pub attachments: Vec<Attachment>,
}

#[derive(Debug, Clone)]
pub struct ListAttachmentsResponse {
    pub rid: String,
    pub count: u64,
    pub attachments: Vec<Attachment>,

    pub max_media_storage_usage_mb: u64,
    pub media_storage_usage_mb: u64,
    pub last_change: OffsetDateTime,
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
    pub date: OffsetDateTime,
    pub continuation_token: Option<Continuation>,
}

impl ListAttachmentsResponse {
    pub async fn try_from(response: HttpResponse) -> azure_core::Result<Self> {
        let (_status_code, headers, body) = response.deconstruct();
        let body = body.collect().await?;

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
            alt_content_path: alt_content_path_from_headers(&headers)?,
            content_path: content_path_from_headers(&headers)?,
            global_committed_lsn: global_committed_lsn_from_headers(&headers)?,
            number_of_read_regions: number_of_read_regions_from_headers(&headers)?,
            transport_request_id: transport_request_id_from_headers(&headers)?,
            cosmos_llsn: cosmos_llsn_from_headers(&headers)?,
            session_token: session_token_from_headers(&headers)?,
            request_charge: request_charge_from_headers(&headers)?,
            service_version: service_version_from_headers(&headers)?,
            activity_id: activity_id_from_headers(&headers)?,
            gateway_version: gateway_version_from_headers(&headers)?,
            date: date_from_headers(&headers)?,
            continuation_token: continuation_token_from_headers_optional(&headers)?,
        })
    }
}

impl Continuable for ListAttachmentsResponse {
    type Continuation = Continuation;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.continuation_token.clone()
    }
}

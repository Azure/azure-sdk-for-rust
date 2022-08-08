use crate::headers::from_headers::*;
use crate::prelude::*;
use crate::resources::document::IndexingDirective;
use crate::resources::Attachment;
use crate::ResourceQuota;
use azure_core::headers::{
    content_type_from_headers, etag_from_headers, session_token_from_headers,
};
use azure_core::SessionToken;
use azure_core::{prelude::*, Response as HttpResponse};
use time::OffsetDateTime;

operation! {
    GetAttachment,
    client: AttachmentClient,
    ?if_match_condition: IfMatchCondition,
    ?consistency_level: ConsistencyLevel
}

impl GetAttachmentBuilder {
    pub fn into_future(self) -> GetAttachment {
        Box::pin(async move {
            let mut request = self.client.attachment_request(azure_core::Method::Get);

            request.insert_headers(&self.if_match_condition);
            if let Some(cl) = &self.consistency_level {
                request.insert_headers(cl);
            }

            crate::cosmos_entity::add_as_partition_key_header_serialized(
                self.client.document_client().partition_key_serialized(),
                &mut request,
            );
            let response = self
                .client
                .pipeline()
                .send(
                    self.context.clone().insert(ResourceType::Attachments),
                    &mut request,
                )
                .await?;

            GetAttachmentResponse::try_from(response).await
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct GetAttachmentResponse {
    pub attachment: Attachment,

    pub content_type: String,
    pub content_location: String,
    pub last_change: OffsetDateTime,
    pub etag: String,
    pub resource_quota: Vec<ResourceQuota>,
    pub resource_usage: Vec<ResourceQuota>,
    pub lsn: u64,
    pub alt_content_path: String,
    pub content_path: String,
    pub global_committed_lsn: u64,
    pub number_of_read_regions: u32,
    pub item_lsn: u64,
    pub transport_request_id: u64,
    pub cosmos_llsn: u64,
    pub cosmos_item_llsn: u64,
    pub session_token: SessionToken,
    pub request_charge: f64,
    pub indexing_directive: Option<IndexingDirective>,
    pub service_version: String,
    pub activity_id: uuid::Uuid,
    pub gateway_version: String,
    pub date: OffsetDateTime,
}

impl GetAttachmentResponse {
    pub async fn try_from(response: HttpResponse) -> azure_core::Result<Self> {
        let (_status_code, headers, body) = response.deconstruct();
        let body = body.collect().await?;

        Ok(Self {
            attachment: serde_json::from_slice(&body)?,
            content_type: content_type_from_headers(&headers)?,
            content_location: content_location_from_headers(&headers)?,
            last_change: last_state_change_from_headers(&headers)?,
            etag: etag_from_headers(&headers)?,
            resource_quota: resource_quota_from_headers(&headers)?,
            resource_usage: resource_usage_from_headers(&headers)?,
            lsn: lsn_from_headers(&headers)?,
            alt_content_path: alt_content_path_from_headers(&headers)?,
            content_path: content_path_from_headers(&headers)?,
            global_committed_lsn: global_committed_lsn_from_headers(&headers)?,
            number_of_read_regions: number_of_read_regions_from_headers(&headers)?,
            item_lsn: item_lsn_from_headers(&headers)?,
            transport_request_id: transport_request_id_from_headers(&headers)?,
            cosmos_llsn: cosmos_llsn_from_headers(&headers)?,
            cosmos_item_llsn: cosmos_item_llsn_from_headers(&headers)?,
            session_token: session_token_from_headers(&headers)?,
            request_charge: request_charge_from_headers(&headers)?,
            indexing_directive: indexing_directive_from_headers_optional(&headers)?,
            service_version: service_version_from_headers(&headers)?,
            activity_id: activity_id_from_headers(&headers)?,
            gateway_version: gateway_version_from_headers(&headers)?,
            date: date_from_headers(&headers)?,
        })
    }
}

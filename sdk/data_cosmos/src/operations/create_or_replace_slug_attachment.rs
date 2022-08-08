use crate::headers::from_headers::*;
use crate::prelude::*;
use crate::resources::Attachment;
use crate::ResourceQuota;
use azure_core::headers;
use azure_core::headers::{
    date_from_headers, etag_from_headers, session_token_from_headers, HeaderValue,
};
use azure_core::Method;
use azure_core::Response as HttpResponse;
use azure_core::SessionToken;
use azure_core::{content_type, prelude::*};
use bytes::Bytes;
use time::OffsetDateTime;

operation! {
    CreateOrReplaceSlugAttachment,
    client: AttachmentClient,
    is_create: bool,
    body: Bytes,
    ?if_match_condition: IfMatchCondition,
    ?consistency_level: ConsistencyLevel,
    ?content_type: String
}

impl CreateOrReplaceSlugAttachmentBuilder {
    pub fn into_future(self) -> CreateOrReplaceSlugAttachment {
        Box::pin(async move {
            let mut request = if self.is_create {
                self.client.attachments_request(Method::Post)
            } else {
                self.client.attachment_request(Method::Put)
            };

            request.insert_headers(&self.if_match_condition);
            if let Some(cl) = &self.consistency_level {
                request.insert_headers(cl);
            }

            crate::cosmos_entity::add_as_partition_key_header_serialized(
                self.client.document_client().partition_key_serialized(),
                &mut request,
            );
            let body = self.body;
            request.insert_header(
                headers::CONTENT_TYPE,
                match self.content_type {
                    Some(content_type) => HeaderValue::from(content_type),
                    None => content_type::TEXT_PLAIN,
                },
            );

            request.insert_header(
                "Slug",
                HeaderValue::from(self.client.attachment_name().to_string()),
            );
            request.insert_header(
                headers::CONTENT_LENGTH,
                HeaderValue::from(format!("{}", body.len())),
            );

            request.set_body(body);
            let response = self
                .client
                .pipeline()
                .send(
                    self.context.clone().insert(ResourceType::Attachments),
                    &mut request,
                )
                .await?;

            CreateOrReplaceSlugAttachmentResponse::try_from(response).await
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct CreateOrReplaceSlugAttachmentResponse {
    pub attachment: Attachment,
    pub max_media_storage_usage_mb: u64,
    pub media_storage_usage_mb: u64,
    pub last_change: OffsetDateTime,
    pub etag: String,
    pub resource_quota: Vec<ResourceQuota>,
    pub resource_usage: Vec<ResourceQuota>,
    pub lsn: u64,
    pub alt_content_path: String,
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
    pub date: OffsetDateTime,
}

impl CreateOrReplaceSlugAttachmentResponse {
    pub async fn try_from(response: HttpResponse) -> azure_core::Result<Self> {
        let (_status_code, headers, body) = response.deconstruct();
        let body = body.collect().await?;

        let attachment: Attachment = serde_json::from_slice(&body)?;

        Ok(Self {
            attachment,
            max_media_storage_usage_mb: max_media_storage_usage_mb_from_headers(&headers)?,
            media_storage_usage_mb: media_storage_usage_mb_from_headers(&headers)?,
            last_change: last_state_change_from_headers(&headers)?,
            etag: etag_from_headers(&headers)?,
            resource_quota: resource_quota_from_headers(&headers)?,
            resource_usage: resource_usage_from_headers(&headers)?,
            lsn: lsn_from_headers(&headers)?,
            alt_content_path: alt_content_path_from_headers(&headers)?,
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
            service_version: service_version_from_headers(&headers)?,
            activity_id: activity_id_from_headers(&headers)?,
            gateway_version: gateway_version_from_headers(&headers)?,
            date: date_from_headers(&headers)?,
        })
    }
}

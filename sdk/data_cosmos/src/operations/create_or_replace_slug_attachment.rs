use crate::headers::{from_headers::*, CommonHeaders};
use crate::prelude::*;
use crate::resources::Attachment;
use azure_core::headers;
use azure_core::headers::HeaderValue;
use azure_core::Method;
use azure_core::Response as HttpResponse;
use azure_core::{content_type, prelude::*};
use bytes::Bytes;

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
    pub common: CommonHeaders,
    pub max_media_storage_usage_mb: u64,
    pub media_storage_usage_mb: u64,
    pub quorum_acked_lsn: u64,
    pub current_write_quorum: u64,
    pub current_replica_set_size: u64,
    pub global_committed_lsn: u64,
    pub number_of_read_regions: u32,
    pub transport_request_id: u64,
    pub cosmos_llsn: u64,
    pub cosmos_quorum_acked_llsn: u64,
}

impl CreateOrReplaceSlugAttachmentResponse {
    pub async fn try_from(response: HttpResponse) -> azure_core::Result<Self> {
        let (_status_code, headers, body) = response.deconstruct();
        let body = body.collect().await?;

        let attachment: Attachment = serde_json::from_slice(&body)?;

        Ok(Self {
            attachment,
            common: CommonHeaders::try_from(&headers)?,
            max_media_storage_usage_mb: max_media_storage_usage_mb_from_headers(&headers)?,
            media_storage_usage_mb: media_storage_usage_mb_from_headers(&headers)?,
            quorum_acked_lsn: quorum_acked_lsn_from_headers(&headers)?,
            current_write_quorum: current_write_quorum_from_headers(&headers)?,
            current_replica_set_size: current_replica_set_size_from_headers(&headers)?,
            global_committed_lsn: global_committed_lsn_from_headers(&headers)?,
            number_of_read_regions: number_of_read_regions_from_headers(&headers)?,
            transport_request_id: transport_request_id_from_headers(&headers)?,
            cosmos_llsn: cosmos_llsn_from_headers(&headers)?,
            cosmos_quorum_acked_llsn: cosmos_quorum_acked_llsn_from_headers(&headers)?,
        })
    }
}

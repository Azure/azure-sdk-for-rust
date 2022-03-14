use crate::headers::from_headers::*;
use crate::prelude::*;
use crate::resources::document::IndexingDirective;
use crate::resources::Attachment;
use crate::ResourceQuota;
use azure_core::headers::{
    content_type_from_headers, etag_from_headers, session_token_from_headers,
};
use azure_core::SessionToken;
use azure_core::{collect_pinned_stream, prelude::*, Response as HttpResponse};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct GetAttachmentBuilder {
    client: AttachmentClient,
    if_match_condition: Option<IfMatchCondition>,
    consistency_level: Option<ConsistencyLevel>,
    context: Context,
}

impl GetAttachmentBuilder {
    pub(crate) fn new(client: AttachmentClient) -> Self {
        Self {
            client,
            if_match_condition: None,
            consistency_level: None,
            context: Context::new(),
        }
    }

    setters! {
        consistency_level: ConsistencyLevel => Some(consistency_level),
        if_match_condition: IfMatchCondition => Some(if_match_condition),
        context: Context => context,
    }

    pub fn into_future(self) -> GetAttachment {
        Box::pin(async move {
            let mut request = self
                .client
                .prepare_pipeline_with_attachment_name(http::Method::GET);

            azure_core::headers::add_optional_header2(&self.if_match_condition, &mut request)?;
            azure_core::headers::add_optional_header2(&self.consistency_level, &mut request)?;

            crate::cosmos_entity::add_as_partition_key_header_serialized2(
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

#[cfg(feature = "into_future")]
impl std::future::IntoFuture for GetAttachmentBuilder {
    type IntoFuture = GetAttachment;
    type Output = <GetAttachment as std::future::Future>::Output;
    fn into_future(self) -> Self::Future {
        Self::into_future(self)
    }
}

/// The future returned by calling `into_future` on the builder.
pub type GetAttachment = futures::future::BoxFuture<'static, crate::Result<GetAttachmentResponse>>;

#[derive(Debug, Clone, PartialEq)]
pub struct GetAttachmentResponse {
    pub attachment: Attachment,

    pub content_type: String,
    pub content_location: String,
    pub last_change: DateTime<Utc>,
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
    pub date: DateTime<Utc>,
}

impl GetAttachmentResponse {
    pub async fn try_from(response: HttpResponse) -> crate::Result<Self> {
        let (_status_code, headers, pinned_stream) = response.deconstruct();
        let body = collect_pinned_stream(pinned_stream).await?;

        Ok(Self {
            attachment: serde_json::from_slice(&body)?,
            content_type: content_type_from_headers(&headers)?.to_owned(),
            content_location: content_location_from_headers(&headers)?.to_owned(),
            last_change: last_state_change_from_headers(&headers)?,
            etag: etag_from_headers(&headers)?,
            resource_quota: resource_quota_from_headers(&headers)?,
            resource_usage: resource_usage_from_headers(&headers)?,
            lsn: lsn_from_headers(&headers)?,
            alt_content_path: alt_content_path_from_headers(&headers)?.to_owned(),
            content_path: content_path_from_headers(&headers)?.to_owned(),
            global_committed_lsn: global_committed_lsn_from_headers(&headers)?,
            number_of_read_regions: number_of_read_regions_from_headers(&headers)?,
            item_lsn: item_lsn_from_headers(&headers)?,
            transport_request_id: transport_request_id_from_headers(&headers)?,
            cosmos_llsn: cosmos_llsn_from_headers(&headers)?,
            cosmos_item_llsn: cosmos_item_llsn_from_headers(&headers)?,
            session_token: session_token_from_headers(&headers)?,
            request_charge: request_charge_from_headers(&headers)?,
            indexing_directive: indexing_directive_from_headers_optional(&headers)?,
            service_version: service_version_from_headers(&headers)?.to_owned(),
            activity_id: activity_id_from_headers(&headers)?,
            gateway_version: gateway_version_from_headers(&headers)?.to_owned(),
            date: date_from_headers(&headers)?,
        })
    }
}

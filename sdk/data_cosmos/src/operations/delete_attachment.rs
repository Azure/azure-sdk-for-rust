use crate::headers::from_headers::*;
use crate::headers::CommonHeaders;
use crate::prelude::*;

use azure_core::prelude::*;
use azure_core::Response as HttpResponse;
use time::OffsetDateTime;

operation! {
    DeleteAttachment,
    client: AttachmentClient,
    ?if_match_condition: IfMatchCondition,
    ?consistency_level: ConsistencyLevel
}

impl DeleteAttachmentBuilder {
    pub fn into_future(self) -> DeleteAttachment {
        Box::pin(async move {
            let mut request = self.client.attachment_request(azure_core::Method::Delete);

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

            DeleteAttachmentResponse::try_from(response).await
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct DeleteAttachmentResponse {
    pub common: CommonHeaders,
    pub max_media_storage_usage_mb: u64,
    pub media_storage_usage_mb: u64,
    pub last_change: OffsetDateTime,
    pub content_path: String,
    pub quorum_acked_lsn: u64,
    pub current_write_quorum: u64,
    pub current_replica_set_size: u64,
    pub global_committed_lsn: u64,
    pub number_of_read_regions: u32,
    pub transport_request_id: u64,
    pub cosmos_llsn: u64,
    pub cosmos_quorum_acked_llsn: u64,
}

impl DeleteAttachmentResponse {
    pub async fn try_from(response: HttpResponse) -> azure_core::Result<Self> {
        let headers = response.headers();

        Ok(Self {
            common: CommonHeaders::try_from(headers)?,
            max_media_storage_usage_mb: max_media_storage_usage_mb_from_headers(headers)?,
            media_storage_usage_mb: media_storage_usage_mb_from_headers(headers)?,
            last_change: last_state_change_from_headers(headers)?,
            content_path: content_path_from_headers(headers)?,
            quorum_acked_lsn: quorum_acked_lsn_from_headers(headers)?,
            current_write_quorum: current_write_quorum_from_headers(headers)?,
            current_replica_set_size: current_replica_set_size_from_headers(headers)?,
            global_committed_lsn: global_committed_lsn_from_headers(headers)?,
            number_of_read_regions: number_of_read_regions_from_headers(headers)?,
            transport_request_id: transport_request_id_from_headers(headers)?,
            cosmos_llsn: cosmos_llsn_from_headers(headers)?,
            cosmos_quorum_acked_llsn: cosmos_quorum_acked_llsn_from_headers(headers)?,
        })
    }
}

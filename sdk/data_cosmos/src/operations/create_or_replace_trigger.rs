use crate::headers::from_headers::*;
use crate::prelude::*;
use crate::resources::trigger::*;
use crate::resources::Trigger;
use crate::ResourceQuota;

use azure_core::headers::{etag_from_headers, session_token_from_headers};
use azure_core::Response as HttpResponse;
use time::OffsetDateTime;

operation! {
    CreateOrReplaceTrigger,
    client: TriggerClient,
    is_create: bool,
    body: String,
    trigger_type: TriggerType,
    trigger_operation: TriggerOperation,
    ?consistency_level: ConsistencyLevel
}

impl CreateOrReplaceTriggerBuilder {
    pub fn into_future(self) -> CreateOrReplaceTrigger {
        Box::pin(async move {
            let mut request = if self.is_create {
                self.client.triggers_request(azure_core::Method::Post)
            } else {
                self.client.trigger_request(azure_core::Method::Put)
            };

            if let Some(cl) = &self.consistency_level {
                request.insert_headers(cl);
            }

            #[derive(Debug, Deserialize, Serialize)]
            struct Request<'a> {
                pub id: &'a str,
                #[serde(rename = "triggerOperation")]
                pub trigger_operation: TriggerOperation,
                #[serde(rename = "triggerType")]
                pub trigger_type: TriggerType,
                pub body: &'a str,
            }

            let request_body = Request {
                id: self.client.trigger_name(),
                trigger_operation: self.trigger_operation,
                trigger_type: self.trigger_type,
                body: &self.body,
            };

            request.set_body(serde_json::to_vec(&request_body)?);
            let response = self
                .client
                .pipeline()
                .send(
                    self.context.clone().insert(ResourceType::Triggers),
                    &mut request,
                )
                .await?;

            CreateOrReplaceTriggerResponse::try_from(response).await
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct CreateOrReplaceTriggerResponse {
    pub trigger: Trigger,
    pub server: String,
    pub last_state_change: OffsetDateTime,
    pub etag: String,
    pub resource_quota: Vec<ResourceQuota>,
    pub resource_usage: Vec<ResourceQuota>,
    pub lsn: u64,
    pub schema_version: String,
    pub alt_content_path: String,
    pub content_path: String,
    pub quorum_acked_lsn: u64,
    pub current_write_quorum: u64,
    pub current_replica_set_size: u64,
    pub role: u32,
    pub global_committed_lsn: u64,
    pub number_of_read_regions: u32,
    pub transport_request_id: u64,
    pub cosmos_llsn: u64,
    pub cosmos_quorum_acked_llsn: u64,
    pub session_token: String,
    pub charge: f64,
    pub service_version: String,
    pub activity_id: uuid::Uuid,
    pub gateway_version: String,
    pub date: OffsetDateTime,
}

impl CreateOrReplaceTriggerResponse {
    pub async fn try_from(response: HttpResponse) -> azure_core::Result<Self> {
        let (_status_code, headers, body) = response.deconstruct();
        let body = body.collect().await?;

        Ok(Self {
            trigger: serde_json::from_slice(&body)?,
            server: server_from_headers(&headers)?,
            last_state_change: last_state_change_from_headers(&headers)?,
            etag: etag_from_headers(&headers)?,
            resource_quota: resource_quota_from_headers(&headers)?,
            resource_usage: resource_usage_from_headers(&headers)?,
            lsn: lsn_from_headers(&headers)?,
            schema_version: schema_version_from_headers(&headers)?,
            alt_content_path: alt_content_path_from_headers(&headers)?,
            content_path: content_path_from_headers(&headers)?,
            quorum_acked_lsn: quorum_acked_lsn_from_headers(&headers)?,
            current_write_quorum: current_write_quorum_from_headers(&headers)?,
            current_replica_set_size: current_replica_set_size_from_headers(&headers)?,
            role: role_from_headers(&headers)?,
            global_committed_lsn: global_committed_lsn_from_headers(&headers)?,
            number_of_read_regions: number_of_read_regions_from_headers(&headers)?,
            transport_request_id: transport_request_id_from_headers(&headers)?,
            cosmos_llsn: cosmos_llsn_from_headers(&headers)?,
            cosmos_quorum_acked_llsn: cosmos_quorum_acked_llsn_from_headers(&headers)?,
            session_token: session_token_from_headers(&headers)?,
            charge: request_charge_from_headers(&headers)?,
            service_version: service_version_from_headers(&headers)?,
            activity_id: activity_id_from_headers(&headers)?,
            gateway_version: gateway_version_from_headers(&headers)?,
            date: date_from_headers(&headers)?,
        })
    }
}

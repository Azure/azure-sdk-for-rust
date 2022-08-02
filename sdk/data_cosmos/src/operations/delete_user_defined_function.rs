use crate::headers::from_headers::*;
use crate::prelude::*;
use crate::ResourceQuota;

use azure_core::headers::session_token_from_headers;
use azure_core::Response as HttpResponse;
use time::OffsetDateTime;

operation! {
    DeleteUserDefinedFunction,
    client: UserDefinedFunctionClient,
    ?consistency_level: ConsistencyLevel
}

impl DeleteUserDefinedFunctionBuilder {
    pub fn into_future(self) -> DeleteUserDefinedFunction {
        Box::pin(async move {
            let mut request = self.client.udf_request(azure_core::Method::Delete);

            if let Some(cl) = &self.consistency_level {
                request.insert_headers(cl);
            }

            let response = self
                .client
                .pipeline()
                .send(
                    self.context
                        .clone()
                        .insert(ResourceType::UserDefinedFunctions),
                    &mut request,
                )
                .await?;

            DeleteUserDefinedFunctionResponse::try_from(response).await
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct DeleteUserDefinedFunctionResponse {
    pub content_location: String,
    pub server: String,
    pub last_state_change: OffsetDateTime,
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

impl DeleteUserDefinedFunctionResponse {
    pub async fn try_from(response: HttpResponse) -> azure_core::Result<Self> {
        let (_status_code, headers, _pinned_stream) = response.deconstruct();

        Ok(Self {
            content_location: content_location_from_headers(&headers)?,
            server: server_from_headers(&headers)?,
            last_state_change: last_state_change_from_headers(&headers)?,
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

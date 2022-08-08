use crate::headers::from_headers::*;
use crate::prelude::*;
use crate::resources::StoredProcedure;
use crate::ResourceQuota;
use azure_core::headers::{etag_from_headers, session_token_from_headers};
use azure_core::Response as HttpResponse;
use time::OffsetDateTime;

operation! {
    CreateStoredProcedure,
    client: StoredProcedureClient,
    function_body: String,
    ?consistency_level: ConsistencyLevel
}

impl CreateStoredProcedureBuilder {
    pub fn into_future(self) -> CreateStoredProcedure {
        Box::pin(async move {
            let mut req = self
                .client
                .stored_procedures_request(azure_core::Method::Post);

            if let Some(cl) = &self.consistency_level {
                req.insert_headers(cl);
            }

            #[derive(Debug, Serialize)]
            struct Request<'a> {
                body: &'a str,
                id: &'a str,
            }
            let body = Request {
                body: &self.function_body,
                id: self.client.stored_procedure_name(),
            };

            req.set_body(serde_json::to_vec(&body)?);

            let response = self
                .client
                .pipeline()
                .send(
                    self.context.clone().insert(ResourceType::StoredProcedures),
                    &mut req,
                )
                .await?;
            CreateStoredProcedureResponse::try_from(response).await
        })
    }
}

/// A stored procedure response
#[derive(Debug, Clone, PartialEq)]
pub struct CreateStoredProcedureResponse {
    pub stored_procedure: StoredProcedure,
    pub charge: f64,
    pub activity_id: uuid::Uuid,
    pub etag: String,
    pub session_token: String,
    pub last_change: OffsetDateTime,
    pub resource_quota: Vec<ResourceQuota>,
    pub resource_usage: Vec<ResourceQuota>,
    pub quorum_acked_lsn: u64,
    pub current_write_quorum: u64,
    pub current_replica_set_size: u64,
}

impl CreateStoredProcedureResponse {
    pub async fn try_from(response: HttpResponse) -> azure_core::Result<Self> {
        let (_status_code, headers, body) = response.deconstruct();
        let body = body.collect().await?;

        Ok(Self {
            stored_procedure: serde_json::from_slice(&body)?,
            charge: request_charge_from_headers(&headers)?,
            activity_id: activity_id_from_headers(&headers)?,
            etag: etag_from_headers(&headers)?,
            session_token: session_token_from_headers(&headers)?,
            last_change: last_state_change_from_headers(&headers)?,
            resource_quota: resource_quota_from_headers(&headers)?,
            resource_usage: resource_usage_from_headers(&headers)?,
            quorum_acked_lsn: quorum_acked_lsn_from_headers(&headers)?,
            current_write_quorum: current_write_quorum_from_headers(&headers)?,
            current_replica_set_size: current_replica_set_size_from_headers(&headers)?,
        })
    }
}

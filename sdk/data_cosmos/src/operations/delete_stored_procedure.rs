use crate::headers::from_headers::*;
use crate::prelude::*;
use crate::ResourceQuota;
use azure_core::headers::session_token_from_headers;
use azure_core::Response as HttpResponse;
use time::OffsetDateTime;

operation! {
    DeleteStoredProcedure,
    client: StoredProcedureClient,
    ?consistency_level: ConsistencyLevel
}

impl DeleteStoredProcedureBuilder {
    pub fn into_future(self) -> DeleteStoredProcedure {
        Box::pin(async move {
            let mut request = self
                .client
                .stored_procedure_request(azure_core::Method::Delete);

            if let Some(cl) = &self.consistency_level {
                request.insert_headers(cl);
            }

            let response = self
                .client
                .pipeline()
                .send(
                    self.context.clone().insert(ResourceType::StoredProcedures),
                    &mut request,
                )
                .await?;

            DeleteStoredProcedureResponse::try_from(response).await
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct DeleteStoredProcedureResponse {
    pub charge: f64,
    pub activity_id: uuid::Uuid,
    pub session_token: String,
    pub last_change: OffsetDateTime,
    pub resource_quota: Vec<ResourceQuota>,
    pub resource_usage: Vec<ResourceQuota>,
}

impl DeleteStoredProcedureResponse {
    pub async fn try_from(response: HttpResponse) -> azure_core::Result<Self> {
        let headers = response.headers();

        Ok(Self {
            charge: request_charge_from_headers(headers)?,
            activity_id: activity_id_from_headers(headers)?,
            session_token: session_token_from_headers(headers)?,
            last_change: last_state_change_from_headers(headers)?,
            resource_quota: resource_quota_from_headers(headers)?,
            resource_usage: resource_usage_from_headers(headers)?,
        })
    }
}

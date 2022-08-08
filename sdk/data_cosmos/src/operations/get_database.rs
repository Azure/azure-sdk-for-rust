use crate::headers::from_headers::*;
use crate::prelude::*;
use crate::ResourceQuota;

use azure_core::headers::{etag_from_headers, session_token_from_headers};
use azure_core::Response as HttpResponse;
use time::OffsetDateTime;

operation! {
    GetDatabase,
    client: DatabaseClient,
    ?consistency_level: ConsistencyLevel
}

impl GetDatabaseBuilder {
    pub fn into_future(self) -> GetDatabase {
        Box::pin(async move {
            let mut request = self.client.database_request(azure_core::Method::Get);
            if let Some(cl) = &self.consistency_level {
                request.insert_headers(cl);
            }

            let response = self
                .client
                .cosmos_client()
                .send(request, self.context.clone(), ResourceType::Databases)
                .await?;
            GetDatabaseResponse::try_from(response).await
        })
    }
}

#[derive(Debug, Clone)]
pub struct GetDatabaseResponse {
    pub database: Database,
    pub charge: f64,
    pub activity_id: uuid::Uuid,
    pub session_token: String,
    pub etag: String,
    pub last_state_change: OffsetDateTime,
    pub resource_quota: Vec<ResourceQuota>,
    pub resource_usage: Vec<ResourceQuota>,
    pub schema_version: String,
    pub service_version: String,
    pub gateway_version: String,
}

impl GetDatabaseResponse {
    pub async fn try_from(response: HttpResponse) -> azure_core::Result<Self> {
        let (_status_code, headers, body) = response.deconstruct();
        let body = body.collect().await?;

        Ok(Self {
            database: serde_json::from_slice(&body)?,
            charge: request_charge_from_headers(&headers)?,
            activity_id: activity_id_from_headers(&headers)?,
            session_token: session_token_from_headers(&headers)?,
            etag: etag_from_headers(&headers)?,
            last_state_change: last_state_change_from_headers(&headers)?,
            resource_quota: resource_quota_from_headers(&headers)?,
            resource_usage: resource_usage_from_headers(&headers)?,
            schema_version: schema_version_from_headers(&headers)?,
            service_version: service_version_from_headers(&headers)?,
            gateway_version: gateway_version_from_headers(&headers)?,
        })
    }
}

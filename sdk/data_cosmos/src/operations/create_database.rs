use crate::headers::from_headers::*;
use crate::prelude::*;
use crate::resources::Database;
use crate::ResourceQuota;
use azure_core::headers::{etag_from_headers, session_token_from_headers};
use azure_core::Response as HttpResponse;
use time::OffsetDateTime;

operation! {
    CreateDatabase,
    client: CosmosClient,
    database_name: String,
    ?consistency_level: ConsistencyLevel
}

impl CreateDatabaseBuilder {
    pub fn into_future(self) -> CreateDatabase {
        Box::pin(async move {
            let mut request = self.client.request("dbs", azure_core::Method::Post);

            #[derive(Serialize)]
            struct CreateDatabaseBody<'a> {
                pub id: &'a str,
            }
            let body = CreateDatabaseBody {
                id: self.database_name.as_str(),
            };

            if let Some(cl) = &self.consistency_level {
                request.insert_headers(cl);
            }
            request.set_body(serde_json::to_vec(&body)?);

            let response = self
                .client
                .send(request, self.context.clone(), ResourceType::Databases)
                .await?;
            CreateDatabaseResponse::try_from(response).await
        })
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct CreateDatabaseResponse {
    pub database: Database,
    pub charge: f64,
    pub etag: String,
    pub session_token: String,
    pub last_state_change: OffsetDateTime,
    pub resource_quota: Vec<ResourceQuota>,
    pub resource_usage: Vec<ResourceQuota>,
    pub quorum_acked_lsn: u64,
    pub current_write_quorum: u64,
    pub current_replica_set_size: u64,
    pub schema_version: String,
    pub service_version: String,
    pub activity_id: uuid::Uuid,
    pub gateway_version: String,
}

impl CreateDatabaseResponse {
    pub async fn try_from(response: HttpResponse) -> azure_core::Result<Self> {
        let (_status_code, headers, body) = response.deconstruct();
        let body = body.collect().await?;

        Ok(Self {
            database: serde_json::from_slice(&body)?,
            charge: request_charge_from_headers(&headers)?,
            etag: etag_from_headers(&headers)?,
            session_token: session_token_from_headers(&headers)?,
            last_state_change: last_state_change_from_headers(&headers)?,
            resource_quota: resource_quota_from_headers(&headers)?,
            resource_usage: resource_usage_from_headers(&headers)?,
            quorum_acked_lsn: quorum_acked_lsn_from_headers(&headers)?,
            current_write_quorum: current_write_quorum_from_headers(&headers)?,
            current_replica_set_size: current_replica_set_size_from_headers(&headers)?,
            schema_version: schema_version_from_headers(&headers)?,
            service_version: service_version_from_headers(&headers)?,
            activity_id: activity_id_from_headers(&headers)?,
            gateway_version: gateway_version_from_headers(&headers)?,
        })
    }
}

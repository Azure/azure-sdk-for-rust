use crate::headers::from_headers::*;
use crate::prelude::*;
use crate::ResourceQuota;
use azure_core::headers::session_token_from_headers;
use azure_core::Response as HttpResponse;

operation! {
    DeleteDatabase,
    client: DatabaseClient,
    ?consistency_level: ConsistencyLevel
}

impl DeleteDatabaseBuilder {
    pub fn into_future(self) -> DeleteDatabase {
        Box::pin(async move {
            let mut request = self.client.database_request(azure_core::Method::Delete);
            if let Some(cl) = &self.consistency_level {
                request.insert_headers(cl);
            }

            let response = self
                .client
                .cosmos_client()
                .send(request, self.context.clone(), ResourceType::Databases)
                .await?;
            DeleteDatabaseResponse::try_from(response).await
        })
    }
}

#[derive(Debug, Clone)]
pub struct DeleteDatabaseResponse {
    pub charge: f64,
    pub activity_id: uuid::Uuid,
    pub session_token: String,
    pub resource_quota: Vec<ResourceQuota>,
    pub resource_usage: Vec<ResourceQuota>,
}

impl DeleteDatabaseResponse {
    pub async fn try_from(response: HttpResponse) -> azure_core::Result<Self> {
        let headers = response.headers();

        let charge = request_charge_from_headers(headers)?;
        let activity_id = activity_id_from_headers(headers)?;

        Ok(Self {
            charge,
            activity_id,
            session_token: session_token_from_headers(headers)?,
            resource_quota: resource_quota_from_headers(headers)?,
            resource_usage: resource_usage_from_headers(headers)?,
        })
    }
}

use crate::headers::{from_headers::*, CommonHeaders};
use crate::prelude::*;
use crate::resources::Database;
use azure_core::Response as HttpResponse;

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

#[derive(Debug, Clone, PartialEq)]
pub struct CreateDatabaseResponse {
    pub database: Database,
    pub common: CommonHeaders,
    pub quorum_acked_lsn: u64,
    pub current_write_quorum: u64,
    pub current_replica_set_size: u64,
}

impl CreateDatabaseResponse {
    pub async fn try_from(response: HttpResponse) -> azure_core::Result<Self> {
        let (_status_code, headers, body) = response.deconstruct();
        let body = body.collect().await?;

        Ok(Self {
            database: serde_json::from_slice(&body)?,
            common: CommonHeaders::try_from(&headers)?,
            quorum_acked_lsn: quorum_acked_lsn_from_headers(&headers)?,
            current_write_quorum: current_write_quorum_from_headers(&headers)?,
            current_replica_set_size: current_replica_set_size_from_headers(&headers)?,
        })
    }
}

use crate::headers::from_headers::*;
use crate::prelude::*;
use crate::resources::Database;
use crate::ResourceQuota;
use azure_core::error::{ErrorKind, ResultExt};
use azure_core::headers::{etag_from_headers, session_token_from_headers};
use azure_core::{collect_pinned_stream, Context, Response as HttpResponse};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct CreateDatabaseBuilder {
    client: CosmosClient,
    database_name: String,
    consistency_level: Option<ConsistencyLevel>,
    context: Context,
}

impl CreateDatabaseBuilder {
    pub(crate) fn new(client: CosmosClient, database_name: String) -> Self {
        Self {
            client,
            database_name,
            consistency_level: None,
            context: Context::new(),
        }
    }

    setters! {
        consistency_level: ConsistencyLevel => Some(consistency_level),
        context: Context => context,
    }

    pub fn insert<E: Send + Sync + 'static>(&mut self, entity: E) -> &mut Self {
        self.context.insert(entity);
        self
    }

    pub fn into_future(self) -> CreateDatabase {
        Box::pin(async move {
            let mut request = self
                .client
                .prepare_request_pipeline("dbs", http::Method::POST);

            let body = CreateDatabaseBody {
                id: self.database_name.as_str(),
            };

            request.insert_headers(&self.consistency_level);
            request.set_body(bytes::Bytes::from(serde_json::to_string(&body)?).into());

            let response = self
                .client
                .send(request, self.context.clone(), ResourceType::Databases)
                .await?;
            CreateDatabaseResponse::try_from(response).await
        })
    }
}

#[cfg(feature = "into_future")]
impl std::future::IntoFuture for CreateDatabaseBuilder {
    type IntoFuture = CreateDatabase;
    type Output = <CreateDatabase as std::future::Future>::Output;
    fn into_future(self) -> Self::IntoFuture {
        Self::into_future(self)
    }
}

/// The future returned by calling `into_future` on the builder.
pub type CreateDatabase =
    futures::future::BoxFuture<'static, azure_core::error::Result<CreateDatabaseResponse>>;

#[derive(Serialize)]
struct CreateDatabaseBody<'a> {
    pub id: &'a str,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct CreateDatabaseResponse {
    pub database: Database,
    pub charge: f64,
    pub etag: String,
    pub session_token: String,
    pub last_state_change: DateTime<Utc>,
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
    pub async fn try_from(response: HttpResponse) -> azure_core::error::Result<Self> {
        let (_status_code, headers, pinned_stream) = response.deconstruct();
        let body: bytes::Bytes = collect_pinned_stream(pinned_stream).await.context(
            azure_core::error::ErrorKind::Io,
            "an error occurred fetching the next part of the byte stream",
        )?;

        let res = || {
            crate::Result::Ok(Self {
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
                schema_version: schema_version_from_headers(&headers)?.to_owned(),
                service_version: service_version_from_headers(&headers)?.to_owned(),
                activity_id: activity_id_from_headers(&headers)?,
                gateway_version: gateway_version_from_headers(&headers)?.to_owned(),
            })
        };

        res().context(
            ErrorKind::DataConversion,
            "error converting headers to CreateDatabaseResponse",
        )
    }
}

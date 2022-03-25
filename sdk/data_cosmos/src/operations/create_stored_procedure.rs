use crate::headers::from_headers::*;
use crate::prelude::*;
use crate::resources::StoredProcedure;
use crate::ResourceQuota;
use azure_core::headers::{etag_from_headers, session_token_from_headers};
use azure_core::{collect_pinned_stream, Context, Response as HttpResponse};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct CreateStoredProcedureBuilder {
    client: StoredProcedureClient,
    function_body: String,
    consistency_level: Option<ConsistencyLevel>,
    context: Context,
}

impl CreateStoredProcedureBuilder {
    pub(crate) fn new(client: StoredProcedureClient, body: String) -> Self {
        Self {
            client,
            function_body: body,
            consistency_level: None,
            context: Context::new(),
        }
    }

    setters! {
        consistency_level: ConsistencyLevel => Some(consistency_level),
        context: Context => context,
    }

    pub fn into_future(self) -> CreateStoredProcedure {
        Box::pin(async move {
            let mut req = self.client.prepare_request_pipeline(http::Method::POST);

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

            req.set_body(bytes::Bytes::from(serde_json::to_string(&body)?).into());

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

#[cfg(feature = "into_future")]
impl std::future::IntoFuture for CreateStoredProcedureBuilder {
    type IntoFuture = CreateStoredProcedure;
    type Output = <CreateStoredProcedure as std::future::Future>::Output;
    fn into_future(self) -> Self::IntoFuture {
        Self::into_future(self)
    }
}

/// The future returned by calling `into_future` on the builder.
pub type CreateStoredProcedure =
    futures::future::BoxFuture<'static, azure_core::error::Result<CreateStoredProcedureResponse>>;

/// A stored procedure response
#[derive(Debug, Clone, PartialEq)]
pub struct CreateStoredProcedureResponse {
    pub stored_procedure: StoredProcedure,
    pub charge: f64,
    pub activity_id: uuid::Uuid,
    pub etag: String,
    pub session_token: String,
    pub last_change: DateTime<Utc>,
    pub resource_quota: Vec<ResourceQuota>,
    pub resource_usage: Vec<ResourceQuota>,
    pub quorum_acked_lsn: u64,
    pub current_write_quorum: u64,
    pub current_replica_set_size: u64,
}

impl CreateStoredProcedureResponse {
    pub async fn try_from(response: HttpResponse) -> azure_core::error::Result<Self> {
        let (_status_code, headers, pinned_stream) = response.deconstruct();
        let body = collect_pinned_stream(pinned_stream).await?;

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

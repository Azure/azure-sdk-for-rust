use crate::headers::from_headers::*;
use crate::prelude::*;
use crate::ResourceQuota;
use azure_core::headers::session_token_from_headers;
use azure_core::prelude::*;
use azure_core::Response as HttpResponse;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct DeleteStoredProcedureBuilder {
    client: StoredProcedureClient,
    consistency_level: Option<ConsistencyLevel>,
    context: Context,
}

impl DeleteStoredProcedureBuilder {
    pub(crate) fn new(client: StoredProcedureClient) -> Self {
        Self {
            client,
            consistency_level: None,
            context: Context::new(),
        }
    }

    setters! {
        consistency_level: ConsistencyLevel => Some(consistency_level),
    }

    pub fn into_future(self) -> DeleteStoredProcedure {
        Box::pin(async move {
            let mut request = self
                .client
                .prepare_pipeline_with_stored_procedure_name(http::Method::DELETE);

            azure_core::headers::add_optional_header2(&self.consistency_level, &mut request)?;

            let response = self
                .client
                .pipeline()
                .send(
                    self.context.clone().insert(ResourceType::Permissions),
                    &mut request,
                )
                .await?;

            DeleteStoredProcedureResponse::try_from(response).await
        })
    }
}

/// The future returned by calling `into_future` on the builder.
pub type DeleteStoredProcedure =
    futures::future::BoxFuture<'static, crate::Result<DeleteStoredProcedureResponse>>;

impl std::future::IntoFuture for DeleteStoredProcedureBuilder {
    type IntoFuture = DeleteStoredProcedure;
    type Output = <DeleteStoredProcedure as std::future::Future>::Output;
    fn into_future(self) -> Self::IntoFuture {
        Self::into_future(self)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct DeleteStoredProcedureResponse {
    pub charge: f64,
    pub activity_id: uuid::Uuid,
    pub session_token: String,
    pub last_change: DateTime<Utc>,
    pub resource_quota: Vec<ResourceQuota>,
    pub resource_usage: Vec<ResourceQuota>,
}

impl DeleteStoredProcedureResponse {
    pub async fn try_from(response: HttpResponse) -> crate::Result<Self> {
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

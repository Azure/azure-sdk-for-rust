use crate::headers::from_headers::*;
use crate::prelude::*;
use crate::ResourceQuota;
use azure_core::headers::session_token_from_headers;
use azure_core::Context;
use azure_core::Response as HttpResponse;

#[derive(Debug, Clone)]
pub struct DeleteDatabaseBuilder {
    client: DatabaseClient,
    consistency_level: Option<ConsistencyLevel>,
    context: Context,
}

impl DeleteDatabaseBuilder {
    pub(crate) fn new(client: DatabaseClient) -> Self {
        Self {
            client,
            consistency_level: None,
            context: Context::new(),
        }
    }

    setters! {
        consistency_level: ConsistencyLevel => Some(consistency_level),
        context: Context => context,
    }

    pub fn into_future(self) -> DeleteDatabase {
        Box::pin(async move {
            let mut request = self.client.prepare_pipeline(http::Method::DELETE);
            azure_core::headers::add_optional_header2(&self.consistency_level, &mut request)?;

            let response = self
                .client
                .cosmos_client()
                .send(request, self.context.clone(), ResourceType::Databases)
                .await?;
            DeleteDatabaseResponse::try_from(response).await
        })
    }
}

/// The future returned by calling `into_future` on the builder.
pub type DeleteDatabase =
    futures::future::BoxFuture<'static, crate::Result<DeleteDatabaseResponse>>;

#[cfg(feature = "into_future")]
impl std::future::IntoFuture for DeleteDatabaseBuilder {
    type IntoFuture = DeleteDatabase;
    type Output = <DeleteDatabase as std::future::Future>::Output;
    fn into_future(self) -> Self::IntoFuture {
        Self::into_future(self)
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
    pub async fn try_from(response: HttpResponse) -> crate::Result<Self> {
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

use super::CreateStoredProcedureResponse;
use crate::prelude::*;
use azure_core::prelude::*;

#[derive(Debug, Clone)]
pub struct ReplaceStoredProcedureBuilder {
    client: StoredProcedureClient,
    function_body: String,
    consistency_level: Option<ConsistencyLevel>,
    context: Context,
}

impl ReplaceStoredProcedureBuilder {
    pub(crate) fn new(client: StoredProcedureClient, function_body: String) -> Self {
        Self {
            client,
            function_body,
            consistency_level: None,
            context: Context::new(),
        }
    }
}

impl ReplaceStoredProcedureBuilder {
    setters! {
        consistency_level: ConsistencyLevel => Some(consistency_level),
    }

    pub fn into_future(self) -> ReplaceStoredProcedure {
        Box::pin(async move {
            let mut req = self
                .client
                .prepare_pipeline_with_stored_procedure_name(http::Method::PUT);

            azure_core::headers::add_optional_header2(&self.consistency_level, &mut req)?;

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
            ReplaceStoredProcedureResponse::try_from(response).await
        })
    }
}

impl std::future::IntoFuture for ReplaceStoredProcedureBuilder {
    type IntoFuture = ReplaceStoredProcedure;
    type Output = <ReplaceStoredProcedure as std::future::Future>::Output;
    fn into_future(self) -> Self::IntoFuture {
        Self::into_future(self)
    }
}

/// The future returned by calling `into_future` on the builder.
pub type ReplaceStoredProcedure =
    futures::future::BoxFuture<'static, crate::Result<ReplaceStoredProcedureResponse>>;

pub type ReplaceStoredProcedureResponse = CreateStoredProcedureResponse;

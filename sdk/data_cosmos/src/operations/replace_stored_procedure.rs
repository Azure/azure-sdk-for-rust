use super::CreateStoredProcedureResponse;
use crate::prelude::*;

operation! {
    ReplaceStoredProcedure,
    client: StoredProcedureClient,
    function_body: String,
    ?consistency_level: ConsistencyLevel
}

impl ReplaceStoredProcedureBuilder {
    pub fn into_future(self) -> ReplaceStoredProcedure {
        Box::pin(async move {
            let mut req = self
                .client
                .stored_procedure_request(azure_core::Method::Put);

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

            req.set_body(serde_json::to_vec(&body)?);

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

pub type ReplaceStoredProcedureResponse = CreateStoredProcedureResponse;

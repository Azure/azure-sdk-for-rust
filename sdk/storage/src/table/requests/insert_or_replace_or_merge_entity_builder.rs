use crate::table::prelude::*;
use crate::table::responses::*;
use azure_core::headers::add_optional_header;
use azure_core::prelude::*;
use http::{method::Method, StatusCode};
use serde::Serialize;
use std::convert::TryInto;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Operation {
    InsertOrReplace,
    InsertOrMerge,
}

#[derive(Debug, Clone)]
pub struct InsertOrReplaceOrMergeEntityBuilder<'a> {
    entity_client: &'a EntityClient,
    operation: Operation,
    timeout: Option<Timeout>,
    client_request_id: Option<ClientRequestId<'a>>,
}

impl<'a> InsertOrReplaceOrMergeEntityBuilder<'a> {
    pub(crate) fn new(entity_client: &'a EntityClient, operation: Operation) -> Self {
        Self {
            entity_client,
            operation,
            timeout: None,
            client_request_id: None,
        }
    }

    setters! {
        timeout: Timeout => Some(timeout),
        client_request_id: ClientRequestId<'a> => Some(client_request_id),
    }

    pub async fn execute<E>(
        &self,
        entity: &E,
    ) -> Result<OperationOnEntityResponse, Box<dyn std::error::Error + Sync + Send>>
    where
        E: Serialize,
    {
        let url = self.entity_client.url();
        println!("url = {}", url);

        let request_body_serialized = serde_json::to_string(entity)?;
        println!("payload == {}", request_body_serialized);

        let request = self.entity_client.prepare_request(
            url.as_str(),
            match self.operation {
                Operation::InsertOrMerge => &crate::table::MERGE,
                Operation::InsertOrReplace => &Method::PUT,
            },
            &|mut request| {
                request = add_optional_header(&self.client_request_id, request);
                request = request.header("Content-Type", "application/json");
                request
            },
            Some(bytes::Bytes::from(request_body_serialized)),
        )?;

        println!("request == {:#?}\n", request);

        let response = self
            .entity_client
            .http_client()
            .execute_request_check_status(request.0, StatusCode::NO_CONTENT)
            .await?;

        Ok((&response).try_into()?)
    }
}

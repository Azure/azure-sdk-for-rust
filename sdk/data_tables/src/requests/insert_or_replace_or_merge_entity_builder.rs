use crate::{prelude::*, responses::*, TransactionOperation};
use azure_core::{headers::add_optional_header, prelude::*};
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
    client_request_id: Option<ClientRequestId>,
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
        client_request_id: ClientRequestId => Some(client_request_id),
    }

    pub async fn execute<E>(&self, entity: &E) -> azure_core::Result<OperationOnEntityResponse>
    where
        E: Serialize,
    {
        let mut url = self.entity_client.url().clone();

        self.timeout.append_to_url_query(&mut url);
        debug!("url = {}", url);

        let request_body_serialized = serde_json::to_string(entity)?;
        debug!("payload == {}", request_body_serialized);

        let request = self.entity_client.prepare_request(
            url.as_str(),
            match self.operation {
                Operation::InsertOrMerge => &crate::MERGE,
                Operation::InsertOrReplace => &Method::PUT,
            },
            &|mut request| {
                request = add_optional_header(&self.client_request_id, request);
                request = request.header("Content-Type", "application/json");
                request
            },
            Some(bytes::Bytes::from(request_body_serialized)),
        )?;

        debug!("request == {:#?}\n", request);

        let response = self
            .entity_client
            .http_client()
            .execute_request_check_status(request.0, StatusCode::NO_CONTENT)
            .await?;

        (&response).try_into()
    }

    pub fn to_transaction_operation<E>(
        &self,
        entity: &E,
    ) -> azure_core::Result<TransactionOperation>
    where
        E: Serialize,
    {
        let url = self.entity_client.url();

        let request = http::Request::builder()
            .method(match self.operation {
                Operation::InsertOrMerge => &crate::MERGE,
                Operation::InsertOrReplace => &Method::PUT,
            })
            .uri(url.as_str());
        let request = add_optional_header(&self.client_request_id, request);
        let request = request.header("Accept", "application/json;odata=fullmetadata");
        let request = request.header("Content-Type", "application/json");

        let request = request.body(serde_json::to_string(entity)?)?;

        Ok(TransactionOperation::new(request))
    }
}

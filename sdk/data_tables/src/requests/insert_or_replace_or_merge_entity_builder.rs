use crate::{prelude::*, responses::*, TransactionOperation};
use azure_core::{headers::*, prelude::*, Method, Request};
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

        let request_body_serialized = serde_json::to_string(entity)?;

        let mut headers = Headers::new();
        headers.add(self.client_request_id.clone());
        headers.insert(CONTENT_TYPE, "application/json");

        let request = self.entity_client.finalize_request(
            url,
            match self.operation {
                Operation::InsertOrMerge => Method::Merge,
                Operation::InsertOrReplace => Method::Put,
            },
            headers,
            Some(bytes::Bytes::from(request_body_serialized)),
        )?;

        let response = self
            .entity_client
            .http_client()
            .execute_request_check_status(&request)
            .await?;

        response.try_into()
    }

    pub fn to_transaction_operation<E>(
        &self,
        entity: &E,
    ) -> azure_core::Result<TransactionOperation>
    where
        E: Serialize,
    {
        let url = self.entity_client.url();

        let mut request = Request::new(
            url.clone(),
            match self.operation {
                Operation::InsertOrMerge => Method::Merge,
                Operation::InsertOrReplace => Method::Put,
            },
        );
        request.add_optional_header(&self.client_request_id);
        request.insert_header(ACCEPT, "application/json;odata=fullmetadata");
        request.insert_header(CONTENT_TYPE, "application/json");
        request.set_body(serde_json::to_vec(entity)?);

        Ok(TransactionOperation::new(request))
    }
}

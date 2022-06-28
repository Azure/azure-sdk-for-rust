use crate::{prelude::*, responses::*, IfMatchCondition, TransactionOperation};
use azure_core::{headers::*, prelude::*, Method, Request};
use serde::Serialize;
use std::convert::TryInto;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Operation {
    Update,
    Merge,
}

#[derive(Debug, Clone)]
pub struct UpdateOrMergeEntityBuilder<'a> {
    entity_client: &'a EntityClient,
    operation: Operation,
    #[allow(unused)]
    timeout: Option<Timeout>,
    client_request_id: Option<ClientRequestId>,
}

impl<'a> UpdateOrMergeEntityBuilder<'a> {
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

    pub async fn execute<E>(
        &self,
        entity: &E,
        if_match_condition: &IfMatchCondition,
    ) -> azure_core::Result<OperationOnEntityResponse>
    where
        E: Serialize,
    {
        let url = self.entity_client.url().clone();

        let request_body_serialized = serde_json::to_string(entity)?;

        let mut headers = Headers::new();
        headers.add(self.client_request_id.clone());
        headers.insert(CONTENT_TYPE, "application/json");
        headers.add(if_match_condition.clone());

        let request = self.entity_client.prepare_request(
            url,
            match self.operation {
                Operation::Merge => Method::Merge,
                Operation::Update => Method::Put,
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
        if_match_condition: &IfMatchCondition,
    ) -> azure_core::Result<TransactionOperation>
    where
        E: Serialize,
    {
        let url = self.entity_client.url();

        let mut request = Request::new(
            url.clone(),
            match self.operation {
                Operation::Merge => Method::Merge,
                Operation::Update => Method::Put,
            },
        );
        request.add_optional_header(&self.client_request_id);
        request.add_mandatory_header(if_match_condition);
        request.insert_header("Accept", "application/json;odata=fullmetadata");
        request.insert_header("Content-Type", "application/json");

        request.set_body(serde_json::to_vec(entity)?);

        Ok(TransactionOperation::new(request))
    }
}

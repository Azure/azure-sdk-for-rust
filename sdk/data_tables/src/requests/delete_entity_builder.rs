use crate::{
    prelude::{IfMatchCondition, *},
    responses::*,
    TransactionOperation,
};
use azure_core::{headers::*, prelude::*, Method, Request};
use std::convert::TryInto;

#[derive(Debug, Clone)]
pub struct DeleteEntityBuilder<'a> {
    entity_client: &'a EntityClient,
    if_match: IfMatchCondition,
    timeout: Option<Timeout>,
    client_request_id: Option<ClientRequestId>,
}

impl<'a> DeleteEntityBuilder<'a> {
    pub(crate) fn new(entity_client: &'a EntityClient) -> Self {
        Self {
            entity_client,
            if_match: IfMatchCondition::Any,
            timeout: None,
            client_request_id: None,
        }
    }

    setters! {
        if_match: IfMatchCondition => if_match,
        timeout: Timeout => Some(timeout),
        client_request_id: ClientRequestId => Some(client_request_id),
    }

    pub async fn execute(&self) -> azure_core::Result<DeleteEntityResponse> {
        let mut url = self.entity_client.url().clone();

        self.timeout.append_to_url_query(&mut url);

        let mut headers = Headers::new();
        headers.add(self.client_request_id.clone());
        headers.add(self.if_match.clone());

        let request = self
            .entity_client
            .finalize_request(url, Method::Delete, headers, None)?;

        let response = self
            .entity_client
            .http_client()
            .execute_request_check_status(&request)
            .await?;

        response.try_into()
    }

    pub fn to_transaction_operation(&self) -> azure_core::Result<TransactionOperation> {
        let url = self.entity_client.url();

        let mut request = Request::new(url.clone(), Method::Delete);
        request.add_optional_header(&self.client_request_id);
        request.insert_header(ACCEPT, "application/json;odata=minimalmetadata");
        request.insert_header(IF_MATCH, "*");
        request.set_body("");

        Ok(TransactionOperation::new(request))
    }
}

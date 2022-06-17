use crate::{
    prelude::{IfMatchCondition, *},
    responses::*,
    TransactionOperation,
};
use azure_core::{
    headers::{add_mandatory_header, add_optional_header},
    prelude::*,
};
use http::{method::Method, StatusCode};
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
        debug!("url = {}", url);

        let request = self.entity_client.prepare_request(
            url.as_str(),
            &Method::DELETE,
            &|mut request| {
                request = add_optional_header(&self.client_request_id, request);
                request = add_mandatory_header(&self.if_match, request);
                request
            },
            None,
        )?;

        debug!("request == {:#?}\n", request);

        let response = self
            .entity_client
            .http_client()
            .execute_request_check_status(request.0, StatusCode::NO_CONTENT)
            .await?;

        (&response).try_into()
    }

    pub fn to_transaction_operation(&self) -> azure_core::Result<TransactionOperation> {
        let url = self.entity_client.url();

        let request = http::Request::builder()
            .method(Method::DELETE)
            .uri(url.as_str());
        let request = add_optional_header(&self.client_request_id, request);
        let request = request.header("Accept", "application/json;odata=minimalmetadata");
        let request = request.header("If-Match", "*");

        let request = request.body("".to_owned())?;

        Ok(TransactionOperation::new(request))
    }
}

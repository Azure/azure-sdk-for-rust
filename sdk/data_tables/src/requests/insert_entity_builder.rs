use crate::prelude::*;
use crate::responses::*;
use crate::TransactionOperation;
use azure_core::headers::{add_mandatory_header, add_optional_header};
use azure_core::prelude::*;
use http::method::Method;
use serde::{de::DeserializeOwned, Serialize};
use std::convert::TryInto;

#[derive(Debug, Clone)]
pub struct InsertEntityBuilder<'a> {
    table_client: &'a TableClient,
    return_entity: ReturnEntity,
    timeout: Option<Timeout>,
    client_request_id: Option<ClientRequestId<'a>>,
}

impl<'a> InsertEntityBuilder<'a> {
    pub(crate) fn new(table_client: &'a TableClient) -> Self {
        Self {
            table_client,
            return_entity: false.into(),
            timeout: None,
            client_request_id: None,
        }
    }

    setters! {
        return_entity: ReturnEntity => return_entity,
        timeout: Timeout => Some(timeout),
        client_request_id: ClientRequestId<'a> => Some(client_request_id),
    }

    pub async fn execute<E>(
        &self,
        entity: &E,
    ) -> Result<InsertEntityResponse<E>, Box<dyn std::error::Error + Sync + Send>>
    where
        E: Serialize + DeserializeOwned,
    {
        let mut url = self.table_client.url().to_owned();
        url.path_segments_mut()
            .map_err(|_| "Invalid table URL")?
            .pop()
            .push(self.table_client.table_name());

        self.timeout.append_to_url_query(&mut url);
        debug!("url = {}", url);

        let request_body_serialized = serde_json::to_string(entity)?;
        debug!("payload == {}", request_body_serialized);

        let request = self.table_client.prepare_request(
            url.as_str(),
            &Method::POST,
            &|mut request| {
                request = add_optional_header(&self.client_request_id, request);
                request = add_mandatory_header(&self.return_entity, request);
                request = request.header("Accept", "application/json;odata=fullmetadata");
                request = request.header("Content-Type", "application/json");
                request
            },
            Some(bytes::Bytes::from(request_body_serialized)),
        )?;

        debug!("request == {:#?}\n", request);

        let response = self
            .table_client
            .http_client()
            .execute_request_check_status(request.0, self.return_entity.expected_return_code())
            .await?;

        Ok((&response).try_into()?)
    }

    pub fn to_transaction_operation<E>(
        &self,
        entity: &E,
    ) -> Result<TransactionOperation, Box<dyn std::error::Error + Send + Sync>>
    where
        E: Serialize,
    {
        let mut url = self.table_client.url().to_owned();
        url.path_segments_mut()
            .map_err(|_| "Invalid table URL")?
            .pop()
            .push(self.table_client.table_name());

        let request = http::Request::builder()
            .method(Method::POST)
            .uri(url.as_str());
        let request = add_optional_header(&self.client_request_id, request);
        let request = request.header("Accept", "application/json;odata=fullmetadata");
        let request = request.header("Content-Type", "application/json");

        let request = request.body(serde_json::to_string(entity)?)?;

        Ok(TransactionOperation::new(request))
    }
}

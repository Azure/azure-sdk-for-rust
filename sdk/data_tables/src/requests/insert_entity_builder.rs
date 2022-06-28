use crate::{prelude::*, responses::*, TransactionOperation};
use azure_core::Method;
use azure_core::{
    error::{Error, ErrorKind},
    prelude::*,
    Request,
};
use serde::{de::DeserializeOwned, Serialize};
use std::convert::TryInto;

#[derive(Debug, Clone)]
pub struct InsertEntityBuilder<'a> {
    table_client: &'a TableClient,
    return_entity: ReturnEntity,
    timeout: Option<Timeout>,
    client_request_id: Option<ClientRequestId>,
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
        client_request_id: ClientRequestId => Some(client_request_id),
    }

    pub async fn execute<E>(&self, entity: &E) -> azure_core::Result<InsertEntityResponse<E>>
    where
        E: Serialize + DeserializeOwned,
    {
        let mut url = self.table_client.url().to_owned();
        url.path_segments_mut()
            .map_err(|()| Error::message(ErrorKind::Other, "invalid table URL"))?
            .pop()
            .push(self.table_client.table_name());

        self.timeout.append_to_url_query(&mut url);

        let request_body_serialized = serde_json::to_string(entity)?;

        let mut request = self.table_client.prepare_request(
            url,
            Method::POST,
            Some(bytes::Bytes::from(request_body_serialized)),
        )?;
        request.add_optional_header(&self.client_request_id);
        request.add_mandatory_header(&self.return_entity);
        request.insert_header("Accept", "application/json;odata=fullmetadata");
        request.insert_header("Content-Type", "application/json");

        let response = self
            .table_client
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
        let mut url = self.table_client.url().to_owned();
        url.path_segments_mut()
            .map_err(|()| Error::message(ErrorKind::Other, "invalid table URL"))?
            .pop()
            .push(self.table_client.table_name());

        let mut request = Request::new(url, Method::POST);
        request.add_optional_header(&self.client_request_id);
        request.insert_header("Accept", "application/json;odata=fullmetadata");
        request.insert_header("Content-Type", "application/json");

        request.set_body(serde_json::to_vec(entity)?);

        Ok(TransactionOperation::new(request))
    }
}

use crate::{prelude::*, responses::*};
use azure_core::Method;
use azure_core::{
    error::{Error, ErrorKind},
    prelude::*,
};
use std::convert::TryInto;

#[derive(Debug, Clone)]
pub struct DeleteTableBuilder<'a> {
    table_client: &'a TableClient,
    client_request_id: Option<ClientRequestId>,
}

impl<'a> DeleteTableBuilder<'a> {
    pub(crate) fn new(table_client: &'a TableClient) -> Self {
        Self {
            table_client,
            client_request_id: None,
        }
    }

    setters! {
        client_request_id: ClientRequestId => Some(client_request_id),
    }

    pub async fn execute(&self) -> azure_core::Result<DeleteTableResponse> {
        let mut url = self.table_client.url().to_owned();
        url.path_segments_mut()
            .map_err(|()| Error::message(ErrorKind::Other, "invalid table URL"))?
            .pop()
            .push(&format!("Tables('{}')", self.table_client.table_name()));

        let mut request = self
            .table_client
            .prepare_request(url, Method::Delete, None)?;
        request.add_optional_header(&self.client_request_id);
        request.insert_header("Accept", "application/json");

        let response = self
            .table_client
            .http_client()
            .execute_request_check_status(&request)
            .await?;

        response.try_into()
    }
}

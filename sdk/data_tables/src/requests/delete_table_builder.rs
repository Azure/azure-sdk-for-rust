use crate::{prelude::*, responses::*};
use azure_core::{
    error::{Error, ErrorKind, Result},
    headers::add_optional_header,
    prelude::*,
};
use http::{method::Method, status::StatusCode};
use std::convert::TryInto;

#[cfg(test)]
use std::println as debug;

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

    pub async fn execute(&self) -> Result<DeleteTableResponse> {
        let mut url = self.table_client.url().to_owned();
        url.path_segments_mut()
            .map_err(|()| Error::message(ErrorKind::Other, "invalid table URL"))?
            .pop()
            .push(&format!("Tables('{}')", self.table_client.table_name()));
        debug!("url = {}", url);

        let request = self.table_client.prepare_request(
            url.as_str(),
            &Method::DELETE,
            &|mut request| {
                request = add_optional_header(&self.client_request_id, request);
                request = request.header("Accept", "application/json");
                request
            },
            None,
        )?;

        debug!("request == {:#?}\n", request);

        let response = self
            .table_client
            .http_client()
            .execute_request_check_status(request.0, StatusCode::NO_CONTENT)
            .await?;

        Ok((&response).try_into()?)
    }
}

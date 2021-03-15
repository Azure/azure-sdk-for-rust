use crate::table::prelude::*;
use crate::table::responses::*;
use azure_core::headers::add_optional_header;
use azure_core::prelude::*;
use http::method::Method;
use http::status::StatusCode;
use std::convert::TryInto;

#[derive(Debug, Clone)]
pub struct DeleteTableBuilder<'a> {
    table_client: &'a TableClient,
    client_request_id: Option<ClientRequestId<'a>>,
}

impl<'a> DeleteTableBuilder<'a> {
    pub(crate) fn new(table_client: &'a TableClient) -> Self {
        Self {
            table_client,
            client_request_id: None,
        }
    }

    setters! {
        client_request_id: ClientRequestId<'a> => Some(client_request_id),
    }

    pub async fn execute(
        &self,
    ) -> Result<DeleteTableResponse, Box<dyn std::error::Error + Sync + Send>> {
        let url = self
            .table_client
            .url()
            .join(&format!("/Tables('{}')", self.table_client.table_name()))?;
        debug!("url = {}", url);

        let request = self.table_client.prepare_request(
            url.as_str(),
            &Method::DELETE,
            &|mut request| {
                request = add_optional_header(&self.client_request_id, request);
                request = request.header("Content-Type", "application/json");
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

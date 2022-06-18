use crate::{prelude::*, responses::*};
use azure_core::prelude::*;
use http::method::Method;
use std::convert::TryInto;

#[derive(Debug, Clone)]
pub struct CreateTableBuilder<'a> {
    table_client: &'a TableClient,
    client_request_id: Option<ClientRequestId>,
}

impl<'a> CreateTableBuilder<'a> {
    pub(crate) fn new(table_client: &'a TableClient) -> Self {
        Self {
            table_client,
            client_request_id: None,
        }
    }

    setters! {
        client_request_id: ClientRequestId => Some(client_request_id),
    }

    pub async fn execute(&self) -> azure_core::Result<CreateTableResponse> {
        let url = self.table_client.url();

        #[derive(Debug, Clone, Serialize)]
        struct RequestBody<'a> {
            #[serde(rename = "TableName")]
            table_name: &'a str,
        }

        let request_body_serialized = serde_json::to_string(&RequestBody {
            table_name: self.table_client.table_name(),
        })?;

        let mut request = self.table_client.prepare_request(
            url.as_str(),
            Method::POST,
            Some(bytes::Bytes::from(request_body_serialized)),
        )?;
        request.add_optional_header(&self.client_request_id);
        request.insert_header("Accept", "application/json;odata=fullmetadata");
        request.insert_header("Content-Type", "application/json");
        request.insert_header("Prefer", "return-content");

        let response = self
            .table_client
            .execute_request_check_status(&request)
            .await?;

        response.try_into()
    }
}

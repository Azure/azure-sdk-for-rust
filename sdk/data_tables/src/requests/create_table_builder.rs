use crate::{prelude::*, responses::*};
use azure_core::{headers::*, prelude::*, Method};
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
        let url = self.table_client.url().clone();

        #[derive(Debug, Clone, Serialize)]
        struct RequestBody<'a> {
            #[serde(rename = "TableName")]
            table_name: &'a str,
        }

        let request_body_serialized = serde_json::to_string(&RequestBody {
            table_name: self.table_client.table_name(),
        })?;

        let mut headers = Headers::new();
        headers.add(self.client_request_id.clone());
        headers.insert(ACCEPT, "application/json;odata=fullmetadata");
        headers.insert(CONTENT_TYPE, "application/json");
        headers.insert(PREFER, "return-content");

        let request = self.table_client.finalize_request(
            url,
            Method::Post,
            headers,
            Some(bytes::Bytes::from(request_body_serialized)),
        )?;

        let response = self
            .table_client
            .http_client()
            .execute_request_check_status(&request)
            .await?;

        response.try_into()
    }
}

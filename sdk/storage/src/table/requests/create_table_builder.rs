use crate::table::prelude::*;
use crate::table::responses::*;
use azure_core::headers::add_optional_header;
use azure_core::prelude::*;
use http::method::Method;
use http::status::StatusCode;
use std::convert::TryInto;

#[derive(Debug, Clone)]
pub struct CreateTableBuilder<'a> {
    table_client: &'a TableClient,
    client_request_id: Option<ClientRequestId<'a>>,
}

impl<'a> CreateTableBuilder<'a> {
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
    ) -> Result<CreateTableResponse, Box<dyn std::error::Error + Sync + Send>> {
        let url = self.table_client.url();
        debug!("url = {}", url);

        #[derive(Debug, Clone, Serialize)]
        struct RequestBody<'a> {
            #[serde(rename = "TableName")]
            table_name: &'a str,
        }

        let request_body_serialized = serde_json::to_string(&RequestBody {
            table_name: self.table_client.table_name(),
        })?;
        debug!("payload == {}", request_body_serialized);

        let request = self.table_client.prepare_request(
            url.as_str(),
            &Method::POST,
            &|mut request| {
                request = add_optional_header(&self.client_request_id, request);
                request = request.header("Accept", "application/json;odata=fullmetadata");
                request = request.header("Content-Type", "application/json");
                request = request.header("Prefer", "return-content");
                request
            },
            Some(bytes::Bytes::from(request_body_serialized)),
        )?;

        debug!("request == {:#?}\n", request);

        let response = self
            .table_client
            .http_client()
            .execute_request_check_status(request.0, StatusCode::CREATED)
            .await?;

        Ok((&response).try_into()?)
    }
}

use crate::prelude::*;
use azure_core::{headers::*, prelude::*, to_json, Method, Response};
use azure_storage::headers::CommonStorageResponseHeaders;
use serde::Serialize;
use std::convert::TryInto;

operation! {
    CreateTable,
    client: TableClient,
}

impl CreateTableBuilder {
    pub fn into_future(mut self) -> CreateTable {
        Box::pin(async move {
            let url = self.client.url()?;

            #[derive(Debug, Clone, Serialize)]
            struct RequestBody<'a> {
                #[serde(rename = "TableName")]
                table_name: &'a str,
            }

            let body = to_json(&RequestBody {
                table_name: self.client.table_name(),
            })?
            .into();

            let mut headers = Headers::new();
            headers.insert(ACCEPT, "application/json;odata=fullmetadata");
            headers.add(ContentType::APPLICATION_JSON);
            headers.insert(PREFER, "return-content");

            let mut request =
                TableClient::finalize_request(url, Method::Post, headers, Some(body))?;

            let response = self.client.send(&mut self.context, &mut request).await?;

            CreateTableResponse::try_from(response).await
        })
    }
}

#[derive(Debug, Clone)]
pub struct CreateTableResponse {
    pub common_storage_response_headers: CommonStorageResponseHeaders,
    pub table: Table,
}

impl CreateTableResponse {
    async fn try_from(response: Response) -> azure_core::Result<Self> {
        let (_, headers, body) = response.deconstruct();
        Ok(CreateTableResponse {
            common_storage_response_headers: (&headers).try_into()?,
            table: body.json().await?,
        })
    }
}

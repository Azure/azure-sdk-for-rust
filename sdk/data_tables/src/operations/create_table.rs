use crate::prelude::*;
use azure_core::{collect_pinned_stream, Context, Method, Response};
use azure_storage::core::headers::CommonStorageResponseHeaders;
use std::convert::TryInto;

#[derive(Debug, Clone)]
pub struct CreateTableBuilder {
    table_client: TableClient,
    context: Context,
}

impl CreateTableBuilder {
    pub(crate) fn new(table_client: TableClient) -> Self {
        Self {
            table_client,
            context: Context::new(),
        }
    }

    setters! {
        context: Context => context,
    }

    pub fn into_future(mut self) -> FutureResponse {
        Box::pin(async move {
            let url = self.table_client.url().clone();

            #[derive(Debug, Clone, Serialize)]
            struct RequestBody<'a> {
                #[serde(rename = "TableName")]
                table_name: &'a str,
            }

            let body = serde_json::to_string(&RequestBody {
                table_name: self.table_client.table_name(),
            })?
            .into();

            let mut request = self
                .table_client
                .prepare_request(url, Method::POST, Some(body))?;
            request.insert_header("Accept", "application/json;odata=fullmetadata");
            request.insert_header("Content-Type", "application/json");
            request.insert_header("Prefer", "return-content");

            let response = self
                .table_client
                .send(&mut self.context, &mut request)
                .await?;

            CreateTableResponse::try_from(response).await
        })
    }
}

pub type FutureResponse =
    futures::future::BoxFuture<'static, azure_core::Result<CreateTableResponse>>;

#[cfg(feature = "into_future")]
impl std::future::IntoFuture for CreateTableBuilder {
    type IntoFuture = FutureResponse;
    type Output = <FutureResponse as std::future::Future>::Output;
    fn into_future(self) -> Self::IntoFuture {
        Self::into_future(self)
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
        let body = collect_pinned_stream(body).await?;

        Ok(CreateTableResponse {
            common_storage_response_headers: (&headers).try_into()?,
            table: serde_json::from_slice(&body)?,
        })
    }
}

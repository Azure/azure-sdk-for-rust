use crate::prelude::*;
use azure_core::{collect_pinned_stream, headers::*, Context, Method, Response};
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

            let mut headers = Headers::new();
            headers.insert(ACCEPT, "application/json;odata=fullmetadata");
            headers.insert(CONTENT_TYPE, "application/json");
            headers.insert(PREFER, "return-content");

            let mut request =
                self.table_client
                    .finalize_request(url, Method::Post, headers, Some(body))?;

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

use crate::{clients::table_service_client::TableServiceClient, operations::MINIMAL_METADATA};
use azure_core::{headers::HeaderName, setters, Context, Response};
use azure_storage::Result;

#[derive(Debug, Clone)]
pub struct CreateTableBuilder {
    client: TableServiceClient,
    table_name: String,
    context: Context,
}

type CreateTable = futures::future::BoxFuture<'static, Result<CreateTableResponse>>;

impl CreateTableBuilder {
    pub fn new(client: TableServiceClient, table_name: String) -> Self {
        Self {
            client,
            table_name,
            context: Context::new(),
        }
    }

    setters! {
        context: Context => context,
    }

    pub fn into_future(self) -> CreateTable {
        Box::pin(async move {
            let mut request = self
                .client
                .prepare_request_pipeline("Tables", http::Method::POST);

            let body = CreateTableRequest {
                table_name: self.table_name.as_str(),
            };
            let body = serde_json::to_string(&body).unwrap(); // todo - use ? instate of unwrap
            let body = bytes::Bytes::from(body);

            let headers = request.headers_mut();
            let body_hash = base64::encode(md5::compute(&body).as_ref());
            headers.insert(HeaderName::from("content-md5"), body_hash);
            headers.insert(HeaderName::from("content-length"), body.len().to_string());
            headers.insert(HeaderName::from("prefer"), "return-content");
            headers.insert(HeaderName::from("accept"), MINIMAL_METADATA);
            request.set_body(body);

            let response = self.client.send(request, self.context).await?;
            CreateTableResponse::try_from(response).await
        })
    }
}

#[derive(serde::Serialize)]
struct CreateTableRequest<'a> {
    #[serde(rename = "TableName")]
    table_name: &'a str,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct CreateTableResponse {
    #[serde(rename = "odata.metadata")]
    pub odata_metadata: String,

    #[serde(rename = "TableName")]
    pub table_name: String,
}

impl CreateTableResponse {
    pub async fn try_from(response: Response) -> azure_storage::Result<Self> {
        let body = response.into_body_string().await;
        Ok(serde_json::from_slice(body.as_bytes())?)
    }
}

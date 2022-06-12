use crate::clients::table_service_client::TableServiceClient;
use azure_core::{headers::HeaderName, setters, Context, Response};
use azure_storage::Result;

use super::MINIMAL_METADATA;

#[derive(Debug, Clone)]
pub struct DeleteTableBuilder {
    client: TableServiceClient,
    table_name: String,
    context: Context,
}

type DeleteTable = futures::future::BoxFuture<'static, Result<DeleteTableResponse>>;

impl DeleteTableBuilder {
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

    pub fn into_future(self) -> DeleteTable {
        Box::pin(async move {
            let mut request = self.client.prepare_request_pipeline(
                &format!("Tables('{}')", &self.table_name),
                http::Method::DELETE,
            );

            request
                .headers_mut()
                .insert(HeaderName::from("accept"), MINIMAL_METADATA);

            let response = self.client.send(request, self.context).await?;
            DeleteTableResponse::try_from(response).await
        })
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct DeleteTableResponse;

impl DeleteTableResponse {
    pub async fn try_from(_: Response) -> Result<Self> {
        Ok(DeleteTableResponse)
    }
}

use crate::clients::table_service_client::TableServiceClient;
use azure_core::{setters, Context, Response, EMPTY_BODY};
use azure_storage::Result;
use url::Url;

#[derive(Debug, Clone)]
pub struct DeleteTableBuilder<'a> {
    table_client: &'a TableServiceClient,
    table_name: String,
    context: Context,
    timeout: Option<i32>,
}

impl<'a> DeleteTableBuilder<'a> {
    pub fn new(table_client: &'a TableServiceClient, table_name: String) -> Self {
        Self {
            table_name,
            table_client,
            timeout: None,
            context: Context::new(),
        }
    }

    setters! {
        context: Context => context,
        timeout: i32 => Some(timeout),
    }

    pub fn into_future(&self) -> DeleteTable {
        let builder = self.table_client.pipeline_request(
            http::Method::DELETE,
            format!("Tables('{}')", &self.table_name).as_str(),
        );

        let uri = builder.uri_ref().unwrap().to_string();
        let mut request = builder
            .uri(self.path_and_query(&uri))
            .body(EMPTY_BODY)
            .unwrap()
            .into();

        let table_client = self.table_client.clone();
        let mut context = self.context.clone();

        Box::pin(async move {
            DeleteTableResponse::try_from(
                table_client
                    .pipeline()
                    .send(&mut context, &mut request)
                    .await?,
            )
            .await
        })
    }

    fn path_and_query(&self, uri: &str) -> String {
        let mut uri = Url::parse(uri).unwrap();
        if let Some(timeout) = self.timeout {
            uri.query_pairs_mut()
                .append_pair("timeout", &timeout.to_string());
        }
        uri.to_string()
    }
}

type DeleteTable = futures::future::BoxFuture<'static, Result<DeleteTableResponse>>;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct DeleteTableResponse;

impl DeleteTableResponse {
    pub async fn try_from(_: Response) -> Result<Self> {
        Ok(DeleteTableResponse)
    }
}

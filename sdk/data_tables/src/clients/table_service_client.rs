use crate::requests::ListTablesBuilder;
use azure_core::error::{ErrorKind, Result, ResultExt};
use azure_storage::core::clients::{StorageAccountClient, StorageClient};
use bytes::Bytes;
use http::{
    method::Method,
    request::{Builder, Request},
};
use std::sync::Arc;
use url::Url;

pub trait AsTableServiceClient {
    fn as_table_service_client(&self) -> Result<Arc<TableServiceClient>>;
}

impl AsTableServiceClient for Arc<StorageClient> {
    fn as_table_service_client(&self) -> Result<Arc<TableServiceClient>> {
        TableServiceClient::new(self.clone())
    }
}

#[derive(Debug, Clone)]
pub struct TableServiceClient {
    storage_client: Arc<StorageClient>,
    url: Url,
}

impl TableServiceClient {
    pub(crate) fn new(storage_client: Arc<StorageClient>) -> Result<Arc<Self>> {
        let mut url = storage_client
            .storage_account_client()
            .table_storage_url()
            .to_owned();
        url.path_segments_mut()
            .map_err(|_| url::ParseError::SetHostOnCannotBeABaseUrl)?
            .push("Tables");

        Ok(Arc::new(Self {
            storage_client,
            url,
        }))
    }

    pub fn list(&self) -> ListTablesBuilder {
        ListTablesBuilder::new(self)
    }

    pub(crate) fn url(&self) -> &Url {
        &self.url
    }

    pub(crate) fn storage_account_client(&self) -> &StorageAccountClient {
        self.storage_client.storage_account_client()
    }

    pub(crate) fn http_client(&self) -> &dyn azure_core::HttpClient {
        self.storage_client.http_client()
    }

    pub(crate) fn prepare_request(
        &self,
        url: &str,
        method: &Method,
        http_header_adder: &dyn Fn(Builder) -> Builder,
        request_body: Option<Bytes>,
    ) -> Result<(Request<Bytes>, url::Url)> {
        self.storage_client
            .storage_account_client()
            .prepare_request(
                url,
                method,
                http_header_adder,
                azure_storage::core::clients::ServiceType::Table,
                request_body,
            )
            .context(ErrorKind::Other, "failed to prepare request")
    }
}

#[cfg(test)]
#[cfg(feature = "test_integration")]
mod integration_tests {
    use super::*;
    use crate::{core::prelude::*, table::clients::AsTableClient};
    use futures::StreamExt;

    fn get_emulator_client() -> Arc<StorageClient> {
        StorageAccountClient::new_emulator_default().as_storage_client()
    }

    #[tokio::test]
    async fn test_list() {
        let storage_account = get_emulator_client();
        let table_client = storage_account
            .as_table_service_client()
            .expect("a table service client");

        println!("Create a table in the storage account");
        let table = table_client.as_table_client("TableServiceClientList");
        match table.create().execute().await {
            _ => {}
        }

        println!("Check that the table is listed correctly");
        let mut stream = Box::pin(table_client.list().stream());
        while let Some(result) = stream.next().await {
            let result = result.expect("the request should succeed");
            let has_table = result
                .tables
                .iter()
                .any(|t| t.name == "TableServiceClientList");
            assert!(has_table, "the table should be present in the tables list");
        }
    }
}

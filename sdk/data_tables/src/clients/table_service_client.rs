use crate::operations::ListTablesBuilder;
use azure_core::{headers::Headers, Body, Context, Method, Request, Response};
use azure_storage::clients::{ServiceType, StorageClient};
use url::Url;

use super::TableClient;

pub trait AsTableServiceClient {
    fn table_service_client(&self) -> azure_core::Result<TableServiceClient>;
}

impl AsTableServiceClient for StorageClient {
    fn table_service_client(&self) -> azure_core::Result<TableServiceClient> {
        TableServiceClient::new(self.clone())
    }
}

#[derive(Debug, Clone)]
pub struct TableServiceClient {
    storage_client: StorageClient,
    url: Url,
}

impl TableServiceClient {
    pub(crate) fn new(storage_client: StorageClient) -> azure_core::Result<Self> {
        let mut url = storage_client.table_storage_url().to_owned();
        url.path_segments_mut()
            .map_err(|_| url::ParseError::SetHostOnCannotBeABaseUrl)?
            .push("Tables");

        Ok(Self {
            storage_client,
            url,
        })
    }

    pub fn list(&self) -> ListTablesBuilder {
        ListTablesBuilder::new(self.clone())
    }

    pub(crate) fn url(&self) -> &Url {
        &self.url
    }

    pub(crate) fn storage_client(&self) -> &StorageClient {
        &self.storage_client
    }

    pub fn table_client<S: Into<String>>(&self, s: S) -> TableClient {
        TableClient::new(self.clone(), s)
    }

    pub(crate) fn finalize_request(
        &self,
        url: Url,
        method: Method,
        headers: Headers,
        request_body: Option<Body>,
    ) -> azure_core::Result<Request> {
        self.storage_client
            .finalize_request(url, method, headers, request_body)
    }

    pub(crate) async fn send(
        &self,
        context: &mut Context,
        request: &mut Request,
    ) -> azure_core::Result<Response> {
        self.storage_client
            .send(context, request, ServiceType::Table)
            .await
    }
}

#[cfg(test)]
#[cfg(feature = "test_integration")]
mod integration_tests {
    use super::*;
    use crate::{core::prelude::*, table::clients::AsTableClient};
    use futures::StreamExt;

    fn get_emulator_client() -> StorageClient {
        StorageClient::new_emulator_default()
    }

    #[tokio::test]
    async fn test_list() {
        let storage_account = get_emulator_client();
        let table_client = storage_account
            .table_service_client()
            .expect("a table service client");

        println!("Create a table in the storage account");
        let table = table_client.table_client("TableServiceClientList");
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

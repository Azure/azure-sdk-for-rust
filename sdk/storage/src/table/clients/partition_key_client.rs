use crate::table::prelude::*;
use crate::table::requests::*;
use crate::{core::clients::StorageAccountClient, AzureStorageError};
use bytes::Bytes;
use http::method::Method;
use http::request::{Builder, Request};
use std::sync::Arc;

pub trait AsPartitionKeyClient<PK: Into<String>> {
    fn as_partition_key_client(&self, partition_key: PK) -> Arc<PartitionKeyClient>;
}

impl<PK: Into<String>> AsPartitionKeyClient<PK> for Arc<TableClient> {
    fn as_partition_key_client(&self, partition_key: PK) -> Arc<PartitionKeyClient> {
        PartitionKeyClient::new(self.clone(), partition_key)
    }
}

#[derive(Debug, Clone)]
pub struct PartitionKeyClient {
    table_client: Arc<TableClient>,
    partition_key: String,
}

impl PartitionKeyClient {
    pub(crate) fn new<PK: Into<String>>(
        table_client: Arc<TableClient>,
        partition_key: PK,
    ) -> Arc<Self> {
        Arc::new(Self {
            table_client,
            partition_key: partition_key.into(),
        })
    }

    pub fn submit_transaction(&self) -> SubmitTransactionBuilder {
        SubmitTransactionBuilder::new(self)
    }

    pub fn partition_key(&self) -> &str {
        &self.partition_key
    }

    pub(crate) fn table_client(&self) -> &TableClient {
        &self.table_client
    }

    pub(crate) fn storage_account_client(&self) -> &StorageAccountClient {
        self.table_client.storage_account_client()
    }

    pub(crate) fn http_client(&self) -> &dyn azure_core::HttpClient {
        self.table_client.http_client()
    }

    pub(crate) fn prepare_request(
        &self,
        url: &str,
        method: &Method,
        http_header_adder: &dyn Fn(Builder) -> Builder,
        request_body: Option<Bytes>,
    ) -> Result<(Request<Bytes>, url::Url), AzureStorageError> {
        self.table_client
            .prepare_request(url, method, http_header_adder, request_body)
    }
}


#[cfg(test)]
#[cfg(feature = "test_integration")]
mod integration_tests {
    use super::*;
    use azure_core::prelude::*;
    use futures::StreamExt;
    use crate::{core::prelude::*, table::clients::{AsTableClient, AsTableServiceClient}};
    use url::Url;

    #[derive(Debug, Clone, Serialize, Deserialize)]
    struct TestEntity {
        #[serde(rename = "PartitionKey")]
        pub city: String,
        pub name: String,
        #[serde(rename = "RowKey")]
        pub surname: String,
    }

    fn get_emulator_client() -> Arc<TableServiceClient> {
        let blob_storage_url = Url::parse("http://127.0.0.1:10000").expect("the default local storage emulator URL");
        let table_storage_url = Url::parse("http://127.0.0.1:10002").expect("the default local storage emulator URL");

        let http_client: Arc<Box<dyn HttpClient>> = Arc::new(Box::new(reqwest::Client::new()));
        let storage_account =
            StorageAccountClient::new_emulator(http_client, &blob_storage_url, &table_storage_url)
                .as_storage_client();

        storage_account.as_table_service_client().expect("a table service client")
    }


}

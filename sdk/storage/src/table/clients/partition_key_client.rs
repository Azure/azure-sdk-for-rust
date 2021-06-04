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
    use crate::{
        core::prelude::*,
        table::clients::{AsTableClient, AsTableServiceClient},
    };
    use azure_core::prelude::*;
    use http::StatusCode;
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
        let blob_storage_url =
            Url::parse("http://127.0.0.1:10000").expect("the default local storage emulator URL");
        let queue_storage_url =
            Url::parse("http://127.0.0.1:10001").expect("the default local storage emulator URL");
        let table_storage_url =
            Url::parse("http://127.0.0.1:10002").expect("the default local storage emulator URL");
        let filesystem_url =
            Url::parse("http://127.0.0.1:10004").expect("the default local storage emulator URL");

        let http_client: Arc<dyn HttpClient> = Arc::new(reqwest::Client::new());
        let storage_account = StorageAccountClient::new_emulator(
            http_client,
            &blob_storage_url,
            &table_storage_url,
            &queue_storage_url,
            &filesystem_url,
        )
        .as_storage_client();

        storage_account
            .as_table_service_client()
            .expect("a table service client")
    }

    #[tokio::test]
    async fn test_transaction() {
        let table_service = get_emulator_client();
        let table = table_service.as_table_client("PartitionKeyClientTransaction");

        println!("Delete the table (if it exists)");
        match table.delete().execute().await {
            _ => {}
        }

        println!("Create the table");
        table
            .create()
            .execute()
            .await
            .expect("the table should be created");

        let partition_client = table.as_partition_key_client("Milan");

        println!("Create the transaction");
        let mut transaction = Transaction::default();

        let entity1 = TestEntity {
            city: partition_client.partition_key().to_owned(),
            name: "Francesco".to_owned(),
            surname: "Cogno".to_owned(),
        };

        let entity2 = TestEntity {
            city: partition_client.partition_key().to_owned(),
            name: "Francesco".to_owned(),
            surname: "Potter".to_owned(),
        };

        transaction
            .add(
                table
                    .insert()
                    .to_transaction_operation(&entity1)
                    .expect("a transaction operation"),
            )
            .add(
                table
                    .insert()
                    .to_transaction_operation(&entity2)
                    .expect("a transaction operation"),
            );

        let response = partition_client
            .submit_transaction()
            .execute(&transaction)
            .await
            .expect("the transaction to complete");
        for response in response.operation_responses {
            assert_eq!(
                response.status_code,
                StatusCode::CREATED,
                "each of the entities should be inserted"
            );
        }

        // TODO: Confirm that the entities were in fact inserted (and that the status codes aren't a lie)
    }
}

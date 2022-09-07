use crate::{operations::*, prelude::*, transaction::TransactionOperations};
use azure_core::{headers::Headers, Body, Context, Method, Request, Response, Url};

#[derive(Debug, Clone)]
pub struct PartitionKeyClient {
    table_client: TableClient,
    partition_key: String,
}

impl PartitionKeyClient {
    pub(crate) fn new<PK: Into<String>>(table_client: TableClient, partition_key: PK) -> Self {
        Self {
            table_client,
            partition_key: partition_key.into(),
        }
    }

    pub fn transaction(&self) -> TransactionBuilder {
        TransactionBuilder::new(self.clone(), TransactionOperations::new())
    }

    pub fn partition_key(&self) -> &str {
        &self.partition_key
    }

    pub fn entity_client<RK: Into<String>>(&self, row_key: RK) -> azure_core::Result<EntityClient> {
        EntityClient::new(self.clone(), row_key)
    }

    pub(crate) fn table_client(&self) -> &TableClient {
        &self.table_client
    }

    pub(crate) fn finalize_request(
        &self,
        url: Url,
        method: Method,
        headers: Headers,
        request_body: Option<Body>,
    ) -> azure_core::Result<Request> {
        self.table_client
            .finalize_request(url, method, headers, request_body)
    }

    pub(crate) async fn send(
        &self,
        context: &mut Context,
        request: &mut Request,
    ) -> azure_core::Result<Response> {
        self.table_client.send(context, request).await
    }

    pub(crate) fn url(&self) -> azure_core::Result<url::Url> {
        self.table_client.url()
    }
}

#[cfg(test)]
#[cfg(feature = "test_integration")]
mod integration_tests {
    use super::*;

    #[derive(Debug, Clone, Serialize, Deserialize)]
    struct TestEntity {
        #[serde(rename = "PartitionKey")]
        pub city: String,
        pub name: String,
        #[serde(rename = "RowKey")]
        pub surname: String,
    }

    fn get_emulator_client() -> TableServiceClient {
        crate::clients::TableServiceClientBuilder::emulator()
            .retry(azure_core::RetryOptions::none())
            .build()
    }

    #[ignore = "enable test once transactions are working in Azurite #297"]
    #[tokio::test]
    async fn test_transaction() {
        let table_service = get_emulator_client();
        let table = table_service.table_client("PartitionKeyClientTransaction");

        println!("Delete the table (if it exists)");
        let _ = table.delete().into_future().await;

        println!("Create the table");
        table
            .create()
            .into_future()
            .await
            .expect("the table should be created");

        let partition_client = table.partition_key_client("Milan");

        println!("Create the transaction");

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
        let transaction = partition_client.transaction();

        let response = transaction
            .insert(&entity1)
            .unwrap()
            .insert(&entity2)
            .unwrap()
            .into_future()
            .await
            .expect("transaction compete");

        for response in response.operation_responses {
            assert!(
                response.status_code.is_success(),
                "each of the entities should be inserted"
            );
        }

        // TODO: Confirm that the entities were in fact inserted (and that the status codes aren't a lie)
    }
}

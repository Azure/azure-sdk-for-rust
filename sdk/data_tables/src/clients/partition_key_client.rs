use crate::{operations::*, prelude::*};
use azure_core::{headers::Headers, Context, Method, Request, Response, Url};
use azure_storage::core::clients::StorageClient;
use bytes::Bytes;

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
        TransactionBuilder::new(self.clone())
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

    pub(crate) fn storage_client(&self) -> &StorageClient {
        self.table_client.storage_client()
    }

    pub(crate) fn finalize_request(
        &self,
        url: Url,
        method: Method,
        headers: Headers,
        request_body: Option<Bytes>,
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
}

#[cfg(test)]
#[cfg(feature = "test_integration")]
mod integration_tests {
    use super::*;
    use crate::{
        core::prelude::*,
        table::clients::{AsTableClient, AsTableServiceClient},
    };
    use http::StatusCode;

    #[derive(Debug, Clone, Serialize, Deserialize)]
    struct TestEntity {
        #[serde(rename = "PartitionKey")]
        pub city: String,
        pub name: String,
        #[serde(rename = "RowKey")]
        pub surname: String,
    }

    fn get_emulator_client() -> TableServiceClient {
        let storage_account = StorageClient::new_emulator_default();
        storage_account
            .table_service_client()
            .expect("a table service client")
    }

    #[ignore = "enable test once transactions are working in Azurite #297"]
    #[tokio::test]
    async fn test_transaction() {
        let table_service = get_emulator_client();
        let table = table_service.table_client("PartitionKeyClientTransaction");

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
        let mut transaction = partition_client.transaction();

        let response = transaction
            .insert(&entity1)
            .insert(&entity2)
            .into_future()
            .await
            .expect("transaction compete");

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

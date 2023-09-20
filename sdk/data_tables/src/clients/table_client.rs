use crate::{clients::*, operations::*};
use azure_core::{headers::Headers, Body, Context, Method, Request, Response, Url};
use serde::{de::DeserializeOwned, Serialize};

#[derive(Debug, Clone)]
pub struct TableClient {
    table_service_client: TableServiceClient,
    table_name: String,
}

impl TableClient {
    pub(crate) fn new<S: Into<String>>(table_service_client: TableServiceClient, s: S) -> Self {
        Self {
            table_service_client,
            table_name: s.into(),
        }
    }

    pub fn table_name(&self) -> &str {
        &self.table_name
    }

    pub fn create(&self) -> CreateTableBuilder {
        CreateTableBuilder::new(self.clone())
    }

    pub fn query(&self) -> QueryEntityBuilder {
        QueryEntityBuilder::new(self.clone())
    }

    pub fn delete(&self) -> DeleteTableBuilder {
        DeleteTableBuilder::new(self.clone())
    }

    pub fn insert<E: Serialize, R: DeserializeOwned + Send>(
        &self,
        entity: E,
    ) -> azure_core::Result<InsertEntityBuilder<R>> {
        let body = serde_json::to_string(&entity)?.into();
        Ok(InsertEntityBuilder::new(self.clone(), body))
    }

    pub(crate) fn url(&self) -> azure_core::Result<url::Url> {
        self.table_service_client.url()
    }

    pub fn partition_key_client<PK: Into<String>>(&self, partition_key: PK) -> PartitionKeyClient {
        PartitionKeyClient::new(self.clone(), partition_key)
    }

    pub(crate) async fn send(
        &self,
        context: &mut Context,
        request: &mut Request,
    ) -> azure_core::Result<Response> {
        self.table_service_client.send(context, request).await
    }

    pub(crate) fn finalize_request(
        url: Url,
        method: Method,
        headers: Headers,
        request_body: Option<Body>,
    ) -> azure_core::Result<Request> {
        TableServiceClient::finalize_request(url, method, headers, request_body)
    }
}

#[cfg(test)]
#[cfg(feature = "test_integration")]
mod integration_tests {
    use super::*;
    use futures::StreamExt;

    #[derive(Debug, Clone, Serialize, Deserialize)]
    struct TestEntity {
        #[serde(rename = "PartitionKey")]
        pub city: String,
        pub name: String,
        #[serde(rename = "RowKey")]
        pub surname: String,
    }

    fn get_emulator_client() -> TableServiceClient {
        TableServiceClientBuilder::emulator()
            .retry(azure_core::RetryOptions::none())
            .build()
    }

    #[tokio::test]
    async fn test_create_delete() {
        let table_client = get_emulator_client();
        let table = table_client.table_client("TableClientCreateDelete");

        assert_eq!(
            table.table_name(),
            "TableClientCreateDelete",
            "the table name should match what was provided"
        );

        println!("Create the table");
        let _ = table.create().await.unwrap();

        println!("Validate that the table was created");
        let mut stream = table_client.list().into_stream();
        while let Some(result) = stream.next().await {
            let result = result.expect("the request should succeed");

            let has_table = result
                .tables
                .iter()
                .any(|t| t.name == "TableClientCreateDelete");
            assert!(has_table, "the table should be present in the tables list");
        }

        println!("Delete the table");
        table
            .delete()
            .await
            .expect("we should be able to delete the table");

        println!("Validate that the table was deleted");
        let mut stream = table_client.list().into_stream();
        while let Some(result) = stream.next().await {
            let result = result.expect("the request should succeed");
            let has_table = result
                .tables
                .iter()
                .any(|t| t.name == "TableClientCreateDelete");
            assert!(
                !has_table,
                "the table should not be present in the tables list"
            );
        }
    }

    #[tokio::test]
    async fn test_insert() {
        let table_client = get_emulator_client();

        let table = table_client.table_client("TableClientInsert");
        assert_eq!(
            table.table_name(),
            "TableClientInsert",
            "the table name should match what was provided"
        );

        println!("Delete the table (if it exists)");
        let _ = table.delete().await;

        println!("Create the table");
        table.create().await.expect("the table should be created");

        let entity = TestEntity {
            city: "Milan".to_owned(),
            name: "Francesco".to_owned(),
            surname: "Cogno".to_owned(),
        };

        println!("Insert an entity into the table");
        let _: InsertEntityResponse<TestEntity> = table
            .insert(&entity)
            .unwrap()
            .return_entity(true)
            .await
            .expect("the insert operation should succeed");

        // TODO: Validate that the entity was inserted
    }
}

use crate::{prelude::*, requests::*};
use azure_core::{
    error::{Error, ErrorKind},
    Request,
};
use bytes::Bytes;
use http::method::Method;
use std::sync::Arc;
use url::Url;

pub trait AsEntityClient<RK: Into<String>> {
    fn as_entity_client(&self, row_key: RK) -> azure_core::Result<Arc<EntityClient>>;
}

impl<RK: Into<String>> AsEntityClient<RK> for Arc<PartitionKeyClient> {
    fn as_entity_client(&self, row_key: RK) -> azure_core::Result<Arc<EntityClient>> {
        EntityClient::new(self.clone(), row_key)
    }
}

#[derive(Debug, Clone)]
pub struct EntityClient {
    partition_key_client: Arc<PartitionKeyClient>,
    row_key: String,
    url: Url,
}

impl EntityClient {
    pub(crate) fn new<RK: Into<String>>(
        partition_key_client: Arc<PartitionKeyClient>,
        row_key: RK,
    ) -> azure_core::Result<Arc<Self>> {
        let row_key = row_key.into();
        let mut url = partition_key_client
            .storage_account_client()
            .table_storage_url()
            .to_owned();
        url.path_segments_mut()
            .map_err(|_e| {
                Error::message(
                    ErrorKind::DataConversion,
                    "failed to get path segments from EntityClient url",
                )
            })?
            .push(&format!(
                "{}(PartitionKey='{}',RowKey='{}')",
                partition_key_client.table_client().table_name(),
                partition_key_client.partition_key(),
                &row_key
            ));

        Ok(Arc::new(Self {
            partition_key_client,
            row_key,
            url,
        }))
    }

    pub fn row_key(&self) -> &str {
        &self.row_key
    }

    pub fn get(&self) -> GetEntityBuilder {
        GetEntityBuilder::new(self)
    }

    pub fn update(&self) -> UpdateOrMergeEntityBuilder {
        UpdateOrMergeEntityBuilder::new(self, update_or_merge_entity_builder::Operation::Update)
    }

    pub fn merge(&self) -> UpdateOrMergeEntityBuilder {
        UpdateOrMergeEntityBuilder::new(self, update_or_merge_entity_builder::Operation::Merge)
    }

    pub fn insert_or_replace(&self) -> InsertOrReplaceOrMergeEntityBuilder {
        InsertOrReplaceOrMergeEntityBuilder::new(
            self,
            insert_or_replace_or_merge_entity_builder::Operation::InsertOrReplace,
        )
    }

    pub fn insert_or_merge(&self) -> InsertOrReplaceOrMergeEntityBuilder {
        InsertOrReplaceOrMergeEntityBuilder::new(
            self,
            insert_or_replace_or_merge_entity_builder::Operation::InsertOrMerge,
        )
    }

    pub fn delete(&self) -> DeleteEntityBuilder {
        DeleteEntityBuilder::new(self)
    }

    pub(crate) fn url(&self) -> &Url {
        &self.url
    }

    pub(crate) fn http_client(&self) -> &dyn azure_core::HttpClient {
        self.partition_key_client.http_client()
    }

    pub(crate) fn prepare_request(
        &self,
        url: &str,
        method: Method,
        request_body: Option<Bytes>,
    ) -> azure_core::Result<Request> {
        self.partition_key_client
            .prepare_request(url, method, request_body)
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

    #[derive(Debug, Clone, Serialize, Deserialize)]
    struct TestEntity {
        #[serde(rename = "PartitionKey")]
        pub city: String,
        pub name: String,
        #[serde(rename = "RowKey")]
        pub surname: String,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    struct TestEntity2 {
        #[serde(rename = "PartitionKey")]
        pub city: String,
        #[serde(rename = "RowKey")]
        pub surname: String,
        pub country: String,
    }

    fn get_emulator_client() -> Arc<TableServiceClient> {
        let storage_account = StorageAccountClient::new_emulator_default().as_storage_client();
        storage_account
            .as_table_service_client()
            .expect("a table service client")
    }

    #[tokio::test]
    async fn test_update() {
        let table_client = get_emulator_client();

        let table = table_client.as_table_client("EntityClientUpdate");

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

        let entity = TestEntity {
            city: "Milan".to_owned(),
            name: "Francesco".to_owned(),
            surname: "Cogno".to_owned(),
        };

        let entity_client = table
            .as_partition_key_client(&entity.city)
            .as_entity_client(&entity.surname)
            .expect("an entity client");

        entity_client
            .update()
            .execute(&entity, &crate::table::IfMatchCondition::Any)
            .await
            .expect_err("the update should fail if the entity doesn't exist");

        table
            .insert()
            .execute(&entity)
            .await
            .expect("the entity should be inserted");

        // TODO: Confirm that the entity was inserted

        entity_client
            .update()
            .execute(&entity, &crate::table::IfMatchCondition::Any)
            .await
            .expect("the update operation should complete");

        // TODO: Confirm that the entity was updated
    }

    #[tokio::test]
    async fn test_merge() {
        let table_client = get_emulator_client();

        let table = table_client.as_table_client("EntityClientMerge");

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

        let entity = TestEntity {
            city: "Milan".to_owned(),
            name: "Francesco".to_owned(),
            surname: "Cogno".to_owned(),
        };

        let entity2 = TestEntity2 {
            city: "Milan".to_owned(),
            surname: "Cogno".to_owned(),
            country: "Italy".to_owned(),
        };

        let entity_client = table
            .as_partition_key_client(&entity.city)
            .as_entity_client(&entity.surname)
            .expect("an entity client");

        entity_client
            .merge()
            .execute(&entity2, &crate::table::IfMatchCondition::Any)
            .await
            .expect_err("the merge should fail if the entity doesn't exist");

        table
            .insert()
            .execute(&entity)
            .await
            .expect("the entity should be inserted");

        // TODO: Confirm that the entity was inserted

        entity_client
            .merge()
            .execute(&entity2, &crate::table::IfMatchCondition::Any)
            .await
            .expect("the merge operation should complete");

        // TODO: Confirm that the entity was updated with fields from entity2
    }

    #[tokio::test]
    async fn test_insert_or_replace() {
        let table_client = get_emulator_client();

        let table = table_client.as_table_client("EntityClientInsertOrReplace");

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

        let mut entity = TestEntity {
            city: "Milan".to_owned(),
            name: "Francesco".to_owned(),
            surname: "Cogno".to_owned(),
        };

        let entity_client = table
            .as_partition_key_client(&entity.city)
            .as_entity_client(&entity.surname)
            .expect("an entity client");
        entity_client
            .insert_or_replace()
            .execute(&entity)
            .await
            .expect("the insert or replace operation should complete");

        // TODO: Confirm that the entity was inserted

        entity.name = "Doe".to_owned();
        entity_client
            .insert_or_replace()
            .execute(&entity)
            .await
            .expect("the insert or replace operation should complete");

        // TODO: Confirm that the entity was updated
    }

    #[tokio::test]
    async fn test_insert_or_merge() {
        let table_client = get_emulator_client();

        let table = table_client.as_table_client("EntityClientInsertOrMerge");

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

        let mut entity = TestEntity {
            city: "Milan".to_owned(),
            name: "Francesco".to_owned(),
            surname: "Cogno".to_owned(),
        };

        let entity_client = table
            .as_partition_key_client(&entity.city)
            .as_entity_client(&entity.surname)
            .expect("an entity client");
        entity_client
            .insert_or_merge()
            .execute(&entity)
            .await
            .expect("the insert or replace operation should complete");

        // TODO: Confirm that the entity was inserted

        entity.name = "Doe".to_owned();
        entity_client
            .insert_or_merge()
            .execute(&entity)
            .await
            .expect("the insert or replace operation should complete");

        // TODO: Confirm that the entity was updated

        let entity2 = TestEntity2 {
            city: "Milan".to_owned(),
            surname: "Cogno".to_owned(),
            country: "Italy".to_owned(),
        };

        entity_client
            .insert_or_merge()
            .execute(&entity2)
            .await
            .expect("the insert or merge operation should complete");

        // TODO: Confirm that the entity was merged
    }
}

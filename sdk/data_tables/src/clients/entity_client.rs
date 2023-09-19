use crate::{operations::*, prelude::*};
use azure_core::{headers::Headers, Body, Context, Method, Request, Response};
use serde::{de::DeserializeOwned, Serialize};
use url::Url;

#[derive(Debug, Clone)]
pub struct EntityClient {
    partition_key_client: PartitionKeyClient,
    row_key: String,
}

impl EntityClient {
    pub(crate) fn new<RK: Into<String>>(
        partition_key_client: PartitionKeyClient,
        row_key: RK,
    ) -> Self {
        let row_key = row_key.into();

        Self {
            partition_key_client,
            row_key,
        }
    }

    pub fn row_key(&self) -> &str {
        &self.row_key
    }

    pub fn get<T: DeserializeOwned + Send>(&self) -> GetEntityBuilder<T> {
        GetEntityBuilder::new(self.clone())
    }

    pub fn update<E: Serialize>(
        &self,
        entity: E,
        if_match_condition: IfMatchCondition,
    ) -> azure_core::Result<UpdateOrMergeEntityBuilder> {
        let body = serde_json::to_string(&entity)?.into();
        Ok(UpdateOrMergeEntityBuilder::new(
            self.clone(),
            body,
            if_match_condition,
            UpdateOperation::Update,
        ))
    }

    pub fn merge<E: Serialize>(
        &self,
        entity: E,
        if_match_condition: IfMatchCondition,
    ) -> azure_core::Result<UpdateOrMergeEntityBuilder> {
        let body = serde_json::to_string(&entity)?.into();
        Ok(UpdateOrMergeEntityBuilder::new(
            self.clone(),
            body,
            if_match_condition,
            UpdateOperation::Merge,
        ))
    }

    pub fn insert_or_replace<E: Serialize>(
        &self,
        entity: E,
    ) -> azure_core::Result<InsertOrReplaceOrMergeEntityBuilder> {
        let body = serde_json::to_string(&entity)?.into();
        Ok(InsertOrReplaceOrMergeEntityBuilder::new(
            self.clone(),
            body,
            InsertOperation::InsertOrReplace,
        ))
    }

    pub fn insert_or_merge<E: Serialize>(
        &self,
        entity: E,
    ) -> azure_core::Result<InsertOrReplaceOrMergeEntityBuilder> {
        let body = serde_json::to_string(&entity)?.into();
        Ok(InsertOrReplaceOrMergeEntityBuilder::new(
            self.clone(),
            body,
            InsertOperation::InsertOrMerge,
        ))
    }

    pub fn delete(&self) -> DeleteEntityBuilder {
        DeleteEntityBuilder::new(self.clone())
    }

    pub(crate) fn url(&self) -> azure_core::Result<Url> {
        let mut url = self.partition_key_client.url()?;
        url.path_segments_mut()
            // This cannot fail since the url is guranteed to not be a relative url
            .unwrap()
            .pop()
            .push(&format!(
                "{}(PartitionKey='{}',RowKey='{}')",
                self.partition_key_client.table_client().table_name(),
                self.partition_key_client.partition_key(),
                &self.row_key
            ));
        Ok(url)
    }

    pub(crate) fn finalize_request(
        url: Url,
        method: Method,
        headers: Headers,
        request_body: Option<Body>,
    ) -> azure_core::Result<Request> {
        PartitionKeyClient::finalize_request(url, method, headers, request_body)
    }

    pub(crate) async fn send(
        &self,
        context: &mut Context,
        request: &mut Request,
    ) -> azure_core::Result<Response> {
        self.partition_key_client.send(context, request).await
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

    #[derive(Debug, Clone, Serialize, Deserialize)]
    struct TestEntity2 {
        #[serde(rename = "PartitionKey")]
        pub city: String,
        #[serde(rename = "RowKey")]
        pub surname: String,
        pub country: String,
    }

    fn get_emulator_client() -> TableServiceClient {
        crate::clients::TableServiceClientBuilder::emulator()
            .retry(azure_core::RetryOptions::none())
            .build()
    }

    #[tokio::test]
    async fn test_update() {
        let table_client = get_emulator_client();

        let table = table_client.table_client("EntityClientUpdate");

        println!("Delete the table (if it exists)");
        let _ = table.delete().await;

        println!("Create the table");
        table.create().await.expect("the table should be created");

        let entity = TestEntity {
            city: "Milan".to_owned(),
            name: "Francesco".to_owned(),
            surname: "Cogno".to_owned(),
        };

        let entity_client = table
            .partition_key_client(&entity.city)
            .entity_client(&entity.surname);

        entity_client
            .update(&entity, IfMatchCondition::Any)
            .expect("entity could not be serialized")
            .await
            .expect_err("the update should fail if the entity doesn't exist");

        table
            .insert::<&TestEntity, TestEntity>(&entity)
            .expect("entity could not be serialized")
            .await
            .expect("the entity should be inserted");

        // TODO: Confirm that the entity was inserted

        entity_client
            .update(&entity, IfMatchCondition::Any)
            .expect("entity could not be serialized")
            .await
            .expect("the update operation should complete");

        // TODO: Confirm that the entity was updated
    }

    #[tokio::test]
    async fn test_merge() {
        let table_client = get_emulator_client();

        let table = table_client.table_client("EntityClientMerge");

        println!("Delete the table (if it exists)");
        let _ = table.delete().await;

        println!("Create the table");
        table.create().await.expect("the table should be created");

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
            .partition_key_client(&entity.city)
            .entity_client(&entity.surname);

        entity_client
            .merge(&entity2, IfMatchCondition::Any)
            .expect("entity could not be serialized")
            .await
            .expect_err("the merge should fail if the entity doesn't exist");

        table
            .insert::<&TestEntity, TestEntity>(&entity)
            .expect("entity could not be serialized")
            .await
            .expect("the entity should be inserted");

        // TODO: Confirm that the entity was inserted

        entity_client
            .merge(&entity2, IfMatchCondition::Any)
            .expect("entity could not be serialized")
            .await
            .expect("the merge operation should complete");

        // TODO: Confirm that the entity was updated with fields from entity2
    }

    #[tokio::test]
    async fn test_insert_or_replace() {
        let table_client = get_emulator_client();

        let table = table_client.table_client("EntityClientInsertOrReplace");

        println!("Delete the table (if it exists)");
        let _ = table.delete().await;

        println!("Create the table");
        table.create().await.expect("the table should be created");

        let mut entity = TestEntity {
            city: "Milan".to_owned(),
            name: "Francesco".to_owned(),
            surname: "Cogno".to_owned(),
        };

        let entity_client = table
            .partition_key_client(&entity.city)
            .entity_client(&entity.surname);
        entity_client
            .insert_or_replace(&entity)
            .expect("entity could not be serialized")
            .await
            .expect("the insert or replace operation should complete");

        // TODO: Confirm that the entity was inserted

        entity.name = "Doe".to_owned();
        entity_client
            .insert_or_replace(&entity)
            .expect("entity could not be serialized")
            .await
            .expect("the insert or replace operation should complete");

        // TODO: Confirm that the entity was updated
    }

    #[tokio::test]
    async fn test_insert_or_merge() {
        let table_client = get_emulator_client();

        let table = table_client.table_client("EntityClientInsertOrMerge");

        println!("Delete the table (if it exists)");
        let _ = table.delete().await;

        println!("Create the table");
        table.create().await.expect("the table should be created");

        let mut entity = TestEntity {
            city: "Milan".to_owned(),
            name: "Francesco".to_owned(),
            surname: "Cogno".to_owned(),
        };

        let entity_client = table
            .partition_key_client(&entity.city)
            .entity_client(&entity.surname);
        entity_client
            .insert_or_merge(&entity)
            .expect("entity could not be serialized")
            .await
            .expect("the insert or replace operation should complete");

        // TODO: Confirm that the entity was inserted

        entity.name = "Doe".to_owned();
        entity_client
            .insert_or_merge(&entity)
            .expect("entity could not be serialized")
            .await
            .expect("the insert or replace operation should complete");

        // TODO: Confirm that the entity was updated

        let entity2 = TestEntity2 {
            city: "Milan".to_owned(),
            surname: "Cogno".to_owned(),
            country: "Italy".to_owned(),
        };

        entity_client
            .insert_or_merge(&entity2)
            .expect("entity could not be serialized")
            .await
            .expect("the insert or merge operation should complete");

        // TODO: Confirm that the entity was merged
    }
}

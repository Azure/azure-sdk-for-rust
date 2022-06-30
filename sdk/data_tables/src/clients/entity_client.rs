use crate::{operations::*, prelude::*};
use azure_core::{
    error::{Error, ErrorKind},
    headers::Headers,
    Context, Method, Request, Response,
};
use bytes::Bytes;
use serde::{de::DeserializeOwned, Serialize};
use std::sync::Arc;
use url::Url;

pub trait AsEntityClient<RK: Into<String>> {
    fn entity_client(&self, row_key: RK) -> azure_core::Result<Arc<EntityClient>>;
}

impl<RK: Into<String>> AsEntityClient<RK> for Arc<PartitionKeyClient> {
    fn entity_client(&self, row_key: RK) -> azure_core::Result<Arc<EntityClient>> {
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

    pub(crate) fn url(&self) -> &Url {
        &self.url
    }

    pub(crate) fn finalize_request(
        &self,
        url: Url,
        method: Method,
        headers: Headers,
        request_body: Option<Bytes>,
    ) -> azure_core::Result<Request> {
        self.partition_key_client
            .finalize_request(url, method, headers, request_body)
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
        let storage_account = StorageAccountClient::new_emulator_default().storage_client();
        storage_account
            .table_service_client()
            .expect("a table service client")
    }

    #[tokio::test]
    async fn test_update() {
        let table_client = get_emulator_client();

        let table = table_client.table_client("EntityClientUpdate");

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
            .partition_key_client(&entity.city)
            .entity_client(&entity.surname)
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

        let table = table_client.table_client("EntityClientMerge");

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
            .partition_key_client(&entity.city)
            .entity_client(&entity.surname)
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

        let table = table_client.table_client("EntityClientInsertOrReplace");

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
            .partition_key_client(&entity.city)
            .entity_client(&entity.surname)
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

        let table = table_client.table_client("EntityClientInsertOrMerge");

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
            .partition_key_client(&entity.city)
            .entity_client(&entity.surname)
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

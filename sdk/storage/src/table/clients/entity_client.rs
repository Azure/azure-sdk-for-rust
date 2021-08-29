use crate::operations::get_entity::GetEntityOptions;
use crate::operations::get_entity::TableEntity;
use crate::table::prelude::*;
use crate::table::requests::*;
use crate::table_context::TableContext;
use azure_core::Context;
use azure_core::Error;
use azure_core::PipelineContext;
use bytes::Bytes;
use http::method::Method;
use http::request::{Builder, Request};
use std::borrow::Cow;
use std::sync::Arc;
use url::Url;

use super::table_client::PipelineTableClient;

pub trait AsEntityClient<RK: Into<String>> {
    fn as_entity_client(&self, row_key: RK) -> Result<Arc<EntityClient>, url::ParseError>;
}

impl<RK: Into<String>> AsEntityClient<RK> for Arc<PartitionKeyClient> {
    fn as_entity_client(&self, row_key: RK) -> Result<Arc<EntityClient>, url::ParseError> {
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
    ) -> Result<Arc<Self>, url::ParseError> {
        let row_key = row_key.into();
        let mut url = partition_key_client
            .storage_account_client()
            .table_storage_url()
            .to_owned();
        url.path_segments_mut()
            .map_err(|_| url::ParseError::SetHostOnCannotBeABaseUrl)?
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
        method: &Method,
        http_header_adder: &dyn Fn(Builder) -> Builder,
        request_body: Option<Bytes>,
    ) -> Result<(Request<Bytes>, url::Url), crate::Error> {
        //
        self.partition_key_client
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

pub struct PipelineEntityClient {
    table_name: Cow<'static, str>,
    table_client: PipelineTableClient,
}

impl PipelineEntityClient {
    pub fn new<S: Into<Cow<'static, str>>>(
        table_client: PipelineTableClient,
        table_name: S,
    ) -> Self {
        Self {
            table_client,
            table_name: table_name.into(),
        }
    }

    pub async fn get_entity(
        &self,
        ctx: Context,
        partition_key: &str,
        row_key: &str,
        options: GetEntityOptions<'_>,
    ) -> Result<TableEntity, Error> {
        let mut request = self.table_client.prepare_pipeline_request(
            format!(
                "{}(PartitionKey='{}',RowKey='{}')",
                self.table_name, partition_key, row_key
            )
            .as_str(),
            Method::GET,
        );

        options.decorate_request(&mut request)?;

        let table_context = TableContext::default();
        let mut pipeline_context = PipelineContext::new(ctx, table_context);

        let response = self
            .table_client
            .pipeline()
            .send(&mut pipeline_context, &mut request)
            .await?
            .validate(http::StatusCode::OK)
            .await?;

        Ok(TableEntity::try_from(response).await?)
    }
}

#[cfg(test)]
pub mod test_pipeline_table_client {
    use super::PipelineTableClient;
    use crate::{
        operations::{
            create_table::CreateTableOptions, delete_table::DeleteTableOptions,
            get_entity::GetEntityOptions, list_tables::ListTablesOptions, OdataMetadataLevel,
        },
        table::clients::table_client::TableOptions,
        Filter, Top,
    };
    use azure_core::Context;

    #[tokio::test]
    async fn get_entity_test() {
        let email_table_client = emulator_table_client()
            .into_entity_client("emails")
            .get_entity(
                Context::new(),
                "shay@gmail.com",
                "2021-08-03T03:41:54.221695200Z",
                GetEntityOptions::default(),
            )
            .await;

        println!("{:#?}", email_table_client);
    }

    fn emulator_table_client() -> PipelineTableClient {
        PipelineTableClient::emulator(TableOptions::default())
    }
}

use super::table_client::TableClient;
use crate::table::operations::entity::delete_entity::DeleteEntityOptions;
use crate::table::operations::entity::get_entity::QueryEntityOptions;
use crate::table::operations::entity::insert_entity::InsertEntityOptions;
use crate::table::operations::entity::insert_or_merge_entity::InsertOrMergeEntityOptions;
use crate::table::operations::entity::insert_or_replace_entity::InsertOrReplaceEntityOptions;
use crate::table::operations::entity::merge_entity::MergeEntityOptions;
use crate::table::operations::entity::update_entity::UpdateEntityOptions;
use crate::table::operations::entity::EntityResponse;
use crate::table::operations::entity::TableEntity;
use crate::table::table_context::TableContext;
use azure_core::Context;
use azure_core::Error;
use azure_core::PipelineContext;
use bytes::Buf;
use http::method::Method;
use once_cell::sync::Lazy;
use serde::de::DeserializeOwned;
use std::borrow::Cow;

// we need this since the http::Method does not have the MERGE verb. The unwrap is safe here.
static MERGE: Lazy<http::Method> = Lazy::new(|| http::Method::from_bytes(b"MERGE").unwrap());

pub struct EntityClient {
    table_name: Cow<'static, str>,
    table_client: TableClient,
}

/// Operations for working with entities in the Table service.
///  * Query Entities
///  * Insert Entity
///  * Update Entity
///  * Merge Entity
///  * Delete Entity
///  * Insert Or Replace Entity
///  * Insert Or Merge Entity
///
/// Both the PartitionKey and RowKey values must be string values; each key value may be up to 64 KiB in size.
/// If you are using an integer value for the key value, you should convert the integer to a fixed-width string, because they are canonically sorted.
/// For example, you should convert the value 1 to 0000001 to ensure proper sorting.
///
impl EntityClient {
    /// Creates a new EntityClient for the given table_name.
    /// Consuming table_client in the process.
    pub fn new<NAME: Into<Cow<'static, str>>>(table_client: TableClient, table_name: NAME) -> Self {
        Self {
            table_client,
            table_name: table_name.into(),
        }
    }

    /// The Query Entities operation queries entities in a table and includes the $filter and $select options.
    /// A query against the Table service may return a maximum of 1,000 entities at one time and may execute for a maximum of five seconds.
    /// If the result set contains more than 1,000 entities, another request will be created.
    pub async fn query_entity<'a, E: serde::Serialize + DeserializeOwned + TableEntity<'a>>(
        &self,
        ctx: Context,
        partition_key: &str,
        row_key: &str,
        options: QueryEntityOptions,
    ) -> Result<EntityResponse<E>, Error> {
        let mut request = self.table_client.prepare_table_request(
            format!(
                "{}(PartitionKey='{}',RowKey='{}')",
                &self.table_name, partition_key, row_key
            )
            .as_str(),
            Method::GET,
        );

        options.decorate_request_headers(&mut request)?;

        let table_context = TableContext::default();
        let mut pipeline_context = PipelineContext::new(ctx, table_context);

        let response = self
            .table_client
            .pipeline()
            .send(&mut pipeline_context, &mut request)
            .await?
            .validate(http::StatusCode::OK)
            .await?;

        let body_bytes = azure_core::collect_pinned_stream(response.deconstruct().2).await?;
        let entity = serde_json::de::from_reader(body_bytes.reader())?;
        Ok(entity)
    }

    /// The Insert Entity operation inserts a new entity into a table.
    pub async fn insert_entity<'a, E: serde::Serialize + DeserializeOwned + TableEntity<'a>>(
        &self,
        ctx: Context,
        entity: &'a E,
        options: InsertEntityOptions,
    ) -> Result<EntityResponse<E>, Error> {
        let mut request = self
            .table_client
            .prepare_table_request(format!("{}", self.table_name).as_str(), Method::POST);

        options.decorate_request::<E>(&mut request, entity)?;

        let table_context = TableContext::default();
        let mut pipeline_context = PipelineContext::new(ctx, table_context);

        let response = self
            .table_client
            .pipeline()
            .send(&mut pipeline_context, &mut request)
            .await?
            .validate(options.expected_status_code())
            .await?;

        let (_, _, body) = response.deconstruct();
        let body_bytes = azure_core::collect_pinned_stream(body).await?;
        let response = serde_json::de::from_reader(body_bytes.reader())?;
        Ok(response)
    }

    /// TODO: write test, example, read remarks in documentation.
    /// The Update Entity operation updates an existing entity in a table.
    /// The Update Entity operation replaces the entire entity and can be used to remove properties.
    /// If the If-Match header is missing from the request, the service performs an Insert Or Replace Entity (upsert) operation.
    pub async fn update_entity<'a, E: serde::Serialize + TableEntity<'a>>(
        &self,
        ctx: Context,
        entity: &'a E,
        options: UpdateEntityOptions,
    ) -> Result<(), Error> {
        let mut request = self.table_client.prepare_table_request(
            format!(
                "{}(PartitionKey='{}', RowKey='{}')",
                self.table_name,
                entity.partition_key(),
                entity.row_key()
            )
            .as_str(),
            Method::PUT,
        );

        options.decorate_request::<E>(entity, &mut request)?;

        let table_context = TableContext::default();
        let mut pipeline_context = PipelineContext::new(ctx, table_context);

        let _ = self
            .table_client
            .pipeline()
            .send(&mut pipeline_context, &mut request)
            .await?
            .validate(http::StatusCode::NO_CONTENT)
            .await?;

        Ok(())
    }

    /// The Merge Entity operation updates an existing entity by updating the entity's properties.
    /// This operation does not replace the existing entity, as the Update Entity operation does.
    ///
    /// The Table service does not persist null values for properties.
    /// Specifying a property with a null value is equivalent to omitting that property in the request.
    /// Only properties with non-null values will be updated by the Merge Entity operation.
    /// Property cannot be removed with a Merge Entity operation. To remove a property from an entity, replace the entity by calling the Update Entity operation.
    pub async fn merge_entity<'a, E: serde::Serialize + TableEntity<'a>>(
        &self,
        ctx: Context,
        entity: &'a E,
        options: MergeEntityOptions,
    ) -> Result<(), Error> {
        let mut request = self.table_client.prepare_table_request(
            format!(
                "{}(PartitionKey='{}', RowKey='{}')",
                self.table_name,
                entity.partition_key(),
                entity.row_key()
            )
            .as_str(),
            MERGE.clone(),
        );

        options.decorate_request::<E>(&mut request, &entity)?;

        let table_context = TableContext::default();
        let mut pipeline_context = PipelineContext::new(ctx, table_context);

        let _ = self
            .table_client
            .pipeline()
            .send(&mut pipeline_context, &mut request)
            .await?
            .validate(http::StatusCode::NO_CONTENT)
            .await?;

        Ok(())
    }

    /// TODO: write test, example, read remarks in documentation.
    /// The Delete Entity operation deletes an existing entity in a table.
    /// When an entity is successfully deleted, the entity is immediately marked for deletion and is no longer accessible to clients.
    /// The entity is later removed from the Table service during garbage collection.
    pub async fn delete_entity(
        &self,
        ctx: Context,
        partition_key: &str,
        row_key: &str,
        options: DeleteEntityOptions,
    ) -> Result<(), Error> {
        let mut request = self.table_client.prepare_table_request(
            format!(
                "{}(PartitionKey='{}', RowKey='{}')",
                self.table_name, partition_key, row_key
            )
            .as_str(),
            Method::DELETE,
        );

        options.decorate_request(&mut request)?;
        let table_context = TableContext::default();
        let mut pipeline_context = PipelineContext::new(ctx, table_context);

        let _ = self
            .table_client
            .pipeline()
            .send(&mut pipeline_context, &mut request)
            .await?
            .validate(http::StatusCode::NO_CONTENT)
            .await?;

        Ok(())
    }

    /// TODO: add implementation, write test, example, read remarks in documentation.
    /// The Insert Or Replace Entity operation replaces an existing entity or inserts a new entity if it does not exist in the table.
    pub async fn insert_or_replace_entity<'a, E: serde::Serialize + TableEntity<'a>>(
        &self,
        ctx: Context,
        entity: &'a E,
        options: InsertOrReplaceEntityOptions,
    ) -> Result<(), Error> {
        let mut request = self.table_client.prepare_table_request(
            format!(
                "{}(PartitionKey='{}', RowKey='{}')",
                self.table_name,
                entity.partition_key(),
                entity.row_key()
            )
            .as_str(),
            Method::PUT,
        );

        options.decorate_request::<E>(entity, &mut request)?;

        let table_context = TableContext::default();
        let mut pipeline_context = PipelineContext::new(ctx, table_context);

        let _ = self
            .table_client
            .pipeline()
            .send(&mut pipeline_context, &mut request)
            .await?
            .validate(http::StatusCode::NO_CONTENT)
            .await?;

        Ok(())
    }

    /// TODO: add implementation, write test, example, read remarks in documentation.
    /// The Insert Or Merge Entity operation updates an existing entity or inserts a new entity if it does not exist in the table.
    pub async fn insert_or_merge_entity<'a, E: serde::Serialize + TableEntity<'a>>(
        &self,
        ctx: Context,
        entity: &'a E,
        options: InsertOrMergeEntityOptions,
    ) -> Result<(), Error> {
        let mut request = self.table_client.prepare_table_request(
            format!(
                "{}(PartitionKey='{}', RowKey='{}')",
                self.table_name,
                entity.partition_key(),
                entity.row_key()
            )
            .as_str(),
            MERGE.clone(),
        );

        options.decorate_request::<E>(&mut request, entity)?;

        let table_context = TableContext::default();
        let mut pipeline_context = PipelineContext::new(ctx, table_context);

        let _ = self
            .table_client
            .pipeline()
            .send(&mut pipeline_context, &mut request)
            .await?
            .validate(http::StatusCode::NO_CONTENT)
            .await?;

        Ok(())
    }
}

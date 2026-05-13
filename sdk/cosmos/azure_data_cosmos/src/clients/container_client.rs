// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use crate::{
    clients::{offers_client, ClientContext},
    models::{
        BatchResponse, ContainerProperties, ItemResponse, ResourceResponse, ThroughputProperties,
    },
    options::{
        BatchOptions, Precondition, QueryOptions, ReadContainerOptions, ReadFeedRangesOptions,
        SessionToken,
    },
    query::QueryScope,
    transactional_batch::TransactionalBatch,
    DeleteContainerOptions, FeedItemIterator, FeedRange, ItemReadOptions, ItemWriteOptions,
    PartitionKey, Query, ReplaceContainerOptions, ThroughputOptions,
};

use super::ThroughputPoller;
use azure_data_cosmos_driver::models::{
    ContainerReference, CosmosOperation, ItemReference, PartitionKeyKind,
};
use azure_data_cosmos_driver::options::OperationOptions;
use serde::{de::DeserializeOwned, Serialize};

/// A client for working with a specific container in a Cosmos DB account.
///
/// You can get a `Container` by calling [`DatabaseClient::container_client()`](crate::clients::DatabaseClient::container_client()).
#[derive(Clone)]
pub struct ContainerClient {
    container_ref: ContainerReference,
    context: ClientContext,
}

impl ContainerClient {
    pub(crate) async fn new(
        context: ClientContext,
        container_id: &str,
        database_id: &str,
    ) -> azure_core::Result<Self> {
        // Eagerly resolve immutable container metadata from the driver.
        let container_ref = context
            .driver
            .resolve_container(database_id, container_id)
            .await
            .map_err(|e| {
                e.with_context(format!(
                    "failed to resolve container metadata for '{database_id}/{container_id}'"
                ))
            })?;

        Ok(Self {
            container_ref,
            context,
        })
    }

    /// Reads the properties of the container.
    ///
    /// # Arguments
    ///
    /// * `options` - Optional parameters for the request.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # async fn doc() -> Result<(), Box<dyn std::error::Error>> {
    /// # let container_client: azure_data_cosmos::clients::ContainerClient = panic!("this is a non-running example");
    /// let response = container_client.read(None)
    ///     .await?
    ///     .into_model()?;
    /// # }
    /// ```
    pub async fn read(
        &self,
        #[allow(
            unused_variables,
            reason = "The 'options' parameter may be used in the future"
        )]
        options: Option<ReadContainerOptions>,
    ) -> azure_core::Result<ResourceResponse<ContainerProperties>> {
        let operation = CosmosOperation::read_container(self.container_ref.clone());

        let driver_response = self
            .context
            .driver
            .execute_point_operation(operation, OperationOptions::default())
            .await?;

        Ok(ResourceResponse::new(
            crate::driver_bridge::driver_response_to_cosmos_response(driver_response),
        ))
    }

    /// Updates the indexing policy of the container.
    ///
    /// **NOTE**: The [`ContainerProperties::id`] and [`ContainerProperties::partition_key`] must be the same as the existing container, they cannot be changed.
    ///
    /// # Arguments
    ///
    /// * `properties` - The [`ContainerProperties`] to update the container with.
    /// * `options` - Optional parameters for the request.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # async fn doc() -> Result<(), Box<dyn std::error::Error>> {
    /// use azure_data_cosmos::models::{ContainerProperties, IndexingPolicy};
    /// # let container_client: azure_data_cosmos::clients::ContainerClient = panic!("this is a non-running example");
    /// let indexing_policy = IndexingPolicy::default().with_included_path("/index_me");
    /// let new_properties = ContainerProperties::new("MyContainer", "/id".into())
    ///     .with_indexing_policy(indexing_policy);
    /// let response = container_client.replace(new_properties, None)
    ///     .await?
    ///     .into_model()?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn replace(
        &self,
        properties: ContainerProperties,
        #[allow(
            unused_variables,
            reason = "The 'options' parameter may be used in the future"
        )]
        options: Option<ReplaceContainerOptions>,
    ) -> azure_core::Result<ResourceResponse<ContainerProperties>> {
        let body = serde_json::to_vec(&properties)?;
        let operation =
            CosmosOperation::replace_container(self.container_ref.clone()).with_body(body);

        // Control-plane replaces always need the full response body so the
        // caller can inspect the updated resource properties.
        let mut operation_options = OperationOptions::default();
        operation_options.content_response_on_write =
            Some(azure_data_cosmos_driver::options::ContentResponseOnWrite::Enabled);

        let driver_response = self
            .context
            .driver
            .execute_point_operation(operation, operation_options)
            .await?;

        Ok(ResourceResponse::new(
            crate::driver_bridge::driver_response_to_cosmos_response(driver_response),
        ))
    }

    /// Reads container throughput properties, if any.
    ///
    /// This will return `None` if the database does not have a throughput offer configured.
    ///
    /// # Arguments
    /// * `options` - Optional parameters for the request.
    pub async fn read_throughput(
        &self,
        #[allow(
            unused_variables,
            reason = "The 'options' parameter may be used in the future"
        )]
        options: Option<ThroughputOptions>,
    ) -> azure_core::Result<Option<ThroughputProperties>> {
        offers_client::find_offer(
            &self.context.driver,
            self.container_ref.account(),
            self.container_ref.rid(),
        )
        .await
    }

    /// Begins replacing the container throughput properties.
    ///
    /// The Cosmos DB service may process throughput changes asynchronously. The returned
    /// [`ThroughputPoller`] can be awaited directly for the final result, or polled as a
    /// stream to observe progress.
    ///
    /// # Arguments
    /// * `throughput` - The new throughput properties to set.
    /// * `options` - Optional parameters for the request.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use azure_data_cosmos::models::ThroughputProperties;
    /// # async fn example(container_client: azure_data_cosmos::clients::ContainerClient) -> azure_core::Result<()> {
    /// let throughput = container_client
    ///     .begin_replace_throughput(ThroughputProperties::manual(500), None)
    ///     .await? // start the replace operation
    ///     .await? // wait for completion (polls if async)
    ///     .into_model()?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn begin_replace_throughput(
        &self,
        throughput: ThroughputProperties,
        options: Option<ThroughputOptions>,
    ) -> azure_core::Result<ThroughputPoller> {
        #[allow(
            unused_variables,
            reason = "The 'options' variable may be used in the future"
        )]
        let options = options.unwrap_or_default();

        offers_client::begin_replace(
            self.context.driver.clone(),
            self.container_ref.account().clone(),
            self.container_ref.rid(),
            throughput,
        )
        .await
    }

    /// Deletes this container.
    ///
    #[doc = include_str!("../../docs/control-plane-warning.md")]
    ///
    /// # Arguments
    /// * `options` - Optional parameters for the request.
    pub async fn delete(
        &self,
        #[allow(
            unused_variables,
            reason = "The 'options' parameter may be used in the future"
        )]
        options: Option<DeleteContainerOptions>,
    ) -> azure_core::Result<ResourceResponse<()>> {
        let operation = CosmosOperation::delete_container(self.container_ref.clone());

        let driver_response = self
            .context
            .driver
            .execute_point_operation(operation, OperationOptions::default())
            .await?;

        Ok(ResourceResponse::new(
            crate::driver_bridge::driver_response_to_cosmos_response(driver_response),
        ))
    }

    /// Creates a new item in the container.
    ///
    /// # Arguments
    /// * `partition_key` - The partition key of the new item.
    /// * `item_id` - The id of the new item.
    /// * `item` - The item to create. The type must implement [`Serialize`] and [`Deserialize`](serde::Deserialize)
    /// * `options` - Optional parameters for the request
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use serde::{Deserialize, Serialize};
    /// # async fn doc() -> Result<(), Box<dyn std::error::Error>> {
    /// #[derive(Debug, Deserialize, Serialize)]
    /// pub struct Product {
    ///     #[serde(rename = "id")] // Use serde attributes to control serialization
    ///     product_id: String,
    ///     category_id: String,
    ///     product_name: String,
    /// }
    /// let p = Product {
    ///     product_id: "product1".to_string(),
    ///     category_id: "category1".to_string(),
    ///     product_name: "Product #1".to_string(),
    /// };
    /// # let container_client: azure_data_cosmos::clients::ContainerClient = panic!("this is a non-running example");
    /// container_client
    ///     .create_item("category1", "product1", p, None)
    ///     .await?;
    /// # }
    /// ```
    ///
    /// # Content Response on Write
    ///
    /// By default, the newly created item is *not* returned in the HTTP response.
    /// If you want the new item to be returned, set `content_response_on_write` to [`ContentResponseOnWrite::Enabled`](crate::ContentResponseOnWrite::Enabled) on the [`OperationOptions`](crate::OperationOptions) in your [`ItemWriteOptions`](crate::ItemWriteOptions).
    /// You can deserialize the returned item by retrieving the [`ResponseBody`](azure_core::http::response::ResponseBody) using [`ItemResponse::into_body`] and then calling [`ResponseBody::json`](azure_core::http::response::ResponseBody::json), like this:
    ///
    /// ```rust,no_run
    /// use azure_data_cosmos::{ItemWriteOptions, ContentResponseOnWrite, OperationOptions};
    /// use serde::{Deserialize, Serialize};
    /// # async fn doc() -> Result<(), Box<dyn std::error::Error>> {
    /// #[derive(Debug, Deserialize, Serialize)]
    /// pub struct Product {
    ///     #[serde(rename = "id")] // Use serde attributes to control serialization
    ///     product_id: String,
    ///     category_id: String,
    ///     product_name: String,
    /// }
    /// let p = Product {
    ///     product_id: "product1".to_string(),
    ///     category_id: "category1".to_string(),
    ///     product_name: "Product #1".to_string(),
    /// };
    /// # let container_client: azure_data_cosmos::clients::ContainerClient = panic!("this is a non-running example");
    /// let mut operation = OperationOptions::default();
    /// operation.content_response_on_write = Some(ContentResponseOnWrite::Enabled);
    /// let options = ItemWriteOptions::default().with_operation_options(operation);
    /// let created_item = container_client
    ///     .create_item("category1", "product1", p, Some(options))
    ///     .await?
    ///     .into_body().json::<Product>();
    /// # Ok(())
    /// # }
    /// ```
    pub async fn create_item<T: Serialize>(
        &self,
        partition_key: impl Into<PartitionKey>,
        item_id: &str,
        item: T,
        options: Option<ItemWriteOptions>,
    ) -> azure_core::Result<ItemResponse<()>> {
        let options = options.unwrap_or_default();
        let body = serde_json::to_vec(&item)?;

        // Build the driver's item reference from our stored container metadata.
        let item_ref = ItemReference::from_name(
            &self.container_ref,
            partition_key.into(),
            item_id.to_owned(),
        );

        // Create the driver operation and apply ItemWriteOptions fields.
        let operation = CosmosOperation::create_item(item_ref).with_body(body);
        let operation = apply_item_options(operation, options.session_token, options.precondition);

        // Execute through the driver.
        let driver_response = self
            .context
            .driver
            .execute_point_operation(operation, options.operation)
            .await?;

        // Bridge the driver response to the SDK response type.
        Ok(ItemResponse::new(
            crate::driver_bridge::driver_response_to_cosmos_response(driver_response),
        ))
    }

    /// Replaces an existing item in the container.
    ///
    /// # Arguments
    /// * `partition_key` - The partition key of the item to replace.
    /// * `item_id` - The id of the item to replace.
    /// * `item` - The item to create. The type must implement [`Serialize`] and [`Deserialize`](serde::Deserialize)
    /// * `options` - Optional parameters for the request
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use serde::{Deserialize, Serialize};
    /// # async fn doc() -> Result<(), Box<dyn std::error::Error>> {
    /// #[derive(Debug, Deserialize, Serialize)]
    /// pub struct Product {
    ///     #[serde(rename = "id")] // Use serde attributes to control serialization
    ///     product_id: String,
    ///     category_id: String,
    ///     product_name: String,
    /// }
    /// let p = Product {
    ///     product_id: "product1".to_string(),
    ///     category_id: "category1".to_string(),
    ///     product_name: "Product #1".to_string(),
    /// };
    /// # let container_client: azure_data_cosmos::clients::ContainerClient = panic!("this is a non-running example");
    /// container_client
    ///     .replace_item("category1", "product1", p, None)
    ///     .await?;
    /// # }
    /// ```
    ///
    /// # Content Response on Write
    ///
    /// By default, the replaced item is *not* returned in the HTTP response.
    /// If you want the replaced item to be returned, set `content_response_on_write` to [`ContentResponseOnWrite::Enabled`](crate::ContentResponseOnWrite::Enabled) on the [`OperationOptions`](crate::OperationOptions) in your [`ItemWriteOptions`](crate::ItemWriteOptions).
    /// You can deserialize the returned item by retrieving the [`ResponseBody`](azure_core::http::response::ResponseBody) using [`ItemResponse::into_body`] and then calling [`ResponseBody::json`](azure_core::http::response::ResponseBody::json), like this:
    ///
    /// ```rust,no_run
    /// use azure_data_cosmos::{ItemWriteOptions, ContentResponseOnWrite, OperationOptions};
    /// use serde::{Deserialize, Serialize};
    /// # async fn doc() -> Result<(), Box<dyn std::error::Error>> {
    /// #[derive(Debug, Deserialize, Serialize)]
    /// pub struct Product {
    ///     #[serde(rename = "id")] // Use serde attributes to control serialization
    ///     product_id: String,
    ///     category_id: String,
    ///     product_name: String,
    /// }
    /// let p = Product {
    ///     product_id: "product1".to_string(),
    ///     category_id: "category1".to_string(),
    ///     product_name: "Product #1".to_string(),
    /// };
    /// # let container_client: azure_data_cosmos::clients::ContainerClient = panic!("this is a non-running example");
    /// let mut operation = OperationOptions::default();
    /// operation.content_response_on_write = Some(ContentResponseOnWrite::Enabled);
    /// let options = ItemWriteOptions::default().with_operation_options(operation);
    /// let updated_product: Product = container_client
    ///     .replace_item("category1", "product1", p, Some(options))
    ///     .await?
    ///     .into_body().json::<Product>()?;
    /// # }
    /// ```
    pub async fn replace_item<T: Serialize>(
        &self,
        partition_key: impl Into<PartitionKey>,
        item_id: &str,
        item: T,
        options: Option<ItemWriteOptions>,
    ) -> azure_core::Result<ItemResponse<()>> {
        let options = options.unwrap_or_default();
        let body = serde_json::to_vec(&item)?;

        // Build the driver's item reference from our stored container metadata.
        let item_ref = ItemReference::from_name(
            &self.container_ref,
            partition_key.into(),
            item_id.to_owned(),
        );

        // Create the driver operation and apply ItemWriteOptions fields.
        let operation = CosmosOperation::replace_item(item_ref).with_body(body);
        let operation = apply_item_options(operation, options.session_token, options.precondition);

        // Execute through the driver.
        let driver_response = self
            .context
            .driver
            .execute_point_operation(operation, options.operation)
            .await?;

        // Bridge the driver response to the SDK response type.
        Ok(ItemResponse::new(
            crate::driver_bridge::driver_response_to_cosmos_response(driver_response),
        ))
    }

    /// Creates or replaces an item in the container.
    ///
    /// If an item with the same ID is found in the container, it is updated with the provided content.
    /// If no item with the same ID is found in the container, a new item is created with the provided content.
    ///
    /// # Arguments
    /// * `partition_key` - The partition key of the item to create or replace.
    /// * `item_id` - The id of the item to create or replace.
    /// * `item` - The item to create. The type must implement [`Serialize`] and [`Deserialize`](serde::Deserialize)
    /// * `options` - Optional parameters for the request
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use serde::{Deserialize, Serialize};
    /// # async fn doc() -> Result<(), Box<dyn std::error::Error>> {
    /// #[derive(Debug, Deserialize, Serialize)]
    /// pub struct Product {
    ///     #[serde(rename = "id")] // Use serde attributes to control serialization
    ///     product_id: String,
    ///     category_id: String,
    ///     product_name: String,
    /// }
    /// let p = Product {
    ///     product_id: "product1".to_string(),
    ///     category_id: "category1".to_string(),
    ///     product_name: "Product #1".to_string(),
    /// };
    /// # let container_client: azure_data_cosmos::clients::ContainerClient = panic!("this is a non-running example");
    /// container_client
    ///     .upsert_item("category1", "product1", p, None)
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Content Response on Write
    ///
    /// By default, the created/replaced item is *not* returned in the HTTP response.
    /// If you want the created/replaced item to be returned, set `content_response_on_write` to [`ContentResponseOnWrite::Enabled`](crate::ContentResponseOnWrite::Enabled) on the [`OperationOptions`](crate::OperationOptions) in your [`ItemWriteOptions`](crate::ItemWriteOptions).
    /// You can deserialize the returned item by retrieving the [`ResponseBody`](azure_core::http::response::ResponseBody) using [`ItemResponse::into_body`] and then calling [`ResponseBody::json`](azure_core::http::response::ResponseBody::json), like this:
    ///
    /// ```rust,no_run
    /// use azure_data_cosmos::{ItemWriteOptions, ContentResponseOnWrite, OperationOptions};
    /// use serde::{Deserialize, Serialize};
    /// # async fn doc() -> Result<(), Box<dyn std::error::Error>> {
    /// #[derive(Debug, Deserialize, Serialize)]
    /// pub struct Product {
    ///     #[serde(rename = "id")] // Use serde attributes to control serialization
    ///     product_id: String,
    ///     category_id: String,
    ///     product_name: String,
    /// }
    /// let p = Product {
    ///     product_id: "product1".to_string(),
    ///     category_id: "category1".to_string(),
    ///     product_name: "Product #1".to_string(),
    /// };
    /// # let container_client: azure_data_cosmos::clients::ContainerClient = panic!("this is a non-running example");
    /// let mut operation = OperationOptions::default();
    /// operation.content_response_on_write = Some(ContentResponseOnWrite::Enabled);
    /// let options = ItemWriteOptions::default().with_operation_options(operation);
    /// let updated_product = container_client
    ///     .upsert_item("category1", "product1", p, Some(options))
    ///     .await?
    ///     .into_body().json::<Product>()?;
    /// Ok(())
    /// # }
    pub async fn upsert_item<T: Serialize>(
        &self,
        partition_key: impl Into<PartitionKey>,
        item_id: &str,
        item: T,
        options: Option<ItemWriteOptions>,
    ) -> azure_core::Result<ItemResponse<()>> {
        let options = options.unwrap_or_default();
        let body = serde_json::to_vec(&item)?;

        // Build the driver's item reference from our stored container metadata.
        let item_ref = ItemReference::from_name(
            &self.container_ref,
            partition_key.into(),
            item_id.to_owned(),
        );

        // Create the driver operation and apply ItemWriteOptions fields.
        let operation = CosmosOperation::upsert_item(item_ref).with_body(body);
        let operation = apply_item_options(operation, options.session_token, options.precondition);

        // Execute through the driver.
        let driver_response = self
            .context
            .driver
            .execute_point_operation(operation, options.operation)
            .await?;

        // Bridge the driver response to the SDK response type.
        Ok(ItemResponse::new(
            crate::driver_bridge::driver_response_to_cosmos_response(driver_response),
        ))
    }

    /// Reads a specific item from the container.
    ///
    /// # Arguments
    /// * `partition_key` - The partition key of the item to read. See [`PartitionKey`] for more information on how to specify a partition key.
    /// * `item_id` - The id of the item to read.
    /// * `options` - Optional parameters for the request
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use serde::{Deserialize, Serialize};
    /// # async fn doc() -> Result<(), Box<dyn std::error::Error>> {
    /// #[derive(Debug, Deserialize, Serialize)]
    /// pub struct Product {
    ///     #[serde(rename = "id")] // Use serde attributes to control serialization
    ///     product_id: String,
    ///     category_id: String,
    ///     product_name: String,
    /// }
    /// # let container_client: azure_data_cosmos::clients::ContainerClient = panic!("this is a non-running example");
    /// let item: Product = container_client
    ///     .read_item("partition1", "item1", None)
    ///     .await?
    ///     .into_model()?;
    /// println!("Read Item: {:#?}", item);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn read_item<T>(
        &self,
        partition_key: impl Into<PartitionKey>,
        item_id: &str,
        options: Option<ItemReadOptions>,
    ) -> azure_core::Result<ItemResponse<T>> {
        let options = options.unwrap_or_default();

        // Build the driver's item reference from our stored container metadata.
        let item_ref = ItemReference::from_name(
            &self.container_ref,
            partition_key.into(),
            item_id.to_owned(),
        );

        // Create the driver operation.
        let operation = CosmosOperation::read_item(item_ref);
        let operation = apply_item_options(operation, options.session_token, options.precondition);

        // Execute through the driver.
        let driver_response = self
            .context
            .driver
            .execute_point_operation(operation, options.operation)
            .await?;

        // Bridge the driver response to the SDK response type.
        Ok(ItemResponse::new(
            crate::driver_bridge::driver_response_to_cosmos_response(driver_response),
        ))
    }

    /// Deletes an item from the container.
    ///
    /// # Arguments
    /// * `partition_key` - The partition key of the item to delete.
    /// * `item_id` - The id of the item to delete.
    /// * `options` - Optional parameters for the request
    ///
    /// NOTE: The deleted item is never returned by the Cosmos API, so any content response option is ignored.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use serde::{Deserialize, Serialize};
    /// # async fn doc() -> Result<(), Box<dyn std::error::Error>> {
    /// # let container_client: azure_data_cosmos::clients::ContainerClient = panic!("this is a non-running example");
    /// container_client
    ///     .delete_item("partition1", "item1", None)
    ///     .await?;
    /// # }
    /// ```
    pub async fn delete_item(
        &self,
        partition_key: impl Into<PartitionKey>,
        item_id: &str,
        options: Option<ItemWriteOptions>,
    ) -> azure_core::Result<ItemResponse<()>> {
        let options = options.unwrap_or_default();

        // Build the driver's item reference from our stored container metadata.
        let item_ref = ItemReference::from_name(
            &self.container_ref,
            partition_key.into(),
            item_id.to_owned(),
        );

        // Create the driver operation (no body for delete).
        let operation = CosmosOperation::delete_item(item_ref);
        let operation = apply_item_options(operation, options.session_token, options.precondition);

        // Execute through the driver.
        let driver_response = self
            .context
            .driver
            .execute_point_operation(operation, options.operation)
            .await?;

        // Bridge the driver response to the SDK response type.
        Ok(ItemResponse::new(
            crate::driver_bridge::driver_response_to_cosmos_response(driver_response),
        ))
    }

    /// Executes a single-partition query against items in the container.
    ///
    /// The resulting document will be deserialized into the type provided as `T`.
    /// If you want to deserialize the document to a direct representation of the JSON returned, use [`serde_json::Value`] as the target type.
    ///
    /// We recommend using ["turbofish" syntax](https://doc.rust-lang.org/book/appendix-02-operators.html#:~:text=turbofish) (`query_items::<SomeTargetType>(...)`) to specify the target type, as it makes type inference easier.
    ///
    /// **NOTE:** Currently, the Azure Cosmos DB SDK for Rust only supports single-partition querying. Cross-partition queries may be supported in the future.
    ///
    /// # Arguments
    ///
    /// * `query` - The query to execute.
    /// * `scope` - The [`QueryScope`] specifying the scope of the query.
    /// * `options` - Optional parameters for the request.
    ///
    /// # Cross Partition Queries
    ///
    /// Cross-partition queries are significantly limited in the current version of the Cosmos DB SDK.
    /// They are run on the gateway and limited to simple projections (`SELECT`) and filtering (`WHERE`).
    /// For more details, see [the Cosmos DB documentation page on cross-partition queries](https://learn.microsoft.com/en-us/rest/api/cosmos-db/querying-cosmosdb-resources-using-the-rest-api#queries-that-cannot-be-served-by-gateway).
    ///
    /// # Examples
    ///
    /// The `query` parameter accepts anything that can be transformed [`Into`] a [`Query`], and `scope` controls partition targeting.
    /// This allows simple queries without parameters to be expressed easily:
    ///
    /// ```rust,no_run
    /// # async fn doc() -> Result<(), Box<dyn std::error::Error>> {
    /// # use azure_data_cosmos::query::QueryScope;
    /// # let container_client: azure_data_cosmos::clients::ContainerClient = panic!("this is a non-running example");
    /// #[derive(serde::Deserialize)]
    /// struct Customer {
    ///     id: u64,
    ///     name: String,
    /// }
    /// let items = container_client.query_items::<Customer>(
    ///     "SELECT * FROM c",
    ///     QueryScope::partition("some_partition_key"),
    ///     None,
    /// ).await?;
    /// # }
    /// ```
    ///
    /// You can specify parameters by using [`Query::from()`] and [`Query::with_parameter()`]:
    ///
    /// ```rust,no_run
    /// # async fn doc() -> Result<(), Box<dyn std::error::Error>> {
    /// use azure_data_cosmos::{query::QueryScope, Query};
    /// # let container_client: azure_data_cosmos::clients::ContainerClient = panic!("this is a non-running example");
    /// #[derive(serde::Deserialize)]
    /// struct Customer {
    ///     id: u64,
    ///     name: String,
    /// }
    /// let query = Query::from("SELECT COUNT(*) FROM c WHERE c.customer_id = @customer_id")
    ///     .with_parameter("@customer_id", 42)?;
    /// let items = container_client
    ///     .query_items::<Customer>(query, QueryScope::partition("some_partition_key"), None).await?;
    /// # }
    /// ```
    ///
    /// See [`PartitionKey`](crate::PartitionKey) for more information on how to specify a partition key, and [`Query`] for more information on how to specify a query.
    pub async fn query_items<T: DeserializeOwned + Send + 'static>(
        &self,
        query: impl Into<Query>,
        scope: QueryScope,
        options: Option<QueryOptions>,
    ) -> azure_core::Result<FeedItemIterator<T>> {
        let options = options.unwrap_or_default();
        let query = query.into();

        let container_ref = self.container_ref.clone();

        // The first operation to execute in the query items flow.
        // This holds the session token provided by the user, if any.
        let mut initial_operation =
            CosmosOperation::query_items(container_ref.clone(), scope.into())
                .with_body(serde_json::to_vec(&query)?);
        if let Some(token) = options.session_token {
            initial_operation = initial_operation.with_session_token(token);
        }
        if let Some(max_item_count) = options.max_item_count {
            initial_operation = initial_operation.with_max_item_count(max_item_count);
        }
        let plan = self
            .context
            .driver
            .plan_operation(
                initial_operation,
                &options.operation,
                options.continuation_token.as_ref(),
            )
            .await?;
        Ok(FeedItemIterator::new(
            self.context.driver.clone(),
            Some(self.container_ref.clone()),
            plan,
            options.operation,
        ))
    }

    /// Executes a transactional batch of operations.
    ///
    /// All operations in the batch are executed atomically within the same partition key.
    /// If any operation fails, the entire batch is rolled back.
    ///
    /// # Arguments
    /// * `batch` - The [`TransactionalBatch`] containing the operations to execute.
    /// * `options` - Optional parameters for the request.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use azure_data_cosmos::TransactionalBatch;
    /// use serde::{Deserialize, Serialize};
    /// # async fn doc() -> Result<(), Box<dyn std::error::Error>> {
    /// #[derive(Debug, Deserialize, Serialize)]
    /// pub struct Product {
    ///     id: String,
    ///     category: String,
    ///     name: String,
    /// }
    /// # let container_client: azure_data_cosmos::clients::ContainerClient = panic!("this is a non-running example");
    /// let product1 = Product {
    ///     id: "product1".to_string(),
    ///     category: "category1".to_string(),
    ///     name: "Product #1".to_string(),
    /// };
    ///
    /// let batch = TransactionalBatch::new("category1")
    ///     .create_item(product1)?;
    ///
    /// let response = container_client.execute_transactional_batch(batch, None).await?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Limitations
    ///
    /// * Maximum 100 operations per batch
    /// * Maximum payload size is 2 MB
    /// * All operations must target the same partition key
    pub async fn execute_transactional_batch(
        &self,
        batch: TransactionalBatch,
        options: Option<BatchOptions>,
    ) -> azure_core::Result<BatchResponse> {
        let options = options.unwrap_or_default();
        let body = serde_json::to_vec(batch.operations())?;
        let driver_pk = batch.partition_key().clone();

        let operation =
            CosmosOperation::batch(self.container_ref.clone(), driver_pk).with_body(body);
        let operation = apply_batch_options(operation, &options);

        let driver_response = self
            .context
            .driver
            .execute_point_operation(operation, options.operation)
            .await?;

        Ok(BatchResponse::new(
            crate::driver_bridge::driver_response_to_cosmos_response(driver_response),
        ))
    }

    /// Gets the feed ranges for this container.
    pub async fn read_feed_ranges(
        &self,
        options: Option<ReadFeedRangesOptions>,
    ) -> azure_core::Result<Vec<FeedRange>> {
        let options = options.unwrap_or_default();
        let mut ranges = self
            .context
            .driver
            .resolve_all_partition_key_ranges(&self.container_ref, options.force_refresh())
            .await
            .ok_or_else(|| {
                azure_core::Error::with_message(
                    azure_core::error::ErrorKind::Other,
                    "failed to resolve routing map for container",
                )
            })?;

        if ranges.is_empty() && !options.force_refresh() {
            // A valid container always has at least one partition key range.
            // Empty result likely means a stale/failed cache — retry with forced refresh.
            ranges = self
                .context
                .driver
                .resolve_all_partition_key_ranges(&self.container_ref, true)
                .await
                .ok_or_else(|| {
                    azure_core::Error::with_message(
                        azure_core::error::ErrorKind::Other,
                        "failed to resolve routing map for container",
                    )
                })?;
        }

        if ranges.is_empty() {
            return Err(azure_core::Error::with_message(
                azure_core::error::ErrorKind::Other,
                "resolved routing map contains no partition key ranges; \
                 the container may not exist or the service may be unreachable",
            ));
        }

        ranges.iter().map(FeedRange::try_from).collect()
    }

    /// Returns the [`FeedRange`]s covering the given partition key.
    ///
    /// Full keys return a single-element `Vec`. Prefix keys on MultiHash
    /// containers return one or more feed ranges.
    pub async fn feed_range_from_partition_key(
        &self,
        partition_key: impl Into<PartitionKey>,
        options: Option<ReadFeedRangesOptions>,
    ) -> azure_core::Result<Vec<FeedRange>> {
        let partition_key = partition_key.into();
        let driver_pk = partition_key;
        let options = options.unwrap_or_default();
        let pk_def = self.container_ref.partition_key_definition();
        let values = driver_pk.values();

        if values.is_empty() {
            return Err(azure_core::Error::with_message(
                azure_core::error::ErrorKind::Other,
                "partition key must have at least one component",
            ));
        }
        if values.len() > pk_def.paths().len() {
            return Err(azure_core::Error::with_message(
                azure_core::error::ErrorKind::Other,
                format!(
                    "partition key has {} components but container definition has {} paths",
                    values.len(),
                    pk_def.paths().len()
                ),
            ));
        }

        let is_prefix =
            pk_def.kind() == PartitionKeyKind::MultiHash && values.len() < pk_def.paths().len();
        if !is_prefix && values.len() != pk_def.paths().len() {
            return Err(azure_core::Error::with_message(
                azure_core::error::ErrorKind::Other,
                "prefix partition keys are only supported for MultiHash (hierarchical) containers",
            ));
        }

        let ranges = self
            .context
            .driver
            .resolve_partition_key_ranges_for_key(
                &self.container_ref,
                &driver_pk,
                options.force_refresh(),
            )
            .await
            .ok_or_else(|| {
                azure_core::Error::with_message(
                    azure_core::error::ErrorKind::Other,
                    "failed to resolve routing map for container",
                )
            })?;

        if ranges.is_empty() && !options.force_refresh() {
            // Empty result may indicate a stale cache — retry with refresh.
            let ranges = self
                .context
                .driver
                .resolve_partition_key_ranges_for_key(&self.container_ref, &driver_pk, true)
                .await
                .ok_or_else(|| {
                    azure_core::Error::with_message(
                        azure_core::error::ErrorKind::Other,
                        "failed to resolve routing map for container",
                    )
                })?;

            if ranges.is_empty() {
                return Err(azure_core::Error::with_message(
                    azure_core::error::ErrorKind::Other,
                    "no partition key ranges found for the given partition key; \
                     the container may not exist or the service may be unreachable",
                ));
            }

            ranges.iter().map(FeedRange::try_from).collect()
        } else {
            ranges.iter().map(FeedRange::try_from).collect()
        }
    }

    /// Gets the most up-to-date session token from a list of feed range and session token pairs
    /// for a specific target feed range.
    ///
    /// This method merges session tokens from feed ranges that overlap with the target,
    /// handling partition split and merge scenarios automatically. It is useful when
    /// maintaining your own session token cache across multiple clients.
    ///
    /// Session tokens and feed ranges are scoped to a single container. Only pass session
    /// tokens and feed ranges obtained from this container.
    ///
    /// # Arguments
    ///
    /// * `feed_ranges_to_session_tokens` - Pairs of feed ranges and their associated session tokens.
    /// * `target_feed_range` - The feed range to get the most up-to-date session token for.
    ///
    /// # Errors
    ///
    /// Returns an error if no input feed ranges overlap with the target feed range,
    /// or if any session token string is malformed.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use azure_data_cosmos::{clients::ContainerClient, FeedRange, SessionToken};
    /// # async fn example(container: ContainerClient) -> azure_core::Result<()> {
    /// let feed_range = FeedRange::full();
    /// let token_a: SessionToken = "0:1#100#3=50".into();
    /// let token_b: SessionToken = "0:1#200#3=60".into();
    ///
    /// let latest = container.get_latest_session_token(
    ///     &[(feed_range.clone(), token_a), (feed_range, token_b)],
    ///     &FeedRange::full(),
    /// )?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn get_latest_session_token(
        &self,
        feed_ranges_to_session_tokens: &[(FeedRange, SessionToken)],
        target_feed_range: &FeedRange,
    ) -> azure_core::Result<SessionToken> {
        crate::session_helpers::get_latest_session_token(
            feed_ranges_to_session_tokens,
            target_feed_range,
        )
    }
}

/// Applies optional `session_token` and `precondition` to a [`CosmosOperation`].
///
/// Both [`ItemReadOptions`] and [`ItemWriteOptions`] carry these fields;
/// this helper avoids duplicating the wiring logic in every item operation.
fn apply_item_options(
    mut operation: CosmosOperation,
    session_token: Option<SessionToken>,
    precondition: Option<Precondition>,
) -> CosmosOperation {
    if let Some(session_token) = session_token {
        operation = operation.with_session_token(session_token);
    }
    if let Some(precondition) = precondition {
        operation = operation.with_precondition(precondition);
    }
    operation
}

/// Applies [`BatchOptions`] fields to a [`CosmosOperation`].
///
/// [`BatchOptions`] carries a session token but no precondition (ETag-based
/// conditions are specified per-operation within the batch itself).
fn apply_batch_options(mut operation: CosmosOperation, options: &BatchOptions) -> CosmosOperation {
    if let Some(session_token) = &options.session_token {
        operation = operation.with_session_token(session_token.clone());
    }
    operation
}

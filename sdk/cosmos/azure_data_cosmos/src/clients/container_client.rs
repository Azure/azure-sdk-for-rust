// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use crate::{
    clients::OffersClient,
    models::{ContainerProperties, CosmosResponse, ThroughputProperties},
    options::{BatchOptions, QueryOptions, ReadContainerOptions},
    pipeline::GatewayPipeline,
    resource_context::{ResourceLink, ResourceType},
    transactional_batch::{TransactionalBatch, TransactionalBatchResponse},
    DeleteContainerOptions, FeedItemIterator, ItemOptions, PartitionKey, Query,
    ReplaceContainerOptions, ThroughputOptions,
};
use std::sync::Arc;

use crate::cosmos_request::CosmosRequest;
use crate::handler::container_connection::ContainerConnection;
use crate::operation_context::OperationType;
use crate::routing::container_cache::ContainerCache;
use crate::routing::global_endpoint_manager::GlobalEndpointManager;
use crate::routing::global_partition_endpoint_manager::GlobalPartitionEndpointManager;
use crate::routing::partition_key_range_cache::PartitionKeyRangeCache;
use azure_core::http::headers::AsHeaders;
use azure_core::http::Context;
use serde::{de::DeserializeOwned, Serialize};

/// A client for working with a specific container in a Cosmos DB account.
///
/// You can get a `Container` by calling [`DatabaseClient::container_client()`](crate::clients::DatabaseClient::container_client()).
#[derive(Clone)]
pub struct ContainerClient {
    link: ResourceLink,
    items_link: ResourceLink,
    pipeline: Arc<GatewayPipeline>,
    container_connection: Arc<ContainerConnection>,
    container_id: String,
}

impl ContainerClient {
    pub(crate) async fn new(
        pipeline: Arc<GatewayPipeline>,
        database_link: &ResourceLink,
        container_id: &str,
        global_endpoint_manager: Arc<GlobalEndpointManager>,
        global_partition_endpoint_manager: Arc<GlobalPartitionEndpointManager>,
    ) -> Self {
        let link = database_link
            .feed(ResourceType::Containers)
            .item(container_id);
        let items_link = link.feed(ResourceType::Documents);

        let container_cache = Arc::from(ContainerCache::new(
            pipeline.clone(),
            link.clone(),
            global_endpoint_manager.clone(),
        ));
        let partition_key_range_cache = Arc::from(PartitionKeyRangeCache::new(
            pipeline.clone(),
            database_link.clone(),
            container_cache.clone(),
            global_endpoint_manager.clone(),
        ));
        let container_connection = Arc::from(ContainerConnection::new(
            pipeline.clone(),
            container_cache,
            partition_key_range_cache,
            global_partition_endpoint_manager.clone(),
        ));

        Self {
            link,
            items_link,
            pipeline,
            container_connection,
            container_id: container_id.to_string(),
        }
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
    #[tracing::instrument(skip_all, fields(id = self.container_id))]
    pub async fn read(
        &self,
        #[allow(
            unused_variables,
            reason = "The 'options' parameter may be used in the future"
        )]
        options: Option<ReadContainerOptions>,
    ) -> azure_core::Result<CosmosResponse<ContainerProperties>> {
        let cosmos_request =
            CosmosRequest::builder(OperationType::Read, self.link.clone()).build()?;
        let response: CosmosResponse<ContainerProperties> = self
            .container_connection
            .send(cosmos_request, Context::default())
            .await?;

        // Populate the container cache so that subsequent item operations
        // (which resolve container properties for partition-level routing)
        // find them already cached and avoid an extra metadata fetch.
        if let Ok(properties) = response.deserialize_body::<ContainerProperties>() {
            self.container_connection
                .populate_container_cache(self.container_id.clone(), properties)
                .await;
        }

        Ok(response)
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
    #[tracing::instrument(skip_all, fields(id = self.container_id))]
    pub async fn replace(
        &self,
        properties: ContainerProperties,
        #[allow(
            unused_variables,
            reason = "The 'options' parameter may be used in the future"
        )]
        options: Option<ReplaceContainerOptions>,
    ) -> azure_core::Result<CosmosResponse<ContainerProperties>> {
        let cosmos_request = CosmosRequest::builder(OperationType::Replace, self.link.clone())
            .json(&properties)
            .build()?;
        self.container_connection
            .send(cosmos_request, Context::default())
            .await
    }

    /// Reads container throughput properties, if any.
    ///
    /// This will return `None` if the database does not have a throughput offer configured.
    ///
    /// # Arguments
    /// * `options` - Optional parameters for the request.
    #[tracing::instrument(skip_all, fields(id = self.container_id))]
    pub async fn read_throughput(
        &self,
        #[allow(
            unused_variables,
            reason = "The 'options' parameter may be used in the future"
        )]
        options: Option<ThroughputOptions>,
    ) -> azure_core::Result<Option<ThroughputProperties>> {
        // We need to get the RID for the database.
        let db = self.read(None).await?.into_model()?;
        let resource_id = db
            .system_properties
            .resource_id
            .expect("service should always return a '_rid' for a container");

        let offers_client = OffersClient::new(self.pipeline.clone(), resource_id);
        offers_client.read(Context::default()).await
    }

    /// Replaces the container throughput properties.
    ///
    /// Note that throughput changes may not take effect immediately.
    /// The service processes the change asynchronously, so you may need to poll
    /// [`ContainerClient::read_throughput()`] to confirm the new throughput is in effect.
    ///
    /// # Arguments
    /// * `throughput` - The new throughput properties to set.
    /// * `options` - Optional parameters for the request.
    #[tracing::instrument(skip_all, fields(id = self.container_id))]
    pub async fn replace_throughput(
        &self,
        throughput: ThroughputProperties,
        options: Option<ThroughputOptions>,
    ) -> azure_core::Result<CosmosResponse<ThroughputProperties>> {
        #[allow(
            unused_variables,
            reason = "The 'options' variable may be used in the future"
        )]
        let options = options.unwrap_or_default();

        // We need to get the RID for the database.
        let db = self.read(None).await?.into_model()?;
        let resource_id = db
            .system_properties
            .resource_id
            .expect("service should always return a '_rid' for a container");

        let offers_client = OffersClient::new(self.pipeline.clone(), resource_id);
        offers_client.replace(Context::default(), throughput).await
    }

    /// Deletes this container.
    ///
    #[doc = include_str!("../../docs/control-plane-warning.md")]
    ///
    /// # Arguments
    /// * `options` - Optional parameters for the request.
    #[tracing::instrument(skip_all, fields(id = self.container_id))]
    pub async fn delete(
        &self,
        #[allow(
            unused_variables,
            reason = "The 'options' parameter may be used in the future"
        )]
        options: Option<DeleteContainerOptions>,
    ) -> azure_core::Result<CosmosResponse<()>> {
        let cosmos_request =
            CosmosRequest::builder(OperationType::Delete, self.link.clone()).build()?;
        self.container_connection
            .send(cosmos_request, Context::default())
            .await
    }

    /// Creates a new item in the container.
    ///
    /// # Arguments
    /// * `partition_key` - The partition key of the new item.
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
    ///     .create_item("category1", p, None)
    ///     .await?;
    /// # }
    /// ```
    ///
    /// # Content Response on Write
    ///
    /// By default, the newly created item is *not* returned in the HTTP response.
    /// If you want the new item to be returned, set the [`ItemOptions::with_content_response_on_write_enabled()`] option to `true`.
    /// You can deserialize the returned item by retrieving the [`ResponseBody`](azure_core::http::response::ResponseBody) using [`CosmosResponse::into_body`] and then calling [`ResponseBody::json`](azure_core::http::response::ResponseBody::json), like this:
    ///
    /// ```rust,no_run
    /// use azure_data_cosmos::ItemOptions;
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
    /// let options = ItemOptions::default().with_content_response_on_write_enabled(true);
    /// let created_item = container_client
    ///     .create_item("category1", p, Some(options))
    ///     .await?
    ///     .into_body().json::<Product>();
    /// # Ok(())
    /// # }
    /// ```
    #[tracing::instrument(skip_all, fields(id = self.container_id))]
    pub async fn create_item<T: Serialize>(
        &self,
        partition_key: impl Into<PartitionKey>,
        item: T,
        options: Option<ItemOptions>,
    ) -> azure_core::Result<CosmosResponse<()>> {
        let options = options.clone().unwrap_or_default();
        let excluded_regions = options.excluded_regions.clone();
        let mut cosmos_request =
            CosmosRequest::builder(OperationType::Create, self.items_link.clone())
                .json(&item)
                .partition_key(partition_key.into())
                .excluded_regions(excluded_regions)
                .build()?;
        options.apply_headers(&mut cosmos_request.headers);

        self.container_connection
            .send(cosmos_request, Context::default())
            .await
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
    /// If you want the replaced item to be returned, set the [`ItemOptions::with_content_response_on_write_enabled()`] option to `true`.
    /// You can deserialize the returned item by retrieving the [`ResponseBody`](azure_core::http::response::ResponseBody) using [`CosmosResponse::into_body`] and then calling [`ResponseBody::json`](azure_core::http::response::ResponseBody::json), like this:
    ///
    /// ```rust,no_run
    /// use azure_data_cosmos::ItemOptions;
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
    /// let options = ItemOptions::default().with_content_response_on_write_enabled(true);
    /// let updated_product: Product = container_client
    ///     .replace_item("category1", "product1", p, Some(options))
    ///     .await?
    ///     .into_body().json::<Product>()?;
    /// # }
    /// ```
    #[tracing::instrument(skip_all, fields(id = self.container_id))]
    pub async fn replace_item<T: Serialize>(
        &self,
        partition_key: impl Into<PartitionKey>,
        item_id: &str,
        item: T,
        options: Option<ItemOptions>,
    ) -> azure_core::Result<CosmosResponse<()>> {
        let link = self.items_link.item(item_id);
        let options = options.clone().unwrap_or_default();
        let excluded_regions = options.excluded_regions.clone();
        let mut cosmos_request = CosmosRequest::builder(OperationType::Replace, link)
            .json(&item)
            .partition_key(partition_key.into())
            .excluded_regions(excluded_regions)
            .build()?;
        options.apply_headers(&mut cosmos_request.headers);

        self.container_connection
            .send(cosmos_request, Context::default())
            .await
    }

    /// Creates or replaces an item in the container.
    ///
    /// If an item with the same ID is found in the container, it is updated with the provided content.
    /// If no item with the same ID is found in the container, a new item is created with the provided content.
    ///
    /// # Arguments
    /// * `partition_key` - The partition key of the item to create or replace.
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
    ///     .upsert_item("category1", p, None)
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Content Response on Write
    ///
    /// By default, the created/replaced item is *not* returned in the HTTP response.
    /// If you want the created/replaced item to be returned, set the [`ItemOptions::with_content_response_on_write_enabled()`] option to `true`.
    /// You can deserialize the returned item by retrieving the [`ResponseBody`](azure_core::http::response::ResponseBody) using [`CosmosResponse::into_body`] and then calling [`ResponseBody::json`](azure_core::http::response::ResponseBody::json), like this:
    ///
    /// ```rust,no_run
    /// use azure_data_cosmos::ItemOptions;
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
    /// let options = ItemOptions::default().with_content_response_on_write_enabled(true);
    /// let updated_product = container_client
    ///     .upsert_item("category1", p, Some(options))
    ///     .await?
    ///     .into_body().json::<Product>()?;
    /// Ok(())
    /// # }
    #[tracing::instrument(skip_all, fields(id = self.container_id))]
    pub async fn upsert_item<T: Serialize>(
        &self,
        partition_key: impl Into<PartitionKey>,
        item: T,
        options: Option<ItemOptions>,
    ) -> azure_core::Result<CosmosResponse<()>> {
        let options = options.clone().unwrap_or_default();
        let excluded_regions = options.excluded_regions.clone();
        let mut cosmos_request =
            CosmosRequest::builder(OperationType::Upsert, self.items_link.clone())
                .json(&item)
                .partition_key(partition_key.into())
                .excluded_regions(excluded_regions)
                .build()?;
        options.apply_headers(&mut cosmos_request.headers);

        return self
            .container_connection
            .send(cosmos_request, Context::default())
            .await;
    }

    /// Reads a specific item from the container.
    ///
    /// # Arguments
    /// * `partition_key` - The partition key of the item to read. See [`PartitionKey`] for more information on how to specify a partition key.
    /// * `item_id` - The id of the item to read.
    /// * `options` - Optional parameters for the request
    ///
    /// NOTE: The read item is always returned, so the [`ItemOptions::with_content_response_on_write_enabled()`] option is ignored.
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
    #[tracing::instrument(skip_all, fields(id = self.container_id))]
    pub async fn read_item<T>(
        &self,
        partition_key: impl Into<PartitionKey>,
        item_id: &str,
        options: Option<ItemOptions>,
    ) -> azure_core::Result<CosmosResponse<T>> {
        let mut options = options.unwrap_or_default();

        // Read APIs should always return the item, ignoring whatever the user set.
        options = options.with_content_response_on_write_enabled(true);

        let link = self.items_link.item(item_id);
        let excluded_regions = options.excluded_regions.clone();
        let mut cosmos_request = CosmosRequest::builder(OperationType::Read, link)
            .partition_key(partition_key.into())
            .excluded_regions(excluded_regions)
            .build()?;
        options.apply_headers(&mut cosmos_request.headers);

        self.container_connection
            .send(cosmos_request, Context::default())
            .await
    }

    /// Deletes an item from the container.
    ///
    /// # Arguments
    /// * `partition_key` - The partition key of the item to delete.
    /// * `item_id` - The id of the item to delete.
    /// * `options` - Optional parameters for the request
    ///
    /// NOTE: The deleted item is never returned by the Cosmos API, so the [`ItemOptions::with_content_response_on_write_enabled()`] option is ignored.
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
    #[tracing::instrument(skip_all, fields(id = self.container_id))]
    pub async fn delete_item(
        &self,
        partition_key: impl Into<PartitionKey>,
        item_id: &str,
        options: Option<ItemOptions>,
    ) -> azure_core::Result<CosmosResponse<()>> {
        let link = self.items_link.item(item_id);
        let options = options.clone().unwrap_or_default();
        let excluded_regions = options.excluded_regions.clone();
        let mut cosmos_request = CosmosRequest::builder(OperationType::Delete, link)
            .partition_key(partition_key.into())
            .excluded_regions(excluded_regions)
            .build()?;
        options.apply_headers(&mut cosmos_request.headers);

        self.container_connection
            .send(cosmos_request, Context::default())
            .await
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
    /// * `partition_key` - The partition key to scope the query on, or specify an empty key (`()`) to perform a cross-partition query.
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
    /// The `query` and `partition_key` parameters accept anything that can be transformed [`Into`] their relevant types.
    /// This allows simple queries without parameters to be expressed easily:
    ///
    /// ```rust,no_run
    /// # async fn doc() -> Result<(), Box<dyn std::error::Error>> {
    /// # let container_client: azure_data_cosmos::clients::ContainerClient = panic!("this is a non-running example");
    /// #[derive(serde::Deserialize)]
    /// struct Customer {
    ///     id: u64,
    ///     name: String,
    /// }
    /// let items = container_client.query_items::<Customer>(
    ///     "SELECT * FROM c",
    ///     "some_partition_key",
    ///     None)?;
    /// # }
    /// ```
    ///
    /// You can specify parameters by using [`Query::from()`] and [`Query::with_parameter()`]:
    ///
    /// ```rust,no_run
    /// # async fn doc() -> Result<(), Box<dyn std::error::Error>> {
    /// use azure_data_cosmos::Query;
    /// # let container_client: azure_data_cosmos::clients::ContainerClient = panic!("this is a non-running example");
    /// #[derive(serde::Deserialize)]
    /// struct Customer {
    ///     id: u64,
    ///     name: String,
    /// }
    /// let query = Query::from("SELECT COUNT(*) FROM c WHERE c.customer_id = @customer_id")
    ///     .with_parameter("@customer_id", 42)?;
    /// let items = container_client.query_items::<Customer>(query, "some_partition_key", None)?;
    /// # }
    /// ```
    ///
    /// See [`PartitionKey`](crate::PartitionKey) for more information on how to specify a partition key, and [`Query`] for more information on how to specify a query.
    #[tracing::instrument(skip_all, fields(id = self.container_id))]
    pub fn query_items<T: DeserializeOwned + Send + 'static>(
        &self,
        query: impl Into<Query>,
        partition_key: impl Into<PartitionKey>,
        options: Option<QueryOptions>,
    ) -> azure_core::Result<FeedItemIterator<T>> {
        let options = options.unwrap_or_default();
        let partition_key = partition_key.into();
        let query = query.into();

        let mut headers = azure_core::http::headers::Headers::new();

        // Convert PartitionKey and query options into headers.
        for (name, value) in partition_key.as_headers()? {
            headers.insert(name, value);
        }
        options.apply_headers(&mut headers);

        crate::query::executor::QueryExecutor::new(
            self.pipeline.clone(),
            self.items_link.clone(),
            Context::default(),
            query,
            headers,
        )
        .into_stream()
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
    #[tracing::instrument(skip_all, fields(id = self.container_id))]
    pub async fn execute_transactional_batch(
        &self,
        batch: TransactionalBatch,
        options: Option<BatchOptions>,
    ) -> azure_core::Result<CosmosResponse<TransactionalBatchResponse>> {
        let options = options.unwrap_or_default();
        let partition_key = batch.partition_key().clone();

        let mut cosmos_request =
            CosmosRequest::builder(OperationType::Batch, self.items_link.clone())
                .partition_key(partition_key)
                .json(batch.operations())
                .build()?;
        options.apply_headers(&mut cosmos_request.headers);

        self.container_connection
            .send(cosmos_request, Context::default())
            .await
    }
}

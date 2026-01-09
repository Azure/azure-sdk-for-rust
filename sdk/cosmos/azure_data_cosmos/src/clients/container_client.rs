// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use crate::{
    models::{ContainerProperties, PatchDocument, ThroughputProperties},
    options::{QueryOptions, ReadContainerOptions},
    pipeline::CosmosPipeline,
    resource_context::{ResourceLink, ResourceType},
    DeleteContainerOptions, FeedPager, ItemOptions, PartitionKey, Query, ReplaceContainerOptions,
    ThroughputOptions,
};
use std::sync::Arc;

use crate::cosmos_request::CosmosRequest;
use crate::operation_context::OperationType;
use azure_core::http::response::Response;
use serde::{de::DeserializeOwned, Serialize};

/// A client for working with a specific container in a Cosmos DB account.
///
/// You can get a `Container` by calling [`DatabaseClient::container_client()`](crate::clients::DatabaseClient::container_client()).
#[derive(Clone)]
pub struct ContainerClient {
    link: ResourceLink,
    items_link: ResourceLink,
    pipeline: Arc<CosmosPipeline>,
    container_id: String,
}

impl ContainerClient {
    pub(crate) fn new(
        pipeline: Arc<CosmosPipeline>,
        database_link: &ResourceLink,
        container_id: &str,
    ) -> Self {
        let link = database_link
            .feed(ResourceType::Containers)
            .item(container_id);
        let items_link = link.feed(ResourceType::Documents);

        Self {
            link,
            items_link,
            pipeline,
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
        options: Option<ReadContainerOptions<'_>>,
    ) -> azure_core::Result<Response<ContainerProperties>> {
        let options = options.unwrap_or_default();
        let cosmos_request =
            CosmosRequest::builder(OperationType::Read, self.link.clone()).build()?;
        self.pipeline
            .send(cosmos_request, options.method_options.context)
            .await
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
    /// let new_properties = ContainerProperties {
    ///     id: "MyContainer".into(),
    ///     partition_key: "/id".into(),
    ///     indexing_policy: Some(IndexingPolicy {
    ///         included_paths: vec!["/index_me".into()],
    ///         ..Default::default()
    ///     }),
    ///     ..Default::default()
    /// };
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
        options: Option<ReplaceContainerOptions<'_>>,
    ) -> azure_core::Result<Response<ContainerProperties>> {
        let options = options.unwrap_or_default();
        let cosmos_request = CosmosRequest::builder(OperationType::Replace, self.link.clone())
            .json(&properties)
            .build()?;
        self.pipeline
            .send(cosmos_request, options.method_options.context)
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
        options: Option<ThroughputOptions<'_>>,
    ) -> azure_core::Result<Option<ThroughputProperties>> {
        let options = options.unwrap_or_default();

        // We need to get the RID for the database.
        let db = self.read(None).await?.into_model()?;
        let resource_id = db
            .system_properties
            .resource_id
            .expect("service should always return a '_rid' for a container");

        self.pipeline
            .read_throughput_offer(options.method_options.context, &resource_id)
            .await
    }

    /// Replaces the container throughput properties.
    ///
    /// # Arguments
    /// * `throughput` - The new throughput properties to set.
    /// * `options` - Optional parameters for the request.
    #[tracing::instrument(skip_all, fields(id = self.container_id))]
    pub async fn replace_throughput(
        &self,
        throughput: ThroughputProperties,
        options: Option<ThroughputOptions<'_>>,
    ) -> azure_core::Result<Response<ThroughputProperties>> {
        let options = options.unwrap_or_default();

        // We need to get the RID for the database.
        let db = self.read(None).await?.into_model()?;
        let resource_id = db
            .system_properties
            .resource_id
            .expect("service should always return a '_rid' for a container");

        self.pipeline
            .replace_throughput_offer(options.method_options.context, &resource_id, throughput)
            .await
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
        options: Option<DeleteContainerOptions<'_>>,
    ) -> azure_core::Result<Response<()>> {
        let options = options.unwrap_or_default();
        let cosmos_request =
            CosmosRequest::builder(OperationType::Delete, self.link.clone()).build()?;
        self.pipeline
            .send(cosmos_request, options.method_options.context)
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
    /// If you want the new item to be returned, set the [`ItemOptions::enable_content_response_on_write`] option to `true`.
    /// You can deserialize the returned item by retrieving the [`ResponseBody`](azure_core::http::response::ResponseBody) using [`Response::into_body`] and then calling [`ResponseBody::json`](azure_core::http::response::ResponseBody::json), like this:
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
    /// let options = ItemOptions {
    ///     enable_content_response_on_write: true,
    ///     ..Default::default()
    /// };
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
        options: Option<ItemOptions<'_>>,
    ) -> azure_core::Result<Response<()>> {
        let options = options.clone().unwrap_or_default();
        let cosmos_request = CosmosRequest::builder(OperationType::Create, self.items_link.clone())
            .request_headers(&options)
            .json(&item)
            .partition_key(partition_key.into())
            .build()?;

        self.pipeline
            .send(cosmos_request, options.method_options.context)
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
    /// If you want the replaced item to be returned, set the [`ItemOptions::enable_content_response_on_write`] option to `true`.
    /// You can deserialize the returned item by retrieving the [`ResponseBody`](azure_core::http::response::ResponseBody) using [`Response::into_body`] and then calling [`ResponseBody::json`](azure_core::http::response::ResponseBody::json), like this:
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
    /// let options = ItemOptions {
    ///     enable_content_response_on_write: true,
    ///     ..Default::default()
    /// };
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
        options: Option<ItemOptions<'_>>,
    ) -> azure_core::Result<Response<()>> {
        let link = self.items_link.item(item_id);
        let options = options.clone().unwrap_or_default();
        let cosmos_request = CosmosRequest::builder(OperationType::Replace, link)
            .request_headers(&options)
            .json(&item)
            .partition_key(partition_key.into())
            .build()?;

        self.pipeline
            .send(cosmos_request, options.method_options.context)
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
    /// If you want the created/replaced item to be returned, set the [`ItemOptions::enable_content_response_on_write`] option to `true`.
    /// You can deserialize the returned item by retrieving the [`ResponseBody`](azure_core::http::response::ResponseBody) using [`Response::into_body`] and then calling [`ResponseBody::json`](azure_core::http::response::ResponseBody::json), like this:
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
    /// let options = ItemOptions {
    ///     enable_content_response_on_write: true,
    ///     ..Default::default()
    /// };
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
        options: Option<ItemOptions<'_>>,
    ) -> azure_core::Result<Response<()>> {
        let options = options.clone().unwrap_or_default();
        let cosmos_request = CosmosRequest::builder(OperationType::Upsert, self.items_link.clone())
            .request_headers(&options)
            .json(&item)
            .partition_key(partition_key.into())
            .build()?;

        self.pipeline
            .send(cosmos_request, options.method_options.context)
            .await
    }

    /// Reads a specific item from the container.
    ///
    /// # Arguments
    /// * `partition_key` - The partition key of the item to read. See [`PartitionKey`] for more information on how to specify a partition key.
    /// * `item_id` - The id of the item to read.
    /// * `options` - Optional parameters for the request
    ///
    /// NOTE: The read item is always returned, so the [`ItemOptions::enable_content_response_on_write`] option is ignored.
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
        options: Option<ItemOptions<'_>>,
    ) -> azure_core::Result<Response<T>> {
        let mut options = options.unwrap_or_default();

        // Read APIs should always return the item, ignoring whatever the user set.
        options.enable_content_response_on_write = true;

        let link = self.items_link.item(item_id);
        let cosmos_request = CosmosRequest::builder(OperationType::Read, link)
            .partition_key(partition_key.into())
            .request_headers(&options)
            .build()?;

        self.pipeline
            .send(cosmos_request, options.method_options.context)
            .await
    }

    /// Deletes an item from the container.
    ///
    /// # Arguments
    /// * `partition_key` - The partition key of the item to delete.
    /// * `item_id` - The id of the item to delete.
    /// * `options` - Optional parameters for the request
    ///
    /// NOTE: The deleted item is never returned by the Cosmos API, so the [`ItemOptions::enable_content_response_on_write`] option is ignored.
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
        options: Option<ItemOptions<'_>>,
    ) -> azure_core::Result<Response<()>> {
        let link = self.items_link.item(item_id);
        let options = options.clone().unwrap_or_default();
        let cosmos_request = CosmosRequest::builder(OperationType::Delete, link)
            .partition_key(partition_key.into())
            .request_headers(&options)
            .build()?;

        self.pipeline
            .send(cosmos_request, options.method_options.context)
            .await
    }

    /// Patches an item in the container.
    ///
    /// # Arguments
    /// * `partition_key` - The partition key of the item to patch.
    /// * `item_id` - The id of the item to patch.
    /// * `patch` - The patch document to apply to the item.
    /// * `options` - Optional parameters for the request.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use azure_data_cosmos::models::PatchDocument;
    /// use serde::{Deserialize, Serialize};
    /// # async fn doc() -> Result<(), Box<dyn std::error::Error>> {
    /// # let container_client: azure_data_cosmos::clients::ContainerClient = panic!("this is a non-running example");
    /// let patch = PatchDocument::default().with_add("/some/path", "some value")?;
    /// container_client
    ///     .patch_item("partition1", "item1", patch, None)
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Content Response on Write
    ///
    /// By default, the patched item is *not* returned in the HTTP response.
    /// If you want the patched item to be returned, set the [`ItemOptions::enable_content_response_on_write`] option to `true`.
    /// You can deserialize the returned item by retrieving the [`ResponseBody`](azure_core::http::response::ResponseBody) using [`Response::into_body`] and then calling [`ResponseBody::json`](azure_core::http::response::ResponseBody::json), like this:
    ///
    /// For example:
    ///
    /// ```rust,no_run
    /// use azure_data_cosmos::{models::PatchDocument, ItemOptions};
    /// use serde::Deserialize;
    /// # async fn doc() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client: azure_data_cosmos::clients::ContainerClient = panic!("this is a non-running example");
    /// #[derive(Debug, Deserialize)]
    /// pub struct Product {
    ///     #[serde(rename = "id")] // Use serde attributes to control serialization
    ///     product_id: String,
    ///     category_id: String,
    ///     product_name: String,
    /// }
    /// let options = ItemOptions {
    ///     enable_content_response_on_write: true,
    ///     ..Default::default()
    /// };
    /// let patch = PatchDocument::default().with_add("/some/path", "some value")?;
    /// let patched_item = client
    ///     .patch_item("partition1", "item1", patch, Some(options))
    ///     .await?
    ///     .into_body().json::<Product>()?;
    /// # Ok(())
    /// # }
    /// ```
    #[tracing::instrument(skip_all, fields(id = self.container_id))]
    pub async fn patch_item(
        &self,
        partition_key: impl Into<PartitionKey>,
        item_id: &str,
        patch: PatchDocument,
        options: Option<ItemOptions<'_>>,
    ) -> azure_core::Result<Response<()>> {
        let options = options.clone().unwrap_or_default();
        let link = self.items_link.item(item_id);
        let cosmos_request = CosmosRequest::builder(OperationType::Patch, link)
            .partition_key(partition_key.into())
            .request_headers(&options)
            .json(&patch)
            .build()?;

        self.pipeline
            .send(cosmos_request, options.method_options.context)
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
        options: Option<QueryOptions<'_>>,
    ) -> azure_core::Result<FeedPager<T>> {
        #[cfg_attr(not(feature = "preview_query_engine"), allow(unused_mut))]
        let mut options = options.unwrap_or_default();
        let partition_key = partition_key.into();
        let query = query.into();
        let ctx = options.method_options.context.clone();

        #[cfg(feature = "preview_query_engine")]
        if partition_key.is_empty() {
            if let Some(query_engine) = options.query_engine.take() {
                return crate::query::executor::QueryExecutor::new(
                    self.pipeline.clone(),
                    self.link.clone(),
                    query,
                    options,
                    query_engine,
                )?
                .into_stream();
            }
        }

        let url = self.pipeline.url(&self.items_link);
        self.pipeline
            .send_query_request(ctx, query, url, self.items_link.clone(), |r| {
                r.insert_headers(&options)?;
                r.insert_headers(&partition_key)?;
                Ok(())
            })
    }
}

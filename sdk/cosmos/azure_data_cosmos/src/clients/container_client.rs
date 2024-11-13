// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use crate::{
    constants,
    models::{ContainerProperties, Item, PatchDocument, QueryResults},
    options::{QueryOptions, ReadContainerOptions},
    pipeline::CosmosPipeline,
    resource_context::{ResourceLink, ResourceType},
    DeleteContainerOptions, ItemOptions, PartitionKey, Query, QueryPartitionStrategy,
};

use azure_core::{Method, Pager, Request, Response};
use serde::{de::DeserializeOwned, Serialize};

/// A client for working with a specific container in a Cosmos DB account.
///
/// You can get a `Container` by calling [`DatabaseClient::container_client()`](crate::clients::DatabaseClient::container_client()).
pub struct ContainerClient {
    link: ResourceLink,
    items_link: ResourceLink,
    pipeline: CosmosPipeline,
}

impl ContainerClient {
    pub(crate) fn new(
        pipeline: CosmosPipeline,
        database_link: &ResourceLink,
        container_id: &str,
    ) -> Self {
        let link = database_link
            .feed(ResourceType::Containers)
            .item(container_id);
        let items_link = link.feed(ResourceType::Items);

        Self {
            link,
            items_link,
            pipeline,
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
    /// # async fn doc() {
    /// # use azure_data_cosmos::clients::ContainerClient;
    /// # let container_client: ContainerClient = panic!("this is a non-running example");
    /// let response = container_client.read(None)
    ///     .await.unwrap()
    ///     .deserialize_body()
    ///     .await.unwrap();
    /// # }
    /// ```
    pub async fn read(
        &self,
        options: Option<ReadContainerOptions<'_>>,
    ) -> azure_core::Result<Response<ContainerProperties>> {
        let url = self.pipeline.url(&self.link);
        let mut req = Request::new(url, Method::Get);
        self.pipeline
            .send(
                options.map(|o| o.method_options.context),
                &mut req,
                self.link.clone(),
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
        options: Option<DeleteContainerOptions<'_>>,
    ) -> azure_core::Result<Response> {
        let url = self.pipeline.url(&self.link);
        let mut req = Request::new(url, Method::Delete);
        self.pipeline
            .send(
                options.map(|o| o.method_options.context),
                &mut req,
                self.link.clone(),
            )
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
    /// # use azure_data_cosmos::{clients::ContainerClient, models::Item};
    /// # use serde::{Deserialize, Serialize};
    /// # async fn doc() {
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
    /// # let container_client: ContainerClient = panic!("this is a non-running example");
    /// let created_item = container_client
    ///     .create_item("category1", p, None)
    ///     .await.unwrap()
    ///     .deserialize_body()
    ///     .await.unwrap()
    ///     .unwrap();
    /// println!("Created: {:#?}", created_item);
    /// # }
    /// ```
    pub async fn create_item<T: Serialize>(
        &self,
        partition_key: impl Into<PartitionKey>,
        item: T,
        options: Option<ItemOptions<'_>>,
    ) -> azure_core::Result<Response<Item<T>>> {
        let url = self.pipeline.url(&self.items_link);
        let mut req = Request::new(url, Method::Post);
        req.insert_headers(&partition_key.into())?;
        req.set_json(&item)?;
        self.pipeline
            .send(
                options.map(|o| o.method_options.context),
                &mut req,
                self.items_link.clone(),
            )
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
    /// # use azure_data_cosmos::{clients::ContainerClient, models::Item};
    /// # use serde::{Deserialize, Serialize};
    /// # async fn doc() {
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
    /// # let container_client: ContainerClient = panic!("this is a non-running example");
    /// let updated_item = container_client
    ///     .replace_item("category1", "product1", p, None)
    ///     .await.unwrap()
    ///     .deserialize_body()
    ///     .await.unwrap()
    ///     .unwrap();
    /// println!("Updated Item: {:#?}", updated_item);
    /// # }
    /// ```
    pub async fn replace_item<T: Serialize>(
        &self,
        partition_key: impl Into<PartitionKey>,
        item_id: &str,
        item: T,

        #[allow(unused_variables)]
        // REASON: This is a documented public API so prefixing with '_' is undesirable.
        options: Option<ItemOptions<'_>>,
    ) -> azure_core::Result<Response<Item<T>>> {
        let link = self.items_link.item(item_id);
        let url = self.pipeline.url(&link);
        let mut req = Request::new(url, Method::Put);
        req.insert_headers(&partition_key.into())?;
        req.set_json(&item)?;
        self.pipeline
            .send(options.map(|o| o.method_options.context), &mut req, link)
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
    /// # use azure_data_cosmos::{clients::ContainerClient, models::Item};
    /// # use serde::{Deserialize, Serialize};
    /// # async fn doc() {
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
    /// # let container_client: ContainerClient = panic!("this is a non-running example");
    /// let updated_item = container_client
    ///     .upsert_item("category1", p, None)
    ///     .await.unwrap()
    ///     .deserialize_body()
    ///     .await.unwrap()
    ///     .unwrap();
    /// println!("Updated Item: {:#?}", updated_item);
    /// # }
    /// ```
    pub async fn upsert_item<T: Serialize>(
        &self,
        partition_key: impl Into<PartitionKey>,
        item: T,
        options: Option<ItemOptions<'_>>,
    ) -> azure_core::Result<Response<Item<T>>> {
        let url = self.pipeline.url(&self.items_link);
        let mut req = Request::new(url, Method::Post);
        req.insert_header(constants::IS_UPSERT, "true");
        req.insert_headers(&partition_key.into())?;
        req.set_json(&item)?;
        self.pipeline
            .send(
                options.map(|o| o.method_options.context),
                &mut req,
                self.items_link.clone(),
            )
            .await
    }

    /// Reads a specific item from the container.
    ///
    /// # Arguments
    /// * `partition_key` - The partition key of the item to read.
    /// * `item_id` - The id of the item to read.
    /// * `options` - Optional parameters for the request
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use azure_data_cosmos::{clients::ContainerClient, models::Item};
    /// # use serde::{Deserialize, Serialize};
    /// # async fn doc() {
    /// #[derive(Debug, Deserialize, Serialize)]
    /// pub struct Product {
    ///     #[serde(rename = "id")] // Use serde attributes to control serialization
    ///     product_id: String,
    ///     category_id: String,
    ///     product_name: String,
    /// }
    /// # let container_client: ContainerClient = panic!("this is a non-running example");
    /// let item: Product = container_client
    ///     .read_item("partition1", "item1", None)
    ///     .await.unwrap()
    ///     .deserialize_body()
    ///     .await.unwrap()
    ///     .unwrap();
    /// println!("Read Item: {:#?}", item);
    /// # }
    /// ```
    pub async fn read_item<T: DeserializeOwned>(
        &self,
        partition_key: impl Into<PartitionKey>,
        item_id: &str,
        options: Option<ItemOptions<'_>>,
    ) -> azure_core::Result<Response<Item<T>>> {
        let link = self.items_link.item(item_id);
        let url = self.pipeline.url(&link);
        let mut req = Request::new(url, Method::Get);
        req.insert_headers(&partition_key.into())?;
        self.pipeline
            .send(options.map(|o| o.method_options.context), &mut req, link)
            .await
    }

    /// Deletes an item from the container.
    ///
    /// # Arguments
    /// * `partition_key` - The partition key of the item to delete.
    /// * `item_id` - The id of the item to delete.
    /// * `options` - Optional parameters for the request
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use azure_data_cosmos::{clients::ContainerClient, models::Item};
    /// # use serde::{Deserialize, Serialize};
    /// # async fn doc() {
    /// # let container_client: ContainerClient = panic!("this is a non-running example");
    /// container_client
    ///     .delete_item("partition1", "item1", None)
    ///     .await.unwrap();
    /// # }
    /// ```
    pub async fn delete_item(
        &self,
        partition_key: impl Into<PartitionKey>,
        item_id: &str,
        options: Option<ItemOptions<'_>>,
    ) -> azure_core::Result<Response> {
        let link = self.items_link.item(item_id);
        let url = self.pipeline.url(&link);
        let mut req = Request::new(url, Method::Delete);
        req.insert_headers(&partition_key.into())?;
        self.pipeline
            .send(options.map(|o| o.method_options.context), &mut req, link)
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
    /// # use azure_data_cosmos::{clients::ContainerClient, models::Item};
    /// # use serde::{Deserialize, Serialize};
    /// # async fn doc() {
    /// # let container_client: ContainerClient = panic!("this is a non-running example");
    /// let patch = PatchDocument::new().with_add("/some/path", "some value").unwrap();
    /// container_client
    ///     .patch_item("partition1", "item1", patch, None)
    ///     .await.unwrap();
    /// # }
    /// ```
    ///
    /// # Note
    ///
    /// The Cosmos service does return the patched item in the response.
    /// However, this method does not return `Response<T>` since that would **force** users to provide a generic type parameter, even when they do not wish to deserialize the body.
    /// If you want to deserialize the response, you can use [`Response::deserialize_body_into`] to manually deserialize the body.
    ///
    /// For example:
    ///
    /// ```rust,no_run
    /// # use azure_data_cosmos::{clients::ContainerClient, models::PatchDocument};
    /// # let client: ContainerClient = panic!("this is a non-running example");
    /// let patch = PatchDocument::new().with_add("/some/path", "some value").unwrap();
    /// let response = client.patch_item("partition1", "item1", patch, None).await.unwrap();
    /// let patched_item: serde_json::Value = response.deserialize_body_into().await.unwrap();
    /// ```
    pub async fn patch_item(
        &self,
        partition_key: impl Into<PartitionKey>,
        item_id: &str,
        patch: PatchDocument,
        options: Option<ItemOptions<'_>>,
    ) -> azure_core::Result<Response> {
        let link = self.items_link.item(item_id);
        let url = self.pipeline.url(&link);
        let mut req = Request::new(url, Method::Patch);
        req.insert_headers(&partition_key.into())?;
        req.set_json(&patch)?;

        self.pipeline
            .send(options.map(|o| o.method_options.context), &mut req, link)
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
    /// * `partition_key_strategy` - The partition key to scope the query on.
    /// * `options` - Optional parameters for the request.
    ///
    /// # Examples
    ///
    /// The `query` and `partition_key_strategy` parameters accept anything that can be transformed [`Into`] their relevant types.
    /// This allows simple queries without parameters to be expressed easily:
    ///
    /// ```rust,no_run
    /// # async fn doc() {
    /// # use azure_data_cosmos::clients::ContainerClient;
    /// # let container_client: ContainerClient = panic!("this is a non-running example");
    /// #[derive(serde::Deserialize)]
    /// struct Customer {
    ///     id: u64,
    ///     name: String,
    /// }
    /// let items = container_client.query_items::<Customer>(
    ///     "SELECT * FROM c",
    ///     "some_partition_key",
    ///     None).unwrap();
    /// # }
    /// ```
    ///
    /// You can specify parameters by using [`Query::from()`] and [`Query::with_parameter()`]:
    ///
    /// ```rust,no_run
    /// # async fn doc() {
    /// # use azure_data_cosmos::{clients::ContainerClient, Query};
    /// # let container_client: ContainerClient = panic!("this is a non-running example");
    /// #[derive(serde::Deserialize)]
    /// struct Customer {
    ///     id: u64,
    ///     name: String,
    /// }
    /// let query = Query::from("SELECT COUNT(*) FROM c WHERE c.customer_id = @customer_id")
    ///     .with_parameter("@customer_id", 42).unwrap();
    /// let items = container_client.query_items::<Customer>(query, "some_partition_key", None).unwrap();
    /// # }
    /// ```
    ///
    /// See [`PartitionKey`](crate::PartitionKey) for more information on how to specify a partition key, and [`Query`] for more information on how to specify a query.
    pub fn query_items<T: DeserializeOwned + Send>(
        &self,
        query: impl Into<Query>,
        partition_key: impl Into<QueryPartitionStrategy>,
        options: Option<QueryOptions<'_>>,
    ) -> azure_core::Result<Pager<QueryResults<T>>> {
        let url = self.pipeline.url(&self.items_link);
        let mut base_request = Request::new(url, Method::Post);
        let QueryPartitionStrategy::SinglePartition(partition_key) = partition_key.into();
        base_request.insert_headers(&partition_key)?;

        self.pipeline.send_query_request(
            options.map(|o| o.method_options.context),
            query.into(),
            base_request,
            self.items_link.clone(),
        )
    }
}

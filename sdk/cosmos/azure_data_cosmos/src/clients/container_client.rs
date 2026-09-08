// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use crate::{
    clients::ClientContext,
    diagnostics::CosmosOperationContext,
    feed::{ChangeFeedPageIterator, FeedRange, FeedScope, QueryItemIterator},
    models::{BatchResponse, ChangeFeedItem, ItemResponse, TransactionalBatch},
    options::{
        BatchOptions, BinaryEncodingOptions, ChangeFeedMode, ChangeFeedOptions,
        ChangeFeedStartFrom, ItemReadOptions, ItemWriteOptions, OperationOptions, Precondition,
        QueryOptions, ReadContainerOptions, ReadFeedRangesOptions, SessionToken,
    },
    PartitionKey, Query, ResourceIdentity,
};
#[cfg(feature = "preview_patch")]
use crate::{models::PatchInstructions, options::PatchItemOptions};

use azure_data_cosmos_driver::models::{
    ContainerReference, CosmosOperation, ItemReference, PartitionKeyKind,
};
use serde::{de::DeserializeOwned, Serialize};

use crate::models::{ContainerProperties, ResourceResponse};

#[cfg(feature = "control_plane")]
use super::ThroughputPoller;
#[cfg(feature = "control_plane")]
use crate::{
    models::ThroughputProperties,
    options::{DeleteContainerOptions, ReplaceContainerOptions, ThroughputOptions},
};

/// A client for working with a specific container in a Cosmos DB account.
///
/// You can get a `Container` by calling [`DatabaseClient::container_client()`](crate::clients::DatabaseClient::container_client()).
#[derive(Clone)]
pub struct ContainerClient {
    container_ref: ContainerReference,
    context: ClientContext,
}

impl ContainerClient {
    /// Returns the resolved [`ContainerReference`] for the container this client is attached to.
    #[cfg(feature = "preview_dtx")]
    pub(crate) fn container_reference(&self) -> &ContainerReference {
        &self.container_ref
    }

    pub(crate) async fn new(
        context: ClientContext,
        database: &ResourceIdentity,
        container: ResourceIdentity,
        options: crate::options::ContainerClientOptions,
    ) -> crate::Result<Self> {
        // The container's addressing mode must match the database's: name-with-name
        // or RID-with-RID. Mixing the two is not supported by the service routing.
        let container_ref = match (database, &container) {
            (ResourceIdentity::Name(db_name), ResourceIdentity::Name(container_name)) => context
                .driver
                .resolve_container(db_name, container_name, options.operation)
                .await
                .map_err(|e| {
                    azure_data_cosmos_driver::error::CosmosErrorBuilder::from_error(e)
                        .with_context(format!(
                            "failed to resolve container metadata for '{db_name}/{container_name}'"
                        ))
                        .build()
                })?,
            (ResourceIdentity::Rid(db_rid), ResourceIdentity::Rid(container_rid)) => {
                let resolved = context
                    .driver
                    .resolve_container_by_rid(container_rid.as_str(), options.operation)
                    .await
                    .map_err(|e| {
                        azure_data_cosmos_driver::error::CosmosErrorBuilder::from_error(e)
                            .with_context(format!(
                                "failed to resolve container metadata for RID '{}'",
                                container_rid.as_str()
                            ))
                            .build()
                    })?;
                if resolved.database_rid() != db_rid.as_str() {
                    return Err(azure_data_cosmos_driver::error::CosmosError::builder()
                        .with_status(azure_data_cosmos_driver::error::CosmosStatus::CLIENT_INVALID_RESOURCE_ID)
                        .with_message(format!("container RID '{}' belongs to database '{}', not the addressed database '{}'", container_rid.as_str(), resolved.database_rid(), db_rid.as_str()))
                        .build()
                        .into());
                }
                resolved
            }
            (ResourceIdentity::Name(_), ResourceIdentity::Rid(_))
            | (ResourceIdentity::Rid(_), ResourceIdentity::Name(_)) => {
                return Err(azure_data_cosmos_driver::error::CosmosError::builder()
                    .with_status(azure_data_cosmos_driver::error::CosmosStatus::CLIENT_MIXED_NAME_RID_ADDRESSING)
                    .with_message("database and container must use the same addressing mode: address both by name or both by RID")
                    .build()
                    .into());
            }
        };

        Ok(Self {
            container_ref,
            context,
        })
    }
    /// Builds the SDK-side [`CosmosOperationContext`] for this container's
    /// operations, carrying the operation name plus the database and container
    /// identity the driver context does not know.
    fn operation_context(&self, operation_name: &'static str) -> CosmosOperationContext {
        let context = CosmosOperationContext::new()
            .with_operation_name(operation_name)
            .with_container_name(self.container_ref.name().to_string());
        match self.container_ref.database_name() {
            Some(name) => context.with_database_name(name.to_string()),
            None => context,
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
    pub async fn read(
        &self,
        options: Option<ReadContainerOptions>,
    ) -> crate::Result<ResourceResponse<ContainerProperties>> {
        let options = options.unwrap_or_default();
        let operation = CosmosOperation::read_container(self.container_ref.clone());

        let driver_result = self
            .context
            .driver
            .execute_singleton_operation(operation, options.operation)
            .await;

        Ok(ResourceResponse::new(
            self.context
                .complete_result(driver_result, || self.operation_context("read_container"))?,
        ))
    }

    /// Updates the indexing policy of the container.
    ///
    /// **NOTE**: The [`ContainerProperties::id`] and [`ContainerProperties::partition_key`] must be the same as the existing container, they cannot be changed.
    ///
    #[doc = include_str!("../../docs/control-plane-always-returns-body.md")]
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
    #[cfg(feature = "control_plane")]
    pub async fn replace(
        &self,
        properties: ContainerProperties,
        options: Option<ReplaceContainerOptions>,
    ) -> crate::Result<ResourceResponse<ContainerProperties>> {
        let options = options.unwrap_or_default();
        let body = serde_json::to_vec(&properties)?;
        let operation =
            CosmosOperation::replace_container(self.container_ref.clone()).with_body(body);

        // Control-plane replaces always need the full response body so the
        // caller can inspect the updated resource properties.
        let mut operation_options = options.operation;
        operation_options.content_response_on_write =
            Some(azure_data_cosmos_driver::options::ContentResponseOnWrite::Enabled);

        let driver_result = self
            .context
            .driver
            .execute_singleton_operation(operation, operation_options)
            .await;

        Ok(ResourceResponse::new(
            self.context.complete_result(driver_result, || {
                self.operation_context("replace_container")
            })?,
        ))
    }

    /// Reads container throughput properties, if any.
    ///
    /// This will return `None` if the database does not have a throughput offer configured.
    ///
    /// # Arguments
    /// * `options` - Optional parameters for the request.
    #[cfg(feature = "control_plane")]
    pub async fn read_throughput(
        &self,
        options: Option<ThroughputOptions>,
    ) -> crate::Result<Option<ThroughputProperties>> {
        let options = options.unwrap_or_default();
        crate::clients::offers_client::find_offer_for_container(
            &self.context,
            &self.container_ref,
            options.operation,
            self.operation_context("read_container_throughput"),
        )
        .await
    }

    /// Begins replacing the container throughput properties.
    ///
    /// The Cosmos DB service may process throughput changes asynchronously. The returned
    /// [`ThroughputPoller`] can be awaited directly for the final result, or polled as a
    /// stream to observe progress.
    ///
    #[doc = include_str!("../../docs/control-plane-always-returns-body.md")]
    ///
    /// # Arguments
    /// * `throughput` - The new throughput properties to set.
    /// * `options` - Optional parameters for the request.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use azure_data_cosmos::models::ThroughputProperties;
    /// # async fn example(container_client: azure_data_cosmos::clients::ContainerClient) -> azure_data_cosmos::Result<()> {
    /// let throughput = container_client
    ///     .begin_replace_throughput(ThroughputProperties::manual(500), None)
    ///     .await? // start the replace operation
    ///     .await? // wait for completion (polls if async)
    ///     .into_model()?;
    /// # Ok(())
    /// # }
    /// ```
    #[cfg(feature = "control_plane")]
    pub async fn begin_replace_throughput(
        &self,
        throughput: ThroughputProperties,
        options: Option<ThroughputOptions>,
    ) -> crate::Result<ThroughputPoller> {
        let options = options.unwrap_or_default();

        crate::clients::offers_client::begin_replace_for_container(
            self.context.clone(),
            &self.container_ref,
            throughput,
            options.operation,
            self.operation_context("replace_container_throughput"),
        )
        .await
    }

    /// Deletes this container.
    ///
    #[doc = include_str!("../../docs/control-plane-warning.md")]
    ///
    /// # Arguments
    /// * `options` - Optional parameters for the request.
    #[cfg(feature = "control_plane")]
    pub async fn delete(
        &self,
        options: Option<DeleteContainerOptions>,
    ) -> crate::Result<ResourceResponse<()>> {
        let options = options.unwrap_or_default();
        let operation = CosmosOperation::delete_container(self.container_ref.clone());

        let driver_result = self
            .context
            .driver
            .execute_singleton_operation(operation, options.operation)
            .await;

        Ok(ResourceResponse::new(
            self.context
                .complete_result(driver_result, || self.operation_context("delete_container"))?,
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
    /// If you want the new item to be returned, set `content_response_on_write` to [`ContentResponseOnWrite::Enabled`](crate::options::ContentResponseOnWrite::Enabled) on the [`OperationOptions`](crate::options::OperationOptions) in your [`ItemWriteOptions`](crate::options::ItemWriteOptions).
    /// You can deserialize the returned item by retrieving the [`ResponseBody`](crate::models::ResponseBody) using [`ItemResponse::into_body`] and then calling [`ResponseBody::into_single`](crate::models::ResponseBody::into_single), like this:
    ///
    /// ```rust,no_run
    /// use azure_data_cosmos::options::{ItemWriteOptions, ContentResponseOnWrite, OperationOptions};
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
    ///     .into_body().into_single::<Product>()?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn create_item<T: Serialize>(
        &self,
        partition_key: impl Into<PartitionKey>,
        item_id: &str,
        item: T,
        options: Option<ItemWriteOptions>,
    ) -> crate::Result<ItemResponse> {
        let options = options.unwrap_or_default();
        let (operation_options, binary) =
            resolve_binary_encoding(options.operation, &self.context.binary_encoding);
        let body = serialize_item_body(&item, binary.enabled)?;

        // Build the driver's item reference from our stored container metadata.
        let item_ref = ItemReference::from_name(
            &self.container_ref,
            partition_key.into(),
            item_id.to_owned(),
        );

        // Create the driver operation and apply ItemWriteOptions fields.
        let operation = CosmosOperation::create_item(item_ref).with_body(body);
        let operation = apply_item_options(operation, options.session_token, options.precondition);

        // Execute through the driver, with binary encoding on the operation
        // options so the driver negotiates the wire format and transcoding.
        let driver_result = self
            .context
            .driver
            .execute_singleton_operation(operation, operation_options)
            .await;

        // Bridge the driver response to the SDK response type.
        Ok(ItemResponse::new(
            self.context
                .complete_result(driver_result, || self.operation_context("create_item"))?,
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
    /// If you want the replaced item to be returned, set `content_response_on_write` to [`ContentResponseOnWrite::Enabled`](crate::options::ContentResponseOnWrite::Enabled) on the [`OperationOptions`](crate::options::OperationOptions) in your [`ItemWriteOptions`](crate::options::ItemWriteOptions).
    /// You can deserialize the returned item by retrieving the [`ResponseBody`](crate::models::ResponseBody) using [`ItemResponse::into_body`] and then calling [`ResponseBody::into_single`](crate::models::ResponseBody::into_single), like this:
    ///
    /// ```rust,no_run
    /// use azure_data_cosmos::options::{ItemWriteOptions, ContentResponseOnWrite, OperationOptions};
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
    ///     .replace_item("category1", "product1", p, Some(options))
    ///     .await?
    ///     .into_body().into_single::<Product>()?;
    /// # }
    /// ```
    ///
    /// # Tracked PATCH Items
    ///
    /// If the item participates in client-side tracked PATCH, this full-document
    /// replacement must preserve `_azsdkPatchTracking` and its array order.
    /// Models that do not explicitly represent the property should capture
    /// unknown fields with `#[serde(flatten)]` and round-trip them unchanged.
    pub async fn replace_item<T: Serialize>(
        &self,
        partition_key: impl Into<PartitionKey>,
        item_id: &str,
        item: T,
        options: Option<ItemWriteOptions>,
    ) -> crate::Result<ItemResponse> {
        let options = options.unwrap_or_default();
        let (operation_options, binary) =
            resolve_binary_encoding(options.operation, &self.context.binary_encoding);
        let body = serialize_item_body(&item, binary.enabled)?;

        // Build the driver's item reference from our stored container metadata.
        let item_ref = ItemReference::from_name(
            &self.container_ref,
            partition_key.into(),
            item_id.to_owned(),
        );

        // Create the driver operation and apply ItemWriteOptions fields.
        let operation = CosmosOperation::replace_item(item_ref).with_body(body);
        let operation = apply_item_options(operation, options.session_token, options.precondition);

        // Execute through the driver, with binary encoding on the operation
        // options so the driver negotiates the wire format and transcoding.
        let driver_result = self
            .context
            .driver
            .execute_singleton_operation(operation, operation_options)
            .await;

        // Bridge the driver response to the SDK response type.
        Ok(ItemResponse::new(
            self.context
                .complete_result(driver_result, || self.operation_context("replace_item"))?,
        ))
    }

    /// Applies a JSON-PATCH-style update to an item using either one
    /// server-side PATCH request or client-side Read-Modify-Write. The
    /// client-side path persists a tracking marker only when required for
    /// duplicate suppression.
    ///
    /// **Preview.** Requires the `preview_patch` feature. This API is not
    /// production-ready — see [Retry Semantics](#retry-semantics) below.
    ///
    /// The handler refuses to PATCH paths that overlap the container's
    /// partition-key paths: rewriting the partition key would move the
    /// document to a different physical partition, so such requests are
    /// rejected by the client.
    ///
    /// # Arguments
    /// * `partition_key` - The partition key of the item to patch.
    /// * `item_id` - The id of the item to patch.
    /// * `patch` - The [`PatchInstructions`] describing the ops to apply.
    /// * `options` - Optional parameters for the request.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use azure_data_cosmos::models::{PatchOperation, PatchInstructions};
    /// use serde::{Deserialize, Serialize};
    /// # async fn doc() -> Result<(), Box<dyn std::error::Error>> {
    /// # let container_client: azure_data_cosmos::clients::ContainerClient = panic!("non-running example");
    /// #[derive(Debug, Deserialize, Serialize)]
    /// pub struct Product {
    ///     #[serde(rename = "id")]
    ///     product_id: String,
    ///     display_name: String,
    ///     visits: i64,
    /// }
    ///
    /// let patch = PatchInstructions::from(vec![
    ///     PatchOperation::set("/displayName", serde_json::json!("New name")),
    ///     PatchOperation::increment("/visits", 1i64),
    /// ]);
    /// let updated: Product = container_client
    ///     .patch_item("category1", "product1", patch, None)
    ///     .await?
    ///     .into_model()?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Response Body
    ///
    /// By default both strategies return the post-image. Client-side PATCH
    /// constructs it from the merged document; server-side PATCH requests it
    /// from Cosmos DB. An explicit `content_response_on_write = Disabled`
    /// suppresses the response body for either strategy.
    ///
    /// # Execution Tradeoffs
    ///
    /// Server-side PATCH charges for and resolves conflicts at the changed
    /// paths. Client-side PATCH reads and serializes the complete JSON item
    /// again, then replaces it, so it consumes Read plus Replace request units
    /// and resolves multi-write conflicts at document granularity. Select an
    /// explicit strategy when those differences matter.
    ///
    /// # Retry Semantics
    ///
    /// [`PatchStrategy::Auto`](crate::options::PatchStrategy::Auto) is the
    /// default. It uses server-side PATCH for retry-safe lists containing no
    /// more than 10 instructions. Unsafe or longer lists use tracked
    /// client-side RMW. Explicit server-side PATCH with more than 10
    /// instructions fails with HTTP 400 rather than falling back.
    /// Client-side-only settings do not influence strategy selection and are
    /// ignored on the server path. To provide a stable identity for
    /// application-level retries when client-side execution is selected, use
    /// [`PatchItemOptions::with_tracking_id`]. Explicit unsafe server-side
    /// PATCH is not retried after an ambiguous outcome.
    ///
    /// An explicitly supplied `If-Match` precondition follows standard ETag
    /// semantics and can return HTTP 412 when it does not match.
    #[cfg(feature = "preview_patch")]
    pub async fn patch_item(
        &self,
        partition_key: impl Into<PartitionKey>,
        item_id: &str,
        patch: PatchInstructions,
        options: Option<PatchItemOptions>,
    ) -> crate::Result<ItemResponse> {
        let options = options.unwrap_or_default();
        let body = serde_json::to_vec(&patch).map_err(crate::error::convert_json_encode_error)?;

        let item_ref = ItemReference::from_name(
            &self.container_ref,
            partition_key.into(),
            item_id.to_owned(),
        );

        // Build the PATCH operation. The handler reads the PatchInstructions back
        // out of the body, so we pass it through verbatim.
        let operation = apply_patch_options(
            CosmosOperation::patch_item(item_ref).with_body(body),
            &options,
        );
        let operation = apply_item_options(operation, options.session_token, None);

        let operation_options = apply_patch_operation_options(options.operation, options.strategy);

        let driver_result = self
            .context
            .driver
            .execute_singleton_operation(operation, operation_options)
            .await;

        Ok(ItemResponse::new(
            self.context
                .complete_result(driver_result, || self.operation_context("patch_item"))?,
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
    /// If you want the created/replaced item to be returned, set `content_response_on_write` to [`ContentResponseOnWrite::Enabled`](crate::options::ContentResponseOnWrite::Enabled) on the [`OperationOptions`](crate::options::OperationOptions) in your [`ItemWriteOptions`](crate::options::ItemWriteOptions).
    /// You can deserialize the returned item by retrieving the [`ResponseBody`](crate::models::ResponseBody) using [`ItemResponse::into_body`] and then calling [`ResponseBody::into_single`](crate::models::ResponseBody::into_single), like this:
    ///
    /// ```rust,no_run
    /// use azure_data_cosmos::options::{ItemWriteOptions, ContentResponseOnWrite, OperationOptions};
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
    ///     .into_body().into_single::<Product>()?;
    /// Ok(())
    /// # }
    /// ```
    ///
    /// # Tracked PATCH Items
    ///
    /// When upsert replaces an item that participates in client-side tracked
    /// PATCH, it must preserve `_azsdkPatchTracking` and its array order.
    /// Models that do not explicitly represent the property should capture
    /// unknown fields with `#[serde(flatten)]` and round-trip them unchanged.
    pub async fn upsert_item<T: Serialize>(
        &self,
        partition_key: impl Into<PartitionKey>,
        item_id: &str,
        item: T,
        options: Option<ItemWriteOptions>,
    ) -> crate::Result<ItemResponse> {
        let options = options.unwrap_or_default();
        let (operation_options, binary) =
            resolve_binary_encoding(options.operation, &self.context.binary_encoding);
        let body = serialize_item_body(&item, binary.enabled)?;

        // Build the driver's item reference from our stored container metadata.
        let item_ref = ItemReference::from_name(
            &self.container_ref,
            partition_key.into(),
            item_id.to_owned(),
        );

        // Create the driver operation and apply ItemWriteOptions fields.
        let operation = CosmosOperation::upsert_item(item_ref).with_body(body);
        let operation = apply_item_options(operation, options.session_token, options.precondition);

        // Execute through the driver, with binary encoding on the operation
        // options so the driver negotiates the wire format and transcoding.
        let driver_result = self
            .context
            .driver
            .execute_singleton_operation(operation, operation_options)
            .await;

        // Bridge the driver response to the SDK response type.
        Ok(ItemResponse::new(
            self.context
                .complete_result(driver_result, || self.operation_context("upsert_item"))?,
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
    pub async fn read_item(
        &self,
        partition_key: impl Into<PartitionKey>,
        item_id: &str,
        options: Option<ItemReadOptions>,
    ) -> crate::Result<ItemResponse> {
        let options = options.unwrap_or_default();
        let (operation_options, _binary) =
            resolve_binary_encoding(options.operation, &self.context.binary_encoding);

        // Build the driver's item reference from our stored container metadata.
        let item_ref = ItemReference::from_name(
            &self.container_ref,
            partition_key.into(),
            item_id.to_owned(),
        );

        // Create the driver operation.
        let operation = CosmosOperation::read_item(item_ref);
        let operation = apply_item_options(operation, options.session_token, options.precondition);

        // Execute through the driver, with binary encoding on the operation
        // options so the driver negotiates the wire format and transcoding.
        let driver_result = self
            .context
            .driver
            .execute_singleton_operation(operation, operation_options)
            .await;

        // Bridge the driver response to the SDK response type.
        Ok(ItemResponse::new(
            self.context
                .complete_result(driver_result, || self.operation_context("read_item"))?,
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
    ) -> crate::Result<ItemResponse> {
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
        let driver_result = self
            .context
            .driver
            .execute_singleton_operation(operation, options.operation)
            .await;

        // Bridge the driver response to the SDK response type.
        Ok(ItemResponse::new(
            self.context
                .complete_result(driver_result, || self.operation_context("delete_item"))?,
        ))
    }

    /// Executes a query against items in the container.
    ///
    /// The resulting document will be deserialized into the type provided as `T`.
    /// If you want to deserialize the document to a direct representation of the JSON returned, use [`serde_json::Value`] as the target type.
    ///
    /// We recommend using ["turbofish" syntax](https://doc.rust-lang.org/book/appendix-02-operators.html#:~:text=turbofish) (`query_items::<SomeTargetType>(...)`) to specify the target type, as it makes type inference easier.
    ///
    /// # Arguments
    ///
    /// * `query` - The query to execute.
    /// * `scope` - The [`FeedScope`] specifying the scope of the query.
    /// * `options` - Optional parameters for the request.
    ///
    /// # Cross Partition Queries
    ///
    /// When `scope` spans multiple partitions, the SDK obtains a query plan and composes the
    /// client-side pipeline needed to execute it. Supported features include ordinary projections
    /// and filters, `TOP`, `OFFSET`/`LIMIT`, streaming single- and multiple-column `ORDER BY`, and
    /// ordered or unordered `DISTINCT`.
    ///
    /// Cross-partition vector ordering supports pure `ORDER BY VectorDistance(...)` queries with a
    /// finite `TOP N` or `OFFSET x LIMIT y` window. The SDK buffers that result window before
    /// returning the first page, so use narrow projections and choose a result window appropriate
    /// for the memory available to the application.
    ///
    /// The buffered result can be iterated in pages, but it does not support continuation tokens
    /// for resuming in another process. Aggregates, `GROUP BY`, and hybrid/full-text ranking remain
    /// unsupported when their query plans require client-side stages that have not been
    /// implemented.
    ///
    /// # Examples
    ///
    /// The `query` parameter accepts anything that can be transformed [`Into`] a [`Query`], and `scope` controls partition targeting.
    /// This allows simple queries without parameters to be expressed easily:
    ///
    /// ```rust,no_run
    /// # async fn doc() -> Result<(), Box<dyn std::error::Error>> {
    /// # use azure_data_cosmos::feed::FeedScope;
    /// # let container_client: azure_data_cosmos::clients::ContainerClient = panic!("this is a non-running example");
    /// #[derive(serde::Deserialize)]
    /// struct Customer {
    ///     id: u64,
    ///     name: String,
    /// }
    /// let items = container_client.query_items::<Customer>(
    ///     "SELECT * FROM c",
    ///     FeedScope::partition("some_partition_key"),
    ///     None,
    /// ).await?;
    /// # }
    /// ```
    ///
    /// You can specify parameters by using [`Query::from()`] and [`Query::with_parameter()`]:
    ///
    /// ```rust,no_run
    /// # async fn doc() -> Result<(), Box<dyn std::error::Error>> {
    /// use azure_data_cosmos::{feed::FeedScope, Query};
    /// # let container_client: azure_data_cosmos::clients::ContainerClient = panic!("this is a non-running example");
    /// #[derive(serde::Deserialize)]
    /// struct Customer {
    ///     id: u64,
    ///     name: String,
    /// }
    /// let query = Query::from("SELECT COUNT(*) FROM c WHERE c.customer_id = @customer_id")
    ///     .with_parameter("@customer_id", 42)?;
    /// let items = container_client
    ///     .query_items::<Customer>(query, FeedScope::partition("some_partition_key"), None).await?;
    /// # }
    /// ```
    ///
    /// A raw SQL vector search can bind the query vector as a parameter. Vector ordering must not
    /// specify `ASC` or `DESC`, and must use `TOP N` or `OFFSET`/`LIMIT` when querying across
    /// partitions:
    ///
    /// ```rust,no_run
    /// # async fn doc() -> Result<(), Box<dyn std::error::Error>> {
    /// use azure_data_cosmos::{feed::FeedScope, Query};
    /// use futures::TryStreamExt;
    /// # let container_client: azure_data_cosmos::clients::ContainerClient = panic!("this is a non-running example");
    /// #[derive(serde::Deserialize)]
    /// struct VectorMatch {
    ///     id: String,
    ///     score: f64,
    /// }
    ///
    /// let query_vector = vec![0.1_f32, 0.2, 0.3];
    /// let query = Query::from(
    ///     "SELECT TOP 5 c.id, VectorDistance(c.embedding, @vector, false) AS score \
    ///      FROM c ORDER BY VectorDistance(c.embedding, @vector, false)",
    /// )
    /// .with_parameter("@vector", &query_vector)?;
    /// let mut matches = container_client
    ///     .query_items::<VectorMatch>(query, FeedScope::full_container(), None)
    ///     .await?;
    ///
    /// while let Some(item) = matches.try_next().await? {
    ///     println!("{}: {}", item.id, item.score);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// See [`PartitionKey`](crate::PartitionKey) for more information on how to specify a partition key, and [`Query`] for more information on how to specify a query.
    pub async fn query_items<T: DeserializeOwned + Send + 'static>(
        &self,
        query: impl Into<Query>,
        scope: FeedScope,
        options: Option<QueryOptions>,
    ) -> crate::Result<QueryItemIterator<T>> {
        let options = options.unwrap_or_default();
        let plan_options = options.to_plan_options();
        let query = query.into();

        // Resolve binary encoding so the driver advertises a binary *response*
        // via the negotiation header. Unlike point item writes, the query
        // request body stays text (`application/query+json` is a query spec,
        // not a document), so we do not touch body serialization here — the
        // driver's request-body gate excludes query.
        let (operation_options, _binary) =
            resolve_binary_encoding(options.operation, &self.context.binary_encoding);

        let container_ref = self.container_ref.clone();

        // The first operation to execute in the query items flow.
        // This holds the session token provided by the user, if any.
        let mut initial_operation = CosmosOperation::query_items(
            container_ref.clone(),
            Some(scope.into_feed_range(self.container_ref.partition_key_definition())),
        )
        .with_body(serde_json::to_vec(&query)?);
        if let Some(token) = options.session_token {
            initial_operation = initial_operation.with_session_token(token);
        }
        if let Some(b) = options.populate_index_metrics {
            initial_operation = initial_operation.with_populate_index_metrics(b);
        }
        if let Some(b) = options.populate_query_metrics {
            initial_operation = initial_operation.with_populate_query_metrics(b);
        }
        if let Some(hint) = options.feed.max_item_count {
            initial_operation = initial_operation.with_max_item_count(hint);
        }
        let plan = self
            .context
            .driver
            .plan_operation(
                initial_operation,
                &operation_options,
                options.feed.continuation_token.as_ref(),
                &plan_options,
            )
            .await?;
        Ok(QueryItemIterator::new(
            self.context.driver.clone(),
            Some(self.container_ref.clone()),
            plan,
            operation_options,
            self.context.diagnostics_handlers.clone(),
            self.operation_context("query_items"),
        ))
    }

    /// Queries the change feed for a container, returning a stream of pages.
    ///
    /// The change feed provides an ordered list of changes made to items in the
    /// container. Every change is returned as a
    /// [`ChangeFeedItem<T>`](crate::models::ChangeFeedItem) wire-format
    /// envelope, so bind `T = YourDoc` and read the post-change
    /// document via [`current()`](crate::models::ChangeFeedItem::current).
    ///
    /// The [`mode`](crate::options::ChangeFeedOptions::mode) selects what each
    /// change carries:
    ///
    /// * [`ChangeFeedMode::LatestVersion`] (default) — the latest version of
    ///   each created or replaced item. `current()` holds the item and
    ///   `metadata()` may also be present (for example `lsn` and the commit
    ///   timestamp), while `previous()` is not populated.
    /// * [`ChangeFeedMode::AllVersionsAndDeletes`] — every intermediate version
    ///   plus deletes. The envelope additionally exposes the
    ///   pre-change document
    ///   ([`previous()`](crate::models::ChangeFeedItem::previous)) and change
    ///   [`metadata()`](crate::models::ChangeFeedItem::metadata).
    ///
    /// # Arguments
    /// * `scope` - Determines which partitions to read changes from.
    /// * `start_from` - Where to begin reading when no continuation token is
    ///   provided. Ignored when `options` carries a continuation token, since
    ///   the token holds its own position.
    /// * `options` - Optional parameters controlling mode, session token, and paging.
    ///
    /// # AllVersionsAndDeletes limitations
    ///
    /// * Only [`ChangeFeedStartFrom::Now`] or resuming from a continuation token
    ///   is supported. [`ChangeFeedStartFrom::Beginning`] and
    ///   [`ChangeFeedStartFrom::PointInTime`] are **not** supported, because
    ///   intermediate versions and deletes are only retained within the
    ///   container's retention / continuous-backup window. The service gates
    ///   this and rejects such a request with `400 Bad Request`.
    /// * When starting from [`ChangeFeedStartFrom::Now`], every range is pinned
    ///   to its concrete starting position before the first page is returned, so
    ///   a range that is never served before a checkpoint still resumes from its
    ///   true start rather than resume-time. No intermediate versions or deletes
    ///   are dropped across a resume. `Now` is deliberately **not** rewritten to
    ///   a concrete [`ChangeFeedStartFrom::PointInTime`], which would change its
    ///   semantics; each range instead captures its own server continuation.
    /// * The feed mode is encoded in the continuation token, so a token issued in
    ///   one mode cannot be used to resume in another; attempting to do so is
    ///   rejected. Re-pass [`ChangeFeedMode::AllVersionsAndDeletes`] on resume to
    ///   match the original mode.
    ///
    /// # Examples
    ///
    /// Read the latest version of each change from the beginning:
    ///
    /// ```rust,no_run
    /// use azure_data_cosmos::{clients::ContainerClient, feed::FeedScope, options::ChangeFeedStartFrom};
    /// use futures::StreamExt;
    /// use serde::Deserialize;
    ///
    /// #[derive(Debug, Deserialize)]
    /// struct MyItem { id: String }
    ///
    /// # async fn example(container: ContainerClient) -> Result<(), Box<dyn std::error::Error>> {
    /// // Read all changes from the beginning
    /// let mut pages = container
    ///     .query_change_feed::<MyItem>(
    ///         FeedScope::full_container(),
    ///         ChangeFeedStartFrom::Beginning,
    ///         None,
    ///     )
    ///     .await?;
    ///
    /// while let Some(page) = pages.next().await {
    ///     let page = page?;
    ///     for item in page.items() {
    ///         println!("changed: {:?}", item.current());
    ///     }
    ///     // Save checkpoint for resumption
    ///     let _token = pages.to_continuation_token()?;
    /// }
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// Read every version and delete:
    ///
    /// ```rust,no_run
    /// use azure_data_cosmos::{
    ///     clients::ContainerClient,
    ///     feed::FeedScope,
    ///     options::{ChangeFeedMode, ChangeFeedOptions, ChangeFeedStartFrom},
    /// };
    /// use futures::StreamExt;
    /// use serde::Deserialize;
    ///
    /// // A delete envelope may omit non-key fields, so keep them optional.
    /// #[derive(Debug, Deserialize)]
    /// struct MyItem {
    ///     id: String,
    ///     #[serde(default)]
    ///     value: Option<i64>,
    /// }
    ///
    /// # async fn example(container: ContainerClient) -> Result<(), Box<dyn std::error::Error>> {
    /// let options = ChangeFeedOptions::default().with_mode(ChangeFeedMode::AllVersionsAndDeletes);
    /// let mut pages = container
    ///     .query_change_feed::<MyItem>(
    ///         FeedScope::full_container(),
    ///         ChangeFeedStartFrom::Now,
    ///         Some(options),
    ///     )
    ///     .await?;
    ///
    /// while let Some(page) = pages.next().await {
    ///     for item in page?.items() {
    ///         println!("{:?}: current={:?} previous={:?}",
    ///             item.operation_type(), item.current(), item.previous());
    ///     }
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn query_change_feed<T: DeserializeOwned + Send + 'static>(
        &self,
        scope: FeedScope,
        start_from: ChangeFeedStartFrom,
        options: Option<ChangeFeedOptions>,
    ) -> crate::Result<ChangeFeedPageIterator<ChangeFeedItem<T>>> {
        let options = options.unwrap_or_default();

        let feed_range = scope.into_feed_range(self.container_ref.partition_key_definition());

        // The mode selects the base operation, i.e. which `A-IM` header is sent.
        // Both modes return `ChangeFeedItem<T>` envelopes; AllVersionsAndDeletes
        // additionally populates `previous` and `metadata`.
        let mut initial_operation = match options.mode {
            ChangeFeedMode::AllVersionsAndDeletes => {
                // AllVersionsAndDeletes can only read within the container's
                // retention / continuous-backup window, so it supports starting
                // only from "now" or by resuming a continuation token. Reading
                // from the beginning of the container or from an arbitrary point
                // in time is not supported in this mode; the service gates this
                // and returns a `400 Bad Request`, so it is not re-validated
                // client-side here.
                CosmosOperation::change_feed_all_versions_and_deletes(
                    self.container_ref.clone(),
                    Some(feed_range),
                )
            }
            ChangeFeedMode::LatestVersion => {
                CosmosOperation::change_feed(self.container_ref.clone(), Some(feed_range))
            }
        };

        if let Some(token) = options.session_token {
            initial_operation = initial_operation.with_session_token(token);
        }
        if let Some(hint) = options.feed.max_item_count {
            initial_operation = initial_operation.with_max_item_count(hint);
        }

        // Record the start position on the operation. It is serialized into the
        // continuation token, so partitions that were never polled before a
        // checkpoint can re-apply the original start position on resume instead
        // of silently reading from the beginning. Partitions that have already
        // been polled resume from their saved per-partition ETag, which takes
        // precedence. The driver owns the mapping to wire headers.
        initial_operation = initial_operation.with_change_feed_start(start_from);

        let plan = Box::pin(self.context.driver.plan_operation(
            initial_operation,
            &options.operation,
            options.feed.continuation_token.as_ref(),
            &options.feed.to_plan_options(),
        ))
        .await?;

        Ok(ChangeFeedPageIterator::new(
            self.context.driver.clone(),
            Some(self.container_ref.clone()),
            plan,
            options.operation,
            self.context.diagnostics_handlers.clone(),
            self.operation_context("query_change_feed"),
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
    ) -> crate::Result<BatchResponse> {
        let options = options.unwrap_or_default();
        let body = serde_json::to_vec(batch.operations())?;
        let driver_pk = batch.partition_key().clone();

        let operation =
            CosmosOperation::batch(self.container_ref.clone(), driver_pk).with_body(body);
        let operation = apply_batch_options(operation, &options);

        let driver_result = self
            .context
            .driver
            .execute_singleton_operation(operation, options.operation)
            .await;

        Ok(BatchResponse::new(
            self.context
                .complete_result(driver_result, || self.operation_context("execute_batch"))?,
        ))
    }

    /// Gets the feed ranges for this container.
    pub async fn read_feed_ranges(
        &self,
        options: Option<ReadFeedRangesOptions>,
    ) -> crate::Result<Vec<FeedRange>> {
        let options = options.unwrap_or_default();
        let mut ranges = self
            .context
            .driver
            .resolve_all_partition_key_ranges(&self.container_ref, options.force_refresh())
            .await?;

        if should_force_refresh_feed_ranges(ranges.as_deref(), options.force_refresh()) {
            // A valid container always has at least one partition key range.
            // Missing or empty results likely mean a stale/failed cache.
            ranges = self
                .context
                .driver
                .resolve_all_partition_key_ranges(&self.container_ref, true)
                .await?;
        }

        let ranges = ranges.ok_or_else(|| {
            // Service was reachable but didn't return a usable routing
            // map — a service-side invariant violation, surfaced as a
            // 500 with the client-generated
            // `SERIALIZATION_RESPONSE_BODY_INVALID` sub-status so
            // callers can distinguish it from caller misuse.
            crate::DriverCosmosError::builder()
                .with_status(crate::error::CosmosStatus::SERIALIZATION_RESPONSE_BODY_INVALID)
                .with_message("failed to resolve routing map for container")
                .build()
        })?;

        ranges
            .iter()
            .map(FeedRange::try_from)
            .collect::<Result<Vec<_>, azure_data_cosmos_driver::error::CosmosError>>()
            .map_err(Into::into)
    }

    /// Returns the [`FeedRange`]s covering the given partition key.
    ///
    /// Full keys return a single-element `Vec`. Prefix keys on MultiHash
    /// containers return one or more feed ranges.
    pub async fn feed_range_from_partition_key(
        &self,
        partition_key: impl Into<PartitionKey>,
        options: Option<ReadFeedRangesOptions>,
    ) -> crate::Result<Vec<FeedRange>> {
        let partition_key = partition_key.into();
        let driver_pk = partition_key;
        let options = options.unwrap_or_default();
        let pk_def = self.container_ref.partition_key_definition();
        let values = driver_pk.values();

        if values.is_empty() {
            return Err(crate::DriverCosmosError::builder()
                .with_status(crate::error::CosmosStatus::CLIENT_PARTITION_KEY_EMPTY)
                .with_message("partition key must have at least one component")
                .build()
                .into());
        }
        if values.len() > pk_def.paths().len() {
            return Err(crate::DriverCosmosError::builder()
                .with_status(crate::error::CosmosStatus::CLIENT_PARTITION_KEY_TOO_MANY_COMPONENTS)
                .with_message(format!(
                    "partition key has {} components but container definition has {} paths",
                    values.len(),
                    pk_def.paths().len()
                ))
                .build()
                .into());
        }

        let is_prefix =
            pk_def.kind() == PartitionKeyKind::MultiHash && values.len() < pk_def.paths().len();
        if !is_prefix && values.len() != pk_def.paths().len() {
            return Err(crate::DriverCosmosError::builder()
                .with_status(crate::error::CosmosStatus::CLIENT_PREFIX_PARTITION_KEY_REQUIRES_MULTIHASH)
                .with_message("prefix partition keys are only supported for MultiHash (hierarchical) containers")
                .build().into());
        }

        let ranges = self
            .context
            .driver
            .resolve_partition_key_ranges_for_key(
                &self.container_ref,
                &driver_pk,
                options.force_refresh(),
            )
            .await?
            .ok_or_else(|| {
                crate::DriverCosmosError::builder()
                    .with_status(crate::error::CosmosStatus::SERIALIZATION_RESPONSE_BODY_INVALID)
                    .with_message("failed to resolve routing map for container")
                    .build()
            })?;

        if ranges.is_empty() && !options.force_refresh() {
            // Empty result may indicate a stale cache — retry with refresh.
            let ranges = self
                .context
                .driver
                .resolve_partition_key_ranges_for_key(&self.container_ref, &driver_pk, true)
                .await?
                .ok_or_else(|| {
                    crate::DriverCosmosError::builder()
                        .with_status(
                            crate::error::CosmosStatus::SERIALIZATION_RESPONSE_BODY_INVALID,
                        )
                        .with_message("failed to resolve routing map for container")
                        .build()
                })?;

            if ranges.is_empty() {
                return Err(crate::DriverCosmosError::builder()
                    .with_status(crate::error::CosmosStatus::TRANSPORT_GENERATED_503)
                    .with_message(
                        "no partition key ranges found for the given partition key; \
                         the container may not exist or the service may be unreachable",
                    )
                    .build()
                    .into());
            }

            ranges
                .iter()
                .map(FeedRange::try_from)
                .collect::<Result<Vec<_>, azure_data_cosmos_driver::error::CosmosError>>()
                .map_err(Into::into)
        } else {
            ranges
                .iter()
                .map(FeedRange::try_from)
                .collect::<Result<Vec<_>, azure_data_cosmos_driver::error::CosmosError>>()
                .map_err(Into::into)
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
    /// # use azure_data_cosmos::{clients::ContainerClient};
    /// use azure_data_cosmos::feed::{FeedRange};
    /// use azure_data_cosmos::options::{SessionToken};
    /// # async fn example(container: ContainerClient) -> azure_data_cosmos::Result<()> {
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
    ) -> crate::Result<SessionToken> {
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

/// Serializes an item write body as either Cosmos binary JSON (`binary`) or
/// UTF-8 text JSON.
///
/// The binary path uses the driver's native serde serializer
/// [`binary_json::to_vec`](azure_data_cosmos_driver::binary_json::to_vec),
/// encoding `T` straight to Cosmos binary JSON without an intermediate
/// [`serde_json::Value`]; the text path is the original [`serde_json::to_vec`].
/// Both produce a body the service accepts — the binary form begins with the
/// `0x80` preamble, which the service detects from the first byte, so the
/// request `Content-Type` stays `application/json`.
fn serialize_item_body<T: Serialize>(item: &T, binary: bool) -> crate::Result<Vec<u8>> {
    if binary {
        let body = azure_data_cosmos_driver::binary_json::to_vec(item)
            .map_err(crate::error::convert_binary_encode_error)?;
        tracing::debug!(
            binary_encoding = true,
            "binary encoding applied to item write body"
        );
        Ok(body)
    } else {
        tracing::debug!(
            binary_encoding = false,
            "item write body serialized as text JSON"
        );
        serde_json::to_vec(item).map_err(crate::error::convert_json_encode_error)
    }
}

/// Resolves the effective binary encoding for an item operation, preferring a
/// caller-set per-operation value over the client-level default.
///
/// Returns the resolved options alongside the updated [`OperationOptions`] so
/// the caller drives body serialization from the same decision. The operation
/// field is normalized to `Some(effective)` when enabled (the driver negotiates
/// the binary wire) and `None` when disabled (byte-for-byte unchanged).
fn resolve_binary_encoding(
    mut options: OperationOptions,
    client_default: &BinaryEncodingOptions,
) -> (OperationOptions, BinaryEncodingOptions) {
    let effective = options
        .binary_encoding
        .take()
        .unwrap_or_else(|| client_default.clone());
    // Write `Some` (never `None`, which means "inherit") so a resolved disable
    // overrides driver-layer defaults; the wire is unchanged when disabled.
    options.binary_encoding = Some(effective.clone());
    (options, effective)
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

#[cfg(feature = "preview_patch")]
fn apply_patch_options(
    mut operation: CosmosOperation,
    options: &PatchItemOptions,
) -> CosmosOperation {
    if let Some(precondition) = options.precondition.clone() {
        operation = operation.with_precondition(precondition);
    }
    if let Some(max_attempts) = options.max_attempts {
        operation = operation.with_patch_max_attempts(max_attempts);
    }
    if let Some(tracking_id) = options.tracking_id {
        operation = operation.with_patch_tracking_id(tracking_id.into_driver());
    }
    if let Some(capacity) = options.tracking_capacity {
        operation = operation.with_patch_tracking_capacity(capacity);
    }
    if let Some(retention_seconds) = options.tracking_retention_seconds {
        operation = operation.with_patch_tracking_retention_seconds(retention_seconds);
    }
    operation
}

#[cfg(feature = "preview_patch")]
fn apply_patch_operation_options(
    mut operation_options: OperationOptions,
    strategy: Option<crate::options::PatchStrategy>,
) -> OperationOptions {
    if let Some(strategy) = strategy {
        operation_options.patch_strategy = Some(strategy);
    }
    operation_options
}

fn should_force_refresh_feed_ranges<T>(ranges: Option<&[T]>, force_refresh: bool) -> bool {
    !force_refresh && ranges.is_none_or(<[T]>::is_empty)
}

/// Compile-time guarantee that the futures returned by [`ContainerClient`]
/// helpers are `Send`.
///
/// This function is never called — it exists purely so `cargo build` rejects
/// any regression that accidentally makes a future non-`Send` (e.g. by
/// capturing a non-`Send` cell across an `.await` point). Each method we
/// want covered is referenced below.
#[allow(dead_code, unreachable_code, unused_variables)]
fn _assert_futures_are_send() {
    fn assert_send<T: Send>(_: T) {}
    let client: &ContainerClient = todo!();
    let partition_key: PartitionKey = todo!();
    let item_id: &str = todo!();
    assert_send(client.read_item(partition_key.clone(), item_id, None));
    #[cfg(feature = "preview_patch")]
    {
        let patch: PatchInstructions = todo!();
        let options: Option<PatchItemOptions> = todo!();
        assert_send(client.patch_item(partition_key, item_id, patch, options));
    }
}

#[cfg(test)]
mod tests {
    //! These are sanity checks that [`serialize_item_body`] picks the right
    //! path (text vs binary) and that binary encoding is actually applied —
    //! not full serialize/deserialize coverage. Byte-level codec correctness
    //! lives in the driver's `binary_json` snapshot, golden-vector, and parity
    //! tests.
    use super::*;
    use serde_json::json;

    #[cfg(feature = "preview_patch")]
    #[test]
    fn patch_options_forward_to_driver_operation() {
        let account = azure_data_cosmos_driver::models::AccountReference::with_master_key(
            azure_core::http::Url::parse("https://localhost").unwrap(),
            "test-key",
        );
        let operation = CosmosOperation::read_all_databases(account);
        let tracking_id = "7f5241c9-d7c2-4071-97a3-43bdebf6ef8f"
            .parse::<crate::models::PatchTrackingId>()
            .unwrap();
        let options = PatchItemOptions::default()
            .with_strategy(crate::options::PatchStrategy::ClientSide)
            .with_precondition(Precondition::if_match(azure_core::http::Etag::from(
                "\"etag\"",
            )))
            .with_max_attempts(std::num::NonZeroU8::new(7).unwrap())
            .with_tracking_id(tracking_id)
            .with_tracking_capacity(std::num::NonZeroU16::new(19).unwrap())
            .with_tracking_retention_seconds(std::num::NonZeroU32::new(23).unwrap());

        let operation = apply_patch_options(operation, &options);

        assert_eq!(operation.patch_max_attempts().unwrap().get(), 7);
        assert_eq!(
            operation.patch_tracking_id().unwrap().to_string(),
            tracking_id.to_string()
        );
        assert_eq!(operation.patch_tracking_capacity().unwrap().get(), 19);
        assert_eq!(
            operation.precondition(),
            Some(&Precondition::if_match(azure_core::http::Etag::from(
                "\"etag\""
            )))
        );
        assert_eq!(
            operation.patch_tracking_retention_seconds().unwrap().get(),
            23
        );
        let operation_options = apply_patch_operation_options(options.operation, options.strategy);
        assert_eq!(
            operation_options.patch_strategy,
            Some(crate::options::PatchStrategy::ClientSide)
        );
    }

    #[test]
    fn serialize_item_body_text_matches_serde_to_vec() {
        // The text path is byte-for-byte the original `serde_json::to_vec`.
        let item = json!({ "id": "1", "count": 7, "tags": ["a", "b"] });
        let body = serialize_item_body(&item, false).unwrap();
        assert_eq!(body, serde_json::to_vec(&item).unwrap());
    }

    #[test]
    fn serialize_item_body_binary_round_trips() {
        // The binary path begins with the `0x80` preamble and decodes back to
        // the same value the text path would have serialized.
        let item = json!({ "id": "doc-1", "count": 42, "nested": { "ok": true } });
        let body = serialize_item_body(&item, true).unwrap();
        assert_eq!(body.first(), Some(&0x80));
        let decoded: serde_json::Value =
            azure_data_cosmos_driver::binary_json::decode(&body).unwrap();
        assert_eq!(decoded, item);
    }

    #[test]
    fn serialize_item_body_text_encode_failure_is_request_body_invalid() {
        // A map with non-string keys fails `serde_json::to_vec`; the write path
        // must label it as a request-body (encode) error, not response-body.
        let item: std::collections::HashMap<(i32, i32), i32> =
            std::collections::HashMap::from([((1, 2), 3)]);
        let err = serialize_item_body(&item, false).expect_err("must fail to serialize");
        assert_eq!(
            err.status(),
            crate::error::CosmosStatus::SERIALIZATION_REQUEST_BODY_INVALID
        );
    }

    #[test]
    fn serialize_item_body_binary_differs_from_text() {
        // Sanity check that the two paths actually produce different bytes.
        let item = json!({ "id": "x" });
        let text = serialize_item_body(&item, false).unwrap();
        let binary = serialize_item_body(&item, true).unwrap();
        assert_ne!(text, binary);
        assert_ne!(text.first(), Some(&0x80));
    }

    #[test]
    fn resolve_binary_encoding_uses_client_default_when_operation_unset() {
        // No per-operation value: the client-level default applies. Enabled ⇒
        // the driver option is set.
        let client = BinaryEncodingOptions::new().with_enabled(true);
        let (options, effective) = resolve_binary_encoding(OperationOptions::default(), &client);
        assert!(effective.enabled);
        assert_eq!(options.binary_encoding, Some(client));
    }

    #[test]
    fn resolve_binary_encoding_carries_request_text_response() {
        // Binary on with request_text_response: the driver keeps the wire binary
        // and transcodes the response to text. Both flags carry through.
        let client = BinaryEncodingOptions::new()
            .with_enabled(true)
            .with_request_text_response(true);
        let (options, effective) = resolve_binary_encoding(OperationOptions::default(), &client);
        assert!(effective.enabled);
        assert!(effective.request_text_response);
        let resolved = options.binary_encoding.expect("binary encoding set");
        assert!(resolved.enabled);
        assert!(resolved.request_text_response);
    }

    #[test]
    fn resolve_binary_encoding_omits_option_when_disabled() {
        // Disabled default with no per-op value is preserved as `Some(false)`,
        // not erased to `None`, so it overrides driver-layer defaults.
        let client = BinaryEncodingOptions::new().with_enabled(false);
        let (options, effective) = resolve_binary_encoding(OperationOptions::default(), &client);
        assert!(!effective.enabled);
        assert_eq!(
            options.binary_encoding.map(|b| b.enabled),
            Some(false),
            "resolved disable must be preserved as Some(false) to override driver defaults"
        );
    }

    #[test]
    fn resolve_binary_encoding_operation_disable_overrides_enabled_client() {
        // A per-operation disable wins over an enabled client default.
        let client = BinaryEncodingOptions::new().with_enabled(true);
        let mut operation = OperationOptions::default();
        operation.binary_encoding = Some(BinaryEncodingOptions::new().with_enabled(false));
        let (options, effective) = resolve_binary_encoding(operation, &client);
        assert!(!effective.enabled);
        assert_eq!(
            options.binary_encoding.map(|b| b.enabled),
            Some(false),
            "per-operation disable must be preserved as Some(false)"
        );
    }

    #[test]
    fn resolve_binary_encoding_operation_enable_overrides_disabled_client() {
        // Client disabled, but the caller enabled binary for this operation:
        // the per-operation value wins, so binary is negotiated for this request.
        let client = BinaryEncodingOptions::new().with_enabled(false);
        let operation_be = BinaryEncodingOptions::new()
            .with_enabled(true)
            .with_request_text_response(true);
        let mut operation = OperationOptions::default();
        operation.binary_encoding = Some(operation_be.clone());
        let (options, effective) = resolve_binary_encoding(operation, &client);
        assert!(effective.enabled);
        assert_eq!(options.binary_encoding, Some(operation_be));
    }

    #[test]
    fn feed_ranges_refreshes_missing_or_empty_initial_resolution() {
        assert!(should_force_refresh_feed_ranges::<()>(None, false));
        assert!(should_force_refresh_feed_ranges::<()>(Some(&[]), false));
        assert!(!should_force_refresh_feed_ranges(Some(&[()]), false));
        assert!(!should_force_refresh_feed_ranges::<()>(None, true));
    }
}

// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Cosmos DB operation representation.

use crate::models::{
    AccountReference, ContainerReference, CosmosRequestHeaders, CosmosResourceReference,
    DatabaseReference, FeedRange, ItemReference, OperationType, PartitionKey, Precondition,
    ResourceType,
};
use azure_core::http::Etag;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use time::OffsetDateTime;

/// The position a change feed is started from.
///
/// Passed explicitly when starting a change feed read and persisted inside the
/// continuation token, so that on resume partitions that were never polled
/// before the checkpoint re-apply the feed's original start position instead of
/// silently reading from the beginning. Partitions that already have a saved
/// per-partition continuation resume from it and ignore this value.
///
/// This enum owns the mapping from a start position to its wire header (see
/// [`CosmosOperation::with_change_feed_start`]), so both the initial request
/// and a resume reconstructed from a continuation token stay in sync.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", content = "value", rename_all = "snake_case")]
#[non_exhaustive]
pub enum ChangeFeedStartFrom {
    /// Start from the beginning of the change feed (all retained changes).
    ///
    /// No start header is sent; the server treats the absence of a start header
    /// as "from the beginning".
    Beginning,

    /// Start from the current point in time (wire header `If-None-Match: *`).
    ///
    /// "Now" is evaluated when the request is sent, so on resume a never-polled
    /// partition starts from resume time rather than the original start time.
    /// This is acceptable for LatestVersion (it still converges to the latest
    /// state under at-least-once delivery), but AllVersionsAndDeletes must
    /// resolve "Now" to a concrete start position before persisting so resume
    /// does not drop intermediate versions/deletes.
    Now,

    /// Start from a specific point in time (wire header `If-Modified-Since`).
    ///
    /// The timestamp is persisted in the continuation token as RFC 3339 so
    /// resume is exact, and formatted as RFC 1123 for the wire header.
    PointInTime(#[serde(with = "time::serde::rfc3339")] OffsetDateTime),
}

/// Formats an [`OffsetDateTime`] as an RFC 1123 timestamp (the IMF fixed-date
/// production in RFC 7231) for the `If-Modified-Since` change feed header.
fn format_rfc1123(timestamp: &OffsetDateTime) -> String {
    use time::format_description::FormatItem;
    use time::macros::format_description;
    const RFC1123: &[FormatItem<'_>] = format_description!(
        "[weekday repr:short], [day] [month repr:short] [year] [hour]:[minute]:[second] GMT"
    );
    timestamp
        .to_offset(time::UtcOffset::UTC)
        .format(RFC1123)
        .expect("RFC 1123 formatting of a valid OffsetDateTime cannot fail")
}

/// Represents a Cosmos DB operation with its routing and execution context.
///
/// This is the driver's internal representation of an operation before it is
/// converted into a wire-level HTTP request. It captures the operation intent
/// (create/read/query/etc.), resource routing information, and optional
/// operation-specific settings.
///
/// # Immutable Fields
///
/// The `operation_type` and `resource_type` fields are set at construction time
/// and cannot be changed. Use the factory methods to create operations with the
/// correct types.
///
/// # Examples
///
/// ```no_run
/// use azure_data_cosmos_driver::driver::CosmosDriverRuntime;
/// use azure_data_cosmos_driver::models::{
///     AccountReference, CosmosOperation,
///     ItemReference, PartitionKey,
/// };
/// use azure_data_cosmos_driver::options::{DriverOptions, OperationOptions};
/// use url::Url;
///
/// # async fn example() -> azure_data_cosmos_driver::error::Result<()> {
/// // 1. Set up runtime and driver
/// let runtime = CosmosDriverRuntime::builder().build().await?;
/// let account = AccountReference::with_master_key(
///     Url::parse("https://myaccount.documents.azure.com:443/").unwrap(),
///     "my-key",
/// );
/// let driver = runtime.create_driver(DriverOptions::builder(account).build()).await?;
///
/// // 2. Resolve the container (reads database + container from service, caches result)
/// let container = driver.resolve_container("mydb", "mycontainer").await?;
///
/// // 3. Build and execute item operations
/// let item = ItemReference::from_name(&container, PartitionKey::from("pk1"), "doc1");
/// let result = driver
///     .execute_singleton_operation(CosmosOperation::read_item(item), OperationOptions::default())
///     .await?;
/// # Ok(())
/// # }
/// ```
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct CosmosOperation {
    /// The type of operation (immutable after construction).
    operation_type: OperationType,
    /// The type of resource (derived from resource reference, immutable).
    resource_type: ResourceType,
    /// Reference to the resource being operated on.
    resource_reference: CosmosResourceReference,
    /// Describes how the operation targets the partition key space.
    target: Option<FeedRange>,
    /// Additional request headers to include in the request.
    request_headers: CosmosRequestHeaders,
    /// Optional request body (raw bytes, schema-agnostic).
    body: Option<Vec<u8>>,
    /// Maximum number of Read-Modify-Write attempts the PATCH handler may
    /// make. Only consulted when `operation_type == OperationType::Patch`;
    /// ignored for every other op. `None` selects the handler default (5).
    patch_max_attempts: Option<std::num::NonZeroU8>,
    /// `true` when this operation is a change feed read. Set explicitly by
    /// [`change_feed`](Self::change_feed) rather than inferred from a header,
    /// so future change feed modes can be added without ambiguity.
    is_change_feed: bool,
    /// The original change feed start position, persisted into the continuation
    /// token so never-polled partitions can re-apply it on resume. `None` for
    /// non-change-feed operations.
    change_feed_start: Option<ChangeFeedStartFrom>,
}

impl CosmosOperation {
    /// Returns the operation type.
    pub fn operation_type(&self) -> OperationType {
        self.operation_type
    }

    /// Returns the resource type.
    pub fn resource_type(&self) -> ResourceType {
        self.resource_type
    }

    /// Returns a reference to the resource being operated on.
    pub(crate) fn resource_reference(&self) -> &CosmosResourceReference {
        &self.resource_reference
    }

    /// Computes the request path and signing link for this operation.
    ///
    /// Create and Upsert document operations use feed-style paths (targeting
    /// the collection URL) even though they carry an item id, because the
    /// Cosmos DB REST API POSTs these to the collection feed. All other
    /// operations use the standard resource paths.
    pub(crate) fn compute_resource_paths(&self) -> crate::models::ResourcePaths {
        if matches!(
            self.operation_type,
            OperationType::Create | OperationType::Upsert
        ) && self.resource_type == ResourceType::Document
        {
            self.resource_reference.compute_feed_paths()
        } else {
            self.resource_reference.compute_paths()
        }
    }

    /// Returns the container for this operation, if applicable.
    ///
    /// Returns `None` for account-level and database-level operations.
    pub fn container(&self) -> Option<&ContainerReference> {
        self.resource_reference.container()
    }

    /// Returns the operation target.
    pub fn target(&self) -> Option<&FeedRange> {
        self.target.as_ref()
    }

    /// Returns the partition key for this operation, if applicable.
    pub fn partition_key(&self) -> Option<&PartitionKey> {
        self.target.as_ref().and_then(|t| t.partition_key())
    }

    /// Returns `true` if this is a change feed request.
    ///
    /// Set explicitly by [`change_feed`](Self::change_feed); not inferred from
    /// request headers.
    pub fn is_change_feed(&self) -> bool {
        self.is_change_feed
    }

    /// Returns the request headers.
    pub fn request_headers(&self) -> &CosmosRequestHeaders {
        &self.request_headers
    }

    /// Returns the request body, if set.
    pub fn body(&self) -> Option<&[u8]> {
        self.body.as_deref()
    }

    /// Sets request headers for the operation.
    pub fn with_request_headers(mut self, headers: CosmosRequestHeaders) -> Self {
        self.request_headers = headers;
        self
    }

    /// Sets the session token request header for the operation.
    pub fn with_session_token(
        mut self,
        session_token: impl Into<crate::models::SessionToken>,
    ) -> Self {
        self.request_headers.session_token = Some(session_token.into());
        self
    }

    /// Sets the activity ID request header for the operation.
    pub fn with_activity_id(mut self, activity_id: crate::models::ActivityId) -> Self {
        self.request_headers.activity_id = Some(activity_id);
        self
    }

    /// Enables or disables index-utilization metrics on the response
    /// (the `x-ms-cosmos-populateindexmetrics` request header).
    pub fn with_populate_index_metrics(mut self, enabled: bool) -> Self {
        self.request_headers.populate_index_metrics = Some(enabled);
        self
    }

    /// Enables or disables per-query metrics on the response
    /// (the `x-ms-documentdb-populatequerymetrics` request header).
    pub fn with_populate_query_metrics(mut self, enabled: bool) -> Self {
        self.request_headers.populate_query_metrics = Some(enabled);
        self
    }

    /// Sets the maximum number of items the server should return per page
    /// (the `x-ms-max-item-count` request header).
    ///
    /// Applies to feed-style operations such as queries and read-feed.
    pub fn with_max_item_count(mut self, max_item_count: crate::models::MaxItemCountHint) -> Self {
        self.request_headers.max_item_count = Some(max_item_count);
        self
    }

    /// Sets the precondition for optimistic concurrency control.
    pub fn with_precondition(mut self, precondition: Precondition) -> Self {
        self.request_headers.precondition = Some(precondition);
        self
    }

    /// Sets the `If-Modified-Since` header (pre-formatted RFC 1123 string).
    ///
    /// Used by change feed to start from a specific point in time.
    pub fn with_if_modified_since(mut self, value: String) -> Self {
        self.request_headers.if_modified_since = Some(value);
        self
    }

    /// Records the change feed start position and emits its wire header.
    ///
    /// This is the single source of truth for translating a start position into
    /// the appropriate header, so both the initial request and a resume that
    /// reconstructs the position from a continuation token stay in sync:
    ///
    /// - [`ChangeFeedStartFrom::Beginning`] → no header
    /// - [`ChangeFeedStartFrom::Now`] → `If-None-Match: *`
    /// - [`ChangeFeedStartFrom::PointInTime`] → `If-Modified-Since: <RFC 1123>`
    pub fn with_change_feed_start(mut self, start_from: ChangeFeedStartFrom) -> Self {
        match &start_from {
            ChangeFeedStartFrom::Beginning => {}
            ChangeFeedStartFrom::Now => {
                self.request_headers.precondition =
                    Some(Precondition::if_none_match(Etag::from("*")));
            }
            ChangeFeedStartFrom::PointInTime(timestamp) => {
                self.request_headers.if_modified_since = Some(format_rfc1123(timestamp));
            }
        }
        self.change_feed_start = Some(start_from);
        self
    }

    /// Returns the change feed start position, if one was set.
    pub fn change_feed_start(&self) -> Option<&ChangeFeedStartFrom> {
        self.change_feed_start.as_ref()
    }

    /// Returns the precondition, if set.
    pub fn precondition(&self) -> Option<&Precondition> {
        self.request_headers.precondition.as_ref()
    }

    /// Sets the request body.
    pub fn with_body(mut self, body: Vec<u8>) -> Self {
        self.body = Some(body);
        self
    }

    /// Caps the number of Read-Modify-Write attempts the PATCH handler may make.
    ///
    /// Only consulted when [`operation_type`](Self::operation_type) is
    /// [`OperationType::Patch`]; otherwise the value is ignored. `None`
    /// (the default) selects the handler default (5).
    pub fn with_patch_max_attempts(mut self, max_attempts: std::num::NonZeroU8) -> Self {
        self.patch_max_attempts = Some(max_attempts);
        self
    }

    /// Returns the cap on PATCH Read-Modify-Write attempts, if one was set.
    pub fn patch_max_attempts(&self) -> Option<std::num::NonZeroU8> {
        self.patch_max_attempts
    }

    // ===== Factory Methods =====

    /// Creates a new operation with the specified type, resource reference, and target.
    fn new(
        operation_type: OperationType,
        resource_reference: impl Into<CosmosResourceReference>,
        target: Option<FeedRange>,
    ) -> Self {
        let resource_reference = resource_reference.into();
        let resource_type = resource_reference.resource_type();
        debug_assert!(
            // QueryPlans and non-partitioned resources don't require a partition reference.
            // Point and query operations on partitioned resources require a partition reference for routing.
            operation_type == OperationType::QueryPlan || !resource_type.is_partitioned(operation_type) || target.is_some(),
            "Attempted to create a partitioned operation without an OperationTarget specifying the partitions to access"
        );
        Self {
            operation_type,
            resource_type,
            resource_reference,
            target,
            request_headers: CosmosRequestHeaders::new(),
            body: None,
            patch_max_attempts: None,
            is_change_feed: false,
            change_feed_start: None,
        }
    }

    fn for_item(operation_type: OperationType, item: ItemReference) -> Self {
        let range = FeedRange::for_item(&item);
        Self::new(operation_type, item, Some(range))
    }

    // ===== Control Plane Factory Methods =====

    /// Creates a database in the account.
    ///
    /// Use `with_body()` to provide the database properties JSON:
    /// ```json
    /// {"id": "my-database"}
    /// ```
    ///
    /// # Example
    ///
    /// ```
    /// use azure_data_cosmos_driver::models::{AccountReference, CosmosOperation};
    /// use url::Url;
    ///
    /// let account = AccountReference::with_master_key(
    ///     Url::parse("https://myaccount.documents.azure.com:443/").unwrap(),
    ///     "my-key",
    /// );
    ///
    /// let operation = CosmosOperation::create_database(account)
    ///     .with_body(br#"{"id": "my-database"}"#.to_vec());
    /// ```
    pub fn create_database(account: AccountReference) -> Self {
        let resource_ref: CosmosResourceReference = CosmosResourceReference::from(account)
            .with_resource_type(ResourceType::Database)
            .into_feed_reference();
        Self::new(OperationType::Create, resource_ref, None)
    }

    /// Reads (lists) all databases in the account.
    ///
    /// Returns a feed of database resources.
    pub fn read_all_databases(account: AccountReference) -> Self {
        let resource_ref = Into::<CosmosResourceReference>::into(account)
            .with_resource_type(ResourceType::Database)
            .into_feed_reference();
        Self::new(OperationType::ReadFeed, resource_ref, None)
    }

    /// Queries databases in the account.
    ///
    /// Use `with_body()` to provide the query JSON.
    pub fn query_databases(account: AccountReference) -> Self {
        let resource_ref: CosmosResourceReference = CosmosResourceReference::from(account)
            .with_resource_type(ResourceType::Database)
            .into_feed_reference();
        Self::new(OperationType::Query, resource_ref, None)
    }

    /// Deletes a database.
    ///
    /// # Example
    ///
    /// ```
    /// use azure_data_cosmos_driver::models::{
    ///     AccountReference, CosmosOperation, DatabaseReference,
    /// };
    /// use url::Url;
    ///
    /// let account = AccountReference::with_master_key(
    ///     Url::parse("https://myaccount.documents.azure.com:443/").unwrap(),
    ///     "my-key",
    /// );
    ///
    /// let database = DatabaseReference::from_name(account, "my-database");
    /// let operation = CosmosOperation::delete_database(database);
    /// ```
    pub fn delete_database(database: DatabaseReference) -> Self {
        let resource_ref: CosmosResourceReference = database.into();
        Self::new(OperationType::Delete, resource_ref, None)
    }

    /// Reads a database's properties from the service.
    ///
    /// Returns the database properties payload, including
    /// the system-managed `_rid`, `_ts`, and `_etag`.
    pub fn read_database(database: DatabaseReference) -> Self {
        let resource_ref: CosmosResourceReference = database.into();
        Self::new(OperationType::Read, resource_ref, None)
    }

    /// Creates a container in a database.
    ///
    /// Use `with_body()` to provide the container properties JSON:
    /// ```json
    /// {"id": "my-container", "partitionKey": {"paths": ["/pk"], "kind": "Hash"}}
    /// ```
    ///
    /// # Example
    ///
    /// ```
    /// use azure_data_cosmos_driver::models::{
    ///     AccountReference, CosmosOperation, DatabaseReference,
    /// };
    /// use url::Url;
    ///
    /// let account = AccountReference::with_master_key(
    ///     Url::parse("https://myaccount.documents.azure.com:443/").unwrap(),
    ///     "my-key",
    /// );
    ///
    /// let database = DatabaseReference::from_name(account, "my-database");
    /// let operation = CosmosOperation::create_container(database)
    ///     .with_body(br#"{"id": "my-container", "partitionKey": {"paths": ["/pk"], "kind": "Hash"}}"#.to_vec());
    /// ```
    pub fn create_container(database: DatabaseReference) -> Self {
        let resource_ref: CosmosResourceReference = CosmosResourceReference::from(database)
            .with_resource_type(ResourceType::DocumentCollection)
            .into_feed_reference();
        Self::new(OperationType::Create, resource_ref, None)
    }

    /// Reads (lists) all containers in a database.
    ///
    /// Returns a feed of container resources.
    pub fn read_all_containers(database: DatabaseReference) -> Self {
        let resource_ref: CosmosResourceReference = CosmosResourceReference::from(database)
            .with_resource_type(ResourceType::DocumentCollection)
            .into_feed_reference();
        Self::new(OperationType::ReadFeed, resource_ref, None)
    }

    /// Queries containers in a database.
    ///
    /// Use `with_body()` to provide the query JSON.
    pub fn query_containers(database: DatabaseReference) -> Self {
        let resource_ref: CosmosResourceReference = CosmosResourceReference::from(database)
            .with_resource_type(ResourceType::DocumentCollection)
            .into_feed_reference();
        Self::new(OperationType::Query, resource_ref, None)
    }

    /// Deletes a container.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use azure_data_cosmos_driver::driver::CosmosDriverRuntime;
    /// use azure_data_cosmos_driver::models::{
    ///     AccountReference, CosmosOperation,
    /// };
    /// use azure_data_cosmos_driver::options::{DriverOptions, OperationOptions};
    /// use url::Url;
    ///
    /// # async fn example() -> azure_data_cosmos_driver::error::Result<()> {
    /// let runtime = CosmosDriverRuntime::builder().build().await?;
    /// let account = AccountReference::with_master_key(
    ///     Url::parse("https://myaccount.documents.azure.com:443/").unwrap(),
    ///     "my-key",
    /// );
    /// let driver = runtime.create_driver(DriverOptions::builder(account).build()).await?;
    /// let container = driver.resolve_container("my-database", "my-container").await?;
    ///
    /// let result = driver
    ///     .execute_singleton_operation(
    ///         CosmosOperation::delete_container(container),
    ///         OperationOptions::default(),
    ///     )
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn delete_container(container: ContainerReference) -> Self {
        let resource_ref: CosmosResourceReference = container.into();
        Self::new(OperationType::Delete, resource_ref, None)
    }

    /// Replaces a container's properties.
    ///
    /// Use `with_body()` to provide the updated container properties JSON.
    pub fn replace_container(container: ContainerReference) -> Self {
        let resource_ref: CosmosResourceReference = container.into();
        Self::new(OperationType::Replace, resource_ref, None)
    }

    /// Reads a container's properties from the service.
    ///
    /// Returns the full container properties payload for the container,
    /// including system-managed properties like `_rid`, `_ts`, and `_etag`.
    pub fn read_container(container: ContainerReference) -> Self {
        let resource_ref: CosmosResourceReference = container.into();
        Self::new(OperationType::Read, resource_ref, None)
    }

    /// Reads a container's properties by database and container name.
    ///
    /// Unlike [`read_container`](Self::read_container), this does not require an
    /// already-resolved `ContainerReference`. Use this for initial container
    /// resolution when only the names are known.
    pub fn read_container_by_name(
        database: DatabaseReference,
        container_name: impl Into<std::borrow::Cow<'static, str>>,
    ) -> Self {
        let resource_ref: CosmosResourceReference = CosmosResourceReference::from(database)
            .with_resource_type(ResourceType::DocumentCollection)
            .with_name(container_name.into());
        Self::new(OperationType::Read, resource_ref, None)
    }

    // ===== Data Plane Factory Methods =====

    /// Creates a new item (document) in a container.
    ///
    /// The `ItemReference` contains the container, partition key, and item identifier,
    /// providing all the information needed for the operation.
    /// Use `with_body()` to provide the document JSON.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use azure_data_cosmos_driver::driver::CosmosDriverRuntime;
    /// use azure_data_cosmos_driver::models::{
    ///     AccountReference, CosmosOperation, ItemReference, PartitionKey,
    /// };
    /// use azure_data_cosmos_driver::options::{DriverOptions, OperationOptions};
    /// use url::Url;
    ///
    /// # async fn example() -> azure_data_cosmos_driver::error::Result<()> {
    /// let runtime = CosmosDriverRuntime::builder().build().await?;
    /// let account = AccountReference::with_master_key(
    ///     Url::parse("https://myaccount.documents.azure.com:443/").unwrap(),
    ///     "my-key",
    /// );
    /// let driver = runtime.create_driver(DriverOptions::builder(account).build()).await?;
    /// let container = driver.resolve_container("my-database", "my-container").await?;
    ///
    /// let item = ItemReference::from_name(&container, PartitionKey::from("pk-value"), "doc1");
    /// let result = driver
    ///     .execute_singleton_operation(
    ///         CosmosOperation::create_item(item)
    ///             .with_body(br#"{"id": "doc1", "pk": "pk-value", "data": "hello"}"#.to_vec()),
    ///         OperationOptions::default(),
    ///     )
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn create_item(item: ItemReference) -> Self {
        Self::for_item(OperationType::Create, item)
    }

    /// Reads an item (document) from a container.
    ///
    /// The `ItemReference` contains the container, partition key, and item identifier,
    /// providing all the information needed for the operation.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use azure_data_cosmos_driver::driver::CosmosDriverRuntime;
    /// use azure_data_cosmos_driver::models::{
    ///     AccountReference, CosmosOperation, ItemReference,
    ///     PartitionKey,
    /// };
    /// use azure_data_cosmos_driver::options::{DriverOptions, OperationOptions};
    /// use url::Url;
    ///
    /// # async fn example() -> azure_data_cosmos_driver::error::Result<()> {
    /// let runtime = CosmosDriverRuntime::builder().build().await?;
    /// let account = AccountReference::with_master_key(
    ///     Url::parse("https://myaccount.documents.azure.com:443/").unwrap(),
    ///     "my-key",
    /// );
    /// let driver = runtime.create_driver(DriverOptions::builder(account).build()).await?;
    /// let container = driver.resolve_container("my-database", "my-container").await?;
    ///
    /// let item = ItemReference::from_name(&container, PartitionKey::from("pk-value"), "doc1");
    /// let result = driver
    ///     .execute_singleton_operation(CosmosOperation::read_item(item), OperationOptions::default())
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn read_item(item: ItemReference) -> Self {
        Self::for_item(OperationType::Read, item)
    }

    /// Deletes an item (document) from a container.
    ///
    /// The `ItemReference` contains the container, partition key, and item identifier,
    /// providing all the information needed for the operation.
    pub fn delete_item(item: ItemReference) -> Self {
        Self::for_item(OperationType::Delete, item)
    }

    /// Executes a transactional batch of operations against a single partition.
    ///
    /// All operations in the batch target the same `partition_key` and are
    /// committed atomically. Use `with_body()` to provide the JSON-encoded
    /// array of batch operations.
    pub fn batch(container: ContainerReference, partition_key: PartitionKey) -> Self {
        let range =
            FeedRange::for_partition(partition_key.clone(), container.partition_key_definition());
        let resource_ref: CosmosResourceReference = CosmosResourceReference::from(container)
            .with_resource_type(ResourceType::Document)
            .into_feed_reference();
        Self::new(OperationType::Batch, resource_ref, Some(range))
    }

    /// Upserts (creates or replaces) an item (document) in a container.
    ///
    /// The `ItemReference` contains the container, partition key, and item identifier,
    /// providing all the information needed for the operation.
    /// Use `with_body()` to provide the document JSON.
    /// If an item with the same ID exists, it will be replaced; otherwise, a new item is created.
    pub fn upsert_item(item: ItemReference) -> Self {
        Self::for_item(OperationType::Upsert, item)
    }

    /// Replaces an existing item (document) in a container.
    ///
    /// The `ItemReference` contains the container, partition key, and item identifier,
    /// providing all the information needed for the operation.
    /// Use `with_body()` to provide the new document JSON.
    pub fn replace_item(item: ItemReference) -> Self {
        Self::for_item(OperationType::Replace, item)
    }

    /// Builds a virtual PATCH operation for an item.
    ///
    /// The driver implements PATCH as a client-side Read-Modify-Write loop:
    /// it reads the current item, applies the requested patch operations to
    /// the local JSON document, and issues an ETag-guarded
    /// [`OperationType::Replace`]. The PATCH operation itself is never sent on
    /// the wire; callers build a [`crate::models::PatchInstructions`] and pass it as
    /// the operation body (via [`with_body`](Self::with_body)) — the patch
    /// handler deserializes it before issuing the underlying transport
    /// operations.
    pub fn patch_item(item: ItemReference) -> Self {
        Self::for_item(OperationType::Patch, item)
    }

    /// Builds a distributed transaction coordinator operation.
    #[cfg(feature = "preview_dtx")]
    pub fn distributed_transaction(
        account: AccountReference,
        transaction_type: crate::models::DistributedTransactionType,
    ) -> Self {
        let operation_type = match transaction_type {
            crate::models::DistributedTransactionType::Write => {
                OperationType::CommitDistributedTransaction
            }
            crate::models::DistributedTransactionType::Read => {
                OperationType::ReadDistributedTransaction
            }
        };
        let resource_ref: CosmosResourceReference = CosmosResourceReference::from(account)
            .with_resource_type(ResourceType::DistributedTransactionBatch)
            .with_name(Cow::Borrowed("dtc"));
        Self::new(operation_type, resource_ref, None)
    }

    /// Reads (lists) all items within a single partition.
    ///
    /// Returns a feed of document resources from the specified partition.
    /// This is more efficient than cross-partition reads.
    pub fn read_all_items(container: ContainerReference, partition_key: PartitionKey) -> Self {
        let feed_range =
            FeedRange::for_partition(partition_key, container.partition_key_definition());
        let resource_ref: CosmosResourceReference = CosmosResourceReference::from(container)
            .with_resource_type(ResourceType::Document)
            .into_feed_reference();
        Self::new(OperationType::ReadFeed, resource_ref, Some(feed_range))
    }

    /// Reads (lists) all items across all partitions.
    ///
    /// Returns a feed of document resources from all partitions.
    ///
    /// **Warning:** Cross-partition reads are inherently less efficient than
    /// single-partition reads. Use `read_all_items()` with a partition key
    /// when possible.
    pub fn read_all_items_cross_partition(container: ContainerReference) -> Self {
        let resource_ref: CosmosResourceReference = CosmosResourceReference::from(container)
            .with_resource_type(ResourceType::Document)
            .into_feed_reference();
        Self::new(
            OperationType::ReadFeed,
            resource_ref,
            Some(crate::models::FeedRange::full()),
        )
    }

    /// Creates a change feed read operation for a container.
    ///
    /// Sets the `A-IM` header to `Incremental Feed` (LatestVersion mode) and
    /// marks the operation as a change feed read. The caller sets the start
    /// position via [`with_change_feed_start`](Self::with_change_feed_start),
    /// which both records the marker and emits the matching header.
    ///
    /// Also sets the `x-ms-cosmos-changefeed-wire-format-version` header so the
    /// service returns the structured change feed envelope (`{ current, ... }`)
    /// for every mode. Sending it on LatestVersion (not just
    /// AllVersionsAndDeletes) keeps the response shape consistent across modes:
    /// LatestVersion has no pre-image, but the envelope still carries `current`
    /// plus any per-item metadata, so callers don't have to special-case the
    /// payload per mode. The SDK iterator unwraps `current` back into the
    /// caller's document type.
    ///
    /// `target` scopes the change feed to a specific partition or EPK range.
    /// Pass `None` or `Some(FeedRange::full())` to read the entire container.
    pub fn change_feed(container: ContainerReference, target: Option<FeedRange>) -> Self {
        let resource_ref: CosmosResourceReference = CosmosResourceReference::from(container)
            .with_resource_type(ResourceType::Document)
            .into_feed_reference();
        let mut headers = CosmosRequestHeaders::new();
        headers.incremental_feed = true;
        headers.changefeed_wire_format_version = true;
        let mut operation =
            Self::new(OperationType::ReadFeed, resource_ref, target).with_request_headers(headers);
        operation.is_change_feed = true;
        operation
    }

    /// Queries items in a container.
    ///
    /// Use `with_body()` to provide the query JSON.
    pub fn query_items(container: ContainerReference, target: Option<FeedRange>) -> Self {
        let resource_ref: CosmosResourceReference = CosmosResourceReference::from(container)
            .with_resource_type(ResourceType::Document)
            .into_feed_reference();
        Self::new(OperationType::Query, resource_ref, target)
    }

    /// Creates a query plan request for a container.
    ///
    /// The query plan request is sent to the backend gateway to obtain
    /// execution metadata (partition targeting, rewritten query, etc.)
    /// before issuing the actual cross-partition query.
    ///
    /// Use `with_body()` to provide the query JSON (same as the original query).
    pub fn query_plan(
        container: ContainerReference,
        supported_query_features: Cow<'static, str>,
    ) -> Self {
        let resource_ref: CosmosResourceReference = CosmosResourceReference::from(container)
            .with_resource_type(ResourceType::Document)
            .into_feed_reference();
        let mut headers = CosmosRequestHeaders::new();
        headers.supported_query_features = Some(supported_query_features);
        Self::new(OperationType::QueryPlan, resource_ref, None).with_request_headers(headers)
    }

    /// Creates a read-feed request for partition key ranges in a container.
    ///
    /// Used to populate the partition key range cache for topology resolution.
    #[allow(dead_code)] // Reserved for an upcoming pk-range cache refresh path.
    pub(crate) fn read_partition_key_ranges(container: ContainerReference) -> Self {
        let resource_ref: CosmosResourceReference = CosmosResourceReference::from(container)
            .with_resource_type(ResourceType::PartitionKeyRange)
            .into_feed_reference();
        Self::new(OperationType::ReadFeed, resource_ref, None)
    }

    /// Reads (lists) all partition key ranges for a container.
    ///
    /// Returns a feed of partition key range resources.
    /// Used internally by the partition key range cache to build routing maps.
    ///
    /// **Crate-internal**: this constructor is intentionally not part of the
    /// public API. Public callers should always go through the partition key
    /// range cache (which already invokes this on cache miss) so that reads
    /// benefit from caching, etag-based conditional refresh, and the standard
    /// retry pipeline. Exposing a raw "read all PK ranges" entry point would
    /// invite callers to bypass the cache and hammer the gateway.
    pub(crate) fn read_all_partition_key_ranges(container: ContainerReference) -> Self {
        let resource_ref: CosmosResourceReference = CosmosResourceReference::from(container)
            .with_resource_type(ResourceType::PartitionKeyRange)
            .into_feed_reference();
        Self::new(OperationType::ReadFeed, resource_ref, None)
    }

    /// Returns true if this is a read-only operation.
    pub fn is_read_only(&self) -> bool {
        self.operation_type.is_read_only()
    }

    /// Returns true if this operation is idempotent.
    pub fn is_idempotent(&self) -> bool {
        self.operation_type.is_idempotent()
    }

    /// Returns true if this operation can be planned with a single-node pipeline.
    ///
    /// An operation is "trivial" when it does not require fan-out across multiple
    /// physical partitions. This includes all non-query operations and queries
    /// that target a specific logical partition key (single-partition queries)
    /// OR queries against a non-partitioned resource (Databases, Containers, Offers, etc.).
    ///
    /// Cross-partition queries (those targeting a [`FeedRange`](crate::models::FeedRange))
    /// are **not** trivial and require a backend query plan to determine the
    /// fan-out strategy.
    pub fn is_trivial(&self) -> bool {
        if self.operation_type != OperationType::Query {
            // Change feed is trivial only when targeting a specific logical partition key.
            // Full-container (target=None) and EPK range targets require fan-out.
            if self.is_change_feed() {
                return self.target().and_then(|t| t.partition_key()).is_some();
            }
            // For now, at least, all other non-query operations are trivial.
            return true;
        }

        // A query against a non-partitioned resource is trivial.
        if !self.resource_type.is_partitioned(self.operation_type) {
            return true;
        }

        // Ok, now we have a query, and we have a partitioned resource.
        // That means we need to have a partition key, and know the partition key definition.
        // If we don't have these, it's not trivial.
        let Some(partition_key) = self.target().and_then(|t| t.partition_key()) else {
            return false;
        };

        let Some(pk_def) = self.container().map(|c| c.partition_key_definition()) else {
            // No container, not trivial.
            return false;
        };

        // Finally, a query is trivial ONLY if the partition key is complete (i.e. all PK paths have values).
        pk_def.is_complete(partition_key)
    }

    // -- Offer operations --

    /// Queries offers in the account.
    ///
    /// Use `with_body()` to provide the query JSON and set `Content-Type` and
    /// `x-ms-documentdb-isquery` headers via `OperationOptions::with_custom_headers()`.
    pub fn query_offers(account: AccountReference) -> Self {
        let resource_ref: CosmosResourceReference = CosmosResourceReference::from(account)
            .with_resource_type(ResourceType::Offer)
            .into_feed_reference();
        Self::new(OperationType::Query, resource_ref, None)
    }

    /// Reads a specific offer by its ID.
    ///
    /// For offers, the JSON `"id"` field is the offer RID.
    pub fn read_offer(account: AccountReference, offer_id: impl Into<Cow<'static, str>>) -> Self {
        let resource_ref: CosmosResourceReference = CosmosResourceReference::from(account)
            .with_resource_type(ResourceType::Offer)
            .with_rid(offer_id.into());
        Self::new(OperationType::Read, resource_ref, None)
    }

    /// Replaces a specific offer by its ID.
    ///
    /// For offers, the JSON `"id"` field is the offer RID.
    /// Use `with_body()` to provide the updated offer JSON.
    pub fn replace_offer(
        account: AccountReference,
        offer_id: impl Into<Cow<'static, str>>,
    ) -> Self {
        let resource_ref: CosmosResourceReference = CosmosResourceReference::from(account)
            .with_resource_type(ResourceType::Offer)
            .with_rid(offer_id.into());
        Self::new(OperationType::Replace, resource_ref, None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{
        AccountReference, ContainerProperties, ContainerReference, PartitionKeyDefinition,
        SystemProperties,
    };

    use url::Url;

    fn test_account() -> AccountReference {
        AccountReference::with_master_key(
            Url::parse("https://test.documents.azure.com:443/").unwrap(),
            "test-key",
        )
    }

    fn test_partition_key_definition(path: &str) -> PartitionKeyDefinition {
        serde_json::from_str(&format!(r#"{{"paths":["{path}"]}}"#)).unwrap()
    }

    fn test_container_props() -> ContainerProperties {
        ContainerProperties {
            id: "testcontainer".into(),
            partition_key: test_partition_key_definition("/pk"),
            system_properties: SystemProperties::default(),
        }
    }

    fn test_container() -> ContainerReference {
        ContainerReference::new(
            test_account(),
            "testdb",
            "testdb_rid",
            "testcontainer",
            "testcontainer_rid",
            &test_container_props(),
        )
    }

    #[test]
    fn create_operation() {
        let pk = PartitionKey::from("pk1");
        let item_ref = ItemReference::from_name(&test_container(), pk.clone(), "doc1");
        let op = CosmosOperation::create_item(item_ref);

        assert_eq!(op.operation_type(), OperationType::Create);
        assert_eq!(op.resource_type(), ResourceType::Document);
        assert!(!op.is_read_only());
        assert!(!op.is_idempotent());
    }

    #[test]
    fn read_operation() {
        let pk = PartitionKey::from("pk1");
        let item_ref = ItemReference::from_name(&test_container(), pk.clone(), "doc1");
        let op = CosmosOperation::read_item(item_ref);

        assert_eq!(op.operation_type(), OperationType::Read);
        assert_eq!(op.resource_type(), ResourceType::Document);
        assert!(op.is_read_only());
        assert!(op.is_idempotent());
    }

    #[test]
    fn operation_with_partition_key() {
        let item_ref =
            ItemReference::from_name(&test_container(), PartitionKey::from("pk1"), "doc1");
        let op = CosmosOperation::read_item(item_ref);

        assert!(op
            .target()
            .is_some_and(|target| target.partition_key().is_some()));
    }

    #[test]
    fn operation_with_body() {
        let pk = PartitionKey::from("pk1");
        let item_ref = ItemReference::from_name(&test_container(), pk.clone(), "doc1");
        let body = b"{\"id\":\"doc1\"}".to_vec();
        let op = CosmosOperation::create_item(item_ref).with_body(body.clone());

        assert_eq!(op.body(), Some(body.as_slice()));
    }

    #[test]
    fn replace_is_idempotent() {
        let pk = PartitionKey::from("pk1");
        let item_ref = ItemReference::from_name(&test_container(), pk.clone(), "doc1");
        let op = CosmosOperation::replace_item(item_ref);

        assert!(!op.is_read_only());
        assert!(op.is_idempotent());
    }

    #[test]
    fn upsert_is_not_idempotent() {
        let pk = PartitionKey::from("pk1");
        let item_ref = ItemReference::from_name(&test_container(), pk.clone(), "doc1");
        let op = CosmosOperation::upsert_item(item_ref);

        assert!(!op.is_read_only());
        assert!(!op.is_idempotent());
    }

    #[cfg(feature = "preview_dtx")]
    #[test]
    fn distributed_write_transaction_is_idempotent() {
        let op = CosmosOperation::distributed_transaction(
            test_account(),
            crate::models::DistributedTransactionType::Write,
        );

        assert!(!op.is_read_only());
        assert!(op.is_idempotent());
    }

    #[cfg(feature = "preview_dtx")]
    #[test]
    fn distributed_read_transaction_is_read_only_and_idempotent() {
        let op = CosmosOperation::distributed_transaction(
            test_account(),
            crate::models::DistributedTransactionType::Read,
        );

        assert!(op.is_read_only());
        assert!(op.is_idempotent());
    }

    /// The change feed factory sets both the incremental-feed indicator and the
    /// wire-format-version header so LatestVersion responses use the structured
    /// envelope wire format consistent with AllVersionsAndDeletes.
    #[test]
    fn change_feed_sets_wire_format_header() {
        let op = CosmosOperation::change_feed(test_container(), Some(FeedRange::full()));

        assert!(op.is_change_feed());
        assert!(op.request_headers().incremental_feed);
        assert!(op.request_headers().changefeed_wire_format_version);
    }

    /// Creating a partitioned operation without a partition target panics in
    /// debug builds and silently proceeds in release builds.
    #[test]
    #[cfg_attr(debug_assertions, should_panic)]
    fn rejects_partitioned_operation_without_target() {
        let item_ref =
            ItemReference::from_name(&test_container(), PartitionKey::from("pk1"), "doc1");
        let resource_ref: CosmosResourceReference = item_ref.into();
        let _op = CosmosOperation::new(OperationType::Create, resource_ref, None);
    }
}

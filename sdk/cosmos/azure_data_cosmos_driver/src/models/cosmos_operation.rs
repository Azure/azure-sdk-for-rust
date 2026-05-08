// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Cosmos DB operation representation.

use crate::models::{
    AccountReference, ContainerReference, CosmosRequestHeaders, CosmosResourceReference,
    DatabaseReference, ItemReference, OperationTarget, OperationType, PartitionKey, Precondition,
    ResourceType,
};
use std::borrow::Cow;

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
/// use azure_data_cosmos_driver::options::OperationOptions;
/// use url::Url;
///
/// # async fn example() -> azure_core::Result<()> {
/// // 1. Set up runtime and driver
/// let runtime = CosmosDriverRuntime::builder().build().await?;
/// let account = AccountReference::with_master_key(
///     Url::parse("https://myaccount.documents.azure.com:443/").unwrap(),
///     "my-key",
/// );
/// let driver = runtime.get_or_create_driver(account, None).await?;
///
/// // 2. Resolve the container (reads database + container from service, caches result)
/// let container = driver.resolve_container("mydb", "mycontainer").await?;
///
/// // 3. Build and execute item operations
/// let item = ItemReference::from_name(&container, PartitionKey::from("pk1"), "doc1");
/// let result = driver
///     .execute_operation(CosmosOperation::read_item(item), OperationOptions::default(), None)
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
    target: OperationTarget,
    /// Additional request headers to include in the request.
    request_headers: CosmosRequestHeaders,
    /// Optional request body (raw bytes, schema-agnostic).
    body: Option<Vec<u8>>,
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
    pub fn target(&self) -> &OperationTarget {
        &self.target
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

    /// Sets the precondition for optimistic concurrency control.
    pub fn with_precondition(mut self, precondition: Precondition) -> Self {
        self.request_headers.precondition = Some(precondition);
        self
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

    // ===== Factory Methods =====

    /// Creates a new operation with the specified type, resource reference, and target.
    fn new(
        operation_type: OperationType,
        resource_reference: impl Into<CosmosResourceReference>,
        target: OperationTarget,
    ) -> Self {
        let resource_reference = resource_reference.into();
        let resource_type = resource_reference.resource_type();
        debug_assert!(
            !resource_type.is_partitioned() || target.has_partition_reference(),
            "Attempted to create a partitioned operation without an OperationTarget specifying the partitions to access"
        );
        Self {
            operation_type,
            resource_type,
            resource_reference,
            target,
            request_headers: CosmosRequestHeaders::new(),
            body: None,
        }
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
        Self::new(OperationType::Create, resource_ref, OperationTarget::None)
    }

    /// Reads (lists) all databases in the account.
    ///
    /// Returns a feed of database resources.
    pub fn read_all_databases(account: AccountReference) -> Self {
        let resource_ref = Into::<CosmosResourceReference>::into(account)
            .with_resource_type(ResourceType::Database)
            .into_feed_reference();
        Self::new(OperationType::ReadFeed, resource_ref, OperationTarget::None)
    }

    /// Queries databases in the account.
    ///
    /// Use `with_body()` to provide the query JSON.
    pub fn query_databases(account: AccountReference) -> Self {
        let resource_ref: CosmosResourceReference = CosmosResourceReference::from(account)
            .with_resource_type(ResourceType::Database)
            .into_feed_reference();
        Self::new(OperationType::Query, resource_ref, OperationTarget::None)
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
        Self::new(OperationType::Delete, resource_ref, OperationTarget::None)
    }

    /// Reads a database's properties from the service.
    ///
    /// Returns the database properties payload, including
    /// the system-managed `_rid`, `_ts`, and `_etag`.
    pub fn read_database(database: DatabaseReference) -> Self {
        let resource_ref: CosmosResourceReference = database.into();
        Self::new(OperationType::Read, resource_ref, OperationTarget::None)
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
        Self::new(OperationType::Create, resource_ref, OperationTarget::None)
    }

    /// Reads (lists) all containers in a database.
    ///
    /// Returns a feed of container resources.
    pub fn read_all_containers(database: DatabaseReference) -> Self {
        let resource_ref: CosmosResourceReference = CosmosResourceReference::from(database)
            .with_resource_type(ResourceType::DocumentCollection)
            .into_feed_reference();
        Self::new(OperationType::ReadFeed, resource_ref, OperationTarget::None)
    }

    /// Queries containers in a database.
    ///
    /// Use `with_body()` to provide the query JSON.
    pub fn query_containers(database: DatabaseReference) -> Self {
        let resource_ref: CosmosResourceReference = CosmosResourceReference::from(database)
            .with_resource_type(ResourceType::DocumentCollection)
            .into_feed_reference();
        Self::new(OperationType::Query, resource_ref, OperationTarget::None)
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
    /// use azure_data_cosmos_driver::options::OperationOptions;
    /// use url::Url;
    ///
    /// # async fn example() -> azure_core::Result<()> {
    /// let runtime = CosmosDriverRuntime::builder().build().await?;
    /// let account = AccountReference::with_master_key(
    ///     Url::parse("https://myaccount.documents.azure.com:443/").unwrap(),
    ///     "my-key",
    /// );
    /// let driver = runtime.get_or_create_driver(account, None).await?;
    /// let container = driver.resolve_container("my-database", "my-container").await?;
    ///
    /// let result = driver
    ///     .execute_operation(
    ///         CosmosOperation::delete_container(container),
    ///         OperationOptions::default(),
    ///         None,
    ///     )
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn delete_container(container: ContainerReference) -> Self {
        let resource_ref: CosmosResourceReference = container.into();
        Self::new(OperationType::Delete, resource_ref, OperationTarget::None)
    }

    /// Replaces a container's properties.
    ///
    /// Use `with_body()` to provide the updated container properties JSON.
    pub fn replace_container(container: ContainerReference) -> Self {
        let resource_ref: CosmosResourceReference = container.into();
        Self::new(OperationType::Replace, resource_ref, OperationTarget::None)
    }

    /// Reads a container's properties from the service.
    ///
    /// Returns the full container properties payload for the container,
    /// including system-managed properties like `_rid`, `_ts`, and `_etag`.
    pub fn read_container(container: ContainerReference) -> Self {
        let resource_ref: CosmosResourceReference = container.into();
        Self::new(OperationType::Read, resource_ref, OperationTarget::None)
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
        Self::new(OperationType::Read, resource_ref, OperationTarget::None)
    }

    /// Reads a container's properties by database RID and container RID.
    pub fn read_container_by_rid(
        database: DatabaseReference,
        container_rid: impl Into<std::borrow::Cow<'static, str>>,
    ) -> Self {
        let resource_ref: CosmosResourceReference = CosmosResourceReference::from(database)
            .with_resource_type(ResourceType::DocumentCollection)
            .with_rid(container_rid.into());
        Self::new(OperationType::Read, resource_ref, OperationTarget::None)
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
    /// use azure_data_cosmos_driver::options::OperationOptions;
    /// use url::Url;
    ///
    /// # async fn example() -> azure_core::Result<()> {
    /// let runtime = CosmosDriverRuntime::builder().build().await?;
    /// let account = AccountReference::with_master_key(
    ///     Url::parse("https://myaccount.documents.azure.com:443/").unwrap(),
    ///     "my-key",
    /// );
    /// let driver = runtime.get_or_create_driver(account, None).await?;
    /// let container = driver.resolve_container("my-database", "my-container").await?;
    ///
    /// let item = ItemReference::from_name(&container, PartitionKey::from("pk-value"), "doc1");
    /// let result = driver
    ///     .execute_operation(
    ///         CosmosOperation::create_item(item)
    ///             .with_body(br#"{"id": "doc1", "pk": "pk-value", "data": "hello"}"#.to_vec()),
    ///         OperationOptions::default(),
    ///         None,
    ///     )
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn create_item(item: ItemReference) -> Self {
        let partition_key = item.partition_key().clone();
        Self::new(
            OperationType::Create,
            item,
            OperationTarget::PartitionKey(partition_key),
        )
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
    /// use azure_data_cosmos_driver::options::OperationOptions;
    /// use url::Url;
    ///
    /// # async fn example() -> azure_core::Result<()> {
    /// let runtime = CosmosDriverRuntime::builder().build().await?;
    /// let account = AccountReference::with_master_key(
    ///     Url::parse("https://myaccount.documents.azure.com:443/").unwrap(),
    ///     "my-key",
    /// );
    /// let driver = runtime.get_or_create_driver(account, None).await?;
    /// let container = driver.resolve_container("my-database", "my-container").await?;
    ///
    /// let item = ItemReference::from_name(&container, PartitionKey::from("pk-value"), "doc1");
    /// let result = driver
    ///     .execute_operation(CosmosOperation::read_item(item), OperationOptions::default(), None)
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn read_item(item: ItemReference) -> Self {
        let partition_key = item.partition_key().clone();
        Self::new(
            OperationType::Read,
            item,
            OperationTarget::PartitionKey(partition_key),
        )
    }

    /// Deletes an item (document) from a container.
    ///
    /// The `ItemReference` contains the container, partition key, and item identifier,
    /// providing all the information needed for the operation.
    pub fn delete_item(item: ItemReference) -> Self {
        let partition_key = item.partition_key().clone();
        Self::new(
            OperationType::Delete,
            item,
            OperationTarget::PartitionKey(partition_key),
        )
    }

    /// Executes a transactional batch of operations against a single partition.
    ///
    /// All operations in the batch target the same `partition_key` and are
    /// committed atomically. Use `with_body()` to provide the JSON-encoded
    /// array of batch operations.
    pub fn batch(container: ContainerReference, partition_key: PartitionKey) -> Self {
        let resource_ref: CosmosResourceReference = CosmosResourceReference::from(container)
            .with_resource_type(ResourceType::Document)
            .into_feed_reference();
        Self::new(
            OperationType::Batch,
            resource_ref,
            OperationTarget::PartitionKey(partition_key),
        )
    }

    /// Upserts (creates or replaces) an item (document) in a container.
    ///
    /// The `ItemReference` contains the container, partition key, and item identifier,
    /// providing all the information needed for the operation.
    /// Use `with_body()` to provide the document JSON.
    /// If an item with the same ID exists, it will be replaced; otherwise, a new item is created.
    pub fn upsert_item(item: ItemReference) -> Self {
        let partition_key = item.partition_key().clone();
        Self::new(
            OperationType::Upsert,
            item,
            OperationTarget::PartitionKey(partition_key),
        )
    }

    /// Replaces an existing item (document) in a container.
    ///
    /// The `ItemReference` contains the container, partition key, and item identifier,
    /// providing all the information needed for the operation.
    /// Use `with_body()` to provide the new document JSON.
    pub fn replace_item(item: ItemReference) -> Self {
        let partition_key = item.partition_key().clone();
        Self::new(
            OperationType::Replace,
            item,
            OperationTarget::PartitionKey(partition_key),
        )
    }

    /// Reads (lists) all items within a single partition.
    ///
    /// Returns a feed of document resources from the specified partition.
    /// This is more efficient than cross-partition reads.
    pub fn read_all_items(container: ContainerReference, partition_key: PartitionKey) -> Self {
        let resource_ref: CosmosResourceReference = CosmosResourceReference::from(container)
            .with_resource_type(ResourceType::Document)
            .into_feed_reference();
        Self::new(
            OperationType::ReadFeed,
            resource_ref,
            OperationTarget::PartitionKey(partition_key),
        )
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
            OperationTarget::FeedRange(crate::models::FeedRange::full()),
        )
    }

    /// Queries items in a container.
    ///
    /// The `target` determines partition scope: use
    /// [`OperationTarget::PartitionKey`] for single-partition queries, or
    /// [`OperationTarget::FeedRange`] for cross-partition queries.
    ///
    /// Use `with_body()` to provide the query JSON.
    pub fn query_items(container: ContainerReference, target: OperationTarget) -> Self {
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
    pub(crate) fn query_plan(container: ContainerReference) -> Self {
        let resource_ref: CosmosResourceReference = CosmosResourceReference::from(container)
            .with_resource_type(ResourceType::Document)
            .into_feed_reference();
        let mut headers = CosmosRequestHeaders::new();
        headers.supported_query_features = Some(String::new());
        Self::new(
            OperationType::QueryPlan,
            resource_ref,
            OperationTarget::None,
        )
        .with_request_headers(headers)
    }

    /// Creates a read-feed request for partition key ranges in a container.
    ///
    /// Used to populate the partition key range cache for topology resolution.
    pub(crate) fn read_partition_key_ranges(container: ContainerReference) -> Self {
        let resource_ref: CosmosResourceReference = CosmosResourceReference::from(container)
            .with_resource_type(ResourceType::PartitionKeyRange)
            .into_feed_reference();
        Self::new(OperationType::ReadFeed, resource_ref, OperationTarget::None)
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
        self.operation_type != OperationType::Query
            || !matches!(self.target(), OperationTarget::FeedRange(_))
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
        Self::new(OperationType::Query, resource_ref, OperationTarget::None)
    }

    /// Reads a specific offer by its ID.
    ///
    /// For offers, the JSON `"id"` field is the offer RID.
    pub fn read_offer(account: AccountReference, offer_id: impl Into<Cow<'static, str>>) -> Self {
        let resource_ref: CosmosResourceReference = CosmosResourceReference::from(account)
            .with_resource_type(ResourceType::Offer)
            .with_rid(offer_id.into());
        Self::new(OperationType::Read, resource_ref, OperationTarget::None)
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
        Self::new(OperationType::Replace, resource_ref, OperationTarget::None)
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
        let resource_ref: CosmosResourceReference = item_ref.into();
        let op = CosmosOperation::new(
            OperationType::Create,
            resource_ref,
            OperationTarget::PartitionKey(pk),
        );

        assert_eq!(op.operation_type(), OperationType::Create);
        assert_eq!(op.resource_type(), ResourceType::Document);
        assert!(!op.is_read_only());
        assert!(!op.is_idempotent());
    }

    #[test]
    fn read_operation() {
        let pk = PartitionKey::from("pk1");
        let item_ref = ItemReference::from_name(&test_container(), pk.clone(), "doc1");
        let resource_ref: CosmosResourceReference = item_ref.into();
        let op = CosmosOperation::new(
            OperationType::Read,
            resource_ref,
            OperationTarget::PartitionKey(pk),
        );

        assert_eq!(op.operation_type(), OperationType::Read);
        assert_eq!(op.resource_type(), ResourceType::Document);
        assert!(op.is_read_only());
        assert!(op.is_idempotent());
    }

    #[test]
    fn operation_with_partition_key() {
        let item_ref =
            ItemReference::from_name(&test_container(), PartitionKey::from("pk1"), "doc1");
        let resource_ref: CosmosResourceReference = item_ref.into();
        let op = CosmosOperation::new(
            OperationType::Read,
            resource_ref,
            OperationTarget::PartitionKey(PartitionKey::from("pk1")),
        );

        assert!(matches!(op.target(), OperationTarget::PartitionKey(_)));
    }

    #[test]
    fn operation_with_body() {
        let pk = PartitionKey::from("pk1");
        let item_ref = ItemReference::from_name(&test_container(), pk.clone(), "doc1");
        let resource_ref: CosmosResourceReference = item_ref.into();
        let body = b"{\"id\":\"doc1\"}".to_vec();
        let op = CosmosOperation::new(
            OperationType::Create,
            resource_ref,
            OperationTarget::PartitionKey(pk),
        )
        .with_body(body.clone());

        assert_eq!(op.body(), Some(body.as_slice()));
    }

    #[test]
    fn replace_is_idempotent() {
        let pk = PartitionKey::from("pk1");
        let item_ref = ItemReference::from_name(&test_container(), pk.clone(), "doc1");
        let resource_ref: CosmosResourceReference = item_ref.into();
        let op = CosmosOperation::new(
            OperationType::Replace,
            resource_ref,
            OperationTarget::PartitionKey(pk),
        );

        assert!(!op.is_read_only());
        assert!(op.is_idempotent());
    }

    #[test]
    fn upsert_is_not_idempotent() {
        let pk = PartitionKey::from("pk1");
        let item_ref = ItemReference::from_name(&test_container(), pk.clone(), "doc1");
        let resource_ref: CosmosResourceReference = item_ref.into();
        let op = CosmosOperation::new(
            OperationType::Upsert,
            resource_ref,
            OperationTarget::PartitionKey(pk),
        );

        assert!(!op.is_read_only());
        assert!(!op.is_idempotent());
    }

    /// Creating a partitioned operation without a partition target panics in
    /// debug builds and silently proceeds in release builds.
    #[test]
    #[cfg_attr(debug_assertions, should_panic)]
    fn rejects_partitioned_operation_without_target() {
        let item_ref =
            ItemReference::from_name(&test_container(), PartitionKey::from("pk1"), "doc1");
        let resource_ref: CosmosResourceReference = item_ref.into();
        let _op = CosmosOperation::new(OperationType::Create, resource_ref, OperationTarget::None);
    }
}

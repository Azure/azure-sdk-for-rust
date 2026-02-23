// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Cosmos DB operation representation.

use crate::models::{
    AccountReference, ContainerReference, CosmosRequestHeaders, CosmosResourceReference,
    DatabaseReference, ItemReference, OperationType, PartitionKey, ResourceType,
};

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
///     .execute_operation(CosmosOperation::read_item(item), OperationOptions::new())
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
    /// Optional partition key for data plane operations.
    partition_key: Option<PartitionKey>,
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

    /// Returns the container for this operation, if applicable.
    ///
    /// Returns `None` for account-level and database-level operations.
    pub fn container(&self) -> Option<&ContainerReference> {
        self.resource_reference.container()
    }

    /// Returns the partition key, if set.
    pub fn partition_key(&self) -> Option<&PartitionKey> {
        self.partition_key.as_ref()
    }

    /// Returns the request headers.
    pub fn request_headers(&self) -> &CosmosRequestHeaders {
        &self.request_headers
    }

    /// Returns the request body, if set.
    pub fn body(&self) -> Option<&[u8]> {
        self.body.as_deref()
    }

    /// Sets the partition key for the operation.
    pub fn with_partition_key(mut self, partition_key: impl Into<PartitionKey>) -> Self {
        self.partition_key = Some(partition_key.into());
        self
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
        self.request_headers = self.request_headers.with_session_token(session_token);
        self
    }

    /// Sets the activity ID request header for the operation.
    pub fn with_activity_id(mut self, activity_id: crate::models::ActivityId) -> Self {
        self.request_headers = self.request_headers.with_activity_id(activity_id);
        self
    }

    /// Sets the request body.
    pub fn with_body(mut self, body: Vec<u8>) -> Self {
        self.body = Some(body);
        self
    }

    // ===== Factory Methods =====

    /// Creates a new operation with the specified type and resource reference.
    fn new(
        operation_type: OperationType,
        resource_reference: impl Into<CosmosResourceReference>,
    ) -> Self {
        let resource_reference = resource_reference.into();
        let resource_type = resource_reference.resource_type();
        Self {
            operation_type,
            resource_type,
            resource_reference,
            partition_key: None,
            request_headers: CosmosRequestHeaders::new(),
            body: None,
        }
    }

    /// Creates a new operation with the specified type, resource reference, and partition key.
    fn new_with_partition_key(
        operation_type: OperationType,
        resource_reference: impl Into<CosmosResourceReference>,
        partition_key: PartitionKey,
    ) -> Self {
        let resource_reference = resource_reference.into();
        let resource_type = resource_reference.resource_type();
        Self {
            operation_type,
            resource_type,
            resource_reference,
            partition_key: Some(partition_key),
            request_headers: CosmosRequestHeaders::new(),
            body: None,
        }
    }

    /// Creates a Create operation.
    ///
    /// Accepts any type that can be converted into a `CosmosResourceReference`,
    /// including typed references like `ItemReference`, `ContainerReference`, etc.
    #[cfg(test)]
    pub(crate) fn create(resource_reference: impl Into<CosmosResourceReference>) -> Self {
        Self::new(OperationType::Create, resource_reference)
    }

    /// Creates a Read operation.
    ///
    /// Accepts any type that can be converted into a `CosmosResourceReference`,
    /// including typed references like `ItemReference`, `ContainerReference`, etc.
    #[cfg(test)]
    pub(crate) fn read(resource_reference: impl Into<CosmosResourceReference>) -> Self {
        Self::new(OperationType::Read, resource_reference)
    }

    /// Creates a Replace operation.
    ///
    /// Accepts any type that can be converted into a `CosmosResourceReference`,
    /// including typed references like `ItemReference`, `ContainerReference`, etc.
    #[cfg(test)]
    pub(crate) fn replace(resource_reference: impl Into<CosmosResourceReference>) -> Self {
        Self::new(OperationType::Replace, resource_reference)
    }

    /// Creates an Upsert operation.
    ///
    /// Accepts any type that can be converted into a `CosmosResourceReference`,
    /// including typed references like `ItemReference`.
    #[cfg(test)]
    pub(crate) fn upsert(resource_reference: impl Into<CosmosResourceReference>) -> Self {
        Self::new(OperationType::Upsert, resource_reference)
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
        Self::new(OperationType::Create, resource_ref)
    }

    /// Reads (lists) all databases in the account.
    ///
    /// Returns a feed of database resources.
    pub fn read_all_databases(account: AccountReference) -> Self {
        let resource_ref = Into::<CosmosResourceReference>::into(account)
            .with_resource_type(ResourceType::Database)
            .into_feed_reference();
        Self::new(OperationType::ReadFeed, resource_ref)
    }

    /// Queries databases in the account.
    ///
    /// Use `with_body()` to provide the query JSON.
    pub fn query_databases(account: AccountReference) -> Self {
        let resource_ref: CosmosResourceReference = CosmosResourceReference::from(account)
            .with_resource_type(ResourceType::Database)
            .into_feed_reference();
        Self::new(OperationType::Query, resource_ref)
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
        Self::new(OperationType::Delete, resource_ref)
    }

    /// Reads a database's properties from the service.
    ///
    /// Returns the database properties payload, including
    /// the system-managed `_rid`, `_ts`, and `_etag`.
    pub fn read_database(database: DatabaseReference) -> Self {
        let resource_ref: CosmosResourceReference = database.into();
        Self::new(OperationType::Read, resource_ref)
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
        Self::new(OperationType::Create, resource_ref)
    }

    /// Reads (lists) all containers in a database.
    ///
    /// Returns a feed of container resources.
    pub fn read_all_containers(database: DatabaseReference) -> Self {
        let resource_ref: CosmosResourceReference = CosmosResourceReference::from(database)
            .with_resource_type(ResourceType::DocumentCollection)
            .into_feed_reference();
        Self::new(OperationType::ReadFeed, resource_ref)
    }

    /// Queries containers in a database.
    ///
    /// Use `with_body()` to provide the query JSON.
    pub fn query_containers(database: DatabaseReference) -> Self {
        let resource_ref: CosmosResourceReference = CosmosResourceReference::from(database)
            .with_resource_type(ResourceType::DocumentCollection)
            .into_feed_reference();
        Self::new(OperationType::Query, resource_ref)
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
    ///         OperationOptions::new(),
    ///     )
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn delete_container(container: ContainerReference) -> Self {
        let resource_ref: CosmosResourceReference = container.into();
        Self::new(OperationType::Delete, resource_ref)
    }

    /// Reads a container's properties from the service.
    ///
    /// Returns the full container properties payload for the container,
    /// including system-managed properties like `_rid`, `_ts`, and `_etag`.
    pub fn read_container(container: ContainerReference) -> Self {
        let resource_ref: CosmosResourceReference = container.into();
        Self::new(OperationType::Read, resource_ref)
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
        Self::new(OperationType::Read, resource_ref)
    }

    /// Reads a container's properties by database RID and container RID.
    pub fn read_container_by_rid(
        database: DatabaseReference,
        container_rid: impl Into<std::borrow::Cow<'static, str>>,
    ) -> Self {
        let resource_ref: CosmosResourceReference = CosmosResourceReference::from(database)
            .with_resource_type(ResourceType::DocumentCollection)
            .with_rid(container_rid.into());
        Self::new(OperationType::Read, resource_ref)
    }

    // ===== Data Plane Factory Methods =====

    /// Creates an item (document) in a container.
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
    ///     .execute_operation(
    ///         CosmosOperation::create_item(item)
    ///             .with_body(br#"{"id": "doc1", "pk": "pk-value", "data": "hello"}"#.to_vec()),
    ///         OperationOptions::new(),
    ///     )
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn create_item(item: ItemReference) -> Self {
        let partition_key = item.partition_key().clone();
        Self::new_with_partition_key(OperationType::Create, item, partition_key)
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
    ///     .execute_operation(CosmosOperation::read_item(item), OperationOptions::new())
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn read_item(item: ItemReference) -> Self {
        let partition_key = item.partition_key().clone();
        Self::new_with_partition_key(OperationType::Read, item, partition_key)
    }

    /// Deletes an item (document) from a container.
    ///
    /// The `ItemReference` contains the container, partition key, and item identifier,
    /// providing all the information needed for the operation.
    pub fn delete_item(item: ItemReference) -> Self {
        let partition_key = item.partition_key().clone();
        Self::new_with_partition_key(OperationType::Delete, item, partition_key)
    }

    /// Upserts (creates or replaces) an item (document) in a container.
    ///
    /// The `ItemReference` contains the container, partition key, and item identifier,
    /// providing all the information needed for the operation.
    /// Use `with_body()` to provide the document JSON.
    /// If an item with the same ID exists, it will be replaced; otherwise, a new item is created.
    pub fn upsert_item(item: ItemReference) -> Self {
        let partition_key = item.partition_key().clone();
        Self::new_with_partition_key(OperationType::Upsert, item, partition_key)
    }

    /// Replaces an existing item (document) in a container.
    ///
    /// The `ItemReference` contains the container, partition key, and item identifier,
    /// providing all the information needed for the operation.
    /// Use `with_body()` to provide the new document JSON.
    pub fn replace_item(item: ItemReference) -> Self {
        let partition_key = item.partition_key().clone();
        Self::new_with_partition_key(OperationType::Replace, item, partition_key)
    }

    /// Reads (lists) all items within a single partition.
    ///
    /// Returns a feed of document resources from the specified partition.
    /// This is more efficient than cross-partition reads.
    pub fn read_all_items(container: ContainerReference, partition_key: PartitionKey) -> Self {
        let resource_ref: CosmosResourceReference = CosmosResourceReference::from(container)
            .with_resource_type(ResourceType::Document)
            .into_feed_reference();
        Self::new_with_partition_key(OperationType::ReadFeed, resource_ref, partition_key)
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
        Self::new(OperationType::ReadFeed, resource_ref)
    }

    /// Queries items within a single partition.
    ///
    /// Use `with_body()` to provide the query JSON.
    /// This is more efficient than cross-partition queries.
    pub fn query_items(container: ContainerReference, partition_key: PartitionKey) -> Self {
        let resource_ref: CosmosResourceReference = CosmosResourceReference::from(container)
            .with_resource_type(ResourceType::Document)
            .into_feed_reference();
        Self::new_with_partition_key(OperationType::Query, resource_ref, partition_key)
    }

    /// Queries items across all partitions.
    ///
    /// Use `with_body()` to provide the query JSON.
    ///
    /// **Warning:** Cross-partition queries are inherently less efficient than
    /// single-partition queries. Use `query_items()` with a partition key
    /// when possible.
    pub fn query_items_cross_partition(container: ContainerReference) -> Self {
        let resource_ref: CosmosResourceReference = CosmosResourceReference::from(container)
            .with_resource_type(ResourceType::Document)
            .into_feed_reference();
        Self::new(OperationType::Query, resource_ref)
    }

    /// Returns true if this is a read-only operation.
    pub fn is_read_only(&self) -> bool {
        self.operation_type.is_read_only()
    }

    /// Returns true if this operation is idempotent.
    pub fn is_idempotent(&self) -> bool {
        self.operation_type.is_idempotent()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{AccountReference, ContainerReference, PartitionKeyDefinition};

    use url::Url;

    fn test_account() -> AccountReference {
        AccountReference::with_master_key(
            Url::parse("https://test.documents.azure.com:443/").unwrap(),
            "test-key",
        )
    }

    fn test_container_props() -> crate::models::ContainerProperties {
        crate::models::ContainerProperties::new(
            "testcontainer",
            PartitionKeyDefinition::new(["/pk"]),
        )
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
        let item_ref =
            ItemReference::from_name(&test_container(), PartitionKey::from("pk1"), "doc1");
        let resource_ref: CosmosResourceReference = item_ref.into();
        let op = CosmosOperation::create(resource_ref);

        assert_eq!(op.operation_type(), OperationType::Create);
        assert_eq!(op.resource_type(), ResourceType::Document);
        assert!(!op.is_read_only());
        assert!(!op.is_idempotent());
    }

    #[test]
    fn read_operation() {
        let item_ref =
            ItemReference::from_name(&test_container(), PartitionKey::from("pk1"), "doc1");
        let resource_ref: CosmosResourceReference = item_ref.into();
        let op = CosmosOperation::read(resource_ref);

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
        let op = CosmosOperation::read(resource_ref).with_partition_key(PartitionKey::from("pk1"));

        assert!(op.partition_key().is_some());
    }

    #[test]
    fn operation_with_body() {
        let item_ref =
            ItemReference::from_name(&test_container(), PartitionKey::from("pk1"), "doc1");
        let resource_ref: CosmosResourceReference = item_ref.into();
        let body = b"{\"id\":\"doc1\"}".to_vec();
        let op = CosmosOperation::create(resource_ref).with_body(body.clone());

        assert_eq!(op.body(), Some(body.as_slice()));
    }

    #[test]
    fn replace_is_idempotent() {
        let item_ref =
            ItemReference::from_name(&test_container(), PartitionKey::from("pk1"), "doc1");
        let resource_ref: CosmosResourceReference = item_ref.into();
        let op = CosmosOperation::replace(resource_ref);

        assert!(!op.is_read_only());
        assert!(op.is_idempotent());
    }

    #[test]
    fn upsert_is_not_idempotent() {
        let item_ref =
            ItemReference::from_name(&test_container(), PartitionKey::from("pk1"), "doc1");
        let resource_ref: CosmosResourceReference = item_ref.into();
        let op = CosmosOperation::upsert(resource_ref);

        assert!(!op.is_read_only());
        assert!(!op.is_idempotent());
    }
}

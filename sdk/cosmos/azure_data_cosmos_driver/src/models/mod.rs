// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Data models for Cosmos DB management and metadata operations.
//!
//! This module contains types representing Cosmos DB resources (accounts, databases, containers)
//! and their supporting structures. These are for **metadata/management operations only**.
//!
//! **Important**: This module does NOT contain data plane item/document types.
//! The driver is schema-agnostic - data plane operations work with raw bytes (`&[u8]`).

mod account_reference;
mod activity_id;
mod connection_string;
mod cosmos_headers;
mod cosmos_operation;
mod cosmos_resource_reference;
mod cosmos_result;
mod cosmos_status;
mod etag;
mod partition_key;
mod request_charge;
mod resource_id;
mod resource_reference;
mod user_agent;

pub use account_reference::{AccountReference, AccountReferenceBuilder, AuthOptions};
pub use activity_id::ActivityId;
pub use connection_string::ConnectionString;
pub use cosmos_headers::{CosmosRequestHeaders, CosmosResponseHeaders};
pub use cosmos_operation::CosmosOperation;
pub use cosmos_resource_reference::CosmosResourceReference;
pub use cosmos_result::CosmosResult;
pub use cosmos_status::CosmosStatus;
pub use cosmos_status::SubStatusCode;
pub use etag::{ETag, ETagCondition};
pub use partition_key::PartitionKey;
pub use request_charge::RequestCharge;
pub use resource_reference::ContainerReference;
pub use resource_reference::{DatabaseReference, ItemReference};
pub use resource_reference::{
    PartitionKeyRangeReference, StoredProcedureReference, TriggerReference, UdfReference,
};
pub use user_agent::UserAgent;

pub(crate) use account_reference::AccountEndpoint;

use crate::options::Region;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// Properties of a Cosmos DB account.
///
/// Contains metadata about a Cosmos DB account including its regions and capabilities.
/// Used internally by the driver for routing and caching.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct AccountProperties {
    /// The account's primary (write) region.
    pub write_region: Region,
    /// All readable regions for this account (ordered by preference).
    pub read_regions: Vec<Region>,
    /// The system-assigned resource ID for the account.
    pub rid: Option<String>,
}

impl AccountProperties {
    /// Creates new account properties.
    pub fn new(write_region: Region, read_regions: Vec<Region>) -> Self {
        Self {
            write_region,
            read_regions,
            rid: None,
        }
    }

    /// Sets the account's resource ID.
    pub fn with_rid(mut self, rid: impl Into<String>) -> Self {
        self.rid = Some(rid.into());
        self
    }

    /// Sets account system-managed properties.
    pub fn with_system_properties(mut self, system_properties: SystemProperties) -> Self {
        self.rid = system_properties.rid;
        self
    }
}

/// Properties of a Cosmos DB database.
///
/// Returned by database read/query operations and used when creating databases.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[non_exhaustive]
pub(crate) struct DatabaseProperties {
    /// Unique identifier for the database within the account.
    pub id: Cow<'static, str>,

    /// System-managed properties (e.g., _rid, _ts, _etag).
    #[serde(flatten)]
    pub(crate) system_properties: SystemProperties,
}

impl DatabaseProperties {
    /// Creates new database properties with the given identifier.
    ///
    /// # Panics
    ///
    /// Panics if `id` is empty.
    pub fn new(id: impl Into<Cow<'static, str>>) -> Self {
        let id = id.into();
        assert!(!id.is_empty(), "database id must not be empty");
        Self {
            id,
            system_properties: SystemProperties::default(),
        }
    }

    /// Sets the database's system-managed properties.
    pub(crate) fn with_system_properties(mut self, system_properties: SystemProperties) -> Self {
        self.system_properties = system_properties;
        self
    }
}

/// Properties of a Cosmos DB container.
///
/// Returned by container read/query operations and used when creating/updating containers.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[non_exhaustive]
pub(crate) struct ContainerProperties {
    /// Unique identifier for the container within the database.
    pub id: Cow<'static, str>,

    /// Partition key definition specifying the partition key path(s).
    pub partition_key: PartitionKeyDefinition,

    /// System-managed properties (e.g., _rid, _ts, _etag).
    #[serde(flatten)]
    pub system_properties: SystemProperties,
}

impl ContainerProperties {
    /// Creates new container properties with the given identifier and partition key.
    ///
    /// # Panics
    ///
    /// Panics if `id` is empty.
    pub fn new(id: impl Into<Cow<'static, str>>, partition_key: PartitionKeyDefinition) -> Self {
        let id = id.into();
        assert!(!id.is_empty(), "container id must not be empty");
        Self {
            id,
            partition_key,
            system_properties: SystemProperties::default(),
        }
    }

    /// Sets the container's system-managed properties.
    pub fn with_system_properties(mut self, system_properties: SystemProperties) -> Self {
        self.system_properties = system_properties;
        self
    }
}

/// Immutable container properties that never change after creation.
///
/// This struct captures the subset of [`ContainerProperties`] that is fixed at container
/// creation time: the partition key definition.
/// It is designed to be wrapped in an `Arc` and carried on resolved references.
///
/// Not exposed publicly — use [`ContainerReference::partition_key()`] to access members.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct ImmutableContainerProperties {
    /// Partition key definition specifying the partition key path(s).
    partition_key: PartitionKeyDefinition,
}

impl ImmutableContainerProperties {
    /// Creates immutable container properties from a full `ContainerProperties`.
    pub(crate) fn from_container_properties(props: &ContainerProperties) -> Self {
        Self {
            partition_key: props.partition_key.clone(),
        }
    }

    /// Returns the partition key definition.
    pub(crate) fn partition_key(&self) -> &PartitionKeyDefinition {
        &self.partition_key
    }
}

/// Partition key definition for a container.
///
/// Specifies the JSON path(s) used for partitioning data across physical partitions.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[non_exhaustive]
pub(crate) struct PartitionKeyDefinition {
    /// List of partition key paths (e.g., `["/tenantId"]` for single partition key).
    pub paths: Vec<Cow<'static, str>>,

    /// Partition key version (1 for single, 2 for hierarchical).
    #[serde(default = "default_pk_version")]
    pub version: u32,

    /// Partition key kind (Hash is the standard).
    #[serde(default)]
    pub kind: PartitionKeyKind,
}

impl PartitionKeyDefinition {
    /// Creates a new partition key definition with the given paths.
    ///
    /// Uses version 2 and `Hash` kind by default.
    pub fn new(paths: impl IntoIterator<Item = impl Into<Cow<'static, str>>>) -> Self {
        Self {
            paths: paths.into_iter().map(Into::into).collect(),
            version: 2,
            kind: PartitionKeyKind::Hash,
        }
    }
}

fn default_pk_version() -> u32 {
    2
}

/// Partition key kind.
#[derive(Clone, Copy, Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
#[non_exhaustive]
pub enum PartitionKeyKind {
    /// Hash partitioning (standard).
    #[default]
    Hash,
    /// Range partitioning (legacy).
    Range,
}

/// System-managed properties present on all Cosmos DB resources.
#[derive(Clone, Default, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[non_exhaustive]
pub(crate) struct SystemProperties {
    /// Resource ID (internal identifier).
    #[serde(rename = "_rid", skip_serializing_if = "Option::is_none")]
    pub rid: Option<String>,

    /// Resource timestamp (last modified time in Unix epoch seconds).
    #[serde(rename = "_ts", skip_serializing_if = "Option::is_none")]
    pub ts: Option<u64>,

    /// ETag for optimistic concurrency control.
    #[serde(rename = "_etag", skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
}

impl SystemProperties {
    /// Sets the resource ID.
    pub fn with_rid(mut self, rid: impl Into<String>) -> Self {
        self.rid = Some(rid.into());
        self
    }

    /// Sets the resource timestamp.
    pub fn with_ts(mut self, ts: u64) -> Self {
        self.ts = Some(ts);
        self
    }

    /// Sets the ETag.
    pub fn with_etag(mut self, etag: impl Into<String>) -> Self {
        self.etag = Some(etag.into());
        self
    }
}

// ── ResourceType & OperationType (moved from resource_types.rs) ─────────────

/// The type of resource being operated on.
///
/// Used to identify the Cosmos DB resource category for routing and authorization.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum ResourceType {
    /// Database account (root level).
    DatabaseAccount,
    /// A database within an account.
    Database,
    /// A container (collection) within a database.
    DocumentCollection,
    /// A document (item) within a container.
    Document,
    /// A stored procedure within a container.
    StoredProcedure,
    /// A trigger within a container.
    Trigger,
    /// A user-defined function within a container.
    UserDefinedFunction,
    /// A partition key range within a container.
    PartitionKeyRange,
    /// An offer (throughput configuration).
    Offer,
}

impl ResourceType {
    /// Returns the URL path segment for this resource type.
    pub fn path_segment(self) -> &'static str {
        match self {
            ResourceType::DatabaseAccount => "",
            ResourceType::Database => "dbs",
            ResourceType::DocumentCollection => "colls",
            ResourceType::Document => "docs",
            ResourceType::StoredProcedure => "sprocs",
            ResourceType::Trigger => "triggers",
            ResourceType::UserDefinedFunction => "udfs",
            ResourceType::PartitionKeyRange => "pkranges",
            ResourceType::Offer => "offers",
        }
    }

    /// Returns true if this resource type is metadata (not data plane items).
    pub fn is_metadata(self) -> bool {
        matches!(
            self,
            ResourceType::DatabaseAccount
                | ResourceType::Database
                | ResourceType::DocumentCollection
                | ResourceType::PartitionKeyRange
                | ResourceType::Offer
        )
    }

    /// Returns true if this resource type requires a container reference.
    pub fn requires_container(self) -> bool {
        matches!(
            self,
            ResourceType::Document
                | ResourceType::DocumentCollection
                | ResourceType::StoredProcedure
                | ResourceType::Trigger
                | ResourceType::UserDefinedFunction
                | ResourceType::PartitionKeyRange
        )
    }

    /// Returns true if this resource type requires a database reference.
    pub fn requires_database(self) -> bool {
        matches!(
            self,
            ResourceType::Database
                | ResourceType::DocumentCollection
                | ResourceType::Document
                | ResourceType::StoredProcedure
                | ResourceType::Trigger
                | ResourceType::UserDefinedFunction
                | ResourceType::PartitionKeyRange
        )
    }
}

/// The type of operation being performed.
///
/// Used to determine HTTP method, retry behavior, and authorization requirements.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum OperationType {
    /// Create a new resource.
    Create,
    /// Read an existing resource.
    Read,
    /// Read a feed (list) of resources.
    ReadFeed,
    /// Replace an existing resource.
    Replace,
    /// Delete a resource.
    Delete,
    /// Create or replace a resource.
    Upsert,
    /// Execute a query.
    Query,
    /// Execute a SQL query.
    SqlQuery,
    /// Get a query plan.
    QueryPlan,
    /// Execute a batch operation.
    Batch,
    /// Check resource existence (HEAD).
    Head,
    /// Check feed existence (HEAD).
    HeadFeed,
    /// Execute a stored procedure.
    Execute,
}

impl OperationType {
    /// Returns the HTTP method for this operation type.
    pub fn http_method(self) -> azure_core::http::Method {
        use azure_core::http::Method;
        match self {
            OperationType::Create
            | OperationType::Upsert
            | OperationType::Query
            | OperationType::SqlQuery
            | OperationType::Batch
            | OperationType::QueryPlan
            | OperationType::Execute => Method::Post,
            OperationType::Delete => Method::Delete,
            OperationType::Read => Method::Get,
            OperationType::ReadFeed => Method::Get,
            OperationType::Replace => Method::Put,
            OperationType::Head | OperationType::HeadFeed => Method::Head,
        }
    }

    /// Returns true if the operation does not modify server state.
    pub fn is_read_only(self) -> bool {
        matches!(
            self,
            OperationType::Read
                | OperationType::ReadFeed
                | OperationType::Query
                | OperationType::SqlQuery
                | OperationType::QueryPlan
                | OperationType::Head
                | OperationType::HeadFeed
        )
    }

    /// Returns true if the operation is idempotent (safe to retry).
    pub fn is_idempotent(self) -> bool {
        matches!(
            self,
            OperationType::Read
                | OperationType::ReadFeed
                | OperationType::Query
                | OperationType::SqlQuery
                | OperationType::QueryPlan
                | OperationType::Head
                | OperationType::HeadFeed
                | OperationType::Replace
                | OperationType::Delete
        )
    }
}

// ── SessionToken (moved from session.rs) ────────────────────────────────────

/// A session token for maintaining session consistency.
///
/// Session tokens track the logical sequence number of operations, enabling
/// read-your-writes consistency within a session.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct SessionToken(pub Cow<'static, str>);

impl SessionToken {
    /// Creates a new session token with the given value.
    pub fn new(value: impl Into<Cow<'static, str>>) -> Self {
        Self(value.into())
    }

    /// Returns the session token value as a string slice.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl<T: Into<Cow<'static, str>>> From<T> for SessionToken {
    fn from(value: T) -> Self {
        Self::new(value)
    }
}

impl std::fmt::Display for SessionToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

// ── TriggerInvocation (moved from triggers.rs) ──────────────────────────────

/// Represents a trigger to be invoked before or after an operation.
///
/// Triggers are server-side scripts that can be automatically invoked
/// during create, update, and delete operations on items.
///
/// This type is serialized into request headers to specify which trigger to invoke.
/// For resource references to trigger definitions, see the resource reference types.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct TriggerInvocation {
    /// The name/id of the trigger to invoke.
    pub name: Cow<'static, str>,
}

impl TriggerInvocation {
    /// Creates a new trigger invocation with the given name.
    pub fn new(name: impl Into<Cow<'static, str>>) -> Self {
        Self { name: name.into() }
    }
}

impl From<&'static str> for TriggerInvocation {
    fn from(name: &'static str) -> Self {
        Self::new(name)
    }
}

impl From<String> for TriggerInvocation {
    fn from(name: String) -> Self {
        Self::new(name)
    }
}

// ── ThroughputControlGroupName (moved from throughput_control.rs) ───────────

/// Unique name identifying a throughput control group.
///
/// This name is serialized into request headers when referencing a control group.
/// The group configuration is defined separately via [`ThroughputControlGroupOptions`](crate::options::ThroughputControlGroupOptions).
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ThroughputControlGroupName(pub Cow<'static, str>);

impl ThroughputControlGroupName {
    /// Creates a new throughput control group name.
    pub fn new(name: impl Into<Cow<'static, str>>) -> Self {
        Self(name.into())
    }

    /// Returns the name as a string slice.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<&'static str> for ThroughputControlGroupName {
    fn from(name: &'static str) -> Self {
        Self::new(name)
    }
}

impl From<String> for ThroughputControlGroupName {
    fn from(name: String) -> Self {
        Self::new(name)
    }
}

impl From<Cow<'static, str>> for ThroughputControlGroupName {
    fn from(name: Cow<'static, str>) -> Self {
        Self::new(name)
    }
}

impl std::fmt::Display for ThroughputControlGroupName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

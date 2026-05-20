// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use crate::models::ThroughputProperties;
use std::fmt;
use std::fmt::Display;

// Re-exported types that form part of the azure_data_cosmos public API.
#[doc(inline)]
pub use azure_data_cosmos_driver::models::{
    ETag, MaxItemCount as MaxItemCountHint, Precondition, SessionToken, ThroughputControlGroupName,
};
#[doc(inline)]
pub use azure_data_cosmos_driver::options::{
    ContentResponseOnWrite, EndToEndOperationLatencyPolicy, ExcludedRegions, OperationOptions,
    OperationOptionsBuilder, OperationOptionsView, PriorityLevel, ReadConsistencyStrategy, Region,
    ThroughputControlGroupOptions, UserAgentSuffix,
};

/// Options used when creating a [`CosmosClient`](crate::CosmosClient).
///
/// This struct is used internally by [`CosmosClientBuilder`](crate::CosmosClientBuilder).
/// Use the builder pattern via [`CosmosClient::builder()`](crate::CosmosClient::builder())
/// to configure client options.
#[derive(Clone, Default, Debug)]
#[non_exhaustive]
pub struct CosmosClientOptions {
    /// Default [`OperationOptions`] applied to all requests made by this client,
    /// unless overridden by per-request options.
    pub(crate) operation: OperationOptions,
    pub(crate) user_agent_suffix: Option<UserAgentSuffix>,
    pub(crate) application_region: Option<Region>,
}

impl CosmosClientOptions {
    pub fn with_user_agent_suffix(mut self, suffix: UserAgentSuffix) -> Self {
        self.user_agent_suffix = Some(suffix);
        self
    }

    pub fn with_operation_options(mut self, operation: OperationOptions) -> Self {
        self.operation = operation;
        self
    }
}

/// Options to be passed to [`DatabaseClient::create_container()`](crate::clients::DatabaseClient::create_container()).
#[derive(Clone, Default)]
#[non_exhaustive]
pub struct CreateContainerOptions {
    pub(crate) throughput: Option<ThroughputProperties>,
}

impl CreateContainerOptions {
    /// Sets the throughput properties for the new container.
    pub fn with_throughput(mut self, throughput: ThroughputProperties) -> Self {
        self.throughput = Some(throughput);
        self
    }
}

/// Options to be passed to [`ContainerClient::replace()`](crate::clients::ContainerClient::replace()).
#[derive(Clone, Default)]
#[non_exhaustive]
pub struct ReplaceContainerOptions;

/// Options to be passed to [`CosmosClient::create_database()`](crate::CosmosClient::create_database()).
#[derive(Clone, Default)]
#[non_exhaustive]
pub struct CreateDatabaseOptions;

/// Options to be passed to [`ContainerClient::delete()`](crate::clients::ContainerClient::delete()).
#[derive(Clone, Default)]
#[non_exhaustive]
pub struct DeleteContainerOptions;

/// Options to be passed to [`DatabaseClient::delete()`](crate::clients::DatabaseClient::delete()).
#[derive(Clone, Default)]
#[non_exhaustive]
pub struct DeleteDatabaseOptions;

/// Specifies consistency levels for Cosmos DB accounts.
///
/// This is a model type for account-level consistency properties returned by the service.
/// For per-request consistency, use [`ReadConsistencyStrategy`] via [`OperationOptions`].
///
/// Learn more at [Consistency Levels](https://learn.microsoft.com/azure/cosmos-db/consistency-levels).
#[derive(Clone, Debug)]
pub enum ConsistencyLevel {
    ConsistentPrefix,
    Eventual,
    Session,
    BoundedStaleness,
    Strong,
}

impl Display for ConsistencyLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            ConsistencyLevel::ConsistentPrefix => "ConsistentPrefix",
            ConsistencyLevel::Eventual => "Eventual",
            ConsistencyLevel::Session => "Session",
            ConsistencyLevel::BoundedStaleness => "BoundedStaleness",
            ConsistencyLevel::Strong => "Strong",
        };
        write!(f, "{}", value)
    }
}

/// Options for item point-read operations.
///
/// Used by [`ContainerClient::read_item()`](crate::clients::ContainerClient::read_item).
///
/// General-purpose settings such as custom headers and excluded regions are configured
/// via the [`operation`](Self::operation) field. See [`OperationOptions`] for details.
#[derive(Clone, Default)]
#[non_exhaustive]
pub struct ItemReadOptions {
    /// General-purpose options that apply to this request.
    /// See [`OperationOptions`] for available settings and layered resolution behavior.
    pub operation: OperationOptions,

    /// Session token for session-consistent reads.
    pub session_token: Option<SessionToken>,

    /// Conditional ETag check. For reads, typically [`Precondition::IfNoneMatch`]
    /// (returns 304 Not Modified if unchanged).
    pub precondition: Option<Precondition>,
}

impl ItemReadOptions {
    /// Sets the session token for this request.
    pub fn with_session_token(mut self, session_token: impl Into<SessionToken>) -> Self {
        self.session_token = Some(session_token.into());
        self
    }

    /// Sets a conditional ETag check for this request.
    pub fn with_precondition(mut self, precondition: Precondition) -> Self {
        self.precondition = Some(precondition);
        self
    }

    /// Sets the [`OperationOptions`] for this request.
    pub fn with_operation_options(mut self, operation: OperationOptions) -> Self {
        self.operation = operation;
        self
    }
}

/// Options for item write operations.
///
/// Used by [`ContainerClient::create_item()`](crate::clients::ContainerClient::create_item),
/// [`ContainerClient::replace_item()`](crate::clients::ContainerClient::replace_item),
/// [`ContainerClient::upsert_item()`](crate::clients::ContainerClient::upsert_item), and
/// [`ContainerClient::delete_item()`](crate::clients::ContainerClient::delete_item).
///
/// General-purpose settings such as custom headers, excluded regions, and content
/// response behavior are configured via the [`operation`](Self::operation) field.
/// See [`OperationOptions`] for details.
#[derive(Clone, Default)]
#[non_exhaustive]
pub struct ItemWriteOptions {
    /// General-purpose options that apply to this request.
    /// See [`OperationOptions`] for available settings and layered resolution behavior.
    pub operation: OperationOptions,

    /// Session token for session-consistent writes.
    pub session_token: Option<SessionToken>,

    /// Conditional ETag check. For writes, typically [`Precondition::IfMatch`]
    /// (optimistic concurrency).
    pub precondition: Option<Precondition>,
}

impl ItemWriteOptions {
    /// Sets the session token for this request.
    pub fn with_session_token(mut self, session_token: impl Into<SessionToken>) -> Self {
        self.session_token = Some(session_token.into());
        self
    }

    /// Sets a conditional ETag check for this request.
    pub fn with_precondition(mut self, precondition: Precondition) -> Self {
        self.precondition = Some(precondition);
        self
    }

    /// Sets the [`OperationOptions`] for this request.
    pub fn with_operation_options(mut self, operation: OperationOptions) -> Self {
        self.operation = operation;
        self
    }
}

/// Options for [`ContainerClient::patch_item()`](crate::clients::ContainerClient::patch_item()).
///
/// PATCH is implemented driver-side as a Read-Modify-Write (RMW) loop:
/// the driver reads the current item, applies your [`PatchSpec`](crate::PatchSpec)
/// locally, and issues an ETag-guarded Replace. If the Replace returns
/// 412 PreconditionFailed (another writer raced), the loop restarts.
///
/// The optional [`max_attempts`](Self::max_attempts) field bounds how many
/// times that loop may retry; `None` falls back to the driver default (5).
///
/// # Conditions are not exposed
///
/// PATCH intentionally does **not** expose either flavor of "condition" that
/// peer SDKs surface on their PATCH options:
///
/// * **`Precondition` (`If-Match` / `If-None-Match`).** The handler owns the
///   `If-Match` precondition on the internal Replace and captures the ETag
///   off the matching Read; honoring a caller-set value would either shadow
///   that ETag (silently breaking the RMW guarantee) or require resolving
///   it against the handler's own ETag (no sensible merge). The driver-side
///   PATCH handler rejects any caller-set precondition with an error before
///   issuing any sub-operation.
/// * **SQL filter predicate** (peer SDKs' `FilterPredicate`). Predicate
///   evaluation requires either native wire-level PATCH (so the server
///   evaluates the predicate inside the same transaction) or a client-side
///   SQL subset evaluator; neither is in scope for this preview. The
///   driver's [`PatchSpec`](crate::PatchSpec) has no `condition` field, so
///   there is no way to attach a predicate to a PATCH request.
///
/// The session token lives on the dedicated
/// [`session_token`](Self::session_token) field (mirroring
/// [`ItemReadOptions`] / [`ItemWriteOptions`]). All other general-purpose
/// settings (custom headers, content response behavior, excluded regions,
/// etc.) are configured via [`operation`](Self::operation) — see
/// [`OperationOptions`] for details.
///
/// # Latency
///
/// Because every PATCH is at minimum a Read followed by a Replace, the
/// best-case round-trip floor for ``patch_item`` is **2× the single-RTT
/// cost** of a comparable Read or Replace against the same partition.
/// Each retry triggered by a 412 PreconditionFailed adds another full
/// Read+Replace pair to the wall-clock cost.
///
/// When configuring an end-to-end latency budget via
/// [`OperationOptions`]'s end-to-end request settings, size the budget
/// accordingly — a useful rule of thumb is **≥ 2× the p99 single-RTT
/// budget you would set for a plain Replace**, plus headroom for any
/// 412 retries you want to tolerate. Setting the budget too low can
/// cancel the RMW between the Read and the Replace, producing a
/// timeout error even when the service is healthy.
#[derive(Clone, Default)]
#[non_exhaustive]
pub struct PatchItemOptions {
    /// General-purpose options that apply to this request.
    /// See [`OperationOptions`] for available settings and layered resolution behavior.
    pub operation: OperationOptions,

    /// Session token for session-consistent writes.
    pub session_token: Option<SessionToken>,

    /// Maximum number of Read-Modify-Write attempts the driver may make
    /// before surfacing a 412. `None` selects the driver default (5).
    pub max_attempts: Option<std::num::NonZeroU8>,
}

impl PatchItemOptions {
    /// Sets the session token for this request.
    pub fn with_session_token(mut self, session_token: impl Into<SessionToken>) -> Self {
        self.session_token = Some(session_token.into());
        self
    }

    /// Caps the number of Read-Modify-Write attempts the driver may make.
    pub fn with_max_attempts(mut self, max_attempts: std::num::NonZeroU8) -> Self {
        self.max_attempts = Some(max_attempts);
        self
    }

    /// Sets the [`OperationOptions`] for this request.
    pub fn with_operation_options(mut self, operation: OperationOptions) -> Self {
        self.operation = operation;
        self
    }
}

/// Options for transactional batch operations.
///
/// Used by [`ContainerClient::execute_transactional_batch()`](crate::clients::ContainerClient::execute_transactional_batch()).
/// ETag-based conditional options are specified per-operation within the batch itself.
///
/// General-purpose settings such as custom headers and content response behavior
/// are configured via the [`operation`](Self::operation) field.
/// See [`OperationOptions`] for details.
#[derive(Clone, Default)]
#[non_exhaustive]
pub struct BatchOptions {
    /// General-purpose options that apply to this request.
    /// See [`OperationOptions`] for available settings and layered resolution behavior.
    pub operation: OperationOptions,

    /// Session token for session-consistent batch operations.
    pub session_token: Option<SessionToken>,
}

impl BatchOptions {
    /// Sets the session token for this request.
    pub fn with_session_token(mut self, session_token: impl Into<SessionToken>) -> Self {
        self.session_token = Some(session_token.into());
        self
    }

    /// Sets the [`OperationOptions`] for this request.
    pub fn with_operation_options(mut self, operation: OperationOptions) -> Self {
        self.operation = operation;
        self
    }
}

/// Options to be passed to [`DatabaseClient::query_containers()`](crate::clients::DatabaseClient::query_containers()).
#[derive(Clone, Default)]
#[non_exhaustive]
pub struct QueryContainersOptions;

/// Options to be passed to [`CosmosClient::query_databases()`](crate::CosmosClient::query_databases()).
#[derive(Clone, Default)]
#[non_exhaustive]
pub struct QueryDatabasesOptions;

/// Options for query operations.
///
/// Used by [`ContainerClient::query_items()`](crate::clients::ContainerClient::query_items()).
///
/// General-purpose settings such as custom headers and excluded regions are configured
/// via the [`operation`](Self::operation) field. See [`OperationOptions`] for details.
#[derive(Clone, Default)]
#[non_exhaustive]
pub struct QueryOptions {
    /// General-purpose options that apply to this request.
    /// See [`OperationOptions`] for available settings and layered resolution behavior.
    pub operation: OperationOptions,

    /// Session token for session-consistent queries.
    pub session_token: Option<SessionToken>,

    /// When `true`, request that the service include index utilization metrics
    /// in the response (`x-ms-cosmos-populateindexmetrics`). The decoded JSON is
    /// surfaced via `QueryFeedPage::index_metrics()`.
    pub populate_index_metrics: Option<bool>,

    /// When `true`, request that the service include per-query metrics in the
    /// response (`x-ms-documentdb-populatequerymetrics`). Surfaced via
    /// `QueryFeedPage::query_metrics()`.
    pub populate_query_metrics: Option<bool>,

    /// Maximum number of items the service should return per page
    /// (`x-ms-max-item-count`).
    ///
    /// `None` omits the header so the SDK / service defaults apply. See
    /// [`MaxItemCountHint`] for the two explicit values.
    pub max_item_count: Option<MaxItemCountHint>,
}

impl QueryOptions {
    /// Sets the session token for this request.
    pub fn with_session_token(mut self, session_token: impl Into<SessionToken>) -> Self {
        self.session_token = Some(session_token.into());
        self
    }

    /// Sets the [`OperationOptions`] for this request.
    pub fn with_operation_options(mut self, operation: OperationOptions) -> Self {
        self.operation = operation;
        self
    }

    /// Enables or disables index-utilization metric collection for this query.
    pub fn with_populate_index_metrics(mut self, enable: bool) -> Self {
        self.populate_index_metrics = Some(enable);
        self
    }

    /// Enables or disables per-query metric collection for this query.
    pub fn with_populate_query_metrics(mut self, enable: bool) -> Self {
        self.populate_query_metrics = Some(enable);
        self
    }

    /// Sets the maximum number of items the service should return per page.
    ///
    /// Pass [`MaxItemCountHint::Limit`] with a concrete page size, or
    /// [`MaxItemCountHint::ServerDecides`] to let the service choose.
    pub fn with_max_item_count(mut self, max_item_count: MaxItemCountHint) -> Self {
        self.max_item_count = Some(max_item_count);
        self
    }
}

/// Options to be passed to [`ContainerClient::read()`](crate::clients::ContainerClient::read()).
#[derive(Clone, Default)]
#[non_exhaustive]
pub struct ReadContainerOptions;

/// Options to be passed to [`DatabaseClient::read()`](crate::clients::DatabaseClient::read()).
#[derive(Clone, Default)]
#[non_exhaustive]
pub struct ReadDatabaseOptions;

/// Options to be passed to operations related to Throughput offers.
#[derive(Clone, Default)]
#[non_exhaustive]
pub struct ThroughputOptions;

/// Options for [`ContainerClient::read_feed_ranges()`](crate::clients::ContainerClient::read_feed_ranges)
/// and [`ContainerClient::feed_range_from_partition_key()`](crate::clients::ContainerClient::feed_range_from_partition_key).
#[derive(Clone, Default, Debug)]
#[non_exhaustive]
pub struct ReadFeedRangesOptions {
    force_refresh: bool,
}

impl ReadFeedRangesOptions {
    /// When `true`, discards any cached routing map and fetches a fresh copy from the service.
    pub fn with_force_refresh(mut self, force_refresh: bool) -> Self {
        self.force_refresh = force_refresh;
        self
    }

    pub(crate) fn force_refresh(&self) -> bool {
        self.force_refresh
    }
}

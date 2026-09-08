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
use std::{borrow::Cow, time::Instant};
use time::OffsetDateTime;

/// Which change feed mode a factory should configure.
///
/// Private helper shared by [`CosmosOperation::change_feed`] and
/// [`CosmosOperation::change_feed_all_versions_and_deletes`]; the only
/// difference between the two is the `A-IM` header value they emit.
#[derive(Clone, Copy)]
enum ChangeFeedFactoryMode {
    /// LatestVersion: `A-IM: Incremental Feed`.
    Incremental,
    /// AllVersionsAndDeletes: `A-IM: Full-Fidelity Feed`.
    FullFidelity,
}

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
    /// state under at-least-once delivery). For AllVersionsAndDeletes it is a
    /// documented limitation: a range that is never polled before a checkpoint
    /// can drop the intermediate versions and deletes that occurred between the
    /// original start and the resume. "Now" is deliberately not pinned to a
    /// concrete start position before persisting, because that would change its
    /// semantics; lossless per-range "Now" resolution is a future improvement.
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
/// let container = driver.resolve_container("mydb", "mycontainer", OperationOptions::default()).await?;
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
    /// Stable identity used to detect a previously committed unsafe PATCH.
    patch_tracking_id: Option<crate::models::PatchTrackingId>,
    /// Maximum number of protected PATCH markers retained on the item.
    patch_tracking_capacity: Option<std::num::NonZeroU16>,
    /// Minimum number of whole seconds PATCH markers remain protected.
    patch_tracking_retention_seconds: Option<std::num::NonZeroU32>,
    /// `true` when this operation is a change feed read. Set explicitly by
    /// [`change_feed`](Self::change_feed) rather than inferred from a header,
    /// so future change feed modes can be added without ambiguity.
    is_change_feed: bool,
    /// The original change feed start position, persisted into the continuation
    /// token so never-polled partitions can re-apply it on resume. `None` for
    /// non-change-feed operations.
    change_feed_start: Option<ChangeFeedStartFrom>,
    /// `true` when this operation is one of the internal sub-operations the
    /// PATCH handler's Read-Modify-Write loop dispatches, rather than an
    /// operation the caller requested directly. Set by
    /// [`as_patch_sub_operation`](Self::as_patch_sub_operation); it only
    /// affects [`db_operation_name`](Self::db_operation_name), so the sub-op
    /// is dispatched exactly like the standalone Read/Replace it is.
    is_patch_sub_operation: bool,
    /// Whether a server-side PATCH may be resent after an ambiguous failure.
    /// `None` means strategy resolution has not classified the operation and
    /// therefore fails closed.
    patch_retry_safe: Option<bool>,
    /// `true` when the caller asked for a **text** payload over a binary wire
    /// (`BinaryEncodingOptions::request_text_response`) and this operation
    /// negotiated binary anyway. Recorded at negotiation time so pipeline nodes
    /// that synthesize a page can tell the *wire* format apart from the
    /// *emitted* format — see
    /// [`emits_binary_payload`](Self::emits_binary_payload).
    transcodes_response_to_text: bool,
    /// Internal routing constraint for reads whose correctness depends on
    /// observing the write region rather than the nearest read replica.
    internal_read_routing: InternalReadRouting,
    /// Absolute deadline inherited by internal sub-operations that belong to
    /// one logical operation, such as PATCH Read-Modify-Write.
    absolute_deadline: Option<Instant>,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
enum InternalReadRouting {
    #[default]
    Default,
    PreferredWriteEndpointsNoHedging,
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

    /// Returns the canonical OpenTelemetry `db.operation.name` for this
    /// operation, when it maps to a well-known Cosmos DB operation.
    ///
    /// The returned value uses the semantic-convention names the SDK surfaces
    /// (`read_item`, `create_item`, `query_items`, `query_change_feed`,
    /// `execute_batch`, `read_container`, …). It feeds
    /// [`DiagnosticsContext::operation_name`](crate::diagnostics::DiagnosticsContext::operation_name)
    /// so the emission layer can label spans/logs and
    /// [`is_threshold_violated`](crate::diagnostics::DiagnosticsContext::is_threshold_violated)
    /// can distinguish point from non-point operations for tail-based sampling.
    ///
    /// Operations without a canonical name (query plans, partition-key-range
    /// reads, HEAD probes, stored procedures, triggers, UDFs, distributed
    /// transactions) return `None`, which leaves the diagnostics
    /// `operation_name` unset — identical to the pre-population behavior.
    /// Throughput (offer) operations are also unmapped: the canonical names are
    /// scope-specific (`read_container_throughput` vs. `read_database_throughput`)
    /// but an offer operation carries only the account and the offer ID, so the
    /// scope is not recoverable here. The SDK, which knows whether the caller
    /// addressed a container or a database, supplies those names instead.
    ///
    /// # PATCH sub-operations
    ///
    /// PATCH is a single caller-facing operation that this driver implements as
    /// a Read followed by an ETag-guarded Replace. The two sub-operations report
    /// `patch_read_item` and `patch_replace_item` rather than the bare
    /// `read_item` / `replace_item`, so telemetry encodes *both* facts: that the
    /// work belongs to a PATCH, and which half of the read-modify-write it is.
    /// Naming them `read_item`/`replace_item` would make them indistinguishable
    /// from standalone point operations the caller never issued; naming them
    /// `patch_item` would hide the decomposition entirely. The operation the
    /// caller actually invoked keeps reporting `patch_item` on the root span and
    /// the operation metric.
    pub fn db_operation_name(&self) -> Option<&'static str> {
        let name = match (self.operation_type, self.resource_type) {
            // Data-plane item operations.
            (OperationType::Create, ResourceType::Document) => "create_item",
            (OperationType::Read, ResourceType::Document) if self.is_patch_sub_operation => {
                "patch_read_item"
            }
            (OperationType::Read, ResourceType::Document) => "read_item",
            (OperationType::Replace, ResourceType::Document) if self.is_patch_sub_operation => {
                "patch_replace_item"
            }
            (OperationType::Replace, ResourceType::Document) => "replace_item",
            (OperationType::Delete, ResourceType::Document) => "delete_item",
            (OperationType::Upsert, ResourceType::Document) => "upsert_item",
            (OperationType::Patch, ResourceType::Document) => "patch_item",
            (OperationType::Batch, ResourceType::Document) => "execute_batch",
            (OperationType::Query, ResourceType::Document)
            | (OperationType::SqlQuery, ResourceType::Document) => "query_items",
            // NOTE: `read_all_items` (and, below, `read_all_containers` /
            // `read_all_databases` / the granular `query_containers` /
            // `query_databases`) are this SDK's canonical values. They
            // intentionally diverge from the .NET SDK, which emits
            // `read_feed_ranges` for feed reads and funnels container/database
            // queries through the generic `query_items`. Keep them aligned with
            // this crate's own `read_all_*` / `query_*` public API, not with
            // .NET. See DIAGNOSTICS-CONTRACT.md.
            (OperationType::ReadFeed, ResourceType::Document) => {
                if self.is_change_feed {
                    "query_change_feed"
                } else if self.targets_logical_partition() {
                    // `read_all_items(container, partition_key)` narrows the
                    // feed to one logical partition, which semconv names
                    // distinctly from the cross-partition read.
                    "read_all_items_of_logical_partition"
                } else {
                    "read_all_items"
                }
            }
            // Container (collection) management.
            (OperationType::Create, ResourceType::DocumentCollection) => "create_container",
            (OperationType::Read, ResourceType::DocumentCollection) => "read_container",
            (OperationType::Replace, ResourceType::DocumentCollection) => "replace_container",
            (OperationType::Delete, ResourceType::DocumentCollection) => "delete_container",
            (OperationType::Query, ResourceType::DocumentCollection)
            | (OperationType::SqlQuery, ResourceType::DocumentCollection) => "query_containers",
            (OperationType::ReadFeed, ResourceType::DocumentCollection) => "read_all_containers",
            // Database management.
            (OperationType::Create, ResourceType::Database) => "create_database",
            (OperationType::Read, ResourceType::Database) => "read_database",
            (OperationType::Delete, ResourceType::Database) => "delete_database",
            (OperationType::Query, ResourceType::Database)
            | (OperationType::SqlQuery, ResourceType::Database) => "query_databases",
            (OperationType::ReadFeed, ResourceType::Database) => "read_all_databases",
            // Throughput (offer) management has no driver-layer mapping: the
            // canonical names are scope-specific (`read_container_throughput` /
            // `read_database_throughput` and their `replace_` variants), but an
            // offer operation is addressed by account + offer ID only, so this
            // layer cannot tell a container offer from a database offer. The
            // SDK stamps the scoped name via `CosmosOperationContext`.
            // Everything else has no canonical semconv name.
            _ => return None,
        };
        Some(name)
    }

    /// Returns `true` when this operation targets exactly one logical partition
    /// (or a hierarchical-partition-key prefix), as opposed to an EPK range or
    /// the whole container.
    fn targets_logical_partition(&self) -> bool {
        self.target
            .as_ref()
            .is_some_and(FeedRange::is_logical_partition)
    }

    /// Returns a reference to the resource being operated on.
    pub(crate) fn resource_reference(&self) -> &CosmosResourceReference {
        &self.resource_reference
    }

    /// Returns whether this operation uses feed-style paths.
    ///
    /// Create and Upsert document operations POST to the parent (collection)
    /// URL even though they carry an item id, because that is how the Cosmos DB
    /// REST API models them. Their leaf id therefore never appears in the
    /// request path.
    pub(crate) fn uses_feed_paths(&self) -> bool {
        matches!(
            self.operation_type,
            OperationType::Create | OperationType::Upsert
        ) && self.resource_type == ResourceType::Document
    }

    /// Computes the request path and signing link for this operation.
    ///
    /// Create and Upsert document operations use feed-style paths (targeting
    /// the collection URL) even though they carry an item id, because the
    /// Cosmos DB REST API POSTs these to the collection feed. All other
    /// operations use the standard resource paths.
    pub(crate) fn compute_resource_paths(&self) -> crate::models::ResourcePaths {
        if self.uses_feed_paths() {
            self.resource_reference.compute_feed_paths()
        } else {
            self.resource_reference.compute_paths()
        }
    }

    /// Validates that this operation does not mix name and RID addressing.
    ///
    /// Delegates to
    /// [`CosmosResourceReference::validate_addressing`], telling it whether the
    /// leaf id will appear in the request path so that feed-style operations
    /// (Create/Upsert) are correctly exempted from the leaf check.
    pub(crate) fn validate_addressing(&self) -> crate::error::Result<()> {
        self.resource_reference
            .validate_addressing(!self.uses_feed_paths())
    }

    /// Returns the container for this operation, if applicable.
    ///
    /// Returns `None` for account-level and database-level operations.
    pub fn container(&self) -> Option<&ContainerReference> {
        self.resource_reference.container()
    }

    /// Retargets this operation to refreshed metadata for the same named
    /// container and recomputes logical partition routing.
    pub(crate) fn retarget_container(
        &mut self,
        replacement: ContainerReference,
    ) -> crate::error::Result<()> {
        if self
            .target
            .as_ref()
            .is_some_and(|target| target.partition_key().is_none() && target != &FeedRange::full())
        {
            return Err(crate::error::CosmosError::builder()
                .with_status(crate::error::CosmosStatus::CLIENT_BAD_REQUEST)
                .with_message(
                    "an explicit effective partition key range cannot be carried to a recreated \
                     container; obtain a new feed range from the replacement container",
                )
                .build());
        }
        let replacement_target = self.partition_key().cloned().map(|partition_key| {
            FeedRange::for_partition(partition_key, replacement.partition_key_definition())
        });
        self.resource_reference.retarget_container(replacement)?;
        if replacement_target.is_some() {
            self.target = replacement_target;
        }
        Ok(())
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

    /// Advertises which serialization formats the client accepts in the response
    /// (the `x-ms-cosmos-supported-serialization-formats` request header), e.g.
    /// `JsonText,CosmosBinary`.
    ///
    /// When set, the service may reply with Cosmos binary JSON, which the SDK
    /// auto-detects and decodes; when unset, the response stays text JSON.
    pub fn with_supported_serialization_formats(
        mut self,
        formats: impl Into<Cow<'static, str>>,
    ) -> Self {
        self.request_headers.supported_serialization_formats = Some(formats.into());
        self
    }

    /// Whether this operation advertised Cosmos binary JSON in its
    /// `x-ms-cosmos-supported-serialization-formats` header.
    ///
    /// Describes the **wire**, not what the caller receives: under
    /// `BinaryEncodingOptions::request_text_response` the wire stays binary
    /// while the driver transcodes the response to text on the way out.
    pub(crate) fn negotiates_binary_response(&self) -> bool {
        self.request_headers
            .supported_serialization_formats
            .as_deref()
            .is_some_and(|formats| {
                formats
                    .split(',')
                    .any(|format| format.trim().eq_ignore_ascii_case("CosmosBinary"))
            })
    }

    /// Records that the driver will transcode this operation's response back to
    /// text before returning it to the caller.
    pub(crate) fn transcoding_response_to_text(mut self) -> Self {
        self.transcodes_response_to_text = true;
        self
    }

    /// Whether the driver must transcode this operation's response body to text
    /// before handing it back.
    ///
    /// Decided once at negotiation time, so a caller who varies
    /// `request_text_response` between pages cannot desynchronize the emitted
    /// encoding from what the pipeline nodes were built to produce.
    pub(crate) fn transcodes_response_to_text(&self) -> bool {
        self.transcodes_response_to_text
    }

    /// Whether pipeline nodes that synthesize a page should emit **binary**
    /// items.
    ///
    /// Distinct from [`negotiates_binary_response`](Self::negotiates_binary_response),
    /// which describes the wire. The two diverge under
    /// `BinaryEncodingOptions::request_text_response`: the wire stays binary
    /// while the driver transcodes on the way out, so a node emitting binary
    /// would re-encode every item only for `execute_plan` to decode it again.
    pub(crate) fn emits_binary_payload(&self) -> bool {
        self.negotiates_binary_response() && !self.transcodes_response_to_text
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

    /// Sets a stable tracking ID for a client-side PATCH.
    ///
    /// Reuse the same ID for application-level retries of the same logical
    /// operation. Prefer a random, unpredictable ID. Supplying an ID opts even
    /// a retry-safe instruction list into marker-based duplicate suppression
    /// when client-side execution is selected. It does not influence strategy
    /// resolution and is ignored by server-side PATCH. If omitted, the driver
    /// generates an ID only for unsafe lists executed client-side.
    pub fn with_patch_tracking_id(mut self, tracking_id: crate::models::PatchTrackingId) -> Self {
        self.patch_tracking_id = Some(tracking_id);
        self
    }

    /// Returns the caller-supplied PATCH tracking ID, if any.
    pub fn patch_tracking_id(&self) -> Option<crate::models::PatchTrackingId> {
        self.patch_tracking_id
    }

    /// Sets the maximum number of PATCH tracking entries retained on one item.
    ///
    /// When the cap is reached after age-based pruning, PATCH evicts the first
    /// entry before appending the new marker.
    pub fn with_patch_tracking_capacity(mut self, capacity: std::num::NonZeroU16) -> Self {
        self.patch_tracking_capacity = Some(capacity);
        self
    }

    /// Returns the configured PATCH tracking capacity, if any.
    pub fn patch_tracking_capacity(&self) -> Option<std::num::NonZeroU16> {
        self.patch_tracking_capacity
    }

    /// Sets the age-based retention window for PATCH tracking entries.
    pub fn with_patch_tracking_retention_seconds(
        mut self,
        retention_seconds: std::num::NonZeroU32,
    ) -> Self {
        self.patch_tracking_retention_seconds = Some(retention_seconds);
        self
    }

    /// Returns the configured PATCH tracking retention in whole seconds, if any.
    pub fn patch_tracking_retention_seconds(&self) -> Option<std::num::NonZeroU32> {
        self.patch_tracking_retention_seconds
    }

    /// Marks this operation as an internal sub-operation of a PATCH's
    /// Read-Modify-Write loop.
    ///
    /// The only effect is on [`db_operation_name`](Self::db_operation_name),
    /// which then reports `patch_read_item` / `patch_replace_item` instead of
    /// `read_item` / `replace_item`. Routing, retries, and the wire request are
    /// unchanged — a PATCH sub-op *is* an ordinary point Read or Replace.
    pub(crate) fn as_patch_sub_operation(mut self) -> Self {
        self.is_patch_sub_operation = true;
        self
    }

    /// Marks this operation as the Read half of PATCH's Read-Modify-Write loop.
    ///
    /// The read prefers write endpoints and cannot be hedged because a response
    /// from another region may not yet contain the write being verified.
    pub(crate) fn as_patch_read_sub_operation(mut self) -> Self {
        self.is_patch_sub_operation = true;
        self.internal_read_routing = InternalReadRouting::PreferredWriteEndpointsNoHedging;
        self
    }

    /// Records whether a server-side PATCH may be resent after an ambiguous
    /// failure, based on the resolved instruction list.
    pub(crate) fn with_patch_retry_safe(mut self, retry_safe: bool) -> Self {
        self.patch_retry_safe = Some(retry_safe);
        self
    }

    /// Returns whether PATCH strategy resolution classified this operation.
    pub(crate) fn patch_strategy_is_resolved(&self) -> bool {
        self.patch_retry_safe.is_some()
    }

    /// Returns whether this internal read should start at preferred write endpoints.
    pub(crate) fn prefers_write_endpoints_for_read(&self) -> bool {
        matches!(
            self.internal_read_routing,
            InternalReadRouting::PreferredWriteEndpointsNoHedging
        )
    }

    /// Returns whether correctness requires hedging to remain disabled.
    pub(crate) fn suppresses_hedging(&self) -> bool {
        matches!(
            self.internal_read_routing,
            InternalReadRouting::PreferredWriteEndpointsNoHedging
        )
    }

    /// Applies an absolute deadline shared by a logical operation's internal
    /// sub-operations.
    pub(crate) fn with_absolute_deadline(mut self, deadline: Option<Instant>) -> Self {
        self.absolute_deadline = deadline;
        self
    }

    /// Returns the absolute deadline inherited from the logical operation.
    pub(crate) fn absolute_deadline(&self) -> Option<Instant> {
        self.absolute_deadline
    }

    /// Returns `true` when this operation is an internal sub-operation of a
    /// PATCH's Read-Modify-Write loop.
    pub fn is_patch_sub_operation(&self) -> bool {
        self.is_patch_sub_operation
    }

    // ===== Factory Methods =====

    /// Creates a new operation with the specified type, resource reference, and target.
    pub(crate) fn new(
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
            patch_tracking_id: None,
            patch_tracking_capacity: None,
            patch_tracking_retention_seconds: None,
            is_change_feed: false,
            change_feed_start: None,
            is_patch_sub_operation: false,
            patch_retry_safe: None,
            transcodes_response_to_text: false,
            internal_read_routing: InternalReadRouting::Default,
            absolute_deadline: None,
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
    /// let container = driver.resolve_container("my-database", "my-container", OperationOptions::default()).await?;
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

    /// Reads a container's properties by database and container RID.
    ///
    /// Like [`read_container_by_name`](Self::read_container_by_name) but addresses
    /// the container by RID. Taking the raw `db_rid` and `container_rid` (rather
    /// than a pre-built [`DatabaseReference`]) makes a mixed name/RID path
    /// unrepresentable: the parent database reference is always constructed
    /// RID-based here, so the request path is guaranteed to be fully RID-based
    /// (`/dbs/{db_rid}/colls/{container_rid}`).
    pub fn read_container_by_rid(
        account: AccountReference,
        db_rid: impl Into<std::borrow::Cow<'static, str>>,
        container_rid: impl Into<std::borrow::Cow<'static, str>>,
    ) -> Self {
        let database = DatabaseReference::from_rid(account, db_rid.into());
        let resource_ref: CosmosResourceReference = CosmosResourceReference::from(database)
            .with_resource_type(ResourceType::DocumentCollection)
            .with_rid(container_rid.into());
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
    /// let container = driver.resolve_container("my-database", "my-container", OperationOptions::default()).await?;
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
    /// let container = driver.resolve_container("my-database", "my-container", OperationOptions::default()).await?;
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

    /// Builds a PATCH operation for an item.
    ///
    /// [`crate::options::PatchStrategy`] selects one server-side request or the
    /// client-side Read-Modify-Write loop. Callers serialize
    /// [`crate::models::PatchInstructions`] into the operation body.
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
        Self::change_feed_with_mode(container, target, ChangeFeedFactoryMode::Incremental)
    }

    /// Creates a full-fidelity (AllVersionsAndDeletes) change feed read
    /// operation for a container.
    ///
    /// Identical to [`change_feed`](Self::change_feed) except it sets the
    /// `A-IM` header to `Full-Fidelity Feed` instead of `Incremental Feed`.
    /// This selects the AllVersionsAndDeletes mode, in which every intermediate
    /// version and delete is returned inside an envelope carrying `current`
    /// (post-image), `previous` (pre-image, when enabled), and `metadata`
    /// (operation type, LSN, timestamps). The SDK does **not** unwrap
    /// `current`; the caller deserializes each item into a `ChangeFeedItem<T>`.
    ///
    /// Like [`change_feed`](Self::change_feed) this also sets the
    /// `x-ms-cosmos-changefeed-wire-format-version` header and marks the
    /// operation as a change feed read. The start position is set via
    /// [`with_change_feed_start`](Self::with_change_feed_start).
    ///
    /// `target` scopes the change feed to a specific partition or EPK range.
    /// Pass `None` or `Some(FeedRange::full())` to read the entire container.
    pub fn change_feed_all_versions_and_deletes(
        container: ContainerReference,
        target: Option<FeedRange>,
    ) -> Self {
        Self::change_feed_with_mode(container, target, ChangeFeedFactoryMode::FullFidelity)
    }

    /// Shared constructor for the change feed factories. The only difference
    /// between LatestVersion and AllVersionsAndDeletes is which `A-IM` value is
    /// emitted; everything else (resource shape, wire-format-version header,
    /// change-feed marking) is identical.
    fn change_feed_with_mode(
        container: ContainerReference,
        target: Option<FeedRange>,
        mode: ChangeFeedFactoryMode,
    ) -> Self {
        let resource_ref: CosmosResourceReference = CosmosResourceReference::from(container)
            .with_resource_type(ResourceType::Document)
            .into_feed_reference();
        let mut headers = CosmosRequestHeaders::new();
        match mode {
            ChangeFeedFactoryMode::Incremental => headers.incremental_feed = true,
            ChangeFeedFactoryMode::FullFidelity => headers.full_fidelity_feed = true,
        }
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

    /// Returns true if this operation may be retried when the backend outcome
    /// is ambiguous — that is, when the request may already have been received
    /// and processed.
    ///
    /// Stored procedure execution returns `false` because its body is opaque.
    /// Server-side PATCH returns the strategy resolver's classification;
    /// unresolved PATCH fails closed. Other data-plane operations retain the
    /// driver's existing retry behavior.
    ///
    /// This is deliberately *not* `is_idempotent`: the driver retries
    /// non-idempotent writes such as `Create` and `Upsert` on purpose.
    ///
    /// Gates both retry layers so they cannot disagree about the same failure:
    /// cross-region failover in the operation pipeline, and the same-endpoint
    /// shard retry in the transport pipeline.
    pub fn allows_ambiguous_outcome_retry(&self) -> bool {
        match self.operation_type {
            OperationType::Execute => false,
            OperationType::Patch => self.patch_retry_safe.unwrap_or(false),
            _ => true,
        }
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

    /// A container addressed purely by RID (no name-based path available).
    fn test_container_by_rid() -> ContainerReference {
        ContainerReference::new_by_rid(
            test_account(),
            "Lx1BAA==",
            "testcontainer",
            "Lx1BALxJyZ8=",
            &test_container_props(),
        )
    }

    fn replacement_container() -> ContainerReference {
        let mut properties = test_container_props();
        properties.partition_key = test_partition_key_definition("/replacement-pk");
        ContainerReference::new(
            test_account(),
            "testdb",
            "testdb_rid",
            "testcontainer",
            "replacement_container_rid",
            &properties,
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
    fn retarget_rejects_explicit_epk_range() {
        let range = FeedRange::new("10".into(), "20".into()).unwrap();
        let mut operation = CosmosOperation::query_items(test_container(), Some(range.clone()));

        let error = operation
            .retarget_container(replacement_container())
            .unwrap_err();

        assert_eq!(
            error.status(),
            crate::error::CosmosStatus::CLIENT_BAD_REQUEST
        );
        assert_eq!(operation.container().unwrap().rid(), "testcontainer_rid");
        assert_eq!(operation.target(), Some(&range));
    }

    #[test]
    fn create_item_on_rid_container_allows_name_id() {
        // Create POSTs to the parent collection URL, so the item name never
        // reaches the wire. Confirmed live: this succeeds on a RID-addressed
        // container.
        let item_ref =
            ItemReference::from_name(&test_container_by_rid(), PartitionKey::from("pk1"), "doc1");
        let op = CosmosOperation::create_item(item_ref);
        assert!(op.uses_feed_paths());
        op.validate_addressing()
            .expect("create on a RID container may carry a name id");
        assert_eq!(
            op.compute_resource_paths().request_path(),
            "/dbs/Lx1BAA==/colls/Lx1BALxJyZ8=/docs"
        );
    }

    #[test]
    fn read_item_on_rid_container_rejects_name_id() {
        // Read puts the leaf in the path, where the service tries to parse it as
        // a ResourceId. Confirmed live: this returns
        // `400 Failed to parse the value 'doc1' as ResourceId`, so fail fast.
        let item_ref =
            ItemReference::from_name(&test_container_by_rid(), PartitionKey::from("pk1"), "doc1");
        let op = CosmosOperation::read_item(item_ref);
        assert!(!op.uses_feed_paths());
        let err = op
            .validate_addressing()
            .expect_err("a name leaf under a RID parent must be rejected");
        assert_eq!(
            err.status(),
            crate::error::CosmosStatus::CLIENT_MIXED_NAME_RID_ADDRESSING
        );
    }

    #[test]
    fn read_item_on_rid_container_accepts_item_rid() {
        // The supported way to point-read on a RID-addressed container: address
        // the item by RID too, so the whole path is RID-based.
        let item_ref = ItemReference::from_rid(
            &test_container_by_rid(),
            PartitionKey::from("pk1"),
            "Lx1BALxJyZ8BAAAAAAAAAA==",
        );
        let op = CosmosOperation::read_item(item_ref);
        op.validate_addressing()
            .expect("a RID leaf under a RID parent is consistent");

        let paths = op.compute_resource_paths();
        assert_eq!(
            paths.request_path(),
            "/dbs/Lx1BAA==/colls/Lx1BALxJyZ8=/docs/Lx1BALxJyZ8BAAAAAAAAAA=="
        );
        assert_eq!(paths.signing_link(), "lx1balxjyz8baaaaaaaaaa==");
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

    /// Both retry layers consult this one predicate, so a failure the transport
    /// pipeline declines to retry on another shard cannot then be retried
    /// cross-region by the operation pipeline — strictly more expensive for
    /// identical duplicate-execution semantics.
    #[test]
    fn ambiguous_outcome_retry_covers_non_idempotent_writes() {
        let pk = PartitionKey::from("pk1");
        let item = |op: fn(ItemReference) -> CosmosOperation| {
            op(ItemReference::from_name(
                &test_container(),
                pk.clone(),
                "doc1",
            ))
        };

        for op in [
            item(CosmosOperation::create_item),
            item(CosmosOperation::upsert_item),
            item(CosmosOperation::replace_item),
            item(CosmosOperation::delete_item),
            item(CosmosOperation::read_item),
            CosmosOperation::batch(test_container(), pk.clone()),
        ] {
            assert!(
                op.allows_ambiguous_outcome_retry(),
                "{:?} must stay eligible for retry after an ambiguous failure",
                op.operation_type()
            );
        }
    }

    #[test]
    fn server_patch_ambiguous_retry_requires_resolved_safety() {
        let patch = || {
            CosmosOperation::patch_item(ItemReference::from_name(
                &test_container(),
                PartitionKey::from("pk1"),
                "doc1",
            ))
        };

        assert!(!patch().allows_ambiguous_outcome_retry());
        assert!(patch()
            .with_patch_retry_safe(true)
            .allows_ambiguous_outcome_retry());
        assert!(!patch()
            .with_patch_retry_safe(false)
            .allows_ambiguous_outcome_retry());
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
        assert!(!op.request_headers().full_fidelity_feed);
        assert!(op.request_headers().changefeed_wire_format_version);
    }

    /// The full-fidelity (AllVersionsAndDeletes) factory sets the
    /// full-fidelity indicator instead of the incremental one, while keeping
    /// the wire-format-version header and change-feed marking.
    #[test]
    fn change_feed_all_versions_and_deletes_sets_full_fidelity_header() {
        let op = CosmosOperation::change_feed_all_versions_and_deletes(
            test_container(),
            Some(FeedRange::full()),
        );

        assert!(op.is_change_feed());
        assert!(op.request_headers().full_fidelity_feed);
        assert!(!op.request_headers().incremental_feed);
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

    #[test]
    fn db_operation_name_maps_item_operations() {
        let item =
            || ItemReference::from_name(&test_container(), PartitionKey::from("pk1"), "doc1");

        assert_eq!(
            CosmosOperation::create_item(item()).db_operation_name(),
            Some("create_item")
        );
        assert_eq!(
            CosmosOperation::read_item(item()).db_operation_name(),
            Some("read_item")
        );
        assert_eq!(
            CosmosOperation::replace_item(item()).db_operation_name(),
            Some("replace_item")
        );
        assert_eq!(
            CosmosOperation::upsert_item(item()).db_operation_name(),
            Some("upsert_item")
        );
        assert_eq!(
            CosmosOperation::delete_item(item()).db_operation_name(),
            Some("delete_item")
        );
        assert_eq!(
            CosmosOperation::patch_item(item()).db_operation_name(),
            Some("patch_item")
        );
    }

    #[test]
    fn db_operation_name_distinguishes_patch_sub_operations() {
        let item =
            || ItemReference::from_name(&test_container(), PartitionKey::from("pk1"), "doc1");

        // A PATCH is one caller-facing operation implemented as a Read plus an
        // ETag-guarded Replace. The sub-ops report names that encode both the
        // owning PATCH and which half of the read-modify-write they are, so
        // telemetry neither hides the decomposition nor makes the sub-ops look
        // like standalone point operations the caller never issued.
        assert_eq!(
            CosmosOperation::read_item(item())
                .as_patch_sub_operation()
                .db_operation_name(),
            Some("patch_read_item")
        );
        assert_eq!(
            CosmosOperation::replace_item(item())
                .as_patch_sub_operation()
                .db_operation_name(),
            Some("patch_replace_item")
        );

        // The operation the caller actually invoked is unaffected.
        assert_eq!(
            CosmosOperation::patch_item(item()).db_operation_name(),
            Some("patch_item")
        );
        assert!(!CosmosOperation::patch_item(item()).is_patch_sub_operation());
    }

    #[test]
    fn patch_sub_operation_marker_is_off_by_default() {
        let item =
            || ItemReference::from_name(&test_container(), PartitionKey::from("pk1"), "doc1");

        assert!(!CosmosOperation::read_item(item()).is_patch_sub_operation());
        assert!(!CosmosOperation::replace_item(item()).is_patch_sub_operation());
        assert!(!CosmosOperation::read_item(item()).prefers_write_endpoints_for_read());
        assert!(!CosmosOperation::read_item(item()).suppresses_hedging());
        assert!(CosmosOperation::read_item(item())
            .as_patch_sub_operation()
            .is_patch_sub_operation());
        assert!(!CosmosOperation::read_item(item())
            .as_patch_sub_operation()
            .prefers_write_endpoints_for_read());

        let patch_read = CosmosOperation::read_item(item()).as_patch_read_sub_operation();
        assert!(patch_read.is_patch_sub_operation());
        assert!(patch_read.prefers_write_endpoints_for_read());
        assert!(patch_read.suppresses_hedging());
    }

    #[test]
    fn patch_sub_operation_marker_only_renames_read_and_replace() {
        let item =
            || ItemReference::from_name(&test_container(), PartitionKey::from("pk1"), "doc1");

        // The marker is only ever set on the two sub-ops the PATCH handler
        // dispatches. Guard the mapping anyway so a stray marker on any other
        // operation cannot silently invent a name.
        assert_eq!(
            CosmosOperation::create_item(item())
                .as_patch_sub_operation()
                .db_operation_name(),
            Some("create_item")
        );
        assert_eq!(
            CosmosOperation::upsert_item(item())
                .as_patch_sub_operation()
                .db_operation_name(),
            Some("upsert_item")
        );
        assert_eq!(
            CosmosOperation::delete_item(item())
                .as_patch_sub_operation()
                .db_operation_name(),
            Some("delete_item")
        );
    }

    #[test]
    fn db_operation_name_maps_feed_and_query_operations() {
        assert_eq!(
            CosmosOperation::query_items(test_container(), Some(FeedRange::full()))
                .db_operation_name(),
            Some("query_items")
        );
        assert_eq!(
            CosmosOperation::change_feed(test_container(), Some(FeedRange::full()))
                .db_operation_name(),
            Some("query_change_feed")
        );
        assert_eq!(
            CosmosOperation::read_all_items_cross_partition(test_container()).db_operation_name(),
            Some("read_all_items")
        );
        assert_eq!(
            CosmosOperation::read_all_items(test_container(), PartitionKey::from("pk1"))
                .db_operation_name(),
            Some("read_all_items_of_logical_partition")
        );
        assert_eq!(
            CosmosOperation::batch(test_container(), PartitionKey::from("pk1")).db_operation_name(),
            Some("execute_batch")
        );
    }

    #[test]
    fn db_operation_name_change_feed_ignores_logical_partition_scope() {
        // A change feed scoped to one logical partition is still
        // `query_change_feed`; semconv has no partition-scoped variant for it.
        let container = test_container();
        let range = FeedRange::for_partition(
            PartitionKey::from("pk1"),
            container.partition_key_definition(),
        );
        assert_eq!(
            CosmosOperation::change_feed(container, Some(range)).db_operation_name(),
            Some("query_change_feed")
        );
    }

    #[test]
    fn db_operation_name_maps_metadata_operations() {
        let db = DatabaseReference::from_name(test_account(), "testdb");

        assert_eq!(
            CosmosOperation::read_container(test_container()).db_operation_name(),
            Some("read_container")
        );
        assert_eq!(
            CosmosOperation::create_container(db.clone()).db_operation_name(),
            Some("create_container")
        );
        assert_eq!(
            CosmosOperation::read_database(db.clone()).db_operation_name(),
            Some("read_database")
        );
        assert_eq!(
            CosmosOperation::query_databases(test_account()).db_operation_name(),
            Some("query_databases")
        );
    }

    #[test]
    fn db_operation_name_none_for_throughput_operations() {
        // Offer operations carry no database/container scope, and semconv only
        // defines scoped throughput names, so the driver leaves them unmapped
        // and the SDK supplies `read_container_throughput` /
        // `read_database_throughput` (and their `replace_` variants).
        assert_eq!(
            CosmosOperation::query_offers(test_account()).db_operation_name(),
            None
        );
        assert_eq!(
            CosmosOperation::read_offer(test_account(), "offer-rid").db_operation_name(),
            None
        );
        assert_eq!(
            CosmosOperation::replace_offer(test_account(), "offer-rid").db_operation_name(),
            None
        );
    }

    #[test]
    fn db_operation_name_none_for_unmapped_operations() {
        // Query plans have no canonical semconv operation name.
        let op = CosmosOperation::query_plan(test_container(), std::borrow::Cow::Borrowed(""));
        assert_eq!(op.db_operation_name(), None);
    }

    #[test]
    fn with_supported_serialization_formats_sets_header_field() {
        let item_ref =
            ItemReference::from_name(&test_container(), PartitionKey::from("pk1"), "doc1");
        let op = CosmosOperation::create_item(item_ref)
            .with_supported_serialization_formats("JsonText,CosmosBinary");
        assert_eq!(
            op.request_headers()
                .supported_serialization_formats
                .as_deref(),
            Some("JsonText,CosmosBinary"),
        );
    }
}

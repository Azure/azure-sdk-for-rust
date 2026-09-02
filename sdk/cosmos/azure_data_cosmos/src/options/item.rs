// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Options for item-level point reads, writes, and patch operations.

#[cfg(feature = "preview_patch")]
use crate::models::PatchTrackingId;
use azure_data_cosmos_driver::models::{Precondition, SessionToken};
use azure_data_cosmos_driver::options::OperationOptions;
#[cfg(feature = "preview_patch")]
use azure_data_cosmos_driver::options::PatchStrategy;

/// Options for item point-read operations.
///
/// Used by [`ContainerClient::read_item()`](crate::clients::ContainerClient::read_item).
///
/// General-purpose settings such as custom headers and excluded regions are configured
/// via the [`with_operation_options`](Self::with_operation_options) setter. See [`OperationOptions`] for details.
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
/// response behavior are configured via the [`with_operation_options`](Self::with_operation_options) setter.
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
/// **Preview.** Requires the `preview_patch` feature. Unsafe instruction lists
/// use persisted tracking entries to suppress duplicate application after an
/// ambiguous transport failure. See [Retry Semantics](crate::clients::ContainerClient::patch_item()).
///
/// PATCH can execute server-side as one request or through the tracked
/// client-side Read-Modify-Write (RMW) loop. [`PatchStrategy::Auto`] is the
/// default: it uses server-side PATCH for retry-safe lists containing at most
/// 10 instructions, and client-side RMW for unsafe or longer lists.
///
/// Explicit [`PatchStrategy::ServerSide`] never falls back. Cosmos DB rejects
/// more than 10 instructions with HTTP 400. [`PatchStrategy::ClientSide`] has
/// no corresponding instruction-count limit.
///
/// The optional [`max_attempts`](Self::max_attempts) field bounds only the
/// client-side loop; `None` falls back to the driver default (5).
///
/// # Conditions
///
/// PATCH exposes ETag preconditions but not SQL filter predicates:
///
/// * **`Precondition::IfMatch`.** Server-side PATCH sends the condition to
///   Cosmos DB. Client-side PATCH evaluates it against each write-region
///   `LatestCommitted` Read, then uses that Read's ETag as the internal Replace
///   concurrency guard. `IfNoneMatch` is a read condition and is rejected for
///   PATCH.
/// * **SQL filter predicate** (peer SDKs' `FilterPredicate`). Predicate
///   evaluation requires either native wire-level PATCH (so the server
///   evaluates the predicate inside the same transaction) or a client-side
///   SQL subset evaluator; neither is in scope for this preview. The
///   driver's [`PatchInstructions`](crate::models::PatchInstructions) has no `condition` field, so
///   there is no way to attach a predicate to a PATCH request.
///
/// The session token lives on the dedicated
/// [`session_token`](Self::session_token) field (mirroring
/// [`ItemReadOptions`] / [`ItemWriteOptions`]). All other general-purpose
/// settings (custom headers, content response behavior, excluded regions,
/// etc.) are configured via [`with_operation_options`](Self::with_operation_options) — see
/// [`OperationOptions`] for details. In particular, an explicit
/// `content_response_on_write = Disabled` suppresses the PATCH response body
/// for both server-side and client-side execution.
///
/// # Latency
///
/// Server-side PATCH has a one-request latency floor. Client-side PATCH is at
/// minimum a Read followed by a Replace, and each 412 retry adds another full
/// Read+Replace pair.
///
/// When configuring an end-to-end latency budget via
/// [`OperationOptions`]'s end-to-end request settings, the budget applies once
/// to the complete logical PATCH, including every Read, Replace, retry, and
/// terminal verification. Size the budget
/// accordingly when `ClientSide` is possible, including through `Auto`
/// fallback. A useful rule of thumb is **≥ 2× the p99 single-RTT budget for a
/// plain Replace**, plus headroom for 412 retries.
#[cfg(feature = "preview_patch")]
#[derive(Clone, Default)]
#[non_exhaustive]
pub struct PatchItemOptions {
    /// General-purpose options that apply to this request.
    /// See [`OperationOptions`] for available settings and layered resolution behavior.
    pub operation: OperationOptions,

    /// How this PATCH should execute.
    ///
    /// `None` inherits the layered default, which resolves to
    /// [`PatchStrategy::Auto`]. Client-side-only settings do not influence
    /// strategy resolution. When both this field and
    /// [`operation.patch_strategy`](OperationOptions::patch_strategy) are set,
    /// this field takes precedence.
    pub strategy: Option<PatchStrategy>,

    /// Session token for session-consistent writes.
    pub session_token: Option<SessionToken>,

    /// `If-Match` ETag check applied to the item before PATCH commits.
    pub precondition: Option<Precondition>,

    /// Maximum number of client-side Read-Modify-Write attempts the driver may
    /// make before surfacing a 412. Ignored by server-side PATCH. `None`
    /// selects the driver default (5).
    pub max_attempts: Option<std::num::NonZeroU8>,

    /// Stable identity for application-level retries of this PATCH.
    ///
    /// See [`Self::with_tracking_id`] for usage and guarantees.
    pub tracking_id: Option<PatchTrackingId>,

    /// Maximum number of PATCH tracking entries retained on the
    /// item. `None` selects
    /// [`DEFAULT_PATCH_TRACKING_CAPACITY`](crate::models::DEFAULT_PATCH_TRACKING_CAPACITY).
    /// When full after time-based pruning, the oldest entry is evicted.
    pub tracking_capacity: Option<std::num::NonZeroU16>,

    /// Number of whole seconds PATCH tracking entries remain eligible for
    /// duplicate suppression unless FIFO capacity pressure evicts them first.
    /// `None` selects
    /// [`PATCH_TRACKING_RETENTION`](crate::models::PATCH_TRACKING_RETENTION).
    pub tracking_retention_seconds: Option<std::num::NonZeroU32>,
}

#[cfg(feature = "preview_patch")]
impl PatchItemOptions {
    /// Sets the session token for this request.
    pub fn with_session_token(mut self, session_token: impl Into<SessionToken>) -> Self {
        self.session_token = Some(session_token.into());
        self
    }

    /// Sets an `If-Match` ETag check for this PATCH.
    pub fn with_precondition(mut self, precondition: Precondition) -> Self {
        self.precondition = Some(precondition);
        self
    }

    /// Caps the number of Read-Modify-Write attempts the driver may make.
    pub fn with_max_attempts(mut self, max_attempts: std::num::NonZeroU8) -> Self {
        self.max_attempts = Some(max_attempts);
        self
    }

    /// Selects how this PATCH executes.
    pub fn with_strategy(mut self, strategy: PatchStrategy) -> Self {
        self.strategy = Some(strategy);
        self
    }

    /// Sets the stable identity for this logical PATCH operation.
    ///
    /// This setting is effective only when strategy resolution selects
    /// client-side execution. It does not influence strategy selection and is
    /// ignored by server-side PATCH.
    ///
    /// On the client-side path, supplying an ID opts even a retry-safe
    /// instruction list into marker-based duplicate suppression. Persist and
    /// reuse the same random, unpredictable ID only for application-level
    /// retries of the same logical operation against the same item, including
    /// across process restarts. Reusing it for a different operation suppresses
    /// that operation. Cooperating writers must preserve the reserved tracking
    /// property and are trusted not to forge entries.
    ///
    /// When omitted, the driver generates an ID for unsafe client-side lists.
    /// The effective ID is available from the response, diagnostics, and errors
    /// so a retry can reuse it after an ambiguous failure. Duplicate suppression
    /// remains bounded by the configured retention window and tracking capacity.
    /// Tracking entries are visible in stored and returned JSON and count toward
    /// item size and indexing costs.
    pub fn with_tracking_id(mut self, tracking_id: PatchTrackingId) -> Self {
        self.tracking_id = Some(tracking_id);
        self
    }

    /// Sets the maximum number of tracking entries retained on an item.
    pub fn with_tracking_capacity(mut self, capacity: std::num::NonZeroU16) -> Self {
        self.tracking_capacity = Some(capacity);
        self
    }

    /// Sets the retention window used when pruning tracking entries by age.
    pub fn with_tracking_retention_seconds(
        mut self,
        retention_seconds: std::num::NonZeroU32,
    ) -> Self {
        self.tracking_retention_seconds = Some(retention_seconds);
        self
    }

    /// Sets the [`OperationOptions`] for this request.
    pub fn with_operation_options(mut self, operation: OperationOptions) -> Self {
        self.operation = operation;
        self
    }
}

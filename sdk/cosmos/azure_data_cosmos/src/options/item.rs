// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Options for item-level point reads, writes, and patch operations.

use azure_data_cosmos_driver::models::{Precondition, SessionToken};
use azure_data_cosmos_driver::options::OperationOptions;

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
/// PATCH is implemented driver-side as a Read-Modify-Write (RMW) loop:
/// the driver reads the current item, applies your [`PatchInstructions`](crate::models::PatchInstructions)
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
///   driver's [`PatchInstructions`](crate::models::PatchInstructions) has no `condition` field, so
///   there is no way to attach a predicate to a PATCH request.
///
/// The session token lives on the dedicated
/// [`session_token`](Self::session_token) field (mirroring
/// [`ItemReadOptions`] / [`ItemWriteOptions`]). All other general-purpose
/// settings (custom headers, content response behavior, excluded regions,
/// etc.) are configured via [`with_operation_options`](Self::with_operation_options) — see
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

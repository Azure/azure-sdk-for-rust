// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Options for item-level point reads, writes, and patch operations.

use azure_data_cosmos_driver::models::{Precondition, SessionToken};
use azure_data_cosmos_driver::options::{OperationOptions, PatchStrategy};

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
/// A patch executes one of two ways, selected by [`strategy`](Self::strategy):
///
/// * **Server-side** — the operation list is sent to the service as a single
///   request. One round trip, and on a multi-write-region account the service
///   resolves concurrent patches at the *path* level, so two writers touching
///   different properties of the same item both survive.
/// * **Client-side** — the driver reads the item, applies the operations
///   locally, and issues an ETag-guarded Replace, restarting on `412`. Two
///   round trips minimum, and conflict resolution is document-level
///   last-writer-wins, so a concurrent write to an unrelated property is lost.
///
/// [`PatchStrategy::Auto`] (the default) runs server-side whenever that is
/// safe and falls back to the client-side loop otherwise — see
/// [`PatchStrategy`] for what "safe" means.
///
/// [`max_attempts`](Self::max_attempts) bounds the client-side loop only; it
/// has no effect on a server-side patch, so under [`PatchStrategy::Auto`] it
/// applies only when the fallback runs.
///
/// # Conditions are not exposed yet
///
/// Neither flavor of "condition" that peer SDKs surface on their PATCH options
/// is available here yet:
///
/// * **`Precondition` (`If-Match` / `If-None-Match`).** The client-side handler
///   owns the `If-Match` on its internal Replace and captures the ETag from the
///   matching Read; a caller-set value would shadow it. Caller-set
///   preconditions are rejected before any sub-operation is issued.
/// * **SQL filter predicate** (peer SDKs' `FilterPredicate`). This requires the
///   server to evaluate the predicate inside the same transaction, so it is
///   meaningful only on the server-side path;
///   [`PatchInstructions`](crate::models::PatchInstructions) has no `condition`
///   field, so there is no way to attach one.
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
/// A server-side patch costs a single round trip, like a Replace. The
/// client-side loop is at minimum a Read followed by a Replace, so its
/// best-case floor is **2× the single-RTT cost** of a comparable Read or
/// Replace against the same partition, and each `412` retry adds another
/// Read+Replace pair.
///
/// When configuring an end-to-end latency budget via [`OperationOptions`],
/// size it for whichever path can run. If [`PatchStrategy::ClientSide`] is
/// possible — including via [`PatchStrategy::Auto`]'s fallback — a useful rule
/// of thumb is **≥ 2× the p99 single-RTT budget you would set for a plain
/// Replace**, plus headroom for the retries you want to tolerate. Too small a
/// budget can cancel the loop between the Read and the Replace, producing a
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
    ///
    /// Applies to the client-side path only.
    pub max_attempts: Option<std::num::NonZeroU8>,

    /// How this patch should execute.
    ///
    /// `None` inherits the client or account default
    /// ([`PatchStrategy::Auto`]).
    pub strategy: Option<PatchStrategy>,
}

impl PatchItemOptions {
    /// Sets the session token for this request.
    pub fn with_session_token(mut self, session_token: impl Into<SessionToken>) -> Self {
        self.session_token = Some(session_token.into());
        self
    }

    /// Caps the number of Read-Modify-Write attempts the driver may make.
    ///
    /// Applies to the client-side path only.
    pub fn with_max_attempts(mut self, max_attempts: std::num::NonZeroU8) -> Self {
        self.max_attempts = Some(max_attempts);
        self
    }

    /// Selects how this patch executes.
    pub fn with_strategy(mut self, strategy: PatchStrategy) -> Self {
        self.strategy = Some(strategy);
        self
    }

    /// Sets the [`OperationOptions`] for this request.
    pub fn with_operation_options(mut self, operation: OperationOptions) -> Self {
        self.operation = operation;
        self
    }
}

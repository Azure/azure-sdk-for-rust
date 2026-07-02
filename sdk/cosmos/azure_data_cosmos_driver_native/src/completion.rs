// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Completion queue, completion record, and operation handle types + their C
//! ABI surface.
//!
//! Implements the async-invocation model from spec section 3.1 + section 3.6.
//!
//! # Concurrency model (spec section 3.1.3 + section 7)
//!
//! Each `cosmos_completion_queue_t` is **multi-producer / single-consumer**:
//! any thread holding the pointer may enqueue (a successful submit on a Tokio
//! worker thread); only one thread at a time should call
//! [`cosmos_completion_queue_wait`].
//! The wrapper does not enforce the single-consumer rule in v1 (no internal
//! lock around the consumer-side dequeue beyond the queue's own mutex);
//! calling from two threads simultaneously is undefined behavior. See section 9 Q12.
//!
//! The crate ships the full FFI surface plus internal test-only helpers
//! ([`__test_only_enqueue_completion`]) so the receive-loop contract can be
//! validated end-to-end independently of the real submit pipeline.

use std::collections::VecDeque;
use std::ffi::{c_char, c_void, CString};
use std::sync::atomic::{AtomicBool, AtomicU8, Ordering};
use std::sync::{Arc, Condvar, Mutex};
use std::time::{Duration, Instant};

use azure_data_cosmos_driver::error::CosmosError as DriverCosmosError;
use azure_data_cosmos_driver::models::{
    ContainerReference as DriverContainerReference, CosmosResponse, ResponseBody,
};

use crate::container_ref::ContainerRefHandle;
use crate::driver::DriverHandle;
use crate::error::CosmosErrorCode;
use crate::response_header::{
    synthesize_response_headers, CosmosResponseHeader, OwnedResponseHeaders,
};
use crate::runtime::RuntimeContext;
use crate::safety::MutexExt;

// ─────────────────────────────────────────────────────────────────────────────
// Outcome enum (cosmos_completion_outcome_t)
// ─────────────────────────────────────────────────────────────────────────────

/// Per spec section 3.6.1, every completion has exactly one of these outcomes.
///
/// `CosmosCompletionOutcomeUnknown` is a forward-compat sentinel — older C clients
/// linked against a newer runtime that grew a variant see this value and can
/// route it through their default branch.
#[repr(i32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CosmosCompletionOutcome {
    /// The operation completed successfully; the completion's response fields
    /// (status, headers, body, …) are populated.
    CosmosCompletionOutcomeOk = 0,
    /// The operation failed; the completion's inline error fields (status,
    /// message, …) are populated (message/detail only when the queue opted in
    /// via `include_error_details`).
    CosmosCompletionOutcomeError = 1,
    /// The operation was cancelled via [`cosmos_operation_handle_cancel`] or
    /// [`cosmos_completion_queue_shutdown`].
    CosmosCompletionOutcomeCancelled = 2,
    /// Reserved sentinel for any non-zero outcome introduced after this spec.
    CosmosCompletionOutcomeUnknown = 255,
}

// ─────────────────────────────────────────────────────────────────────────────
// OperationHandle lifecycle state (cosmos_operation_handle_state_t)
// ─────────────────────────────────────────────────────────────────────────────

/// Per spec section 3.6.2, the four lifecycle states an operation handle observes.
#[repr(i32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CosmosOperationHandleState {
    /// Submission succeeded; no completion has been posted yet.
    CosmosOperationHandleStateInFlight = 0,
    /// Completion was posted with `outcome == CosmosCompletionOutcomeOk`.
    CosmosOperationHandleStateCompleted = 1,
    /// Completion was posted with `outcome == CosmosCompletionOutcomeError`.
    CosmosOperationHandleStateFailed = 2,
    /// Completion was posted with `outcome == CosmosCompletionOutcomeCancelled`.
    CosmosOperationHandleStateCancelled = 3,
}

impl CosmosOperationHandleState {
    fn from_u8(v: u8) -> Self {
        match v {
            0 => Self::CosmosOperationHandleStateInFlight,
            1 => Self::CosmosOperationHandleStateCompleted,
            2 => Self::CosmosOperationHandleStateFailed,
            3 => Self::CosmosOperationHandleStateCancelled,
            // Defensive — should not happen since we only ever store the
            // values above.
            _ => Self::CosmosOperationHandleStateInFlight,
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// CompletionQueue lifecycle state (cosmos_completion_queue_state_t)
// ─────────────────────────────────────────────────────────────────────────────

/// Per spec section 3.1.3, the three queue-lifecycle states.
#[repr(i32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CosmosCompletionQueueState {
    /// Submits and waits both succeed.
    CosmosCompletionQueueStateRunning = 0,
    /// `cosmos_completion_queue_shutdown` has been called; submits fail
    /// pre-flight; pending
    /// completions can still be drained via `_wait` until empty.
    CosmosCompletionQueueStateShutdown = 1,
    /// Shutdown + queue empty + no in-flight ops. Safe to `_free` without
    /// blocking.
    CosmosCompletionQueueStateDrained = 2,
}

impl CosmosCompletionQueueState {
    fn from_u8(v: u8) -> Self {
        match v {
            0 => Self::CosmosCompletionQueueStateRunning,
            1 => Self::CosmosCompletionQueueStateShutdown,
            2 => Self::CosmosCompletionQueueStateDrained,
            _ => Self::CosmosCompletionQueueStateRunning,
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// OperationHandle
// ─────────────────────────────────────────────────────────────────────────────

/// Internal state shared between the producer's `OperationHandle` and the
/// completion record's borrowed handle.
pub(crate) struct OperationInner {
    /// Lifecycle state — encoded as one of [`CosmosOperationHandleState`].
    state: AtomicU8,
    /// True once `cosmos_operation_handle_cancel` has been called on any
    /// handle pointing at this inner. The submit pipeline's
    /// `enqueue_into_inner` reads it to set `was_cancel_requested` on the
    /// published completion (so the receive loop can tell "cancel won" from
    /// "cancel lost the race").
    cancel_requested: AtomicBool,
    /// Wakes the submit task's `tokio::select!` cancel branch. A cancel
    /// stores a permit via `notify_one`, so a cancel that races ahead of the
    /// task starting to wait is still observed (the permit is consumed on the
    /// first poll of `notified()`).
    pub(crate) cancel_notify: tokio::sync::Notify,
}

impl OperationInner {
    fn new() -> Self {
        Self {
            state: AtomicU8::new(
                CosmosOperationHandleState::CosmosOperationHandleStateInFlight as u8,
            ),
            cancel_requested: AtomicBool::new(false),
            cancel_notify: tokio::sync::Notify::new(),
        }
    }
}

/// The C ABI handle for an in-flight (or just-completed) operation
/// (`cosmos_operation_handle_t`).
///
/// Each handle is its own `Box`; the `OperationInner` state behind it is
/// `Arc`-shared with the published completion.
pub struct OperationHandle {
    pub(crate) inner: Arc<OperationInner>,
}

impl OperationHandle {
    fn new_raw() -> *mut Self {
        Box::into_raw(Box::new(OperationHandle {
            inner: Arc::new(OperationInner::new()),
        }))
    }

    /// Allocates a fresh handle (same as `new_raw`) but exposed to
    /// `crate::submit` so the submit pipeline can mint the producer-side
    /// handle that pairs with the in-flight task.
    pub(crate) fn allocate() -> *mut Self {
        Self::new_raw()
    }

    /// Returns a cloned `Arc<OperationInner>` for the submit pipeline's
    /// tokio-side task to consult / write to.
    pub(crate) fn inner_arc(p: *const OperationHandle) -> Option<Arc<OperationInner>> {
        Self::from_ptr(p).map(|h| Arc::clone(&h.inner))
    }

    /// Build a fresh handle that shares the inner state of an existing handle.
    #[allow(dead_code, reason = "first caller is the submit pipeline")]
    fn clone_arc(p: *const OperationHandle) -> Option<*mut OperationHandle> {
        let handle = Self::from_ptr(p)?;
        Some(Box::into_raw(Box::new(OperationHandle {
            inner: Arc::clone(&handle.inner),
        })))
    }

    fn from_ptr<'a>(p: *const OperationHandle) -> Option<&'a OperationHandle> {
        if p.is_null() {
            None
        } else {
            // SAFETY: caller guarantees `p` came from `new_raw` or `clone_arc`.
            Some(unsafe { &*p })
        }
    }

    pub(crate) fn drop_raw(p: *mut OperationHandle) {
        if p.is_null() {
            return;
        }
        // SAFETY: caller guarantees `p` came from a library API and has not
        // already been freed.
        unsafe {
            drop(Box::from_raw(p));
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Completion
// ─────────────────────────────────────────────────────────────────────────────

/// Owned, `#[repr(C)]` completion handed to the host by
/// [`cosmos_completion_queue_wait`].
///
/// The host **owns** each completion between the wait that produced it and the
/// matching [`cosmos_completion_queue_free_completions`] call. It reads the
/// inline scalar fields directly and treats every pointer field as
/// **borrowed** — valid only until the free. To retain any string / body past
/// the free, the host must copy it into its own memory first.
///
/// Error detail is **inline** (`http_status_code` / `sub_status` / `message` /
/// …) rather than a separate error handle. The degenerate driver-creation and
/// container-resolution completions carry their result in the owned `driver` /
/// `container` fields, which the host may detach with
/// [`cosmos_completion_take_driver`] / [`cosmos_completion_take_container`]
/// (otherwise the free reclaims them).
#[repr(C)]
pub struct CosmosCompletion {
    /// The completion outcome (`Ok` / `Error` / `Cancelled` / `Unknown`).
    pub outcome: CosmosCompletionOutcome,
    /// Coarse status code (always populated).
    pub status: CosmosErrorCode,
    /// The host's opaque pointer-sized cookie, round-tripped verbatim from
    /// submit; the wrapper never dereferences it.
    pub user_data: isize,
    /// `1` iff cancellation was observed before the completion posted.
    pub was_cancel_requested: u8,
    /// Wire HTTP status code, or `0` when there is no wire response.
    pub http_status_code: u16,
    /// Cosmos sub-status code, or `-1` when absent.
    pub sub_status: i32,
    /// Request charge in Request Units, or `0.0` when absent.
    pub request_charge: f64,
    /// Retry-after hint in milliseconds, or `-1` when absent.
    pub retry_after_ms: i64,
    /// `1` iff an error completion originated from a service wire response.
    pub is_from_wire: u8,
    /// Borrowed error message (NUL-terminated UTF-8), or NULL on a non-error
    /// completion / when error details are suppressed.
    pub message: *const c_char,
    /// Borrowed activity id, or NULL when absent.
    pub activity_id: *const c_char,
    /// Borrowed session token, or NULL when absent.
    pub session_token: *const c_char,
    /// Borrowed ETag, or NULL when absent.
    pub etag: *const c_char,
    /// Borrowed server-header continuation token, or NULL when absent.
    pub continuation: *const c_char,
    /// Borrowed planner-derived next-page continuation token, or NULL when this
    /// was the last page / not a feed response.
    pub next_continuation: *const c_char,
    /// Borrowed error backtrace, or NULL when none was captured.
    pub backtrace: *const c_char,
    /// Borrowed `(id, value)` response-header list; NULL when empty. Resolve
    /// each id to its wire name with
    /// [`cosmos_header_name`](crate::response_header::cosmos_header_name).
    pub headers: *const CosmosResponseHeader,
    /// Number of entries addressable from `headers`.
    pub headers_len: usize,
    /// Borrowed response body bytes, or NULL when the body is empty.
    pub body: *const u8,
    /// Number of bytes addressable from `body`.
    pub body_len: usize,
    /// Reserved for a future diagnostics handle; always NULL for now.
    pub diagnostics: *mut c_void,
    /// Owned driver handle for a `get_or_create` completion, else NULL. Detach
    /// with [`cosmos_completion_take_driver`] or let the free reclaim it.
    pub driver: *mut DriverHandle,
    /// Owned container reference for a `resolve_container` completion, else
    /// NULL. Detach with [`cosmos_completion_take_container`] or let the free
    /// reclaim it.
    pub container: *mut ContainerRefHandle,
    /// Opaque owner of the borrowed allocations. The host never touches this;
    /// [`cosmos_completion_queue_free_completions`] reclaims it.
    pub backing: *mut CosmosCompletionBacking,
}

/// Opaque owner of a completion's heap allocations.
///
/// Holds the driver response (and thus the body bytes), the synthesized header
/// list, and the `CString` copies the completion's borrowed string pointers
/// reference, keeping them valid until the completion is freed. cbindgen emits
/// it as an opaque forward-declared struct the C side only ever holds by
/// pointer.
pub struct CosmosCompletionBacking {
    response: Option<CosmosResponse>,
    headers: OwnedResponseHeaders,
    message: Option<CString>,
    activity_id: Option<CString>,
    session_token: Option<CString>,
    etag: Option<CString>,
    continuation: Option<CString>,
    next_continuation: Option<CString>,
    backtrace: Option<CString>,
}

/// Internal queue item: owns every allocation a completion needs before it is
/// handed to the host, converted to an FFI [`CosmosCompletion`] at drain time
/// by [`PendingCompletion::into_ffi`].
pub(crate) struct PendingCompletion {
    outcome: CosmosCompletionOutcome,
    status: CosmosErrorCode,
    user_data: isize,
    was_cancel_requested: bool,
    http_status_code: u16,
    sub_status: i32,
    request_charge: f64,
    retry_after_ms: i64,
    is_from_wire: bool,
    message: Option<CString>,
    activity_id: Option<CString>,
    session_token: Option<CString>,
    etag: Option<CString>,
    continuation: Option<CString>,
    next_continuation: Option<CString>,
    backtrace: Option<CString>,
    headers: OwnedResponseHeaders,
    response: Option<CosmosResponse>,
    driver: Option<Arc<DriverHandle>>,
    container: Option<DriverContainerReference>,
    /// Producing operation's shared state (advanced to a terminal state at
    /// enqueue time); dropped when the completion is converted for the host.
    op_inner: Arc<OperationInner>,
}

/// Builds a NUL-terminated copy of `s`, stripping any interior NUL bytes so the
/// conversion cannot fail.
fn to_cstring(s: impl Into<String>) -> Option<CString> {
    CString::new(s.into().replace('\0', "")).ok()
}

/// Borrowed pointer to an optional `CString`, or NULL.
fn cstr_ptr(o: &Option<CString>) -> *const c_char {
    o.as_ref().map_or(std::ptr::null(), |c| c.as_ptr())
}

/// Borrowed `(ptr, len)` view of a response body, normalizing empty / feed /
/// no-payload bodies to `(NULL, 0)`.
fn body_view(response: &CosmosResponse) -> (*const u8, usize) {
    match response.body() {
        // Normalize an empty `Bytes` body to a NULL pointer so it matches the
        // documented "NULL pointer + 0 length when empty" contract.
        ResponseBody::Bytes(b) if b.is_empty() => (std::ptr::null(), 0),
        ResponseBody::Bytes(b) => (b.as_ptr(), b.len()),
        ResponseBody::Items(items) => items
            .first()
            .filter(|b| !b.is_empty())
            .map(|b| (b.as_ptr(), b.len()))
            .unwrap_or((std::ptr::null(), 0)),
        ResponseBody::NoPayload => (std::ptr::null(), 0),
    }
}

impl PendingCompletion {
    fn base(
        outcome: CosmosCompletionOutcome,
        status: CosmosErrorCode,
        user_data: isize,
        op_inner: Arc<OperationInner>,
    ) -> Self {
        Self {
            outcome,
            status,
            user_data,
            was_cancel_requested: false,
            http_status_code: 0,
            sub_status: -1,
            request_charge: 0.0,
            retry_after_ms: -1,
            is_from_wire: false,
            message: None,
            activity_id: None,
            session_token: None,
            etag: None,
            continuation: None,
            next_continuation: None,
            backtrace: None,
            headers: OwnedResponseHeaders::empty(),
            response: None,
            driver: None,
            container: None,
            op_inner,
        }
    }

    /// Successful completion carrying a response page (or a degenerate
    /// end-of-feed shell when `response` is `None`) plus an optional planner
    /// next-page token.
    pub(crate) fn ok_response(
        user_data: isize,
        op_inner: Arc<OperationInner>,
        response: Option<CosmosResponse>,
        next_continuation: Option<String>,
    ) -> Self {
        let mut p = Self::base(
            CosmosCompletionOutcome::CosmosCompletionOutcomeOk,
            CosmosErrorCode::CosmosErrorCodeSuccess,
            user_data,
            op_inner,
        );
        p.next_continuation = next_continuation.and_then(to_cstring);
        if let Some(resp) = response {
            let headers = resp.headers();
            p.http_status_code = u16::from(resp.status().status_code());
            p.sub_status = resp
                .status()
                .sub_status()
                .map_or(-1, |s| i32::from(s.value()));
            p.request_charge = headers
                .request_charge
                .as_ref()
                .map(|c| c.value())
                .unwrap_or(0.0);
            p.retry_after_ms = headers
                .retry_after_ms
                .map_or(-1, |ms| i64::try_from(ms).unwrap_or(i64::MAX));
            p.activity_id = headers
                .activity_id
                .as_ref()
                .and_then(|a| to_cstring(a.as_str()));
            p.session_token = headers
                .session_token
                .as_ref()
                .and_then(|t| to_cstring(t.as_str()));
            p.etag = headers
                .etag
                .as_ref()
                .and_then(|e| to_cstring(e.to_string()));
            p.continuation = headers
                .continuation
                .as_ref()
                .and_then(|c| to_cstring(c.clone()));
            p.headers = synthesize_response_headers(headers);
            p.response = Some(resp);
        }
        p
    }

    /// Successful degenerate completion carrying a freshly-created driver.
    pub(crate) fn ok_driver(
        user_data: isize,
        op_inner: Arc<OperationInner>,
        driver: Arc<DriverHandle>,
    ) -> Self {
        let mut p = Self::base(
            CosmosCompletionOutcome::CosmosCompletionOutcomeOk,
            CosmosErrorCode::CosmosErrorCodeSuccess,
            user_data,
            op_inner,
        );
        p.driver = Some(driver);
        p
    }

    /// Successful degenerate completion carrying a resolved container.
    pub(crate) fn ok_container(
        user_data: isize,
        op_inner: Arc<OperationInner>,
        container: DriverContainerReference,
    ) -> Self {
        let mut p = Self::base(
            CosmosCompletionOutcome::CosmosCompletionOutcomeOk,
            CosmosErrorCode::CosmosErrorCodeSuccess,
            user_data,
            op_inner,
        );
        p.container = Some(container);
        p
    }

    /// Error completion. When `include_details` is set the rich error fields
    /// (message, wire status, headers, backtrace) are flattened inline;
    /// otherwise only the coarse `status` is populated.
    pub(crate) fn error(
        user_data: isize,
        op_inner: Arc<OperationInner>,
        err: DriverCosmosError,
        coarse: CosmosErrorCode,
        include_details: bool,
    ) -> Self {
        let mut p = Self::base(
            CosmosCompletionOutcome::CosmosCompletionOutcomeError,
            coarse,
            user_data,
            op_inner,
        );
        if include_details {
            p.http_status_code = u16::from(err.status().status_code());
            p.sub_status = err
                .status()
                .sub_status()
                .map_or(-1, |s| i32::from(s.value()));
            p.is_from_wire = err.is_from_wire();
            p.message = to_cstring(err.to_string());
            p.backtrace = err
                .backtrace()
                .and_then(|bt| to_cstring(bt.as_ref().to_string()));
            if let Some(resp) = err.response() {
                let headers = resp.headers();
                p.activity_id = headers
                    .activity_id
                    .as_ref()
                    .and_then(|a| to_cstring(a.as_str()));
                p.session_token = headers
                    .session_token
                    .as_ref()
                    .and_then(|t| to_cstring(t.as_str()));
                p.etag = headers
                    .etag
                    .as_ref()
                    .and_then(|e| to_cstring(e.to_string()));
                p.retry_after_ms = headers
                    .retry_after_ms
                    .map_or(-1, |ms| i64::try_from(ms).unwrap_or(i64::MAX));
            }
        }
        p
    }

    /// Error completion carrying only a coarse code (no rich detail) — used for
    /// the submit panic firewall.
    pub(crate) fn error_coarse(
        user_data: isize,
        op_inner: Arc<OperationInner>,
        coarse: CosmosErrorCode,
    ) -> Self {
        Self::base(
            CosmosCompletionOutcome::CosmosCompletionOutcomeError,
            coarse,
            user_data,
            op_inner,
        )
    }

    /// Cancelled completion.
    pub(crate) fn cancelled(user_data: isize, op_inner: Arc<OperationInner>) -> Self {
        Self::base(
            CosmosCompletionOutcome::CosmosCompletionOutcomeCancelled,
            CosmosErrorCode::CosmosErrorCodeOperationCancelled,
            user_data,
            op_inner,
        )
    }

    /// Moves the owned allocations into a heap-stable backing box and returns
    /// the `#[repr(C)]` completion with borrowed pointers into it plus any
    /// owned side-payload handles. `op_inner` is dropped here (its terminal
    /// state was set at enqueue time).
    fn into_ffi(self) -> CosmosCompletion {
        let outcome = self.outcome;
        let status = self.status;
        let user_data = self.user_data;
        let was_cancel_requested = u8::from(self.was_cancel_requested);
        let http_status_code = self.http_status_code;
        let sub_status = self.sub_status;
        let request_charge = self.request_charge;
        let retry_after_ms = self.retry_after_ms;
        let is_from_wire = u8::from(self.is_from_wire);

        let driver = self
            .driver
            .map_or(std::ptr::null_mut(), DriverHandle::from_arc_into_raw);
        let container = self
            .container
            .map_or(std::ptr::null_mut(), ContainerRefHandle::into_raw);

        let backing = Box::new(CosmosCompletionBacking {
            response: self.response,
            headers: self.headers,
            message: self.message,
            activity_id: self.activity_id,
            session_token: self.session_token,
            etag: self.etag,
            continuation: self.continuation,
            next_continuation: self.next_continuation,
            backtrace: self.backtrace,
        });

        // Borrowed pointers into the (now heap-stable) backing box.
        let message = cstr_ptr(&backing.message);
        let activity_id = cstr_ptr(&backing.activity_id);
        let session_token = cstr_ptr(&backing.session_token);
        let etag = cstr_ptr(&backing.etag);
        let continuation = cstr_ptr(&backing.continuation);
        let next_continuation = cstr_ptr(&backing.next_continuation);
        let backtrace = cstr_ptr(&backing.backtrace);
        let (headers, headers_len) = backing.headers.as_ptr_len();
        let (body, body_len) = backing
            .response
            .as_ref()
            .map_or((std::ptr::null(), 0), body_view);

        CosmosCompletion {
            outcome,
            status,
            user_data,
            was_cancel_requested,
            http_status_code,
            sub_status,
            request_charge,
            retry_after_ms,
            is_from_wire,
            message,
            activity_id,
            session_token,
            etag,
            continuation,
            next_continuation,
            backtrace,
            headers,
            headers_len,
            body,
            body_len,
            diagnostics: std::ptr::null_mut(),
            driver,
            container,
            backing: Box::into_raw(backing),
        }
    }
}

impl CosmosCompletion {
    /// Frees the backing box and any un-detached owned side-payload handles,
    /// NULLing each reclaimed field so a double free within the same
    /// `free_completions` call is a no-op.
    fn free_inner(&mut self) {
        if !self.driver.is_null() {
            crate::driver::cosmos_driver_free(self.driver);
            self.driver = std::ptr::null_mut();
        }
        if !self.container.is_null() {
            crate::container_ref::cosmos_container_ref_free(self.container);
            self.container = std::ptr::null_mut();
        }
        if !self.backing.is_null() {
            // SAFETY: `backing` came from `Box::into_raw` in `into_ffi`; it is
            // reclaimed exactly once (the field is NULLed below).
            drop(unsafe { Box::from_raw(self.backing) });
            self.backing = std::ptr::null_mut();
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// CompletionQueue
// ─────────────────────────────────────────────────────────────────────────────

/// Inner queue state protected by a single mutex.
struct QueueInner {
    deque: VecDeque<PendingCompletion>,
    state: CosmosCompletionQueueState,
    /// Count of operations that have been admitted by the submit pre-flight
    /// (`reserve_in_flight`) but whose completion has not yet been drained by
    /// the host. An op is in-flight for its whole submit → run → enqueue →
    /// drain lifecycle. Gating the `SHUTDOWN` → `DRAINED` transition on this
    /// reaching zero (in addition to an empty deque) prevents the queue from
    /// declaring itself drained while an op is still running but has not yet
    /// enqueued its completion — which would otherwise strand that op handle
    /// `IN_FLIGHT` and leak the host continuation.
    in_flight: u32,
}

/// Per-queue options. Mirrors `cosmos_completion_queue_options_t` from spec section 3.1.2 — the
/// queue honors `max_capacity` and `include_error_details`; `capacity_hint`
/// is recorded but currently does not trigger any diagnostic (a one-shot
/// warning when the soft hint is exceeded is a follow-up).
#[derive(Clone, Copy, Debug)]
pub struct CqOptions {
    pub capacity_hint: u32,
    pub max_capacity: u32,
    pub include_error_details: bool,
}

impl Default for CqOptions {
    fn default() -> Self {
        Self {
            capacity_hint: 0,
            max_capacity: 0,
            include_error_details: true,
        }
    }
}

/// Layout of the `cosmos_completion_queue_options_t` struct as it appears at the C ABI
/// boundary. Caller-owned, pass-by-value (per section 3.1.2 the layout is published
/// for inputs).
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct CosmosCompletionQueueOptions {
    pub capacity_hint: u32,
    pub max_capacity: u32,
    /// Whether to capture rich error payloads, as a C boolean (`0` = false,
    /// non-zero = true). Read as a `u8` rather than a Rust `bool` so an
    /// arbitrary host-written byte cannot produce an invalid `bool` (which
    /// would be undefined behavior).
    pub include_error_details: u8,
}

impl From<CosmosCompletionQueueOptions> for CqOptions {
    fn from(c: CosmosCompletionQueueOptions) -> Self {
        Self {
            capacity_hint: c.capacity_hint,
            max_capacity: c.max_capacity,
            include_error_details: c.include_error_details != 0,
        }
    }
}

/// Internal `Arc`-shared queue state.
pub(crate) struct CompletionQueueInner {
    inner: Mutex<QueueInner>,
    /// Signalled whenever a new completion is enqueued.
    data_available: Condvar,
    /// Signalled whenever a completion is drained (so a back-pressured
    /// producer waiting on `_wait_writable` can wake).
    space_available: Condvar,
    options: CqOptions,
    /// Keep the runtime alive for the queue's lifetime. The submit pipeline
    /// clones this Arc to spawn per-operation tasks.
    pub(crate) runtime: Arc<RuntimeContext>,
}

impl CompletionQueueInner {
    /// Borrows the runtime backing this queue. Used by
    /// [`crate::submit`] to spawn per-operation tasks on the same
    /// Tokio runtime the queue was built against.
    pub(crate) fn runtime(&self) -> &Arc<RuntimeContext> {
        &self.runtime
    }

    /// Atomically admits a new operation, reserving an in-flight slot for the
    /// whole submit → run → enqueue → drain lifecycle. Used by the submit
    /// pre-flight ([`crate::submit`]) instead of separate state/length checks
    /// so the admission decision and the reservation happen under a single
    /// lock acquisition.
    ///
    /// Rejects (without reserving) when the queue is no longer running
    /// (`QUEUE_SHUTDOWN`) or — for a capacity-bounded queue — when it is
    /// already at its hard cap (`QUEUE_FULL`). Reserving here, rather than
    /// only checking at enqueue time, keeps `in_flight` an accurate count of
    /// admitted-but-undrained ops so the `SHUTDOWN` → `DRAINED` transition
    /// cannot race ahead of an op that is still running.
    pub(crate) fn reserve_in_flight(&self) -> Result<(), CosmosErrorCode> {
        let mut guard = self.inner.lock_recover();
        if guard.state != CosmosCompletionQueueState::CosmosCompletionQueueStateRunning {
            return Err(CosmosErrorCode::CosmosErrorCodeQueueShutdown);
        }
        if self.options.max_capacity > 0 && guard.deque.len() as u32 >= self.options.max_capacity {
            return Err(CosmosErrorCode::CosmosErrorCodeQueueFull);
        }
        guard.in_flight += 1;
        Ok(())
    }

    /// Releases a previously-reserved in-flight slot without delivering a
    /// completion. Used only on the rare pre-flight abort path (the operation
    /// handle could not be allocated after the slot was reserved), so the
    /// queue can still reach `DRAINED`.
    pub(crate) fn release_in_flight(&self) {
        let mut guard = self.inner.lock_recover();
        guard.in_flight = guard.in_flight.saturating_sub(1);
        maybe_mark_drained(&mut guard);
    }

    /// Returns whether this queue captures rich error payloads.
    pub(crate) fn include_error_details(&self) -> bool {
        self.options.include_error_details
    }
}

/// The C ABI handle for a completion queue (`cosmos_completion_queue_t`).
///
/// The `CompletionQueueInner` state behind it is `Arc`-shared so the submit
/// pipeline's spawned tasks survive a concurrent `cosmos_completion_queue_free` from the
/// producer side.
pub struct CompletionQueue {
    inner: Arc<CompletionQueueInner>,
}

impl CompletionQueue {
    fn new_raw(runtime: Arc<RuntimeContext>, options: CqOptions) -> *mut Self {
        Box::into_raw(Box::new(CompletionQueue {
            inner: Arc::new(CompletionQueueInner {
                inner: Mutex::new(QueueInner {
                    deque: VecDeque::new(),
                    state: CosmosCompletionQueueState::CosmosCompletionQueueStateRunning,
                    in_flight: 0,
                }),
                data_available: Condvar::new(),
                space_available: Condvar::new(),
                options,
                runtime,
            }),
        }))
    }

    /// Borrows the inner queue state for the submit pipeline. Returns
    /// `None` on NULL input; otherwise the caller can clone the runtime
    /// `Arc`, inspect capacity / state, and route through
    /// [`CompletionQueue::enqueue`] when the spawned task finishes.
    pub(crate) fn inner_arc(p: *const CompletionQueue) -> Option<Arc<CompletionQueueInner>> {
        Self::from_ptr(p).map(|h| Arc::clone(&h.inner))
    }

    fn from_ptr<'a>(p: *const CompletionQueue) -> Option<&'a CompletionQueue> {
        if p.is_null() {
            None
        } else {
            // SAFETY: caller guarantees `p` was obtained from `new_raw` and
            // has not been freed.
            Some(unsafe { &*p })
        }
    }

    fn drop_raw(p: *mut CompletionQueue) {
        if p.is_null() {
            return;
        }
        // SAFETY: caller guarantees `p` came from `new_raw` and has not
        // already been freed.
        unsafe {
            drop(Box::from_raw(p));
        }
    }

    /// Internal: pushes a completion onto the queue identified by the
    /// raw pointer.
    pub(crate) fn enqueue(p: *const CompletionQueue, c: PendingCompletion) -> CosmosErrorCode {
        let Some(handle) = Self::from_ptr(p) else {
            return CosmosErrorCode::CosmosErrorCodeInvalidArgument;
        };
        Self::enqueue_into_inner(&handle.inner, c)
    }

    /// Internal: pushes a completion onto the queue identified by an
    /// already-cloned `Arc<CompletionQueueInner>`. Used by the submit
    /// pipeline's spawned tasks so they survive concurrent
    /// `cosmos_completion_queue_free` from the producer side.
    pub(crate) fn enqueue_into_inner(
        inner: &Arc<CompletionQueueInner>,
        mut c: PendingCompletion,
    ) -> CosmosErrorCode {
        let mut guard = inner.inner.lock_recover();

        // If the producer-side handle's cancel flag is set, mark the
        // completion so the receive loop can distinguish "cancel won" from
        // "cancel lost the race" per spec section 3.6.1.
        if c.op_inner.cancel_requested.load(Ordering::Acquire) {
            c.was_cancel_requested = true;
        }
        // The terminal operation-handle state this completion implies. It is
        // stored on *every* path below \u2014 whether the completion is delivered
        // or rejected \u2014 so the op handle never stays stuck `IN_FLIGHT`: a host
        // that observes the outcome via `cosmos_operation_handle_state` is
        // always released and its `user_data` is always reclaimable (the
        // exactly-one-completion invariant, spec section 3.6).
        let next_state = match c.outcome {
            CosmosCompletionOutcome::CosmosCompletionOutcomeOk => {
                CosmosOperationHandleState::CosmosOperationHandleStateCompleted
            }
            CosmosCompletionOutcome::CosmosCompletionOutcomeError => {
                CosmosOperationHandleState::CosmosOperationHandleStateFailed
            }
            CosmosCompletionOutcome::CosmosCompletionOutcomeCancelled => {
                CosmosOperationHandleState::CosmosOperationHandleStateCancelled
            }
            CosmosCompletionOutcome::CosmosCompletionOutcomeUnknown => {
                CosmosOperationHandleState::CosmosOperationHandleStateFailed
            }
        };

        // A `DRAINED` queue has no consumer left (the host saw the queue empty
        // after shutdown and stopped draining), so a late completion is
        // dropped. `SHUTDOWN` (not yet drained) still *accepts* completions
        // from already-admitted in-flight ops: shutdown only blocks *new*
        // submissions, it must never strand work that was already running.
        if guard.state == CosmosCompletionQueueState::CosmosCompletionQueueStateDrained {
            c.op_inner.state.store(next_state as u8, Ordering::Release);
            guard.in_flight = guard.in_flight.saturating_sub(1);
            maybe_mark_drained(&mut guard);
            return CosmosErrorCode::CosmosErrorCodeQueueShutdown;
        }

        // Capacity backstop. Per-submit reservation (`reserve_in_flight`)
        // normally keeps the deque within bounds, but a burst of concurrent
        // submits can still race it to the hard cap before any drains; reject
        // the overflow rather than exceed the cap. The handle is still
        // advanced to its terminal state so nothing is stranded.
        if inner.options.max_capacity > 0 && guard.deque.len() as u32 >= inner.options.max_capacity
        {
            c.op_inner.state.store(next_state as u8, Ordering::Release);
            guard.in_flight = guard.in_flight.saturating_sub(1);
            maybe_mark_drained(&mut guard);
            return CosmosErrorCode::CosmosErrorCodeQueueFull;
        }

        c.op_inner.state.store(next_state as u8, Ordering::Release);
        guard.deque.push_back(c);
        inner.data_available.notify_one();
        CosmosErrorCode::CosmosErrorCodeSuccess
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// FFI: cosmos_completion_queue_*
// ─────────────────────────────────────────────────────────────────────────────

/// Create a completion queue bound to `runtime`. Returns NULL if `runtime`
/// is NULL.
#[no_mangle]
pub extern "C" fn cosmos_completion_queue_create(
    runtime: *const RuntimeContext,
    options: *const CosmosCompletionQueueOptions,
) -> *mut CompletionQueue {
    let Some(inner_rt) = RuntimeContext::inner_arc(runtime) else {
        return std::ptr::null_mut();
    };
    let opts = if options.is_null() {
        CqOptions::default()
    } else {
        // SAFETY: `options` is non-null and the caller guarantees it points
        // at a fully-initialized `CosmosCompletionQueueOptions` valid for the
        // duration of the call (per section 3.2 Pattern A inputs).
        CqOptions::from(*unsafe { &*options })
    };
    CompletionQueue::new_raw(inner_rt, opts)
}

/// Free a completion queue. NULL is a no-op.
///
/// The "blocks until in-flight ops drain" contract from spec section 3.1.2 is
/// observable here: if anyone enqueued completions but never drained, this
/// drops them (and thus their pending allocations).
#[no_mangle]
pub extern "C" fn cosmos_completion_queue_free(queue: *mut CompletionQueue) {
    if queue.is_null() {
        return;
    }
    tracing::trace!(?queue, "freeing cosmos_completion_queue_t");
    CompletionQueue::drop_raw(queue);
}

/// Pops one ready completion, handling the in-flight / space bookkeeping.
/// Blocks per `timeout_ms` semantics:
///
/// - `0` → poll once, `None` if empty.
/// - `UINT32_MAX` → wait without a timeout (returns `None` only on
///   shutdown / drained).
/// - otherwise → wait up to that many milliseconds.
fn wait_one(inner_arc: &Arc<CompletionQueueInner>, timeout_ms: u32) -> Option<PendingCompletion> {
    // Borrow through the cloned `Arc` so the mutex/condvar this thread may park
    // on outlives a concurrent `cosmos_completion_queue_free`.
    let inner = &**inner_arc;
    let mut guard = inner.inner.lock_recover();

    if timeout_ms == 0 {
        return pop_ready(inner, &mut guard);
    }

    let deadline = if timeout_ms == u32::MAX {
        None
    } else {
        Some(Instant::now() + Duration::from_millis(u64::from(timeout_ms)))
    };

    loop {
        if let Some(c) = pop_ready(inner, &mut guard) {
            return Some(c);
        }
        // Nothing ready. Promote to DRAINED once a requested shutdown has fully
        // completed (no in-flight ops, empty deque), then surface `None` so a
        // consumer that called shutdown can stop looping. While ops are still
        // in flight we keep waiting so their completions are delivered.
        maybe_mark_drained(&mut guard);
        if guard.state == CosmosCompletionQueueState::CosmosCompletionQueueStateDrained {
            return None;
        }
        guard = match deadline {
            None => inner
                .data_available
                .wait(guard)
                .unwrap_or_else(std::sync::PoisonError::into_inner),
            Some(d) => {
                let remaining = d.saturating_duration_since(Instant::now());
                if remaining.is_zero() {
                    return None;
                }
                let (g, timed_out) = inner
                    .data_available
                    .wait_timeout(guard, remaining)
                    .unwrap_or_else(std::sync::PoisonError::into_inner);
                if timed_out.timed_out() && g.deque.is_empty() {
                    return None;
                }
                g
            }
        };
    }
}

/// Pops a ready completion under an already-held lock, decrementing the
/// in-flight count, signalling space, and advancing the drain state.
fn pop_ready(
    inner: &CompletionQueueInner,
    guard: &mut std::sync::MutexGuard<'_, QueueInner>,
) -> Option<PendingCompletion> {
    let c = guard.deque.pop_front()?;
    guard.in_flight = guard.in_flight.saturating_sub(1);
    inner.space_available.notify_one();
    maybe_mark_drained(guard);
    Some(c)
}

fn maybe_mark_drained(guard: &mut std::sync::MutexGuard<'_, QueueInner>) {
    // DRAINED requires more than an empty deque: every admitted op must also
    // have been drained (`in_flight == 0`). Without the in-flight check a
    // queue could declare itself drained while an op is still running but has
    // not yet enqueued its completion — stranding that op handle `IN_FLIGHT`.
    if guard.state == CosmosCompletionQueueState::CosmosCompletionQueueStateShutdown
        && guard.deque.is_empty()
        && guard.in_flight == 0
    {
        guard.state = CosmosCompletionQueueState::CosmosCompletionQueueStateDrained;
    }
}

/// Wait for and drain up to `max` completions in a single call.
///
/// Blocks until at least one completion is available or `timeout_ms` elapses
/// (see `wait_one` for the timeout semantics), then drains additional
/// already-queued completions without blocking again. Writes each into
/// `out[0..max]` and returns the count written.
///
/// This is the sole wait entry point. Every completion written **must** be
/// released with [`cosmos_completion_queue_free_completions`] (the whole
/// `out` run at once, or element-by-element). Returns `0` (writing nothing)
/// on NULL `out`, `max == 0`, NULL queue, or shutdown / drained / timeout.
#[no_mangle]
pub extern "C" fn cosmos_completion_queue_wait(
    queue: *mut CompletionQueue,
    out: *mut CosmosCompletion,
    max: usize,
    timeout_ms: u32,
) -> usize {
    // Consumer entry point that runs on the host's thread and writes through a
    // caller-supplied buffer. Guard it so a panic can never unwind across the
    // FFI boundary into the host runtime.
    crate::safety::ffi_guard(0usize, || {
        if out.is_null() || max == 0 {
            return 0;
        }
        let Some(inner_arc) = CompletionQueue::inner_arc(queue) else {
            return 0;
        };
        let Some(first) = wait_one(&inner_arc, timeout_ms) else {
            return 0;
        };
        // SAFETY: caller guarantees `out` addresses at least `max` slots of
        // `CosmosCompletion`.
        unsafe { out.write(first.into_ffi()) };
        let mut count = 1usize;
        while count < max {
            let Some(next) = wait_one(&inner_arc, 0) else {
                break;
            };
            // SAFETY: `count < max`; same writable contract as above.
            unsafe { out.add(count).write(next.into_ffi()) };
            count += 1;
        }
        count
    })
}

/// Free a run of `count` completions produced by
/// [`cosmos_completion_queue_wait`], reclaiming each one's backing allocation
/// (body bytes, header list, cached strings) and any owned side-payload handle
/// (`driver` / `container`) the host did not detach. NULL / `count == 0` is a
/// no-op.
///
/// After this call every borrowed pointer the completions handed out is
/// invalid. The `out` array memory itself is caller-owned and is not freed.
#[no_mangle]
pub extern "C" fn cosmos_completion_queue_free_completions(
    completions: *mut CosmosCompletion,
    count: usize,
) {
    if completions.is_null() || count == 0 {
        return;
    }
    tracing::trace!(?completions, count, "freeing cosmos_completion_t run");
    // SAFETY: caller guarantees `completions` addresses `count` initialized
    // completions produced by `cosmos_completion_queue_wait`.
    let slice = unsafe { std::slice::from_raw_parts_mut(completions, count) };
    for c in slice {
        c.free_inner();
    }
}

/// Detaches and returns the owned driver handle from a `get_or_create`
/// completion, transferring ownership to the caller (a subsequent
/// [`cosmos_completion_queue_free_completions`] no longer reclaims it). Returns
/// NULL when `c` is NULL, the completion carries no driver, or it was already
/// taken. The caller frees the returned handle with `cosmos_driver_free`.
#[no_mangle]
pub extern "C" fn cosmos_completion_take_driver(c: *mut CosmosCompletion) -> *mut DriverHandle {
    // SAFETY: caller guarantees `c` is a valid completion pointer or NULL.
    let Some(c) = (unsafe { c.as_mut() }) else {
        return std::ptr::null_mut();
    };
    let p = c.driver;
    c.driver = std::ptr::null_mut();
    p
}

/// Detaches and returns the owned container reference from a
/// `resolve_container` completion. Same ownership-transfer semantics as
/// [`cosmos_completion_take_driver`]; free the returned handle with
/// `cosmos_container_ref_free`.
#[no_mangle]
pub extern "C" fn cosmos_completion_take_container(
    c: *mut CosmosCompletion,
) -> *mut ContainerRefHandle {
    // SAFETY: caller guarantees `c` is a valid completion pointer or NULL.
    let Some(c) = (unsafe { c.as_mut() }) else {
        return std::ptr::null_mut();
    };
    let p = c.container;
    c.container = std::ptr::null_mut();
    p
}

/// Block until the queue has room for at least one more pending completion,
/// or `timeout_ms` elapses.
#[no_mangle]
pub extern "C" fn cosmos_completion_queue_wait_writable(
    queue: *mut CompletionQueue,
    timeout_ms: u32,
) -> bool {
    // Clone the `Arc<CompletionQueueInner>` (rather than borrowing the storage
    // box) so the inner mutex/condvar this thread may park on outlives a
    // concurrent `cosmos_completion_queue_free`. This mirrors
    // `cosmos_completion_queue_wait`; borrowing the box here would otherwise
    // leave a use-after-free window for a thread blocked in
    // `space_available.wait(..)` when the producer frees the queue.
    let Some(inner_arc) = CompletionQueue::inner_arc(queue) else {
        return false;
    };
    let inner = &*inner_arc;
    if inner.options.max_capacity == 0 {
        // Unbounded — always writable.
        return true;
    }
    let mut guard = inner.inner.lock_recover();
    if guard.state != CosmosCompletionQueueState::CosmosCompletionQueueStateRunning {
        return false;
    }
    if (guard.deque.len() as u32) < inner.options.max_capacity {
        return true;
    }

    if timeout_ms == 0 {
        return false;
    }
    let deadline = if timeout_ms == u32::MAX {
        None
    } else {
        Some(Instant::now() + Duration::from_millis(u64::from(timeout_ms)))
    };

    loop {
        guard = match deadline {
            None => inner
                .space_available
                .wait(guard)
                .unwrap_or_else(std::sync::PoisonError::into_inner),
            Some(d) => {
                let remaining = d.saturating_duration_since(Instant::now());
                if remaining.is_zero() {
                    return false;
                }
                let (g, timed_out) = inner
                    .space_available
                    .wait_timeout(guard, remaining)
                    .unwrap_or_else(std::sync::PoisonError::into_inner);
                if timed_out.timed_out() {
                    return (g.deque.len() as u32) < inner.options.max_capacity
                        && g.state
                            == CosmosCompletionQueueState::CosmosCompletionQueueStateRunning;
                }
                g
            }
        };
        if guard.state != CosmosCompletionQueueState::CosmosCompletionQueueStateRunning {
            return false;
        }
        if (guard.deque.len() as u32) < inner.options.max_capacity {
            return true;
        }
    }
}

/// Signal shutdown: marks the queue as shutting down so no *new* submissions
/// are accepted, and wakes any thread blocked in
/// `cosmos_completion_queue_wait` / `_wait_writable`. Operations already in
/// flight are left to run to completion — their completions are still accepted
/// and can be drained — and the queue advances to `DRAINED` once the last
/// in-flight op has been drained. Idempotent.
#[no_mangle]
pub extern "C" fn cosmos_completion_queue_shutdown(queue: *mut CompletionQueue) {
    // Clone the `Arc` so the inner state survives a concurrent
    // `cosmos_completion_queue_free` while we hold the lock and signal the
    // condvars.
    let Some(inner_arc) = CompletionQueue::inner_arc(queue) else {
        return;
    };
    let inner = &*inner_arc;
    let mut guard = inner.inner.lock_recover();
    if guard.state == CosmosCompletionQueueState::CosmosCompletionQueueStateRunning {
        guard.state = CosmosCompletionQueueState::CosmosCompletionQueueStateShutdown;
        // Promote straight to DRAINED when nothing is outstanding (no pending
        // completions and no in-flight ops).
        maybe_mark_drained(&mut guard);
    }
    inner.data_available.notify_all();
    inner.space_available.notify_all();
}

/// Returns the queue's current lifecycle state.
#[no_mangle]
pub extern "C" fn cosmos_completion_queue_state(
    queue: *const CompletionQueue,
) -> CosmosCompletionQueueState {
    let Some(q) = CompletionQueue::from_ptr(queue) else {
        return CosmosCompletionQueueState::CosmosCompletionQueueStateRunning;
    };
    let guard = q.inner.inner.lock_recover();
    CosmosCompletionQueueState::from_u8(guard.state as u8)
}

// ─────────────────────────────────────────────────────────────────────────────
// FFI: cosmos_operation_handle_*
// ─────────────────────────────────────────────────────────────────────────────

/// Request cooperative cancellation. Idempotent and non-blocking.
///
/// Sets the cancel-requested flag and wakes the submit task's
/// `tokio::select!` cancel branch (via a stored `Notify` permit, so a cancel
/// that races ahead of the task is still observed). The task then drops the
/// in-flight driver future and posts a `CANCELLED` completion. If the
/// operation already produced a completion before the cancel was observed,
/// the cancel is a no-op for the outcome but is still reflected in
/// `cosmos_completion_was_cancel_requested`.
#[no_mangle]
pub extern "C" fn cosmos_operation_handle_cancel(op: *mut OperationHandle) {
    // Clone the `Arc<OperationInner>` so the inner state survives a concurrent
    // `cosmos_operation_handle_free` (matching the rest of the crate's
    // ownership model). A borrowed reference would leave a use-after-free
    // window if another thread freed the handle mid-call.
    let Some(inner) = OperationHandle::inner_arc(op) else {
        return;
    };
    inner.cancel_requested.store(true, Ordering::Release);
    // Store a permit so the submit task observes the cancel even if it has
    // not yet reached its `notified()` await point.
    inner.cancel_notify.notify_one();
}

/// Poll the operation's lifecycle state. Returns `InFlight` if `op` is NULL.
#[no_mangle]
pub extern "C" fn cosmos_operation_handle_state(
    op: *const OperationHandle,
) -> CosmosOperationHandleState {
    // Clone the `Arc<OperationInner>` (rather than borrowing) for the same
    // survive-concurrent-free reason as `cosmos_operation_handle_cancel`.
    let Some(inner) = OperationHandle::inner_arc(op) else {
        return CosmosOperationHandleState::CosmosOperationHandleStateInFlight;
    };
    CosmosOperationHandleState::from_u8(inner.state.load(Ordering::Acquire))
}

/// Free the FFI handle. Does NOT cancel the operation — call
/// `cosmos_operation_handle_cancel` first if needed. NULL is a no-op.
///
/// Drops this handle's `Arc` reference. If the completion record still holds
/// its own reference, the inner operation state stays alive.
#[no_mangle]
pub extern "C" fn cosmos_operation_handle_free(op: *mut OperationHandle) {
    if op.is_null() {
        return;
    }
    tracing::trace!(?op, "freeing cosmos_operation_handle_t");
    OperationHandle::drop_raw(op);
}

// ─────────────────────────────────────────────────────────────────────────────
// Test-only helpers
//
// These let integration tests synthesize completions end-to-end without the
// real submit pipeline.
// ─────────────────────────────────────────────────────────────────────────────

/// Test-only: allocate a new operation handle, returning the producer-side
/// raw pointer.
#[doc(hidden)]
pub fn __test_only_create_operation_handle() -> *mut OperationHandle {
    OperationHandle::new_raw()
}

/// Test-only: reserve an in-flight slot on `queue`, simulating what the real
/// submit pre-flight does when it admits an operation. Lets tests exercise the
/// `in_flight` accounting (which gates the `SHUTDOWN` → `DRAINED` transition)
/// without standing up the full submit pipeline. Returns the coarse code from
/// [`CompletionQueueInner::reserve_in_flight`].
#[doc(hidden)]
pub fn __test_only_reserve_in_flight(queue: *mut CompletionQueue) -> CosmosErrorCode {
    let Some(inner_arc) = CompletionQueue::inner_arc(queue) else {
        return CosmosErrorCode::CosmosErrorCodeInvalidArgument;
    };
    match inner_arc.reserve_in_flight() {
        Ok(()) => CosmosErrorCode::CosmosErrorCodeSuccess,
        Err(code) => code,
    }
}

/// Test-only: synthesize a completion record and enqueue it onto `queue`.
///
/// Cloning the operation handle's `Arc` keeps both the producer-side handle
/// and the completion-side borrow alive independently per spec section 3.6.2.
/// An `Error` outcome flattens `error` (or a synthetic placeholder) inline,
/// honoring the queue's `include_error_details`.
#[doc(hidden)]
pub fn __test_only_enqueue_completion(
    queue: *mut CompletionQueue,
    op_handle: *mut OperationHandle,
    outcome: CosmosCompletionOutcome,
    status: CosmosErrorCode,
    // Test ergonomics: callers pass a pointer-cast cookie; we store it as the
    // `isize` the real submit path uses. This helper is `#[doc(hidden)]` and
    // never crosses the (now `intptr_t`) C ABI.
    user_data: *mut c_void,
    error: Option<DriverCosmosError>,
) -> CosmosErrorCode {
    let Some(storage) = CompletionQueue::from_ptr(queue) else {
        return CosmosErrorCode::CosmosErrorCodeInvalidArgument;
    };
    let Some(op_storage) = OperationHandle::from_ptr(op_handle) else {
        return CosmosErrorCode::CosmosErrorCodeInvalidArgument;
    };
    let include_error = storage.inner.options.include_error_details;
    let op_inner = Arc::clone(&op_storage.inner);
    let ud = user_data as isize;
    let pending = match outcome {
        CosmosCompletionOutcome::CosmosCompletionOutcomeError => {
            let err = error.unwrap_or_else(|| {
                DriverCosmosError::builder()
                    .with_message("synthetic test error")
                    .build()
            });
            PendingCompletion::error(ud, op_inner, err, status, include_error)
        }
        CosmosCompletionOutcome::CosmosCompletionOutcomeCancelled => {
            let mut p = PendingCompletion::cancelled(ud, op_inner);
            p.status = status;
            p
        }
        other => {
            let mut p = PendingCompletion::ok_response(ud, op_inner, None, None);
            p.outcome = other;
            p.status = status;
            p
        }
    };
    CompletionQueue::enqueue(queue, pending)
}

// ─────────────────────────────────────────────────────────────────────────────
// Tests
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::runtime::__test_only_create_default_runtime;
    use std::mem::MaybeUninit;

    fn fresh_queue(max_capacity: u32, include_error_details: bool) -> *mut CompletionQueue {
        let rt = __test_only_create_default_runtime();
        let opts = CosmosCompletionQueueOptions {
            capacity_hint: 0,
            max_capacity,
            include_error_details: include_error_details as u8,
        };
        let q = cosmos_completion_queue_create(rt, &opts as *const _);
        // Runtime is held internally via Arc; we can free the producer-side
        // handle right away. The public builder makes this rebinding
        // ergonomic.
        crate::runtime::cosmos_runtime_free(rt);
        q
    }

    /// Drains exactly one completion via the array wait, returning it by value.
    fn wait_one_ffi(q: *mut CompletionQueue, timeout_ms: u32) -> Option<CosmosCompletion> {
        let mut slot = MaybeUninit::<CosmosCompletion>::uninit();
        let n = cosmos_completion_queue_wait(q, slot.as_mut_ptr(), 1, timeout_ms);
        if n == 0 {
            None
        } else {
            // SAFETY: `n == 1` means one completion was written into `slot`.
            Some(unsafe { slot.assume_init() })
        }
    }

    /// Frees a single completion produced by [`wait_one_ffi`].
    fn free_one(mut c: CosmosCompletion) {
        cosmos_completion_queue_free_completions(&mut c as *mut CosmosCompletion, 1);
    }

    #[test]
    fn create_and_free_queue() {
        let q = fresh_queue(0, true);
        assert!(!q.is_null());
        assert_eq!(
            cosmos_completion_queue_state(q),
            CosmosCompletionQueueState::CosmosCompletionQueueStateRunning
        );
        cosmos_completion_queue_free(q);
    }

    #[test]
    fn poll_returns_null_on_empty() {
        let q = fresh_queue(0, true);
        assert!(wait_one_ffi(q, 0).is_none());
        cosmos_completion_queue_free(q);
    }

    #[test]
    fn wait_zero_polls_without_blocking() {
        let q = fresh_queue(0, true);
        let start = Instant::now();
        let result = wait_one_ffi(q, 0);
        let elapsed = start.elapsed();
        assert!(result.is_none());
        // Should be near-instantaneous (well under 50ms even on slow CI).
        assert!(elapsed < Duration::from_millis(50), "elapsed: {elapsed:?}");
        cosmos_completion_queue_free(q);
    }

    #[test]
    fn wait_with_short_timeout_returns_null_on_empty() {
        let q = fresh_queue(0, true);
        let start = Instant::now();
        let result = wait_one_ffi(q, 20);
        let elapsed = start.elapsed();
        assert!(result.is_none());
        // Should wait close to the timeout.
        assert!(elapsed >= Duration::from_millis(15));
        cosmos_completion_queue_free(q);
    }

    #[test]
    fn synthetic_round_trip_carries_user_data_and_outcome() {
        let q = fresh_queue(0, true);
        let op = __test_only_create_operation_handle();
        let token = 0xDEAD_BEEFu64;
        let code = __test_only_enqueue_completion(
            q,
            op,
            CosmosCompletionOutcome::CosmosCompletionOutcomeOk,
            CosmosErrorCode::CosmosErrorCodeSuccess,
            token as *mut c_void,
            None,
        );
        assert_eq!(code, CosmosErrorCode::CosmosErrorCodeSuccess);

        // Operation handle state should reflect Completed.
        assert_eq!(
            cosmos_operation_handle_state(op),
            CosmosOperationHandleState::CosmosOperationHandleStateCompleted
        );

        let c = wait_one_ffi(q, 100).expect("a completion was delivered");
        assert_eq!(
            c.outcome,
            CosmosCompletionOutcome::CosmosCompletionOutcomeOk
        );
        assert_eq!(c.user_data, token as isize);
        assert_eq!(c.status, CosmosErrorCode::CosmosErrorCodeSuccess);
        assert_eq!(c.was_cancel_requested, 0);

        free_one(c);
        cosmos_operation_handle_free(op);
        cosmos_completion_queue_free(q);
    }

    #[test]
    fn shutdown_transitions_through_drained() {
        let q = fresh_queue(0, true);
        assert_eq!(
            cosmos_completion_queue_state(q),
            CosmosCompletionQueueState::CosmosCompletionQueueStateRunning
        );
        cosmos_completion_queue_shutdown(q);
        // No completions were enqueued, so we should land in Drained
        // immediately.
        assert_eq!(
            cosmos_completion_queue_state(q),
            CosmosCompletionQueueState::CosmosCompletionQueueStateDrained
        );
        // Further submits should be rejected with QueueShutdown.
        let op = __test_only_create_operation_handle();
        let code = __test_only_enqueue_completion(
            q,
            op,
            CosmosCompletionOutcome::CosmosCompletionOutcomeOk,
            CosmosErrorCode::CosmosErrorCodeSuccess,
            std::ptr::null_mut(),
            None,
        );
        assert_eq!(code, CosmosErrorCode::CosmosErrorCodeQueueShutdown);
        // Even though the completion was rejected, the operation handle must
        // never be left stuck `IN_FLIGHT`: it is advanced to a terminal state
        // so a host polling `cosmos_operation_handle_state` is released and its
        // `user_data` is reclaimable.
        assert_eq!(
            cosmos_operation_handle_state(op),
            CosmosOperationHandleState::CosmosOperationHandleStateCompleted
        );
        cosmos_operation_handle_free(op);
        cosmos_completion_queue_free(q);
    }

    #[test]
    fn in_flight_op_keeps_queue_out_of_drained_until_delivered() {
        // Reproduces the lost-completion bug: an op admitted by pre-flight but
        // still running (no completion enqueued yet) must keep the queue in
        // SHUTDOWN — not DRAINED — so its completion is still accepted and
        // delivered, rather than dropped.
        let q = fresh_queue(0, true);
        // Simulate the submit pre-flight admitting one operation.
        assert_eq!(
            __test_only_reserve_in_flight(q),
            CosmosErrorCode::CosmosErrorCodeSuccess
        );

        // Shutting down now must NOT jump straight to Drained — an op is still
        // in flight.
        cosmos_completion_queue_shutdown(q);
        assert_eq!(
            cosmos_completion_queue_state(q),
            CosmosCompletionQueueState::CosmosCompletionQueueStateShutdown
        );

        // The in-flight op finishes and enqueues its completion. Even though
        // the queue is SHUTDOWN, the completion is accepted (shutdown only
        // blocks *new* submissions).
        let op = __test_only_create_operation_handle();
        let code = __test_only_enqueue_completion(
            q,
            op,
            CosmosCompletionOutcome::CosmosCompletionOutcomeOk,
            CosmosErrorCode::CosmosErrorCodeSuccess,
            0x42 as *mut c_void,
            None,
        );
        assert_eq!(code, CosmosErrorCode::CosmosErrorCodeSuccess);
        assert_eq!(
            cosmos_completion_queue_state(q),
            CosmosCompletionQueueState::CosmosCompletionQueueStateShutdown
        );

        // The host drains it (the completion is delivered, not lost). Draining
        // the last in-flight op flips the queue to Drained.
        let c = wait_one_ffi(q, 100).expect("completion delivered");
        assert_eq!(c.user_data, 0x42);
        assert_eq!(
            cosmos_completion_queue_state(q),
            CosmosCompletionQueueState::CosmosCompletionQueueStateDrained
        );

        free_one(c);
        cosmos_operation_handle_free(op);
        cosmos_completion_queue_free(q);
    }

    #[test]
    fn shutdown_drains_then_signals_done() {
        let q = fresh_queue(0, true);
        let op = __test_only_create_operation_handle();
        // Enqueue first, then shut down → should be in Shutdown (not Drained)
        // until we drain.
        __test_only_enqueue_completion(
            q,
            op,
            CosmosCompletionOutcome::CosmosCompletionOutcomeOk,
            CosmosErrorCode::CosmosErrorCodeSuccess,
            std::ptr::null_mut(),
            None,
        );
        cosmos_completion_queue_shutdown(q);
        assert_eq!(
            cosmos_completion_queue_state(q),
            CosmosCompletionQueueState::CosmosCompletionQueueStateShutdown
        );

        let c = wait_one_ffi(q, 100).expect("completion delivered");
        free_one(c);

        // After draining, the next wait should return nothing immediately and
        // state should flip to Drained.
        assert!(wait_one_ffi(q, 50).is_none());
        assert_eq!(
            cosmos_completion_queue_state(q),
            CosmosCompletionQueueState::CosmosCompletionQueueStateDrained
        );

        cosmos_operation_handle_free(op);
        cosmos_completion_queue_free(q);
    }

    #[test]
    fn cancel_flips_handle_state_and_completion_flag() {
        let q = fresh_queue(0, true);
        let op = __test_only_create_operation_handle();
        // Cancel before enqueueing → completion should carry the
        // was_cancel_requested flag.
        cosmos_operation_handle_cancel(op);
        __test_only_enqueue_completion(
            q,
            op,
            CosmosCompletionOutcome::CosmosCompletionOutcomeCancelled,
            CosmosErrorCode::CosmosErrorCodeOperationCancelled,
            std::ptr::null_mut(),
            None,
        );
        let c = wait_one_ffi(q, 100).expect("completion delivered");
        assert_eq!(
            c.outcome,
            CosmosCompletionOutcome::CosmosCompletionOutcomeCancelled
        );
        assert_eq!(c.was_cancel_requested, 1);
        assert_eq!(c.status, CosmosErrorCode::CosmosErrorCodeOperationCancelled);
        // The operation handle's state should reflect Cancelled.
        assert_eq!(
            cosmos_operation_handle_state(op),
            CosmosOperationHandleState::CosmosOperationHandleStateCancelled
        );

        free_one(c);
        cosmos_operation_handle_free(op);
        cosmos_completion_queue_free(q);
    }

    #[test]
    fn cancel_lost_the_race_keeps_natural_outcome_with_flag() {
        let q = fresh_queue(0, true);
        let op = __test_only_create_operation_handle();
        // Cancel arrives after a successful outcome was already chosen by
        // the producer-side. The spec says was_cancel_requested = true,
        // outcome = Ok. We exercise the same code path: cancel before
        // enqueue with outcome = Ok.
        cosmos_operation_handle_cancel(op);
        __test_only_enqueue_completion(
            q,
            op,
            CosmosCompletionOutcome::CosmosCompletionOutcomeOk,
            CosmosErrorCode::CosmosErrorCodeSuccess,
            std::ptr::null_mut(),
            None,
        );
        let c = wait_one_ffi(q, 100).expect("completion delivered");
        assert_eq!(
            c.outcome,
            CosmosCompletionOutcome::CosmosCompletionOutcomeOk
        );
        assert_eq!(c.was_cancel_requested, 1);
        free_one(c);
        cosmos_operation_handle_free(op);
        cosmos_completion_queue_free(q);
    }

    #[test]
    fn max_capacity_rejects_at_limit() {
        let q = fresh_queue(2, true);
        for _ in 0..2 {
            let op = __test_only_create_operation_handle();
            let code = __test_only_enqueue_completion(
                q,
                op,
                CosmosCompletionOutcome::CosmosCompletionOutcomeOk,
                CosmosErrorCode::CosmosErrorCodeSuccess,
                std::ptr::null_mut(),
                None,
            );
            assert_eq!(code, CosmosErrorCode::CosmosErrorCodeSuccess);
            cosmos_operation_handle_free(op);
        }
        // Third one rejected.
        let op3 = __test_only_create_operation_handle();
        let code = __test_only_enqueue_completion(
            q,
            op3,
            CosmosCompletionOutcome::CosmosCompletionOutcomeOk,
            CosmosErrorCode::CosmosErrorCodeSuccess,
            std::ptr::null_mut(),
            None,
        );
        assert_eq!(code, CosmosErrorCode::CosmosErrorCodeQueueFull);
        // A capacity rejection must still advance the handle out of
        // `IN_FLIGHT` so the host is never stranded waiting on it.
        assert_eq!(
            cosmos_operation_handle_state(op3),
            CosmosOperationHandleState::CosmosOperationHandleStateCompleted
        );
        cosmos_operation_handle_free(op3);

        // Drain one and verify writable.
        let c = wait_one_ffi(q, 0).expect("completion present");
        free_one(c);
        assert!(cosmos_completion_queue_wait_writable(q, 0));

        // Drain the rest.
        let c2 = wait_one_ffi(q, 0).expect("completion present");
        free_one(c2);
        cosmos_completion_queue_free(q);
    }

    #[test]
    fn wait_writable_is_immediate_on_unbounded_queue() {
        let q = fresh_queue(0, true);
        assert!(cosmos_completion_queue_wait_writable(q, 0));
        cosmos_completion_queue_free(q);
    }

    #[test]
    fn wait_writable_times_out_on_full() {
        let q = fresh_queue(1, true);
        let op = __test_only_create_operation_handle();
        __test_only_enqueue_completion(
            q,
            op,
            CosmosCompletionOutcome::CosmosCompletionOutcomeOk,
            CosmosErrorCode::CosmosErrorCodeSuccess,
            std::ptr::null_mut(),
            None,
        );
        let start = Instant::now();
        let ok = cosmos_completion_queue_wait_writable(q, 30);
        assert!(!ok, "should time out on a full queue");
        assert!(start.elapsed() >= Duration::from_millis(20));
        // Drain.
        let c = wait_one_ffi(q, 0).expect("completion present");
        free_one(c);
        cosmos_operation_handle_free(op);
        cosmos_completion_queue_free(q);
    }

    #[test]
    fn wait_drains_multiple_in_one_call() {
        let q = fresh_queue(0, true);
        let op = __test_only_create_operation_handle();
        for i in 0..5 {
            __test_only_enqueue_completion(
                q,
                op,
                CosmosCompletionOutcome::CosmosCompletionOutcomeOk,
                CosmosErrorCode::CosmosErrorCodeSuccess,
                i as *mut c_void,
                None,
            );
        }
        let mut buf: [MaybeUninit<CosmosCompletion>; 16] = [const { MaybeUninit::uninit() }; 16];
        let count = cosmos_completion_queue_wait(q, buf.as_mut_ptr().cast(), buf.len(), 100);
        assert_eq!(count, 5);
        for (i, slot) in buf.iter().take(count).enumerate() {
            // SAFETY: the wait initialized the first `count` slots.
            let c = unsafe { slot.assume_init_ref() };
            // user_data preserved in order.
            assert_eq!(c.user_data, i as isize);
        }
        // Free the whole run in one call.
        cosmos_completion_queue_free_completions(buf.as_mut_ptr().cast(), count);
        cosmos_operation_handle_free(op);
        cosmos_completion_queue_free(q);
    }

    #[test]
    fn error_completion_carries_inline_fields() {
        use azure_data_cosmos_driver::error::{CosmosError, CosmosStatus};
        let q = fresh_queue(0, true);
        let op = __test_only_create_operation_handle();
        let err = CosmosError::builder()
            .with_status(CosmosStatus::new(azure_core::http::StatusCode::NotFound))
            .with_message("test error")
            .build();
        __test_only_enqueue_completion(
            q,
            op,
            CosmosCompletionOutcome::CosmosCompletionOutcomeError,
            CosmosErrorCode::CosmosErrorCodeNotFound,
            std::ptr::null_mut(),
            Some(err),
        );

        let c = wait_one_ffi(q, 100).expect("completion delivered");
        assert_eq!(c.status, CosmosErrorCode::CosmosErrorCodeNotFound);
        assert_eq!(c.http_status_code, 404);
        assert!(!c.message.is_null());
        // SAFETY: `message` is a NUL-terminated string owned by the completion.
        let msg = unsafe { std::ffi::CStr::from_ptr(c.message) }
            .to_string_lossy()
            .into_owned();
        assert!(msg.contains("test error"), "got: {msg}");

        free_one(c);
        cosmos_operation_handle_free(op);
        cosmos_completion_queue_free(q);
    }

    #[test]
    fn include_error_details_false_suppresses_rich_fields() {
        use azure_data_cosmos_driver::error::{CosmosError, CosmosStatus};
        let q = fresh_queue(0, /* include_error_details = */ false);
        let op = __test_only_create_operation_handle();
        let err = CosmosError::builder()
            .with_status(CosmosStatus::new(azure_core::http::StatusCode::Conflict))
            .with_message("dropped")
            .build();
        __test_only_enqueue_completion(
            q,
            op,
            CosmosCompletionOutcome::CosmosCompletionOutcomeError,
            CosmosErrorCode::CosmosErrorCodeConflict,
            std::ptr::null_mut(),
            Some(err),
        );
        let c = wait_one_ffi(q, 100).expect("completion delivered");
        // Coarse status survives.
        assert_eq!(c.status, CosmosErrorCode::CosmosErrorCodeConflict);
        // Rich detail suppressed.
        assert!(c.message.is_null());
        assert_eq!(c.http_status_code, 0);
        free_one(c);
        cosmos_operation_handle_free(op);
        cosmos_completion_queue_free(q);
    }

    #[test]
    fn shutdown_wakes_blocked_waiter() {
        let q = fresh_queue(0, true);
        let q_addr = q as usize;
        // Spawn a thread that will sit in the wait with a long timeout.
        let handle = std::thread::spawn(move || {
            let q = q_addr as *mut CompletionQueue;
            let start = Instant::now();
            let got = wait_one_ffi(q, 5_000); // 5s timeout
            (got.is_none(), start.elapsed())
        });
        std::thread::sleep(Duration::from_millis(20));
        cosmos_completion_queue_shutdown(q);
        let (was_empty, elapsed) = handle.join().unwrap();
        assert!(was_empty, "shutdown should return no completion from wait");
        assert!(
            elapsed < Duration::from_millis(500),
            "wait should wake within ~milliseconds of shutdown, took {elapsed:?}"
        );
        cosmos_completion_queue_free(q);
    }

    #[test]
    fn shutdown_wakes_blocked_writable_waiter() {
        // A thread parked in `cosmos_completion_queue_wait_writable` on a full
        // bounded queue must wake on shutdown and report not-writable. This also
        // exercises the `Arc`-clone path that lets the parked thread's
        // mutex/condvar survive a concurrent free of the queue handle.
        let q = fresh_queue(1, true);
        let op = __test_only_create_operation_handle();
        // Fill the single slot so writability blocks.
        __test_only_enqueue_completion(
            q,
            op,
            CosmosCompletionOutcome::CosmosCompletionOutcomeOk,
            CosmosErrorCode::CosmosErrorCodeSuccess,
            std::ptr::null_mut(),
            None,
        );
        let q_addr = q as usize;
        let handle = std::thread::spawn(move || {
            let q = q_addr as *mut CompletionQueue;
            let start = Instant::now();
            let writable = cosmos_completion_queue_wait_writable(q, 5_000); // 5s timeout
            (writable, start.elapsed())
        });
        std::thread::sleep(Duration::from_millis(20));
        cosmos_completion_queue_shutdown(q);
        let (writable, elapsed) = handle.join().unwrap();
        assert!(!writable, "shutdown should report not-writable");
        assert!(
            elapsed < Duration::from_millis(500),
            "wait_writable should wake within ~milliseconds of shutdown, took {elapsed:?}"
        );
        // Drain the queued completion and free.
        let c = wait_one_ffi(q, 0).expect("completion present");
        free_one(c);
        cosmos_operation_handle_free(op);
        cosmos_completion_queue_free(q);
    }
}

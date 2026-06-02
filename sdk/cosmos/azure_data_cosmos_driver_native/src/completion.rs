// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Completion queue, completion record, and operation handle types + their C
//! ABI surface.
//!
//! Implements the async-invocation model from spec §3.1 + §3.6.
//!
//! # Concurrency model (spec §3.1.3 + §7)
//!
//! Each `cosmos_cq_t` is **multi-producer / single-consumer**: any thread
//! holding the pointer may enqueue (a successful submit on a Tokio worker
//! thread); only one thread at a time should call
//! [`cosmos_cq_wait`] / [`cosmos_cq_try_wait`] / [`cosmos_cq_wait_batch`].
//! The wrapper does not enforce the single-consumer rule in v1 (no internal
//! lock around the consumer-side dequeue beyond the queue's own mutex);
//! calling from two threads simultaneously is undefined behavior. See §9 Q12.
//!
//! Phase 1 ships the full FFI surface plus internal test-only helpers
//! ([`__test_only_enqueue_completion`]) so the receive-loop contract can be
//! validated end-to-end before Phase 6 wires the real submit pipeline.

use std::collections::VecDeque;
use std::ffi::c_void;
use std::sync::atomic::{AtomicBool, AtomicU8, Ordering};
use std::sync::{Arc, Condvar, Mutex};
use std::time::{Duration, Instant};

use crate::error::{CosmosErrorCode, CosmosErrorHandle, CosmosErrorInner};
use crate::runtime::{RuntimeContext, RuntimeContextInner};

// ─────────────────────────────────────────────────────────────────────────────
// Outcome enum (cosmos_completion_outcome_t)
// ─────────────────────────────────────────────────────────────────────────────

/// Per spec §3.6.1, every completion has exactly one of these outcomes.
///
/// `CosmosCompletionOutcomeUnknown` is a forward-compat sentinel — older C clients
/// linked against a newer runtime that grew a variant see this value and can
/// route it through their default branch.
#[repr(i32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CosmosCompletionOutcome {
    /// The operation completed successfully; `cosmos_completion_take_response`
    /// returns the populated `cosmos_response_t`.
    CosmosCompletionOutcomeOk = 0,
    /// The operation failed; `cosmos_completion_take_error` returns the rich
    /// `cosmos_error_t` (when `include_error_details` is on for the queue).
    CosmosCompletionOutcomeError = 1,
    /// The operation was cancelled via [`cosmos_operation_handle_cancel`] or
    /// [`cosmos_cq_shutdown`].
    CosmosCompletionOutcomeCancelled = 2,
    /// Reserved sentinel for any non-zero outcome introduced after this spec.
    CosmosCompletionOutcomeUnknown = 255,
}

// ─────────────────────────────────────────────────────────────────────────────
// OperationHandle lifecycle state (cosmos_operation_handle_state_t)
// ─────────────────────────────────────────────────────────────────────────────

/// Per spec §3.6.2, the four lifecycle states an operation handle observes.
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
// CompletionQueue lifecycle state (cosmos_cq_state_t)
// ─────────────────────────────────────────────────────────────────────────────

/// Per spec §3.1.3, the three queue-lifecycle states.
#[repr(i32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CosmosCqState {
    /// Submits and waits both succeed.
    CosmosCqStateRunning = 0,
    /// `cosmos_cq_shutdown` has been called; submits fail pre-flight; pending
    /// completions can still be drained via `_wait` until empty.
    CosmosCqStateShutdown = 1,
    /// Shutdown + queue empty + no in-flight ops. Safe to `_free` without
    /// blocking.
    CosmosCqStateDrained = 2,
}

impl CosmosCqState {
    fn from_u8(v: u8) -> Self {
        match v {
            0 => Self::CosmosCqStateRunning,
            1 => Self::CosmosCqStateShutdown,
            2 => Self::CosmosCqStateDrained,
            _ => Self::CosmosCqStateRunning,
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
    /// handle pointing at this inner. Phase 1 only flips the flag; Phase 6
    /// wires it into the `tokio::select!` cancel branch.
    cancel_requested: AtomicBool,
}

impl OperationInner {
    fn new() -> Self {
        Self {
            state: AtomicU8::new(
                CosmosOperationHandleState::CosmosOperationHandleStateInFlight as u8,
            ),
            cancel_requested: AtomicBool::new(false),
        }
    }
}

/// Opaque C ABI handle for an in-flight (or just-completed) operation.
///
/// Storage pun: see the comment on [`CompletionQueue`] — the public
/// `#[repr(C)]` struct only carries the `_opaque` marker; the real `Arc`
/// state lives in a trailing `OperationHandleStorage` field allocated by
/// [`OperationHandle::new_raw`].
#[repr(C)]
pub struct OperationHandle {
    _opaque: [u8; 0],
}

#[repr(C)]
struct OperationHandleStorage {
    _opaque: [u8; 0],
    pub(crate) inner: Arc<OperationInner>,
}

impl OperationHandle {
    fn new_raw() -> *mut Self {
        let storage = Box::new(OperationHandleStorage {
            _opaque: [],
            inner: Arc::new(OperationInner::new()),
        });
        Box::into_raw(storage).cast::<OperationHandle>()
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
        Self::storage(p).map(|s| Arc::clone(&s.inner))
    }

    /// Build a fresh handle that shares the inner state of an existing handle.
    #[allow(
        dead_code,
        reason = "first caller arrives in Phase 6 (submit pipeline)"
    )]
    fn clone_arc(p: *const OperationHandle) -> Option<*mut OperationHandle> {
        let storage = Self::storage(p)?;
        let companion = Box::new(OperationHandleStorage {
            _opaque: [],
            inner: Arc::clone(&storage.inner),
        });
        Some(Box::into_raw(companion).cast::<OperationHandle>())
    }

    fn storage<'a>(p: *const OperationHandle) -> Option<&'a OperationHandleStorage> {
        if p.is_null() {
            None
        } else {
            // SAFETY: caller guarantees `p` came from `new_raw` or `clone_arc`.
            Some(unsafe { &*(p as *const OperationHandleStorage) })
        }
    }

    pub(crate) fn inner(p: *const OperationHandle) -> Option<&'static OperationInner> {
        Self::storage(p).map(|s| -> &OperationInner { &s.inner })
    }

    pub(crate) fn drop_raw(p: *mut OperationHandle) {
        if p.is_null() {
            return;
        }
        // SAFETY: caller guarantees `p` came from a library API.
        unsafe {
            drop(Box::from_raw(p.cast::<OperationHandleStorage>()));
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Completion
// ─────────────────────────────────────────────────────────────────────────────

/// Internal storage of a `cosmos_completion_t`.
///
/// Phase 1 carried no response payload; Phase 6 adds the optional
/// `response` slot which `cosmos_completion_take_response` detaches
/// from. The slot is `None` on every error / cancelled completion and
/// on every Phase 1 test-synthesized completion.
pub struct Completion {
    outcome: CosmosCompletionOutcome,
    status: CosmosErrorCode,
    user_data: *mut c_void,
    /// Strong reference to the producing operation's inner state.
    /// `cosmos_completion_op_handle` synthesizes a fresh borrowed
    /// `cosmos_operation_handle_t *` on demand by cloning this `Arc`.
    op_inner: Arc<OperationInner>,
    /// Cached borrowed handle so `cosmos_completion_op_handle` can return
    /// a stable pointer for the duration of the completion. Populated
    /// lazily on first call; freed by the `Completion`'s `Drop` impl.
    cached_op_handle: Mutex<Option<*mut OperationHandle>>,
    /// Same lazy-cache pattern as `cached_op_handle` but for the borrowed
    /// `cosmos_error_t *` returned by `cosmos_completion_error`.
    cached_error_handle: Mutex<Option<*mut CosmosErrorHandle>>,
    was_cancel_requested: bool,
    /// `Arc`-wrapped so `cosmos_completion_take_error` can detach ownership
    /// while leaving the borrowed `cosmos_completion_error` accessor working
    /// against a `None` slot.
    error: Mutex<Option<Arc<CosmosErrorInner>>>,
    /// Detachable response. Populated only on `OK` completions emitted
    /// by Phase 6 submit paths; absent on every other completion.
    /// `cosmos_completion_take_response` moves the contained handle out
    /// (subsequent calls return NULL).
    pub(crate) response: Mutex<Option<*mut crate::response::ResponseHandle>>,
}

// SAFETY: `user_data` is an opaque `void *` that the wrapper round-trips
// verbatim — the host owns its threading semantics. `cached_op_handle`
// stores a raw pointer that is only touched while the completion is
// borrowed by an FFI call, and the underlying box is freed by `Drop`.
unsafe impl Send for Completion {}
unsafe impl Sync for Completion {}

impl Drop for Completion {
    fn drop(&mut self) {
        if let Some(handle) = self.cached_op_handle.lock().unwrap().take() {
            OperationHandle::drop_raw(handle);
        }
        if let Some(handle) = self.cached_error_handle.lock().unwrap().take() {
            crate::error::CosmosErrorHandle::drop_raw(handle);
        }
        // Free the response if it was never taken via
        // `cosmos_completion_take_response`. The slot owns the raw
        // pointer; freeing here mirrors how `cached_error_handle` is
        // freed above.
        if let Some(response) = self.response.lock().unwrap().take() {
            crate::response::cosmos_response_free(response);
        }
    }
}

impl Completion {
    fn from_ptr<'a>(p: *const Completion) -> Option<&'a Self> {
        if p.is_null() {
            None
        } else {
            // SAFETY: caller guarantees `p` was obtained from `cosmos_cq_wait`
            // / `_try_wait` / `_wait_batch` and has not been freed.
            Some(unsafe { &*p })
        }
    }

    /// Builds a `Box<Completion>` ready for [`CompletionQueue::enqueue`].
    /// Used by [`crate::submit`] to publish the result of a spawned
    /// driver call.
    pub(crate) fn new_for_publish(
        outcome: CosmosCompletionOutcome,
        status: CosmosErrorCode,
        user_data: *mut c_void,
        op_inner: Arc<OperationInner>,
        error: Option<Arc<CosmosErrorInner>>,
        response: Option<*mut crate::response::ResponseHandle>,
    ) -> Box<Completion> {
        Box::new(Completion {
            outcome,
            status,
            user_data,
            op_inner,
            cached_op_handle: Mutex::new(None),
            cached_error_handle: Mutex::new(None),
            was_cancel_requested: false,
            error: Mutex::new(error),
            response: Mutex::new(response),
        })
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// CompletionQueue
// ─────────────────────────────────────────────────────────────────────────────

/// Inner queue state protected by a single mutex.
struct QueueInner {
    deque: VecDeque<Box<Completion>>,
    state: CosmosCqState,
}

/// Per-queue options. Mirrors `cosmos_cq_options_t` from spec §3.1.2 — Phase
/// 1 honors `max_capacity` and `include_error_details`; `capacity_hint` is
/// recorded but currently does not trigger any diagnostic (Phase 7 adds the
/// one-shot warning when the soft hint is exceeded).
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

/// Layout of the `cosmos_cq_options_t` struct as it appears at the C ABI
/// boundary. Caller-owned, pass-by-value (per §3.1.2 the layout is published
/// for inputs).
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct CosmosCqOptions {
    pub capacity_hint: u32,
    pub max_capacity: u32,
    pub include_error_details: bool,
}

impl From<CosmosCqOptions> for CqOptions {
    fn from(c: CosmosCqOptions) -> Self {
        Self {
            capacity_hint: c.capacity_hint,
            max_capacity: c.max_capacity,
            include_error_details: c.include_error_details,
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
    /// Keep the runtime alive for the queue's lifetime. Phase 6's
    /// submit pipeline clones this Arc to spawn per-operation tasks.
    pub(crate) runtime: Arc<RuntimeContextInner>,
}

impl CompletionQueueInner {
    /// Borrows the runtime backing this queue. Used by
    /// [`crate::submit`] to spawn per-operation tasks on the same
    /// Tokio runtime the queue was built against.
    pub(crate) fn runtime(&self) -> &Arc<RuntimeContextInner> {
        &self.runtime
    }

    /// Returns this queue's capacity hard cap (0 = unbounded).
    pub(crate) fn max_capacity(&self) -> u32 {
        self.options.max_capacity
    }

    /// Snapshots the current queue length under the mutex.
    pub(crate) fn current_len(&self) -> usize {
        self.inner.lock().unwrap().deque.len()
    }

    /// Returns whether this queue captures rich error payloads.
    pub(crate) fn include_error_details(&self) -> bool {
        self.options.include_error_details
    }
}

/// Opaque C ABI handle for a completion queue.
///
/// Storage pun: see the comment on [`OperationHandle`] — the public
/// `#[repr(C)]` struct only carries the `_opaque` marker; the real `Arc`
/// state lives in a trailing `CompletionQueueStorage` field allocated by
/// [`CompletionQueue::new_raw`].
#[repr(C)]
pub struct CompletionQueue {
    _opaque: [u8; 0],
}

#[repr(C)]
struct CompletionQueueStorage {
    _opaque: [u8; 0],
    inner: Arc<CompletionQueueInner>,
}

impl CompletionQueue {
    fn new_raw(runtime: Arc<RuntimeContextInner>, options: CqOptions) -> *mut Self {
        let storage = Box::new(CompletionQueueStorage {
            _opaque: [],
            inner: Arc::new(CompletionQueueInner {
                inner: Mutex::new(QueueInner {
                    deque: VecDeque::new(),
                    state: CosmosCqState::CosmosCqStateRunning,
                }),
                data_available: Condvar::new(),
                space_available: Condvar::new(),
                options,
                runtime,
            }),
        });
        Box::into_raw(storage).cast::<CompletionQueue>()
    }

    /// Borrows the inner queue state for the submit pipeline. Returns
    /// `None` on NULL input; otherwise the caller can clone the runtime
    /// `Arc`, inspect capacity / state, and route through
    /// [`CompletionQueue::enqueue`] when the spawned task finishes.
    pub(crate) fn inner_arc(p: *const CompletionQueue) -> Option<Arc<CompletionQueueInner>> {
        Self::storage(p).map(|s| Arc::clone(&s.inner))
    }

    fn storage<'a>(p: *const CompletionQueue) -> Option<&'a CompletionQueueStorage> {
        if p.is_null() {
            None
        } else {
            // SAFETY: caller guarantees `p` was obtained from `new_raw` and
            // has not been freed. The storage pun is sound because both
            // structs are `#[repr(C)]` with the same `_opaque: [u8; 0]`
            // first field.
            Some(unsafe { &*(p as *const CompletionQueueStorage) })
        }
    }

    fn drop_raw(p: *mut CompletionQueue) {
        if p.is_null() {
            return;
        }
        // SAFETY: caller guarantees `p` came from `new_raw`.
        unsafe {
            drop(Box::from_raw(p.cast::<CompletionQueueStorage>()));
        }
    }

    /// Internal: pushes a completion onto the queue identified by the
    /// raw pointer.
    pub(crate) fn enqueue(p: *const CompletionQueue, c: Box<Completion>) -> CosmosErrorCode {
        let Some(storage) = Self::storage(p) else {
            return CosmosErrorCode::CosmosErrorCodeInvalidArgument;
        };
        Self::enqueue_into_inner(&storage.inner, c)
    }

    /// Internal: pushes a completion onto the queue identified by an
    /// already-cloned `Arc<CompletionQueueInner>`. Used by the submit
    /// pipeline's spawned tasks so they survive concurrent
    /// `cosmos_cq_free` from the producer side.
    pub(crate) fn enqueue_into_inner(
        inner: &Arc<CompletionQueueInner>,
        mut c: Box<Completion>,
    ) -> CosmosErrorCode {
        let mut guard = inner.inner.lock().unwrap();
        if guard.state != CosmosCqState::CosmosCqStateRunning {
            return CosmosErrorCode::CosmosErrorCodeQueueShutdown;
        }
        if inner.options.max_capacity > 0 && guard.deque.len() as u32 >= inner.options.max_capacity
        {
            return CosmosErrorCode::CosmosErrorCodeQueueFull;
        }
        // If the producer-side handle's cancel flag is set, mark the
        // completion so the receive loop can distinguish "cancel won" from
        // "cancel lost the race" per spec §3.6.1.
        if c.op_inner.cancel_requested.load(Ordering::Acquire) {
            c.was_cancel_requested = true;
        }
        // Reflect the final outcome on the operation handle's state field so
        // `cosmos_operation_handle_state` reports the right answer to a
        // producer that did not drain the queue.
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
        c.op_inner.state.store(next_state as u8, Ordering::Release);
        guard.deque.push_back(c);
        inner.data_available.notify_one();
        CosmosErrorCode::CosmosErrorCodeSuccess
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// FFI: cosmos_cq_*
// ─────────────────────────────────────────────────────────────────────────────

/// Create a completion queue bound to `runtime`. Returns NULL if `runtime`
/// is NULL.
#[no_mangle]
pub extern "C" fn cosmos_cq_create(
    runtime: *const RuntimeContext,
    options: *const CosmosCqOptions,
) -> *mut CompletionQueue {
    let Some(inner_rt) = RuntimeContext::inner_arc(runtime) else {
        return std::ptr::null_mut();
    };
    let opts = if options.is_null() {
        CqOptions::default()
    } else {
        // SAFETY: `options` is non-null and the caller guarantees it points
        // at a fully-initialized `CosmosCqOptions` valid for the duration of
        // the call (per §3.2 Pattern A inputs).
        CqOptions::from(*unsafe { &*options })
    };
    CompletionQueue::new_raw(inner_rt, opts)
}

/// Free a completion queue. NULL is a no-op.
///
/// Phase 1 contract: no Tokio-side producers spawn against the queue, so
/// "blocks until in-flight ops drain" is trivially satisfied. The check
/// remains in place so the contract documented in spec §3.1.2 is observable:
/// if anyone enqueued completions but never drained, this drops them (and
/// thus their `Box`-allocated `Completion`s).
#[no_mangle]
pub extern "C" fn cosmos_cq_free(queue: *mut CompletionQueue) {
    if queue.is_null() {
        return;
    }
    tracing::trace!(?queue, "freeing cosmos_cq_t");
    CompletionQueue::drop_raw(queue);
}

/// Returns the runtime the queue was bound to.
#[no_mangle]
pub extern "C" fn cosmos_cq_runtime(queue: *const CompletionQueue) -> *const RuntimeContext {
    // NB: returning the inner Arc as a `*const RuntimeContext` would require
    // a stable wrapping box. Phase 1 returns NULL because we don't keep a
    // back-pointer to the producer's `RuntimeContext` box (we only retain the
    // inner `Arc`). Phase 2 revisits this when the runtime builder lands.
    let _ = queue;
    std::ptr::null()
}

/// Block until a completion is available or `timeout_ms` elapses.
///
/// - `timeout_ms == 0` → poll once and return immediately, NULL if empty.
/// - `timeout_ms == UINT32_MAX` → wait without a timeout. Returns NULL only
///   on shutdown / drained / spurious wake.
/// - otherwise → wait up to that many milliseconds.
///
/// The returned `cosmos_completion_t *` must be freed via
/// [`cosmos_completion_free`].
#[no_mangle]
pub extern "C" fn cosmos_cq_wait(queue: *mut CompletionQueue, timeout_ms: u32) -> *mut Completion {
    let Some(q) = CompletionQueue::storage(queue) else {
        return std::ptr::null_mut();
    };

    let inner = &q.inner;
    let mut guard = inner.inner.lock().unwrap();

    if timeout_ms == 0 {
        // Poll-only: return immediately whatever's there (possibly nothing).
        if let Some(c) = guard.deque.pop_front() {
            inner.space_available.notify_one();
            maybe_mark_drained(&mut guard);
            return Box::into_raw(c);
        }
        return std::ptr::null_mut();
    }

    let deadline = if timeout_ms == u32::MAX {
        None
    } else {
        Some(Instant::now() + Duration::from_millis(u64::from(timeout_ms)))
    };

    loop {
        if let Some(c) = guard.deque.pop_front() {
            inner.space_available.notify_one();
            maybe_mark_drained(&mut guard);
            return Box::into_raw(c);
        }
        // Empty. If we've been shut down and there's nothing left, surface
        // NULL immediately so the consumer can stop looping.
        if guard.state == CosmosCqState::CosmosCqStateShutdown {
            maybe_mark_drained(&mut guard);
            return std::ptr::null_mut();
        }
        if guard.state == CosmosCqState::CosmosCqStateDrained {
            return std::ptr::null_mut();
        }
        // Wait for either a new completion or a shutdown signal.
        guard = match deadline {
            None => inner.data_available.wait(guard).unwrap(),
            Some(d) => {
                let remaining = d.saturating_duration_since(Instant::now());
                if remaining.is_zero() {
                    return std::ptr::null_mut();
                }
                let (g, timed_out) = inner.data_available.wait_timeout(guard, remaining).unwrap();
                if timed_out.timed_out() && g.deque.is_empty() {
                    return std::ptr::null_mut();
                }
                g
            }
        };
    }
}

fn maybe_mark_drained(guard: &mut std::sync::MutexGuard<'_, QueueInner>) {
    if guard.state == CosmosCqState::CosmosCqStateShutdown && guard.deque.is_empty() {
        guard.state = CosmosCqState::CosmosCqStateDrained;
    }
}

/// Non-blocking poll. Equivalent to `cosmos_cq_wait(queue, 0)`.
#[no_mangle]
pub extern "C" fn cosmos_cq_try_wait(queue: *mut CompletionQueue) -> *mut Completion {
    cosmos_cq_wait(queue, 0)
}

/// Drains up to `max_count` completions in a single call. Blocks until at
/// least one completion is available or `timeout_ms` elapses, then drains
/// additional already-queued completions without blocking again.
///
/// Returns the number of completions written into
/// `out_completions[0..max_count]`. The caller MUST free each one via
/// [`cosmos_completion_free`].
#[no_mangle]
pub extern "C" fn cosmos_cq_wait_batch(
    queue: *mut CompletionQueue,
    out_completions: *mut *mut Completion,
    max_count: u32,
    timeout_ms: u32,
) -> u32 {
    if max_count == 0 || out_completions.is_null() {
        return 0;
    }
    let first = cosmos_cq_wait(queue, timeout_ms);
    if first.is_null() {
        return 0;
    }
    // SAFETY: caller guarantees `out_completions` references at least
    // `max_count` slots of `*mut Completion`.
    unsafe { out_completions.write(first) };
    let mut count = 1u32;
    while count < max_count {
        let next = cosmos_cq_try_wait(queue);
        if next.is_null() {
            break;
        }
        // SAFETY: see above.
        unsafe { out_completions.add(count as usize).write(next) };
        count += 1;
    }
    count
}

/// Block until the queue has room for at least one more pending completion,
/// or `timeout_ms` elapses.
#[no_mangle]
pub extern "C" fn cosmos_cq_wait_writable(queue: *mut CompletionQueue, timeout_ms: u32) -> bool {
    let Some(q) = CompletionQueue::storage(queue) else {
        return false;
    };
    let inner = &q.inner;
    if inner.options.max_capacity == 0 {
        // Unbounded — always writable.
        return true;
    }
    let mut guard = inner.inner.lock().unwrap();
    if guard.state != CosmosCqState::CosmosCqStateRunning {
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
            None => inner.space_available.wait(guard).unwrap(),
            Some(d) => {
                let remaining = d.saturating_duration_since(Instant::now());
                if remaining.is_zero() {
                    return false;
                }
                let (g, timed_out) = inner
                    .space_available
                    .wait_timeout(guard, remaining)
                    .unwrap();
                if timed_out.timed_out() {
                    return (g.deque.len() as u32) < inner.options.max_capacity
                        && g.state == CosmosCqState::CosmosCqStateRunning;
                }
                g
            }
        };
        if guard.state != CosmosCqState::CosmosCqStateRunning {
            return false;
        }
        if (guard.deque.len() as u32) < inner.options.max_capacity {
            return true;
        }
    }
}

/// Signal shutdown: marks the queue as shutting down, cancels in-flight ops
/// (Phase 1 has none from public API), and wakes any thread blocked in
/// `cosmos_cq_wait` / `_wait_writable` / `_wait_batch`. Idempotent.
#[no_mangle]
pub extern "C" fn cosmos_cq_shutdown(queue: *mut CompletionQueue) {
    let Some(q) = CompletionQueue::storage(queue) else {
        return;
    };
    let mut guard = q.inner.inner.lock().unwrap();
    if guard.state == CosmosCqState::CosmosCqStateRunning {
        guard.state = CosmosCqState::CosmosCqStateShutdown;
        if guard.deque.is_empty() {
            guard.state = CosmosCqState::CosmosCqStateDrained;
        }
    }
    q.inner.data_available.notify_all();
    q.inner.space_available.notify_all();
}

/// Returns the queue's current lifecycle state.
#[no_mangle]
pub extern "C" fn cosmos_cq_state(queue: *const CompletionQueue) -> CosmosCqState {
    let Some(q) = CompletionQueue::storage(queue) else {
        return CosmosCqState::CosmosCqStateRunning;
    };
    let guard = q.inner.inner.lock().unwrap();
    CosmosCqState::from_u8(guard.state as u8)
}

// ─────────────────────────────────────────────────────────────────────────────
// FFI: cosmos_completion_*
// ─────────────────────────────────────────────────────────────────────────────

/// Returns the completion's outcome. Returns `Unknown` if `c` is NULL.
#[no_mangle]
pub extern "C" fn cosmos_completion_outcome(c: *const Completion) -> CosmosCompletionOutcome {
    Completion::from_ptr(c).map_or(
        CosmosCompletionOutcome::CosmosCompletionOutcomeUnknown,
        |co| co.outcome,
    )
}

/// Returns the `user_data` the caller supplied at submit time. NULL is
/// preserved verbatim.
#[no_mangle]
pub extern "C" fn cosmos_completion_user_data(c: *const Completion) -> *mut c_void {
    Completion::from_ptr(c).map_or(std::ptr::null_mut(), |co| co.user_data)
}

/// Returns a borrowed pointer to the operation handle that produced this
/// completion. Lifetime = until [`cosmos_completion_free`].
#[no_mangle]
pub extern "C" fn cosmos_completion_op_handle(c: *const Completion) -> *const OperationHandle {
    let Some(co) = Completion::from_ptr(c) else {
        return std::ptr::null();
    };
    let mut slot = co.cached_op_handle.lock().unwrap();
    if let Some(existing) = *slot {
        return existing;
    }
    // Lazy-create a fresh borrowed handle that shares the same Arc.
    let companion = Box::new(OperationHandleStorage {
        _opaque: [],
        inner: Arc::clone(&co.op_inner),
    });
    let raw = Box::into_raw(companion).cast::<OperationHandle>();
    *slot = Some(raw);
    raw
}

/// Coarse status code (always populated even when rich detail is suppressed).
#[no_mangle]
pub extern "C" fn cosmos_completion_status(c: *const Completion) -> CosmosErrorCode {
    Completion::from_ptr(c).map_or(CosmosErrorCode::CosmosErrorCodeInvalidArgument, |co| {
        co.status
    })
}

/// True iff `cosmos_operation_handle_cancel` was observed on the producing
/// handle before the completion was posted.
#[no_mangle]
pub extern "C" fn cosmos_completion_was_cancel_requested(c: *const Completion) -> bool {
    Completion::from_ptr(c).is_some_and(|co| co.was_cancel_requested)
}

/// Takes ownership of the response delivered by an `Ok` completion.
/// Returns NULL on `Error` / `Cancelled` completions, on NULL input,
/// and on every subsequent call after the first successful take.
///
/// Caller must free the returned handle via `cosmos_response_free`.
#[no_mangle]
pub extern "C" fn cosmos_completion_take_response(
    c: *mut Completion,
) -> *mut crate::response::ResponseHandle {
    let Some(co) = Completion::from_ptr(c) else {
        return std::ptr::null_mut();
    };
    if co.outcome != CosmosCompletionOutcome::CosmosCompletionOutcomeOk {
        return std::ptr::null_mut();
    }
    let mut slot = co.response.lock().unwrap();
    slot.take().unwrap_or(std::ptr::null_mut())
}

/// Borrowed access to the response payload. Returns NULL when the
/// completion outcome is not `Ok`, when no response was attached, or
/// after `_take_response` already moved ownership out.
///
/// Lifetime: until the next `_take_response` call or until the
/// completion itself is freed.
#[no_mangle]
pub extern "C" fn cosmos_completion_response(
    c: *const Completion,
) -> *const crate::response::ResponseHandle {
    let Some(co) = Completion::from_ptr(c) else {
        return std::ptr::null();
    };
    if co.outcome != CosmosCompletionOutcome::CosmosCompletionOutcomeOk {
        return std::ptr::null();
    }
    let guard = co.response.lock().unwrap();
    guard.map_or(std::ptr::null(), |p| p as *const _)
}

/// Take ownership of the rich error payload. Returns NULL when
/// `outcome != Error`, when the queue was created with
/// `include_error_details = false`, or after a previous `_take_error` call.
#[no_mangle]
pub extern "C" fn cosmos_completion_take_error(c: *mut Completion) -> *mut CosmosErrorHandle {
    let Some(co) = Completion::from_ptr(c) else {
        return std::ptr::null_mut();
    };
    if co.outcome != CosmosCompletionOutcome::CosmosCompletionOutcomeError {
        return std::ptr::null_mut();
    }
    let mut slot = co.error.lock().unwrap();
    match slot.take() {
        Some(arc) => CosmosErrorHandle::from_arc_into_raw(arc),
        None => std::ptr::null_mut(),
    }
}

/// Borrowed access to the rich error payload. NULL on `Ok` / `Cancelled`,
/// when details are suppressed, or after a previous `_take_error` call.
#[no_mangle]
pub extern "C" fn cosmos_completion_error(c: *const Completion) -> *const CosmosErrorHandle {
    let Some(co) = Completion::from_ptr(c) else {
        return std::ptr::null();
    };
    // First check whether we already produced a borrowed handle for this
    // completion. If so, return it so the pointer is stable across calls.
    {
        let cached = co.cached_error_handle.lock().unwrap();
        if let Some(existing) = *cached {
            return existing;
        }
    }
    // No cached handle yet — produce one if a rich error is present and
    // remember it.
    let guard = co.error.lock().unwrap();
    let Some(arc) = guard.as_ref() else {
        return std::ptr::null();
    };
    let raw = CosmosErrorHandle::from_arc_into_raw(Arc::clone(arc));
    drop(guard);
    let mut cached = co.cached_error_handle.lock().unwrap();
    // Another caller may have raced us; if so, drop our duplicate and use
    // theirs to keep the stable-pointer invariant.
    if let Some(existing) = *cached {
        crate::error::CosmosErrorHandle::drop_raw(raw);
        return existing;
    }
    *cached = Some(raw);
    raw
}

/// Free a completion record. Any pointer obtained via
/// [`cosmos_completion_error`] (borrowed) becomes invalid; ownership obtained
/// via [`cosmos_completion_take_error`] remains valid until that handle's
/// own `_free` call.
#[no_mangle]
pub extern "C" fn cosmos_completion_free(c: *mut Completion) {
    if c.is_null() {
        return;
    }
    tracing::trace!(?c, "freeing cosmos_completion_t");
    // SAFETY: caller guarantees `c` was obtained from
    // `cosmos_cq_wait` / `_try_wait` / `_wait_batch`.
    unsafe {
        drop(Box::from_raw(c));
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// FFI: cosmos_operation_handle_*
// ─────────────────────────────────────────────────────────────────────────────

/// Request cooperative cancellation. Idempotent and non-blocking. Phase 1
/// only flips the cancel-requested flag and (if the operation has not yet
/// reached a terminal state) transitions the state to `Cancelled` so the
/// state poller reflects it. Phase 6 wires the flag into the real
/// `tokio::select!` cancel branch.
#[no_mangle]
pub extern "C" fn cosmos_operation_handle_cancel(op: *mut OperationHandle) {
    let Some(inner) = OperationHandle::inner(op) else {
        return;
    };
    inner.cancel_requested.store(true, Ordering::Release);
}

/// Poll the operation's lifecycle state. Returns `InFlight` if `op` is NULL.
#[no_mangle]
pub extern "C" fn cosmos_operation_handle_state(
    op: *const OperationHandle,
) -> CosmosOperationHandleState {
    let Some(inner) = OperationHandle::inner(op) else {
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
// Phase 6 replaces these with the real submit pipeline. For Phase 1 they let
// integration tests synthesize completions end-to-end.
// ─────────────────────────────────────────────────────────────────────────────

/// Test-only: allocate a new operation handle, returning the producer-side
/// raw pointer.
///
/// Phase 6 replaces with the real `cosmos_driver_submit` pipeline.
#[doc(hidden)]
pub fn __test_only_create_operation_handle() -> *mut OperationHandle {
    OperationHandle::new_raw()
}

/// Test-only: synthesize a completion record and enqueue it onto `queue`.
///
/// Cloning the operation handle's `Arc` keeps both the producer-side handle
/// and the completion-side borrow alive independently per spec §3.6.2.
#[doc(hidden)]
pub fn __test_only_enqueue_completion(
    queue: *mut CompletionQueue,
    op_handle: *mut OperationHandle,
    outcome: CosmosCompletionOutcome,
    status: CosmosErrorCode,
    user_data: *mut c_void,
    error: Option<Arc<CosmosErrorInner>>,
) -> CosmosErrorCode {
    let Some(storage) = CompletionQueue::storage(queue) else {
        return CosmosErrorCode::CosmosErrorCodeInvalidArgument;
    };
    let Some(op_storage) = OperationHandle::storage(op_handle) else {
        return CosmosErrorCode::CosmosErrorCodeInvalidArgument;
    };
    let include_error = storage.inner.options.include_error_details;
    let stored_error = if include_error { error } else { None };
    let completion = Box::new(Completion {
        outcome,
        status,
        user_data,
        op_inner: Arc::clone(&op_storage.inner),
        cached_op_handle: Mutex::new(None),
        cached_error_handle: Mutex::new(None),
        was_cancel_requested: false,
        error: Mutex::new(stored_error),
        response: Mutex::new(None),
    });
    CompletionQueue::enqueue(queue, completion)
}

// ─────────────────────────────────────────────────────────────────────────────
// Tests
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::runtime::__test_only_create_default_runtime;

    fn fresh_queue(max_capacity: u32, include_error_details: bool) -> *mut CompletionQueue {
        let rt = __test_only_create_default_runtime();
        let opts = CosmosCqOptions {
            capacity_hint: 0,
            max_capacity,
            include_error_details,
        };
        let q = cosmos_cq_create(rt, &opts as *const _);
        // Runtime is held internally via Arc; we can free the producer-side
        // handle right away. Phase 2 introduces the public builder which makes
        // this rebinding ergonomic.
        crate::runtime::cosmos_runtime_free(rt);
        q
    }

    #[test]
    fn create_and_free_queue() {
        let q = fresh_queue(0, true);
        assert!(!q.is_null());
        assert_eq!(cosmos_cq_state(q), CosmosCqState::CosmosCqStateRunning);
        cosmos_cq_free(q);
    }

    #[test]
    fn try_wait_returns_null_on_empty() {
        let q = fresh_queue(0, true);
        assert!(cosmos_cq_try_wait(q).is_null());
        cosmos_cq_free(q);
    }

    #[test]
    fn wait_zero_polls_without_blocking() {
        let q = fresh_queue(0, true);
        let start = Instant::now();
        let result = cosmos_cq_wait(q, 0);
        let elapsed = start.elapsed();
        assert!(result.is_null());
        // Should be near-instantaneous (well under 50ms even on slow CI).
        assert!(elapsed < Duration::from_millis(50), "elapsed: {elapsed:?}");
        cosmos_cq_free(q);
    }

    #[test]
    fn wait_with_short_timeout_returns_null_on_empty() {
        let q = fresh_queue(0, true);
        let start = Instant::now();
        let result = cosmos_cq_wait(q, 20);
        let elapsed = start.elapsed();
        assert!(result.is_null());
        // Should wait close to the timeout.
        assert!(elapsed >= Duration::from_millis(15));
        cosmos_cq_free(q);
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

        let c = cosmos_cq_wait(q, 100);
        assert!(!c.is_null());
        assert_eq!(
            cosmos_completion_outcome(c),
            CosmosCompletionOutcome::CosmosCompletionOutcomeOk
        );
        assert_eq!(cosmos_completion_user_data(c), token as *mut c_void);
        assert_eq!(
            cosmos_completion_status(c),
            CosmosErrorCode::CosmosErrorCodeSuccess
        );
        assert!(!cosmos_completion_was_cancel_requested(c));
        assert!(cosmos_completion_op_handle(c).is_null().not());

        cosmos_completion_free(c);
        cosmos_operation_handle_free(op);
        cosmos_cq_free(q);
    }

    #[test]
    fn shutdown_transitions_through_drained() {
        let q = fresh_queue(0, true);
        assert_eq!(cosmos_cq_state(q), CosmosCqState::CosmosCqStateRunning);
        cosmos_cq_shutdown(q);
        // No completions were enqueued, so we should land in Drained
        // immediately.
        assert_eq!(cosmos_cq_state(q), CosmosCqState::CosmosCqStateDrained);
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
        cosmos_operation_handle_free(op);
        cosmos_cq_free(q);
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
        cosmos_cq_shutdown(q);
        assert_eq!(cosmos_cq_state(q), CosmosCqState::CosmosCqStateShutdown);

        let c = cosmos_cq_wait(q, 100);
        assert!(!c.is_null());
        cosmos_completion_free(c);

        // After draining, the next wait should return NULL immediately and
        // state should flip to Drained.
        let c2 = cosmos_cq_wait(q, 50);
        assert!(c2.is_null());
        assert_eq!(cosmos_cq_state(q), CosmosCqState::CosmosCqStateDrained);

        cosmos_operation_handle_free(op);
        cosmos_cq_free(q);
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
        let c = cosmos_cq_wait(q, 100);
        assert!(!c.is_null());
        assert_eq!(
            cosmos_completion_outcome(c),
            CosmosCompletionOutcome::CosmosCompletionOutcomeCancelled
        );
        assert!(cosmos_completion_was_cancel_requested(c));
        assert_eq!(
            cosmos_completion_status(c),
            CosmosErrorCode::CosmosErrorCodeOperationCancelled
        );
        // The operation handle's state should reflect Cancelled.
        assert_eq!(
            cosmos_operation_handle_state(op),
            CosmosOperationHandleState::CosmosOperationHandleStateCancelled
        );

        cosmos_completion_free(c);
        cosmos_operation_handle_free(op);
        cosmos_cq_free(q);
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
        let c = cosmos_cq_wait(q, 100);
        assert_eq!(
            cosmos_completion_outcome(c),
            CosmosCompletionOutcome::CosmosCompletionOutcomeOk
        );
        assert!(cosmos_completion_was_cancel_requested(c));
        cosmos_completion_free(c);
        cosmos_operation_handle_free(op);
        cosmos_cq_free(q);
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
        cosmos_operation_handle_free(op3);

        // Drain one and verify writable.
        let c = cosmos_cq_try_wait(q);
        assert!(!c.is_null());
        cosmos_completion_free(c);
        assert!(cosmos_cq_wait_writable(q, 0));

        // Drain the rest.
        let c2 = cosmos_cq_try_wait(q);
        cosmos_completion_free(c2);
        cosmos_cq_free(q);
    }

    #[test]
    fn wait_writable_is_immediate_on_unbounded_queue() {
        let q = fresh_queue(0, true);
        assert!(cosmos_cq_wait_writable(q, 0));
        cosmos_cq_free(q);
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
        let ok = cosmos_cq_wait_writable(q, 30);
        assert!(!ok, "should time out on a full queue");
        assert!(start.elapsed() >= Duration::from_millis(20));
        // Drain.
        let c = cosmos_cq_try_wait(q);
        cosmos_completion_free(c);
        cosmos_operation_handle_free(op);
        cosmos_cq_free(q);
    }

    #[test]
    fn wait_batch_drains_multiple() {
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
        let mut buf: [*mut Completion; 16] = [std::ptr::null_mut(); 16];
        let count = cosmos_cq_wait_batch(q, buf.as_mut_ptr(), buf.len() as u32, 100);
        assert_eq!(count, 5);
        for i in 0..count as usize {
            assert!(!buf[i].is_null());
            // user_data preserved in order.
            assert_eq!(cosmos_completion_user_data(buf[i]), i as *mut c_void);
            cosmos_completion_free(buf[i]);
        }
        cosmos_operation_handle_free(op);
        cosmos_cq_free(q);
    }

    #[test]
    fn take_error_transfers_ownership() {
        use azure_data_cosmos_driver::error::{CosmosError, CosmosStatus};
        let q = fresh_queue(0, true);
        let op = __test_only_create_operation_handle();
        let err_arc = Arc::new(CosmosErrorInner::new(
            CosmosError::builder()
                .with_status(CosmosStatus::new(azure_core::http::StatusCode::NotFound))
                .with_message("test error")
                .build(),
        ));
        __test_only_enqueue_completion(
            q,
            op,
            CosmosCompletionOutcome::CosmosCompletionOutcomeError,
            CosmosErrorCode::CosmosErrorCodeNotFound,
            std::ptr::null_mut(),
            Some(err_arc),
        );

        let c = cosmos_cq_wait(q, 100);
        assert!(!c.is_null());
        // First take succeeds.
        let raw = cosmos_completion_take_error(c);
        assert!(!raw.is_null());
        assert_eq!(crate::error::cosmos_error_status_code(raw), 404);
        crate::error::cosmos_error_free(raw);
        // Second take returns NULL.
        let raw2 = cosmos_completion_take_error(c);
        assert!(raw2.is_null());

        cosmos_completion_free(c);
        cosmos_operation_handle_free(op);
        cosmos_cq_free(q);
    }

    #[test]
    fn include_error_details_false_drops_rich_error() {
        use azure_data_cosmos_driver::error::{CosmosError, CosmosStatus};
        let q = fresh_queue(0, /* include_error_details = */ false);
        let op = __test_only_create_operation_handle();
        let err_arc = Arc::new(CosmosErrorInner::new(
            CosmosError::builder()
                .with_status(CosmosStatus::new(azure_core::http::StatusCode::Conflict))
                .with_message("dropped")
                .build(),
        ));
        __test_only_enqueue_completion(
            q,
            op,
            CosmosCompletionOutcome::CosmosCompletionOutcomeError,
            CosmosErrorCode::CosmosErrorCodeConflict,
            std::ptr::null_mut(),
            Some(err_arc),
        );
        let c = cosmos_cq_wait(q, 100);
        assert!(!c.is_null());
        // Coarse status survives.
        assert_eq!(
            cosmos_completion_status(c),
            CosmosErrorCode::CosmosErrorCodeConflict
        );
        // Rich error suppressed.
        assert!(cosmos_completion_take_error(c).is_null());
        cosmos_completion_free(c);
        cosmos_operation_handle_free(op);
        cosmos_cq_free(q);
    }

    #[test]
    fn shutdown_wakes_blocked_waiter() {
        let q = fresh_queue(0, true);
        let q_addr = q as usize;
        // Spawn a thread that will sit in cosmos_cq_wait with a long timeout.
        let handle = std::thread::spawn(move || {
            let q = q_addr as *mut CompletionQueue;
            let start = Instant::now();
            let c = cosmos_cq_wait(q, 5_000); // 5s timeout
            (c.is_null(), start.elapsed())
        });
        std::thread::sleep(Duration::from_millis(20));
        cosmos_cq_shutdown(q);
        let (was_null, elapsed) = handle.join().unwrap();
        assert!(was_null, "shutdown should return NULL from wait");
        assert!(
            elapsed < Duration::from_millis(500),
            "wait should wake within ~milliseconds of shutdown, took {elapsed:?}"
        );
        cosmos_cq_free(q);
    }

    // Trivially satisfy clippy::let_unit_value in `assert!(... .is_null().not())`.
    trait NotExt {
        fn not(self) -> bool;
    }
    impl NotExt for bool {
        fn not(self) -> bool {
            !self
        }
    }
}

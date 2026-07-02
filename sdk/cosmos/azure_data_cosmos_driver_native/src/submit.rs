// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Generic submit pipeline.
//!
//! The two canonical operation entry points
//! ([`cosmos_submit_operation`] /
//! [`cosmos_submit_singleton_operation`]), plus the async
//! driver-creation paths ([`cosmos_driver_get_or_create_submit`],
//! [`cosmos_driver_resolve_container_submit`]) all share the same
//! shape:
//!
//! 1. Borrow the runtime, queue, and required handles.
//! 2. Allocate a fresh producer-side `cosmos_operation_handle_t *` and
//!    take a strong reference to its inner state.
//! 3. `tokio::spawn` a task that runs the driver-side async work and,
//!    when it completes, publishes a `Completion` to the queue.
//! 4. Return the producer-side handle (or NULL + a coarse code in
//!    `out_pre_error` on pre-flight failure).
//!
//! The common machinery is internal (`SpawnContext` + `spawn_oneshot`);
//! the per-API entry points are thin wrappers that provide the
//! driver-side future.

use std::future::Future;
use std::sync::Arc;

use azure_data_cosmos_driver::driver::CosmosDriver;
use azure_data_cosmos_driver::models::{AccountReference, ContainerReference, CosmosResponse};
use azure_data_cosmos_driver::options::DriverOptions;

use crate::account_ref::AccountRefHandle;
use crate::completion::{
    CompletionQueue, CompletionQueueInner, OperationHandle, OperationInner, PendingCompletion,
};
use crate::driver::DriverHandle;
use crate::driver_options::DriverOptionsHandle;
use crate::error::CosmosErrorCode;
use crate::op_request::{build_request, CosmosOperationRequest};
use crate::runtime::RuntimeContext;

/// Send-safe encoding of the opaque `user_data` cookie round-tripped
/// to the completion. It is a pointer-sized integer (`isize`/`intptr_t`),
/// never a pointer the wrapper dereferences — the host owns its meaning
/// and threading semantics, so storing it as an integer also keeps the
/// async-block auto-trait analysis from flagging it as `!Send`.
#[derive(Clone, Copy)]
struct UserData(isize);

impl UserData {
    fn new(v: isize) -> Self {
        Self(v)
    }
    fn as_isize(self) -> isize {
        self.0
    }
}

/// Resources captured into every spawned submit task.
///
/// Holding `Arc<CompletionQueueInner>` and `Arc<OperationInner>`
/// directly lets the spawned task survive a concurrent `cosmos_completion_queue_free`
/// or `cosmos_operation_handle_free` from the producer side — the
/// queue's deque drops back-pressured completions cleanly, and the
/// operation-handle state stays consistent through the existing
/// `Arc` plumbing.
struct SpawnContext {
    queue: Arc<CompletionQueueInner>,
    op_inner: Arc<OperationInner>,
    user_data: UserData,
    include_error_details: bool,
}

// SAFETY: `UserData` is `isize`-backed, the other fields are `Arc`s.
unsafe impl Send for SpawnContext {}

/// The success payload a submit task produces, mapped into a completion.
enum SuccessKind {
    /// A CRUD / feed response page (or a degenerate end-of-feed shell when
    /// `response` is `None`) plus an optional planner next-page token. The
    /// response is boxed to keep this enum small (it is otherwise dominated by
    /// the ~700-byte `CosmosResponse`).
    Response {
        response: Option<Box<CosmosResponse>>,
        next_continuation: Option<String>,
    },
    /// A freshly-created driver from `get_or_create`.
    Driver(Arc<DriverHandle>),
    /// A resolved container from `resolve_container`. Boxed to keep this enum
    /// small (the driver `ContainerReference` is otherwise the largest variant).
    Container(Box<ContainerReference>),
}

/// Pre-flight: builds a [`SpawnContext`] + a fresh producer-side
/// `cosmos_operation_handle_t *`. Returns `Err(coarse_code)` on
/// validation failure so the caller can write the coarse code into
/// `out_pre_error` and return NULL.
fn pre_flight_spawn(
    queue: *mut CompletionQueue,
    user_data: isize,
) -> Result<(SpawnContext, *mut OperationHandle), CosmosErrorCode> {
    let Some(queue_inner) = CompletionQueue::inner_arc(queue) else {
        return Err(CosmosErrorCode::CosmosErrorCodeInvalidArgument);
    };
    // Atomically validate the queue is accepting work and reserve an
    // in-flight slot for this op's whole submit → run → enqueue → drain
    // lifecycle. Reserving here (rather than only checking deque length and
    // state separately) keeps the queue's in-flight count accurate, so a
    // shutdown that races this still-running op cannot transition the queue
    // to DRAINED before the op enqueues its completion — which would strand
    // the operation handle as `IN_FLIGHT` forever and leak the host
    // continuation. The reservation is released on drain, or on the abort
    // path below.
    queue_inner.reserve_in_flight()?;

    let op_handle = OperationHandle::allocate();
    let op_inner = match OperationHandle::inner_arc(op_handle) {
        Some(a) => a,
        None => {
            // Should not happen — `allocate` returns non-NULL. Release the
            // reservation we just took so the queue can still reach DRAINED.
            OperationHandle::drop_raw(op_handle);
            queue_inner.release_in_flight();
            return Err(CosmosErrorCode::CosmosErrorCodeInvalidArgument);
        }
    };
    let include_error_details = queue_inner.include_error_details();
    Ok((
        SpawnContext {
            queue: queue_inner,
            op_inner,
            user_data: UserData::new(user_data),
            include_error_details,
        },
        op_handle,
    ))
}

/// Routes one driver-side `Future<Output = Result<R, CosmosError>>`
/// through the standard submit-and-publish pipeline. `to_success`
/// converts the success value into the [`SuccessKind`] the completion
/// delivers; on `Err`, the rich `CosmosError` is flattened inline into the
/// completion (subject to the queue's `include_error_details` option) plus
/// the coarse code.
fn spawn_oneshot<Fut, R>(
    ctx: SpawnContext,
    runtime: Arc<crate::runtime::RuntimeContext>,
    fut: Fut,
    to_success: impl FnOnce(R) -> SuccessKind + Send + 'static,
) where
    Fut: Future<Output = azure_data_cosmos_driver::error::Result<R>> + Send + 'static,
    R: Send + 'static,
{
    use futures::future::FutureExt;

    runtime.tokio.spawn(async move {
        // Run the driver work (and the success-path conversion) behind a panic
        // firewall. Tokio would otherwise isolate a panic at the task boundary,
        // so `enqueue_into_inner` below would never run and the operation handle
        // would stay `IN_FLIGHT` forever — the host continuation hangs and
        // leaks. Catching the panic here lets us still publish exactly one
        // completion, honoring the spec section 3.6 invariant.
        let work = std::panic::AssertUnwindSafe(async move { fut.await.map(to_success) });
        let work = work.catch_unwind();
        tokio::pin!(work);

        // Race the driver work against a cancel signal. `notify_one` (in
        // `cosmos_operation_handle_cancel`) stores a permit, so a cancel that
        // arrives before this task starts waiting is still observed on the
        // first poll of `notified()`. `biased` makes a pending cancel win
        // deterministically over a simultaneously-ready result. On cancel we
        // simply stop awaiting `work`; dropping it cancels the driver future
        // (best-effort cooperative cancellation per spec section 3.6.3).
        let outcome = tokio::select! {
            biased;
            _ = ctx.op_inner.cancel_notify.notified() => None,
            done = &mut work => Some(done),
        };

        let user_data = ctx.user_data.as_isize();
        let completion = match outcome {
            // Cancelled: drop the driver future and synthesize a CANCELLED
            // completion so the host's continuation is released.
            None => PendingCompletion::cancelled(user_data, ctx.op_inner.clone()),
            Some(Ok(Ok(success))) => match success {
                SuccessKind::Response {
                    response,
                    next_continuation,
                } => PendingCompletion::ok_response(
                    user_data,
                    ctx.op_inner.clone(),
                    response.map(|b| *b),
                    next_continuation,
                ),
                SuccessKind::Driver(driver) => {
                    PendingCompletion::ok_driver(user_data, ctx.op_inner.clone(), driver)
                }
                SuccessKind::Container(container) => {
                    PendingCompletion::ok_container(user_data, ctx.op_inner.clone(), *container)
                }
            },
            Some(Ok(Err(err))) => {
                let coarse = CosmosErrorCode::from_driver_error(&err);
                PendingCompletion::error(
                    user_data,
                    ctx.op_inner.clone(),
                    err,
                    coarse,
                    ctx.include_error_details,
                )
            }
            Some(Err(_panic)) => {
                // The driver future (or success conversion) panicked. Surface it
                // as a coarse client-side error so the host's continuation is
                // released with a definitive failure instead of hanging.
                tracing::error!("submit: driver future panicked; synthesizing ERROR completion",);
                PendingCompletion::error_coarse(
                    user_data,
                    ctx.op_inner.clone(),
                    CosmosErrorCode::CosmosErrorCodeClientError,
                )
            }
        };
        let rc = CompletionQueue::enqueue_into_inner(&ctx.queue, completion);
        if rc != CosmosErrorCode::CosmosErrorCodeSuccess {
            // The queue rejected the completion (shutdown / full). The
            // completion is dropped — its owned payloads (if any) are freed by
            // its `Drop`.
            tracing::warn!(
                rc = ?rc,
                "submit: completion dropped (queue shutdown or full)",
            );
        }
    });
}

// ─────────────────────────────────────────────────────────────────────────────
// FFI: cosmos_submit_operation / cosmos_submit_singleton_operation
//
// The two canonical entry points host SDKs use. The host fills a flat
// `cosmos_operation_request_t` (kind + references + body + per-op tweaks +
// options) and submits it; the wrapper builds the driver's `CosmosOperation`
// + `OperationOptions` internally and dispatches to the matching driver
// method. These supersede the per-operation factory / mutator surface.
// ─────────────────────────────────────────────────────────────────────────────

/// Submits a feed-capable operation for asynchronous execution, binding to
/// the driver's planner so a single page is produced per call.
///
/// Unlike [`cosmos_submit_singleton_operation`], this path
/// runs `plan_operation` + `execute_plan` internally so it can both
/// **resume** from an inbound continuation token
/// ([`CosmosOperationRequest::continuation_token`]) and **surface** the
/// next-page token via the completion's `next_continuation` field
/// ([`crate::completion::CosmosCompletion`]).
///
/// Use this for any operation that can return multiple pages — queries,
/// read-all-items, change feed — and drive pagination by re-submitting with
/// the returned next token until the completion delivers an end-of-stream
/// response (status code `0`, NULL next token; see the `Ok(None)` contract
/// below).
///
/// # Completion outcomes
///
/// - **A page of results**: outcome `OK`; the response carries the body,
///   headers, and (when more pages remain) a non-NULL next continuation.
/// - **Feed exhausted** (`Ok(None)` from the driver): outcome `OK` with a
///   degenerate response — status code `0`, empty body, NULL next token.
///   Hosts treat this as end-of-stream.
/// - **Failure**: outcome `ERROR` with the coarse code (+ rich error when
///   the queue opted in).
///
/// # Parameters
///
/// - `driver` — non-NULL driver handle.
/// - `request` — non-NULL [`CosmosOperationRequest`] describing the
///   operation. All borrowed pointers must remain valid for the duration of
///   this call (the wrapper copies everything it needs before returning).
/// - `queue` — non-NULL completion queue.
/// - `user_data` — opaque, pointer-sized integer cookie (`intptr_t`)
///   round-tripped verbatim onto the completion; never dereferenced.
/// - `out_pre_error` — receives the coarse code on pre-flight failure
///   (returns NULL). NULL is accepted.
///
/// # Returns
///
/// A fresh `cosmos_operation_handle_t *` on success, or NULL on pre-flight
/// failure (with `*out_pre_error` populated when non-NULL). Pre-flight
/// failures include malformed requests (`INVALID_ARGUMENT`), invalid option
/// values (`INVALID_OPTION_VALUE`), and the queue states
/// (`QUEUE_SHUTDOWN` / `QUEUE_FULL`).
#[no_mangle]
pub extern "C" fn cosmos_submit_operation(
    driver: *const DriverHandle,
    request: *const CosmosOperationRequest,
    queue: *mut CompletionQueue,
    user_data: isize,
    out_pre_error: *mut CosmosErrorCode,
) -> *mut OperationHandle {
    let write_err = |code: CosmosErrorCode| {
        if !out_pre_error.is_null() {
            // SAFETY: caller-supplied writable slot.
            unsafe {
                *out_pre_error = code;
            }
        }
    };

    let Some(driver_inner) = DriverHandle::inner_arc(driver) else {
        write_err(CosmosErrorCode::CosmosErrorCodeInvalidArgument);
        return std::ptr::null_mut();
    };
    let driver_arc: Arc<CosmosDriver> = Arc::clone(&driver_inner.inner);

    // Build the driver operation + options + inbound continuation from the
    // flat request. Validation failures abort before we spend a spawn.
    // SAFETY: caller guarantees `request`'s pointer fields per the struct
    // contract.
    let built = match unsafe { build_request(request) } {
        Ok(b) => b,
        Err(code) => {
            write_err(code);
            return std::ptr::null_mut();
        }
    };

    let (ctx, op_handle) = match pre_flight_spawn(queue, user_data) {
        Ok(pair) => pair,
        Err(code) => {
            write_err(code);
            return std::ptr::null_mut();
        }
    };

    let runtime = Arc::clone(ctx.queue.runtime());
    let crate::op_request::BuiltRequest {
        operation,
        options,
        continuation,
    } = built;

    spawn_oneshot(
        ctx,
        runtime,
        async move {
            // Plan with the inbound continuation, then execute a single
            // page. Mirrors `CosmosDriver::execute_operation` but threads
            // the continuation token through the planner and retains the
            // plan so we can mint the next-page token.
            let container = operation.container().cloned();
            let mut plan = driver_arc
                .plan_operation(operation, &options, continuation.as_ref())
                .await?;
            let page = driver_arc
                .execute_plan(&mut plan, container, options)
                .await?;
            // After a page, snapshot the next-page token from the plan.
            // Token derivation is best-effort: a failure here (e.g. a
            // non-query trivial op that doesn't support client tokens)
            // simply yields no next token rather than failing the page.
            let next = match page {
                Some(_) => plan
                    .to_continuation_token()
                    .ok()
                    .map(|t| t.as_str().to_owned()),
                None => None,
            };
            Ok((page, next))
        },
        |(page, next): (Option<CosmosResponse>, Option<String>)| SuccessKind::Response {
            response: page.map(Box::new),
            next_continuation: next,
        },
    );
    op_handle
}

/// Submits a singleton (single-result) operation for asynchronous
/// execution, binding to [`CosmosDriver::execute_singleton_operation`].
///
/// Use this for point operations and any operation that returns exactly one
/// result — create / read / replace / delete / patch item, database and
/// container CRUD, read/replace offer. Feed kinds (queries, read-all,
/// change feed) must go through
/// [`cosmos_submit_operation`] instead; submitting one here
/// makes the driver assert in debug builds and yields a
/// `CLIENT_SINGLETON_OPERATION_RETURNED_EMPTY_PAGE`-shaped error in release.
///
/// The inbound [`CosmosOperationRequest::continuation_token`] is ignored on
/// this path (singletons do not paginate).
///
/// # Parameters / Returns
///
/// Identical in shape to [`cosmos_submit_operation`]; the
/// completion always carries a single response (outcome `OK`) or an error
/// (outcome `ERROR`).
#[no_mangle]
pub extern "C" fn cosmos_submit_singleton_operation(
    driver: *const DriverHandle,
    request: *const CosmosOperationRequest,
    queue: *mut CompletionQueue,
    user_data: isize,
    out_pre_error: *mut CosmosErrorCode,
) -> *mut OperationHandle {
    let write_err = |code: CosmosErrorCode| {
        if !out_pre_error.is_null() {
            // SAFETY: caller-supplied writable slot.
            unsafe {
                *out_pre_error = code;
            }
        }
    };

    let Some(driver_inner) = DriverHandle::inner_arc(driver) else {
        write_err(CosmosErrorCode::CosmosErrorCodeInvalidArgument);
        return std::ptr::null_mut();
    };
    let driver_arc: Arc<CosmosDriver> = Arc::clone(&driver_inner.inner);

    // SAFETY: caller guarantees `request`'s pointer fields per the struct
    // contract.
    let built = match unsafe { build_request(request) } {
        Ok(b) => b,
        Err(code) => {
            write_err(code);
            return std::ptr::null_mut();
        }
    };

    let (ctx, op_handle) = match pre_flight_spawn(queue, user_data) {
        Ok(pair) => pair,
        Err(code) => {
            write_err(code);
            return std::ptr::null_mut();
        }
    };

    let runtime = Arc::clone(ctx.queue.runtime());
    // `continuation` is intentionally dropped: singletons do not paginate.
    let crate::op_request::BuiltRequest {
        operation, options, ..
    } = built;

    spawn_oneshot(
        ctx,
        runtime,
        async move {
            driver_arc
                .execute_singleton_operation(operation, options)
                .await
        },
        |response: CosmosResponse| SuccessKind::Response {
            response: Some(Box::new(response)),
            next_continuation: None,
        },
    );
    op_handle
}

// ─────────────────────────────────────────────────────────────────────────────
// FFI: cosmos_driver_get_or_create_submit
// ─────────────────────────────────────────────────────────────────────────────
/// Asynchronous variant of [`crate::driver::cosmos_driver_get_or_create_blocking`].
///
/// Bridges `CosmosDriverRuntime::get_or_create_driver` through the
/// submit pipeline; the completion carries the new driver in its owned
/// `driver` field, which
/// [`crate::completion::cosmos_completion_take_driver`] detaches.
#[no_mangle]
pub extern "C" fn cosmos_driver_get_or_create_submit(
    runtime: *const RuntimeContext,
    account: *const AccountRefHandle,
    options: *const DriverOptionsHandle,
    queue: *mut CompletionQueue,
    user_data: isize,
    out_pre_error: *mut CosmosErrorCode,
) -> *mut OperationHandle {
    let write_err = |code: CosmosErrorCode| {
        if !out_pre_error.is_null() {
            // SAFETY: caller-supplied writable slot.
            unsafe {
                *out_pre_error = code;
            }
        }
    };

    let Some(runtime_inner) = RuntimeContext::inner_arc(runtime) else {
        write_err(CosmosErrorCode::CosmosErrorCodeInvalidArgument);
        return std::ptr::null_mut();
    };
    let Some(account_inner) = AccountRefHandle::from_ptr(account) else {
        write_err(CosmosErrorCode::CosmosErrorCodeInvalidArgument);
        return std::ptr::null_mut();
    };
    let options_owned = if options.is_null() {
        None
    } else {
        DriverOptionsHandle::inner_arc(options).map(|arc| arc.inner.clone())
    };

    let (ctx, op_handle) = match pre_flight_spawn(queue, user_data) {
        Ok(pair) => pair,
        Err(code) => {
            write_err(code);
            return std::ptr::null_mut();
        }
    };

    let driver_runtime: Arc<azure_data_cosmos_driver::driver::CosmosDriverRuntime> =
        Arc::clone(&runtime_inner.driver);
    let account_owned: AccountReference = account_inner.inner.clone();
    let task_runtime = Arc::clone(ctx.queue.runtime());

    spawn_oneshot(
        ctx,
        task_runtime,
        async move {
            // Since #4588 `create_driver` takes a single `DriverOptions`
            // that embeds the account. Use caller-supplied options when
            // present; otherwise build a default from the account.
            let driver_options =
                options_owned.unwrap_or_else(|| DriverOptions::builder(account_owned).build());
            driver_runtime.create_driver(driver_options).await
        },
        |driver_arc: Arc<CosmosDriver>| {
            // The submit's success payload is the freshly-created driver, which
            // the completion carries in its owned `driver` field for the host
            // to detach via `cosmos_completion_take_driver`.
            let driver_inner = Arc::new(DriverHandle { inner: driver_arc });
            SuccessKind::Driver(driver_inner)
        },
    );
    op_handle
}

// ─────────────────────────────────────────────────────────────────────────────
// FFI: cosmos_driver_resolve_container_submit
// ─────────────────────────────────────────────────────────────────────────────

/// Asynchronous variant of
/// [`crate::container_ref::cosmos_driver_resolve_container_blocking`].
///
/// Same shape as the get-or-create variant — the completion carries the
/// resolved container in its owned `container` field, which
/// [`crate::completion::cosmos_completion_take_container`] detaches.
#[no_mangle]
pub extern "C" fn cosmos_driver_resolve_container_submit(
    driver: *const DriverHandle,
    database_id: *const std::os::raw::c_char,
    container_id: *const std::os::raw::c_char,
    queue: *mut CompletionQueue,
    user_data: isize,
    out_pre_error: *mut CosmosErrorCode,
) -> *mut OperationHandle {
    let write_err = |code: CosmosErrorCode| {
        if !out_pre_error.is_null() {
            // SAFETY: caller-supplied writable slot.
            unsafe {
                *out_pre_error = code;
            }
        }
    };

    let Some(driver_inner) = DriverHandle::inner_arc(driver) else {
        write_err(CosmosErrorCode::CosmosErrorCodeInvalidArgument);
        return std::ptr::null_mut();
    };
    let db_id = match try_cstr_to_string(database_id) {
        Ok(s) => s,
        Err(code) => {
            write_err(code);
            return std::ptr::null_mut();
        }
    };
    let container_id = match try_cstr_to_string(container_id) {
        Ok(s) => s,
        Err(code) => {
            write_err(code);
            return std::ptr::null_mut();
        }
    };

    let (ctx, op_handle) = match pre_flight_spawn(queue, user_data) {
        Ok(pair) => pair,
        Err(code) => {
            write_err(code);
            return std::ptr::null_mut();
        }
    };

    let driver_arc: Arc<CosmosDriver> = Arc::clone(&driver_inner.inner);
    let task_runtime = Arc::clone(ctx.queue.runtime());

    spawn_oneshot(
        ctx,
        task_runtime,
        async move {
            driver_arc
                .resolve_container_by_name(&db_id, &container_id)
                .await
        },
        |container_ref: azure_data_cosmos_driver::models::ContainerReference| {
            SuccessKind::Container(Box::new(container_ref))
        },
    );
    op_handle
}

// ─────────────────────────────────────────────────────────────────────────────
// Helpers
// ─────────────────────────────────────────────────────────────────────────────

fn try_cstr_to_string(p: *const std::os::raw::c_char) -> Result<String, CosmosErrorCode> {
    if p.is_null() {
        return Err(CosmosErrorCode::CosmosErrorCodeInvalidArgument);
    }
    // SAFETY: caller contract.
    let cstr = unsafe { std::ffi::CStr::from_ptr(p) };
    cstr.to_str()
        .map(|s| s.to_owned())
        .map_err(|_| CosmosErrorCode::CosmosErrorCodeInvalidUtf8)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ptr;

    #[test]
    fn execute_operation_submit_rejects_null_driver() {
        let mut err = CosmosErrorCode::CosmosErrorCodeSuccess;
        let h = cosmos_submit_operation(ptr::null(), ptr::null(), ptr::null_mut(), 0, &mut err);
        assert!(h.is_null());
        assert_eq!(err, CosmosErrorCode::CosmosErrorCodeInvalidArgument);
    }

    #[test]
    fn execute_singleton_operation_submit_rejects_null_driver() {
        let mut err = CosmosErrorCode::CosmosErrorCodeSuccess;
        let h = cosmos_submit_singleton_operation(
            ptr::null(),
            ptr::null(),
            ptr::null_mut(),
            0,
            &mut err,
        );
        assert!(h.is_null());
        assert_eq!(err, CosmosErrorCode::CosmosErrorCodeInvalidArgument);
    }

    #[test]
    fn get_or_create_submit_rejects_null_runtime() {
        let mut err = CosmosErrorCode::CosmosErrorCodeSuccess;
        let h = cosmos_driver_get_or_create_submit(
            ptr::null(),
            ptr::null(),
            ptr::null(),
            ptr::null_mut(),
            0,
            &mut err,
        );
        assert!(h.is_null());
        assert_eq!(err, CosmosErrorCode::CosmosErrorCodeInvalidArgument);
    }

    #[test]
    fn resolve_container_submit_rejects_null_driver() {
        let mut err = CosmosErrorCode::CosmosErrorCodeSuccess;
        let h = cosmos_driver_resolve_container_submit(
            ptr::null(),
            ptr::null(),
            ptr::null(),
            ptr::null_mut(),
            0,
            &mut err,
        );
        assert!(h.is_null());
        assert_eq!(err, CosmosErrorCode::CosmosErrorCodeInvalidArgument);
    }

    #[test]
    fn pre_flight_spawn_rejects_shutdown_queue() {
        use crate::completion::{
            cosmos_completion_queue_create, cosmos_completion_queue_free,
            cosmos_completion_queue_shutdown, CosmosCompletionQueueOptions,
        };
        use crate::runtime::{__test_only_create_default_runtime, cosmos_runtime_free};

        let rt = __test_only_create_default_runtime();
        let opts = CosmosCompletionQueueOptions {
            capacity_hint: 0,
            max_capacity: 0,
            include_error_details: 1,
        };
        let queue = cosmos_completion_queue_create(rt, &opts as *const _);
        assert!(!queue.is_null());

        // Shut the queue down, then attempt a submit pre-flight. It must be
        // rejected synchronously with QUEUE_SHUTDOWN — no task is spawned.
        cosmos_completion_queue_shutdown(queue);
        let result = pre_flight_spawn(queue, 0);
        assert!(matches!(
            result,
            Err(CosmosErrorCode::CosmosErrorCodeQueueShutdown)
        ));

        cosmos_completion_queue_free(queue);
        cosmos_runtime_free(rt);
    }

    #[test]
    fn spawn_oneshot_cancellation_yields_cancelled_completion() {
        use crate::completion::{
            cosmos_completion_queue_create, cosmos_completion_queue_free,
            cosmos_completion_queue_free_completions, cosmos_completion_queue_wait,
            cosmos_operation_handle_cancel, cosmos_operation_handle_free, CosmosCompletion,
            CosmosCompletionOutcome, CosmosCompletionQueueOptions,
        };
        use crate::runtime::{__test_only_create_default_runtime, cosmos_runtime_free};
        use std::mem::MaybeUninit;

        let rt = __test_only_create_default_runtime();
        let opts = CosmosCompletionQueueOptions {
            capacity_hint: 0,
            max_capacity: 0,
            include_error_details: 1,
        };
        let queue = cosmos_completion_queue_create(rt, &opts as *const _);
        assert!(!queue.is_null());

        let (ctx, op_handle) = pre_flight_spawn(queue, 0).expect("pre-flight ok");
        let runtime = Arc::clone(ctx.queue.runtime());

        // A future that never resolves on its own — only cancellation can end
        // this operation.
        spawn_oneshot(
            ctx,
            runtime,
            futures::future::pending::<azure_data_cosmos_driver::error::Result<CosmosResponse>>(),
            |r: CosmosResponse| SuccessKind::Response {
                response: Some(Box::new(r)),
                next_continuation: None,
            },
        );

        // Request cancellation; the spawned task's select must observe it and
        // post a CANCELLED completion instead of hanging forever.
        cosmos_operation_handle_cancel(op_handle);

        let mut slot = MaybeUninit::<CosmosCompletion>::uninit();
        let n = cosmos_completion_queue_wait(queue, slot.as_mut_ptr(), 1, u32::MAX);
        assert_eq!(n, 1);
        // SAFETY: `n == 1` means one completion was written into `slot`.
        let mut c = unsafe { slot.assume_init() };
        assert_eq!(
            c.outcome,
            CosmosCompletionOutcome::CosmosCompletionOutcomeCancelled
        );
        assert_eq!(c.was_cancel_requested, 1);

        cosmos_completion_queue_free_completions(&mut c as *mut CosmosCompletion, 1);
        cosmos_operation_handle_free(op_handle);
        cosmos_completion_queue_free(queue);
        cosmos_runtime_free(rt);
    }
}

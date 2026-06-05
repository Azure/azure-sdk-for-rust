// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Generic submit pipeline.
//!
//! The two canonical operation entry points
//! ([`cosmos_driver_execute_operation_submit`] /
//! [`cosmos_driver_execute_singleton_operation_submit`]), plus the async
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

use std::ffi::c_void;
use std::future::Future;
use std::sync::Arc;

use azure_data_cosmos_driver::driver::CosmosDriver;
use azure_data_cosmos_driver::models::{AccountReference, CosmosResponse};

use crate::account_ref::AccountRefHandle;
use crate::completion::{
    Completion, CompletionQueue, CompletionQueueInner, CosmosCompletionOutcome, OperationHandle,
    OperationInner,
};
use crate::container_ref::ContainerRefInner;
use crate::driver::{DriverHandle, DriverInner};
use crate::driver_options::DriverOptionsHandle;
use crate::error::{CosmosErrorCode, CosmosErrorInner};
use crate::op_request::{build_request, CosmosOperationRequest};
use crate::response::ResponseHandle;
use crate::runtime::RuntimeContext;

/// Send-safe encoding of the opaque `user_data` pointer round-tripped
/// to the completion. Storing as `usize` instead of `*mut c_void`
/// avoids the async-block auto-trait analysis flagging the raw
/// pointer as `!Send` — the wrapper does not dereference it, and the
/// host owns its threading semantics.
#[derive(Clone, Copy)]
struct UserData(usize);

impl UserData {
    fn new(p: *mut c_void) -> Self {
        Self(p as usize)
    }
    fn as_ptr(self) -> *mut c_void {
        self.0 as *mut c_void
    }
}

/// Resources captured into every spawned submit task.
///
/// Holding `Arc<CompletionQueueInner>` and `Arc<OperationInner>`
/// directly lets the spawned task survive a concurrent `cosmos_cq_free`
/// or `cosmos_operation_handle_free` from the producer side — the
/// queue's deque drops back-pressured completions cleanly, and the
/// operation-handle state stays consistent through Phase 1's existing
/// `Arc` plumbing.
struct SpawnContext {
    queue: Arc<CompletionQueueInner>,
    op_inner: Arc<OperationInner>,
    user_data: UserData,
    include_error_details: bool,
}

// SAFETY: `UserData` is `usize`-backed, the other fields are `Arc`s.
unsafe impl Send for SpawnContext {}

/// Pre-flight: builds a [`SpawnContext`] + a fresh producer-side
/// `cosmos_operation_handle_t *`. Returns `Err(coarse_code)` on
/// validation failure so the caller can write the coarse code into
/// `out_pre_error` and return NULL.
fn pre_flight_spawn(
    queue: *mut CompletionQueue,
    user_data: *mut c_void,
) -> Result<(SpawnContext, *mut OperationHandle), CosmosErrorCode> {
    let Some(queue_inner) = CompletionQueue::inner_arc(queue) else {
        return Err(CosmosErrorCode::CosmosErrorCodeInvalidArgument);
    };
    // Pre-flight queue state. The spawned task does a second check
    // inside the lock when it finally enqueues — this is just an early
    // bailout so we don't spend a spawn on a doomed submit.
    // `current_len() >= max_capacity` is the doomed case for
    // capacity-bounded queues.
    if queue_inner.max_capacity() > 0
        && queue_inner.current_len() as u32 >= queue_inner.max_capacity()
    {
        return Err(CosmosErrorCode::CosmosErrorCodeQueueFull);
    }

    let op_handle = OperationHandle::allocate();
    let op_inner = match OperationHandle::inner_arc(op_handle) {
        Some(a) => a,
        None => {
            // Should not happen — `allocate` returns non-NULL.
            OperationHandle::drop_raw(op_handle);
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
/// through the standard submit-and-publish pipeline. `to_response`
/// converts the success value into the `(response, side_payloads)` the
/// completion delivers; on `Err`, the rich `CosmosError` is converted
/// to the FFI's coarse code + optional rich-error payload per the
/// queue's `include_error_details` option.
fn spawn_oneshot<Fut, R>(
    ctx: SpawnContext,
    runtime: Arc<crate::runtime::RuntimeContextInner>,
    fut: Fut,
    to_response: impl FnOnce(R) -> *mut ResponseHandle + Send + 'static,
) where
    Fut: Future<Output = azure_data_cosmos_driver::error::Result<R>> + Send + 'static,
    R: Send + 'static,
{
    runtime.tokio.spawn(async move {
        let outcome = fut.await;
        let completion = match outcome {
            Ok(value) => {
                let response = to_response(value);
                Completion::new_for_publish(
                    CosmosCompletionOutcome::CosmosCompletionOutcomeOk,
                    CosmosErrorCode::CosmosErrorCodeSuccess,
                    ctx.user_data.as_ptr(),
                    ctx.op_inner.clone(),
                    None,
                    Some(response),
                )
            }
            Err(err) => {
                let coarse = CosmosErrorCode::from_driver_error(&err);
                let stored_error = if ctx.include_error_details {
                    Some(Arc::new(CosmosErrorInner::new(err)))
                } else {
                    None
                };
                Completion::new_for_publish(
                    CosmosCompletionOutcome::CosmosCompletionOutcomeError,
                    coarse,
                    ctx.user_data.as_ptr(),
                    ctx.op_inner.clone(),
                    stored_error,
                    None,
                )
            }
        };
        let rc = CompletionQueue::enqueue_into_inner(&ctx.queue, completion);
        if rc != CosmosErrorCode::CosmosErrorCodeSuccess {
            // The queue rejected the completion (shutdown / full). The
            // completion is dropped — its side payloads (if any) are
            // freed by `Completion::drop`.
            tracing::warn!(
                rc = ?rc,
                "submit: completion dropped (queue shutdown or full)",
            );
        }
    });
}

// ─────────────────────────────────────────────────────────────────────────────
// FFI: cosmos_driver_execute_operation_submit / _execute_singleton_operation_submit
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
/// Unlike [`cosmos_driver_execute_singleton_operation_submit`], this path
/// runs `plan_operation` + `execute_plan` internally so it can both
/// **resume** from an inbound continuation token
/// ([`CosmosOperationRequest::continuation_token`]) and **surface** the
/// next-page token via [`crate::response::cosmos_response_next_continuation`].
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
/// - `user_data` — opaque pointer round-tripped onto the completion.
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
pub extern "C" fn cosmos_driver_execute_operation_submit(
    driver: *const DriverHandle,
    request: *const CosmosOperationRequest,
    queue: *mut CompletionQueue,
    user_data: *mut c_void,
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
        |(page, next): (Option<CosmosResponse>, Option<String>)| {
            ResponseHandle::into_raw_with_next_continuation(page, next)
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
/// [`cosmos_driver_execute_operation_submit`] instead; submitting one here
/// makes the driver assert in debug builds and yields a
/// `CLIENT_SINGLETON_OPERATION_RETURNED_EMPTY_PAGE`-shaped error in release.
///
/// The inbound [`CosmosOperationRequest::continuation_token`] is ignored on
/// this path (singletons do not paginate).
///
/// # Parameters / Returns
///
/// Identical in shape to [`cosmos_driver_execute_operation_submit`]; the
/// completion always carries a single response (outcome `OK`) or an error
/// (outcome `ERROR`).
#[no_mangle]
pub extern "C" fn cosmos_driver_execute_singleton_operation_submit(
    driver: *const DriverHandle,
    request: *const CosmosOperationRequest,
    queue: *mut CompletionQueue,
    user_data: *mut c_void,
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
        |response: CosmosResponse| ResponseHandle::into_raw(response),
    );
    op_handle
}

// ─────────────────────────────────────────────────────────────────────────────
// FFI: cosmos_driver_get_or_create_submit (Phase 3 deferral)
// ─────────────────────────────────────────────────────────────────────────────
/// Asynchronous variant of [`crate::driver::cosmos_driver_get_or_create_blocking`].
///
/// Bridges `CosmosDriverRuntime::get_or_create_driver` through the
/// submit pipeline; the completion delivers a degenerate
/// `cosmos_response_t` from which
/// [`crate::response::cosmos_response_take_driver`] extracts the new
/// driver handle.
///
/// Closes the Phase 3 deferral.
#[no_mangle]
pub extern "C" fn cosmos_driver_get_or_create_submit(
    runtime: *const RuntimeContext,
    account: *const AccountRefHandle,
    options: *const DriverOptionsHandle,
    queue: *mut CompletionQueue,
    user_data: *mut c_void,
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
    let Some(account_inner) = AccountRefHandle::inner_arc(account) else {
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
            driver_runtime
                .get_or_create_driver(account_owned, options_owned)
                .await
        },
        |driver_arc: Arc<CosmosDriver>| {
            // The submit's "response" is a degenerate shell; the real
            // payload is the driver Arc carried in the side-payload
            // slot. `_take_driver` extracts it; the scalar / header
            // accessors return defaults.
            let driver_inner = Arc::new(DriverInner { inner: driver_arc });
            ResponseHandle::into_raw_with_driver(driver_inner)
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
/// Same shape as the get-or-create variant — the completion delivers a
/// degenerate response from which
/// [`crate::response::cosmos_response_take_container`] extracts the
/// resolved container handle.
#[no_mangle]
pub extern "C" fn cosmos_driver_resolve_container_submit(
    driver: *const DriverHandle,
    database_id: *const std::os::raw::c_char,
    container_id: *const std::os::raw::c_char,
    queue: *mut CompletionQueue,
    user_data: *mut c_void,
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
            let container_inner = Arc::new(ContainerRefInner {
                inner: container_ref,
            });
            ResponseHandle::into_raw_with_container(container_inner)
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
        let h = cosmos_driver_execute_operation_submit(
            ptr::null(),
            ptr::null(),
            ptr::null_mut(),
            ptr::null_mut(),
            &mut err,
        );
        assert!(h.is_null());
        assert_eq!(err, CosmosErrorCode::CosmosErrorCodeInvalidArgument);
    }

    #[test]
    fn execute_singleton_operation_submit_rejects_null_driver() {
        let mut err = CosmosErrorCode::CosmosErrorCodeSuccess;
        let h = cosmos_driver_execute_singleton_operation_submit(
            ptr::null(),
            ptr::null(),
            ptr::null_mut(),
            ptr::null_mut(),
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
            ptr::null_mut(),
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
            ptr::null_mut(),
            &mut err,
        );
        assert!(h.is_null());
        assert_eq!(err, CosmosErrorCode::CosmosErrorCodeInvalidArgument);
    }
}

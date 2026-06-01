// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Completion queue (CQ) — the dispatch boundary between the Rust runtime
//! and the host runtime.
//!
//! Every async operation submitted from the host carries an opaque
//! `user_data` pointer (typically a `GCHandle` to a .NET `TaskCompletionSource`,
//! a Java `CompletableFuture`, or a Go channel). When the underlying Rust
//! future resolves, the Rust side packages a [`Completion`] record carrying
//! that exact `user_data` value plus the result, and pushes it onto the CQ.
//!
//! A single host thread per CQ calls [`cosmos_cq_wait`] in a tight loop and
//! dispatches each completion. This is the same pattern that gRPC C-core,
//! libuv, IOCP, and io_uring expose to host runtimes.
//!
//! ## Invariant I4 — single waiter per CQ
//!
//! `cosmos_cq_wait` takes the receiver under a `Mutex`. Two threads waiting
//! on the same CQ at the same time is not undefined behavior — the second
//! caller simply blocks on the mutex — but it defeats the design. The
//! spike documents single-waiter as the supported configuration; the
//! production crate may relax this.

use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;

use crossbeam::channel::{bounded, Receiver, RecvTimeoutError, Sender, TryRecvError};
use tokio::sync::Mutex;

use crate::error::{CosmosStatusCode, FfiError};
use crate::ffi_guard;
use crate::response::CosmosResponseHandle;

/// One slot pushed onto the CQ per finished operation.
pub struct Completion {
    /// Opaque pointer the host attached at submit time. Invariant I1: this
    /// crate never dereferences it — it is bytes that travel through the
    /// pipeline unchanged.
    pub user_data: usize,
    pub outcome: CompletionOutcome,
}

// SAFETY: `user_data` is a `usize` (an integer); it carries no Rust
// references and the Rust side never dereferences it.
unsafe impl Send for Completion {}
unsafe impl Sync for Completion {}

pub enum CompletionOutcome {
    /// Operation succeeded. The boxed response is host-owned from this
    /// point on; the host must free it via `cosmos_response_free`.
    Success(Box<CosmosResponseHandle>),
    /// Operation failed. We split apart `FfiError` here so the FFI surface
    /// can return a status code plus an optional body in a uniform way.
    Failure(FfiError),
}

/// Public opaque type. Reference-counted because the spawned task holds a
/// `Sender` (which carries an `Arc`) while the host holds the `Receiver`.
pub struct CosmosCq {
    pub(crate) tx: Sender<Completion>,
    pub(crate) rx: Mutex<Receiver<Completion>>,
    pub(crate) shutdown: AtomicBool,
}

/// Creates a bounded CQ. `capacity = 0` is treated as 1024 for the spike.
///
/// # Safety
///
/// Returns a heap-allocated handle that must be released with
/// [`cosmos_cq_free`].
#[no_mangle]
pub unsafe extern "C" fn cosmos_cq_new(capacity: u32) -> *mut CosmosCq {
    ffi_guard!(std::ptr::null_mut(), {
        let cap = if capacity == 0 {
            1024
        } else {
            capacity as usize
        };
        let (tx, rx) = bounded::<Completion>(cap);
        Box::into_raw(Box::new(CosmosCq {
            tx,
            rx: Mutex::new(rx),
            shutdown: AtomicBool::new(false),
        }))
    })
}

/// Signals that the host will issue no more `cosmos_cq_wait` calls. After
/// this, `cosmos_cq_wait` returns `QueueShutdown` once the channel drains.
/// Operations already in flight are **not** cancelled — the host should
/// cancel them explicitly first, then call `cosmos_cq_shutdown`.
///
/// # Safety
///
/// `cq` must be non-null and valid.
#[no_mangle]
pub unsafe extern "C" fn cosmos_cq_shutdown(cq: *mut CosmosCq) {
    if cq.is_null() {
        return;
    }
    ffi_guard!((), {
        (*cq).shutdown.store(true, Ordering::SeqCst);
    })
}

/// Releases the CQ. The host must have shut it down first (otherwise still-
/// queued completions leak). For the spike we tolerate the leak and free.
///
/// # Safety
///
/// `cq` must have been returned by [`cosmos_cq_new`] and not yet freed.
#[no_mangle]
pub unsafe extern "C" fn cosmos_cq_free(cq: *mut CosmosCq) {
    if cq.is_null() {
        return;
    }
    ffi_guard!((), {
        let _ = Box::from_raw(cq);
    })
}

/// Blocks the calling host thread until one completion is available, then
/// pops it. `timeout_ms = 0` means "block forever".
///
/// On success, writes the completion's `user_data` to `*out_user_data`,
/// the completion's status code into `*out_status`, and (for `Ok` /
/// `ServiceError`) writes the response handle into `*out_response`.
/// The host owns the response and must free it via `cosmos_response_free`.
/// For `Cancelled` / `InvalidArg` / `InternalError` the response pointer is
/// `null` and a small error message is written into `out_response` as a
/// response handle with status `0` and the message as the body — this lets
/// the host follow a single response-freeing path. (Spike convenience; the
/// production crate will define a separate `cosmos_error_t`.)
///
/// Return value is the **wait outcome**, not the operation outcome:
///   `Ok` — a completion was delivered. Inspect `*out_status` for the op result.
///   `Cancelled` — `timeout_ms` elapsed before any completion arrived.
///   `QueueShutdown` — the CQ was shut down and the channel is drained.
///   `InvalidArg` — null pointer arguments.
///
/// # Safety
///
/// All three `out_*` pointers must be non-null and point to caller-owned
/// memory.
#[no_mangle]
pub unsafe extern "C" fn cosmos_cq_wait(
    cq: *mut CosmosCq,
    timeout_ms: u64,
    out_user_data: *mut usize,
    out_status: *mut i32,
    out_response: *mut *mut CosmosResponseHandle,
) -> i32 {
    ffi_guard!(CosmosStatusCode::InternalError as i32, {
        if cq.is_null() || out_user_data.is_null() || out_status.is_null() || out_response.is_null()
        {
            return CosmosStatusCode::InvalidArg as i32;
        }
        *out_user_data = 0;
        *out_status = CosmosStatusCode::InternalError as i32;
        *out_response = std::ptr::null_mut();

        let cq_ref = &*cq;

        // Acquire receiver under the mutex (I4). `try_lock` first so we
        // don't accidentally serialize with the spawn-time `Sender::send`
        // path (which uses `tx`, not `rx`).
        let rx_guard = match cq_ref.rx.try_lock() {
            Ok(g) => g,
            Err(_) => {
                // Some other thread is already waiting (I4 violation).
                // Tell the host so they can fix their code rather than
                // silently serializing.
                return CosmosStatusCode::InvalidArg as i32;
            }
        };

        let recv_result = if timeout_ms == 0 {
            // Block forever — but use a poll loop so shutdown is observable
            // even when no completions are produced.
            loop {
                match rx_guard.recv_timeout(Duration::from_millis(200)) {
                    Ok(c) => break Ok(c),
                    Err(RecvTimeoutError::Timeout) => {
                        if cq_ref.shutdown.load(Ordering::SeqCst) {
                            // Drain non-blocking to catch any last-moment
                            // completion the producer raced in.
                            match rx_guard.try_recv() {
                                Ok(c) => break Ok(c),
                                Err(TryRecvError::Empty | TryRecvError::Disconnected) => {
                                    break Err(RecvTimeoutError::Disconnected);
                                }
                            }
                        }
                    }
                    Err(RecvTimeoutError::Disconnected) => {
                        break Err(RecvTimeoutError::Disconnected);
                    }
                }
            }
        } else {
            rx_guard.recv_timeout(Duration::from_millis(timeout_ms))
        };

        match recv_result {
            Ok(completion) => {
                *out_user_data = completion.user_data;
                match completion.outcome {
                    CompletionOutcome::Success(resp) => {
                        *out_status = CosmosStatusCode::Ok as i32;
                        *out_response = Box::into_raw(resp);
                    }
                    CompletionOutcome::Failure(err) => {
                        let code = err.code() as i32;
                        *out_status = code;
                        // For Service errors, hand the body back so the host
                        // can surface it. For others, build a synthetic
                        // response carrying the message as the body.
                        let (status_u16, body_bytes) = match err {
                            FfiError::Service { status, body } => (status, body),
                            FfiError::InvalidArg(msg) => (0, msg.as_bytes().to_vec()),
                            FfiError::Cancelled => (0, b"cancelled".to_vec()),
                            FfiError::Internal(msg) => (0, msg.into_bytes()),
                        };
                        let resp = Box::new(CosmosResponseHandle {
                            status: status_u16,
                            body: body_bytes,
                        });
                        *out_response = Box::into_raw(resp);
                    }
                }
                CosmosStatusCode::Ok as i32
            }
            Err(RecvTimeoutError::Timeout) => CosmosStatusCode::Cancelled as i32,
            Err(RecvTimeoutError::Disconnected) => CosmosStatusCode::QueueShutdown as i32,
        }
    })
}

/// Internal helper used by `read_item` to obtain a shared sender for the
/// background task.
pub(crate) fn sender(cq: &CosmosCq) -> Sender<Completion> {
    cq.tx.clone()
}

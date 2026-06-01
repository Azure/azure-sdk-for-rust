// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! The one async operation implemented in the spike: `cosmos_read_item`.
//!
//! This is the smallest possible end-to-end demonstration of the
//! completion-queue model:
//!
//! 1. The host calls `cosmos_read_item(driver, db, container, pk, id, cq,
//!    user_data, &out_op)`.
//! 2. The function builds the operation, spawns a Tokio task onto the
//!    driver's runtime, and returns immediately with a `*mut CosmosOp`.
//! 3. The Tokio task awaits the read, packages a [`Completion`] with the
//!    caller's `user_data`, and pushes it onto the CQ.
//! 4. The host's dedicated CQ-wait thread pops the completion and
//!    dispatches it (in .NET: marshals the `user_data` back to a
//!    `GCHandle` and resolves the `TaskCompletionSource`).
//!
//! The factory itself **never blocks**. The factory's wall time is bounded
//! by allocation + a Tokio `spawn` (microseconds).

use std::ffi::{c_char, CStr};

use crate::cq::{sender, Completion, CompletionOutcome, CosmosCq};
use crate::driver::CosmosDriver;
use crate::error::{CosmosStatusCode, FfiError};
use crate::ffi_guard;
use crate::op::{CosmosOp, OpState};
use crate::response::CosmosResponseHandle;

/// Submits a point read against the given container.
///
/// `user_data` is an opaque integer (`usize`) the host attaches to the
/// submission; it is returned verbatim with the completion. The Rust side
/// never dereferences it (Invariant I1).
///
/// On success writes the operation handle to `*out_op` and returns `Ok`.
/// The handle must be released by `cosmos_op_release` even after the
/// completion has been delivered.
///
/// # Safety
///
/// All pointer arguments must be valid for the duration of the call. The
/// string arguments are copied into Rust-owned storage before the call
/// returns.
#[no_mangle]
#[allow(clippy::too_many_arguments)]
pub unsafe extern "C" fn cosmos_read_item(
    driver: *mut CosmosDriver,
    database_id: *const c_char,
    container_id: *const c_char,
    partition_key: *const c_char,
    item_id: *const c_char,
    cq: *mut CosmosCq,
    user_data: usize,
    out_op: *mut *mut CosmosOp,
) -> i32 {
    ffi_guard!(CosmosStatusCode::InternalError as i32, {
        if driver.is_null()
            || database_id.is_null()
            || container_id.is_null()
            || partition_key.is_null()
            || item_id.is_null()
            || cq.is_null()
            || out_op.is_null()
        {
            if !out_op.is_null() {
                *out_op = std::ptr::null_mut();
            }
            return CosmosStatusCode::InvalidArg as i32;
        }
        *out_op = std::ptr::null_mut();

        let db = match CStr::from_ptr(database_id).to_str() {
            Ok(s) => s.to_owned(),
            Err(_) => return CosmosStatusCode::InvalidArg as i32,
        };
        let container = match CStr::from_ptr(container_id).to_str() {
            Ok(s) => s.to_owned(),
            Err(_) => return CosmosStatusCode::InvalidArg as i32,
        };
        let pk = match CStr::from_ptr(partition_key).to_str() {
            Ok(s) => s.to_owned(),
            Err(_) => return CosmosStatusCode::InvalidArg as i32,
        };
        let id = match CStr::from_ptr(item_id).to_str() {
            Ok(s) => s.to_owned(),
            Err(_) => return CosmosStatusCode::InvalidArg as i32,
        };

        let driver_ref = &*driver;
        let client = driver_ref.client.clone();
        let runtime = driver_ref.runtime.clone();
        let cq_tx = sender(&*cq);

        let state = OpState::new();
        let state_for_task = state.clone();

        let handle = runtime.spawn(async move {
            // Cooperative cancel check before the I/O begins. Aborting the
            // task is the authoritative cancel path; this check just shaves
            // a microsecond if the host cancelled before we got scheduled.
            if state_for_task.is_cancelled() {
                let _ = cq_tx.send(Completion {
                    user_data,
                    outcome: CompletionOutcome::Failure(FfiError::Cancelled),
                });
                return;
            }

            let result = async {
                let db_client = client.database_client(&db);
                let container_client = db_client.container_client(&container).await?;
                let resp = container_client.read_item(&pk, &id, None).await?;
                let status_u16: u16 = resp.status().status_code().into();
                let body = resp.into_body();
                let bytes = if body.is_empty() {
                    Vec::new()
                } else {
                    body.single()?.to_vec()
                };
                Ok::<_, azure_data_cosmos::CosmosError>(CosmosResponseHandle {
                    status: status_u16,
                    body: bytes,
                })
            }
            .await;

            // Re-check cancel: if cancellation was requested *and* the
            // result is an aborted/error path, prefer reporting `Cancelled`
            // so the host's `TaskCompletionSource.SetCanceled()` lights up
            // the right exception. Success paths still report success
            // because that's the actual semantics (race won by the I/O).
            let outcome = match result {
                Ok(resp) => CompletionOutcome::Success(Box::new(resp)),
                Err(_) if state_for_task.is_cancelled() => {
                    CompletionOutcome::Failure(FfiError::Cancelled)
                }
                Err(e) => CompletionOutcome::Failure(FfiError::from(e)),
            };

            let _ = cq_tx.send(Completion { user_data, outcome });
        });

        state.install_abort(handle.abort_handle());

        let op_handle = Box::new(CosmosOp { state });
        *out_op = Box::into_raw(op_handle);
        CosmosStatusCode::Ok as i32
    })
}

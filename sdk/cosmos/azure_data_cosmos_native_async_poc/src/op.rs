// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Per-operation handle exposed to the host.
//!
//! An operation handle is returned by `cosmos_read_item` (and would be
//! returned by every other operation factory once added). It is reference-
//! counted internally via `Arc<OpState>`: the host holds one strong
//! reference (released by [`cosmos_op_release`]) and the Tokio task spawned
//! to drive the operation holds the second. When both drop, the inner
//! state is destroyed.
//!
//! ## Invariant I5 — release vs cancel
//!
//! `cosmos_cancel` flips a flag and aborts the Tokio task; it does **not**
//! release the host's reference. `cosmos_op_release` releases the host's
//! reference; it does **not** cancel the operation (the task continues to
//! completion and its result is dropped into the CQ as usual).
//!
//! The spec's single `cosmos_cancel` conflates these two concerns. Splitting
//! them is the design-finding to file as a spec-edit comment on PR #4461.

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};

use tokio::task::AbortHandle;

use crate::ffi_guard;

pub struct OpState {
    pub(crate) cancelled: AtomicBool,
    pub(crate) abort: Mutex<Option<AbortHandle>>,
}

impl OpState {
    pub fn new() -> Arc<Self> {
        Arc::new(Self {
            cancelled: AtomicBool::new(false),
            abort: Mutex::new(None),
        })
    }

    pub fn install_abort(&self, handle: AbortHandle) {
        if let Ok(mut g) = self.abort.lock() {
            *g = Some(handle);
        }
    }

    pub fn is_cancelled(&self) -> bool {
        self.cancelled.load(Ordering::SeqCst)
    }
}

impl Default for OpState {
    fn default() -> Self {
        Self {
            cancelled: AtomicBool::new(false),
            abort: Mutex::new(None),
        }
    }
}

/// Public opaque type. Each handle is a `Box<Arc<OpState>>` so the host
/// holds exactly one boxed strong reference; the spawned task holds a
/// separate `Arc` clone of the same inner state.
pub struct CosmosOp {
    pub(crate) state: Arc<OpState>,
}

/// Marks the operation as cancelled and aborts the underlying Tokio task.
/// A `Cancelled` completion **will** be pushed to the CQ unless the task
/// has already pushed its own completion (Invariant I2: exactly one
/// completion per operation, races are resolved in favor of "first to
/// reach the channel").
///
/// # Safety
///
/// `op` must be non-null and not yet released.
#[no_mangle]
pub unsafe extern "C" fn cosmos_cancel(op: *mut CosmosOp) {
    if op.is_null() {
        return;
    }
    ffi_guard!((), {
        let op_ref = &*op;
        op_ref.state.cancelled.store(true, Ordering::SeqCst);
        if let Ok(g) = op_ref.state.abort.lock() {
            if let Some(handle) = g.as_ref() {
                handle.abort();
            }
        }
    })
}

/// Releases the host's reference to the operation handle. Does **not**
/// cancel the operation. After this call the pointer must not be used.
///
/// # Safety
///
/// `op` must have been returned by an operation factory and not yet
/// released. `null` is a no-op.
#[no_mangle]
pub unsafe extern "C" fn cosmos_op_release(op: *mut CosmosOp) {
    if op.is_null() {
        return;
    }
    ffi_guard!((), {
        let _ = Box::from_raw(op);
    })
}

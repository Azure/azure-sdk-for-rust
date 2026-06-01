// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Opaque Tokio runtime handle exposed across the FFI boundary.
//!
//! One runtime is expected per process; the host calls
//! [`cosmos_runtime_new`] once and uses the returned `*mut cosmos_runtime_t`
//! to construct every driver and submit every operation. The runtime is a
//! multi-thread Tokio runtime with a worker-thread count the host can pass
//! (or `0` for "Tokio default = number of CPU cores").
//!
//! Releasing the runtime via [`cosmos_runtime_free`] blocks until all
//! spawned tasks complete (Tokio's default drop behavior). Hosts must
//! release every driver, CQ, and pending op handle first or risk leaking.

use std::sync::Arc;

use tokio::runtime::{Builder, Runtime};

use crate::ffi_guard;

/// Public opaque type. Layout is intentionally not exposed.
pub struct CosmosRuntime {
    pub(crate) inner: Arc<Runtime>,
}

/// Creates a multi-threaded Tokio runtime. `worker_threads = 0` means
/// "Tokio default" (number of logical CPUs). Returns `null` on failure
/// (allocation failure or `worker_threads` larger than `i32::MAX`).
///
/// # Safety
///
/// Returns a heap-allocated handle that must be released by
/// [`cosmos_runtime_free`].
#[no_mangle]
pub unsafe extern "C" fn cosmos_runtime_new(worker_threads: u32) -> *mut CosmosRuntime {
    ffi_guard!(std::ptr::null_mut(), {
        let mut builder = Builder::new_multi_thread();
        builder.enable_all();
        builder.thread_name("cosmos-async-poc");
        if worker_threads > 0 {
            builder.worker_threads(worker_threads as usize);
        }
        match builder.build() {
            Ok(rt) => Box::into_raw(Box::new(CosmosRuntime {
                inner: Arc::new(rt),
            })),
            Err(_) => std::ptr::null_mut(),
        }
    })
}

/// Releases the runtime. Blocks until in-flight tasks complete. After this
/// call the pointer must not be used again.
///
/// # Safety
///
/// `rt` must have been returned by [`cosmos_runtime_new`] and not yet freed.
/// Passing `null` is a no-op.
#[no_mangle]
pub unsafe extern "C" fn cosmos_runtime_free(rt: *mut CosmosRuntime) {
    if rt.is_null() {
        return;
    }
    ffi_guard!((), {
        let _ = Box::from_raw(rt);
    })
}

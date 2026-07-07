// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Async runtime wrapper for the C ABI boundary.
//!
//! Owns both a Tokio multi-threaded [`Runtime`] (so the wrapper can drive
//! `async fn` driver code from synchronous FFI entry points) and an
//! `Arc<CosmosDriverRuntime>` (so cached drivers, container caches, and the
//! account-metadata cache stay alive for the lifetime of the handle).
//!
//! The runtime pairs the wrapper-side Tokio runtime with the driver runtime
//! and is built through the public `cosmos_runtime_build` surface (see
//! [`crate::runtime_builder`]).
//!
//! The runtime is **opaque** at the FFI boundary — consumers get a
//! `cosmos_runtime_t *` and never look inside. See spec section 3.1.1 + section 4.1.

use std::sync::Arc;

use azure_data_cosmos_driver::driver::{CosmosDriverRuntime, CosmosDriverRuntimeBuilder};
use tokio::runtime::Runtime;

use crate::runtime_builder::RuntimeBuildError;

/// The C ABI handle for the async runtime (`cosmos_runtime_t`).
///
/// Reference-counted via `Arc` so completion queues can keep the runtime
/// alive for the duration of in-flight operations independently of the C
/// handle's lifetime. Construction always goes through
/// `RuntimeContext::new_default` (test path) or
/// `RuntimeContext::new_with_builder` (production path called from the public
/// `cosmos_runtime_build`).
///
/// - `tokio` — the wrapper-side multi-threaded Tokio runtime. Used to
///   `block_on(...)` driver builder construction at FFI-call time and to
///   spawn the per-operation tasks that drive submits.
/// - `driver` — the underlying `azure_data_cosmos_driver` runtime that owns
///   the per-account driver registry, container cache, account-metadata
///   cache, HTTP transport factory, and so on. Cloning the `Arc` is cheap
///   and is how the driver / account surfaces hand out handles.
pub struct RuntimeContext {
    /// Required at FFI-call time (e.g. `block_on` for the driver builder
    /// and `spawn` for per-operation submits) but never
    /// read directly outside that context, so the field reads as dead
    /// to the compiler.
    #[allow(
        dead_code,
        reason = "consumed via `tokio.block_on` / `tokio.spawn`; the field itself \
                  is never read directly"
    )]
    pub(crate) tokio: Runtime,
    /// `Arc::clone`d into every per-account handle by the driver / account
    /// surfaces.
    #[allow(
        dead_code,
        reason = "first non-test caller is the driver / account references"
    )]
    pub(crate) driver: Arc<CosmosDriverRuntime>,
}

impl RuntimeContext {
    /// Test-only constructor — builds a default Tokio runtime *and* a
    /// default driver runtime via the merged
    /// `CosmosDriverRuntimeBuilder::default()`.
    ///
    /// Used by the internal tests via
    /// [`__test_only_create_default_runtime`]; production callers go
    /// through `cosmos_runtime_build`.
    pub(crate) fn new_default() -> Result<*mut Self, RuntimeBuildError> {
        Self::new_with_builder(CosmosDriverRuntimeBuilder::new())
    }

    /// Production constructor — builds the wrapper-side Tokio runtime,
    /// uses it to `block_on` the supplied driver builder's `build()`, and
    /// wraps the pair in a fresh `cosmos_runtime_t *`.
    ///
    /// Errors are split between the wrapper-side Tokio init (returned as
    /// [`RuntimeBuildError::TokioInit`]) and the driver-side build
    /// (returned as [`RuntimeBuildError::Driver`]) so the FFI surface can
    /// map each to the appropriate coarse code + rich error.
    pub(crate) fn new_with_builder(
        builder: CosmosDriverRuntimeBuilder,
    ) -> Result<*mut Self, RuntimeBuildError> {
        let tokio = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .thread_name("azurecosmosdriver-worker")
            .build()
            .map_err(RuntimeBuildError::TokioInit)?;

        // `CosmosDriverRuntimeBuilder::build()` is `async fn`; bridge from
        // the synchronous FFI call by running on the same Tokio runtime
        // the rest of the wrapper will use. The build itself does not
        // perform network I/O (per-account network probes happen lazily
        // inside `get_or_create_driver`) so blocking the FFI
        // thread here is bounded by local TLS + machine-ID work.
        let driver = tokio
            .block_on(async move { builder.build().await })
            .map_err(RuntimeBuildError::Driver)?;

        Ok(Arc::into_raw(Arc::new(RuntimeContext { tokio, driver })) as *mut RuntimeContext)
    }

    /// Returns a cloned `Arc` to the inner state, used by completion queues
    /// that need to keep the runtime alive for the duration of in-flight
    /// operations.
    pub(crate) fn inner_arc(this: *const RuntimeContext) -> Option<Arc<RuntimeContext>> {
        if this.is_null() {
            return None;
        }
        // SAFETY: caller guarantees `this` was obtained from a successful
        // `new_with_builder()` / `new_default()`. Bumping the strong count
        // before reconstructing the `Arc` leaves the caller's reference
        // intact.
        unsafe {
            Arc::increment_strong_count(this);
            Some(Arc::from_raw(this))
        }
    }

    /// Borrows the wrapper from a raw pointer for the duration of an FFI call.
    #[allow(
        dead_code,
        reason = "first non-test caller is the driver / account refs"
    )]
    pub(crate) fn from_ptr<'a>(p: *const RuntimeContext) -> Option<&'a Self> {
        if p.is_null() {
            None
        } else {
            // SAFETY: caller guarantees `p` was obtained from
            // `new_with_builder()` / `new_default()` and has not been freed.
            Some(unsafe { &*p })
        }
    }

    /// Drops a runtime handle allocated by `new_with_builder()` /
    /// `new_default()`.
    pub(crate) fn drop_raw(this: *mut RuntimeContext) {
        if this.is_null() {
            return;
        }
        // SAFETY: caller guarantees `this` was obtained from
        // `new_with_builder()` / `new_default()` and has not already been
        // freed.
        unsafe {
            drop(Arc::from_raw(this as *const RuntimeContext));
        }
    }
}

/// Lifecycle: free a `cosmos_runtime_t *` previously returned by the runtime
/// builder.
///
/// Drops the FFI handle's `Arc` reference. Any completion queues created
/// against this runtime continue to function until their own `Arc` references
/// are dropped, but no new queues can be created from the now-freed handle.
/// NULL is a no-op.
#[no_mangle]
pub extern "C" fn cosmos_runtime_free(runtime: *mut RuntimeContext) {
    if runtime.is_null() {
        return;
    }
    tracing::trace!(?runtime, "freeing cosmos_runtime_t");
    RuntimeContext::drop_raw(runtime);
}

// ─────────────────────────────────────────────────────────────────────────────
// Test-only constructor
//
// Exposed via an internal Rust API (not `#[no_mangle]`) so integration tests
// can build a queue end-to-end without threading a builder through every test
// setup.
// ─────────────────────────────────────────────────────────────────────────────

/// Test-only: construct a default `cosmos_runtime_t` and return a raw pointer.
///
/// Visible only to the workspace's own tests; not exported as `#[no_mangle]`.
/// Production callers should prefer the public `cosmos_runtime_build`
/// surface; this helper exists so the receive-loop tests do not
/// have to thread a builder through every test setup.
#[doc(hidden)]
pub fn __test_only_create_default_runtime() -> *mut RuntimeContext {
    RuntimeContext::new_default().unwrap_or(std::ptr::null_mut())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn free_handles_null() {
        cosmos_runtime_free(std::ptr::null_mut());
    }

    #[test]
    fn create_and_free_default_runtime() {
        let rt = __test_only_create_default_runtime();
        assert!(!rt.is_null(), "default runtime construction must not fail");
        cosmos_runtime_free(rt);
    }

    #[test]
    fn default_runtime_carries_driver() {
        let rt = __test_only_create_default_runtime();
        assert!(!rt.is_null());
        let inner = RuntimeContext::inner_arc(rt).expect("non-NULL runtime has inner");
        // The driver Arc is independently strong-counted; at this point
        // the storage's clone + this temp keep it live.
        assert!(
            Arc::strong_count(&inner.driver) >= 1,
            "driver Arc must be live"
        );
        drop(inner);
        cosmos_runtime_free(rt);
    }
}

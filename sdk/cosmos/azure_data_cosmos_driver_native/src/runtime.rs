// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Async runtime wrapper for the C ABI boundary.
//!
//! Phase 1 ships only the bare minimum needed to back the completion-queue
//! types: a Tokio multi-threaded runtime owned by [`RuntimeContext`]. Phase 2
//! extends this with the workspace's `Arc<CosmosDriverRuntime>` and the full
//! `cosmos_runtime_builder_*` mirror of `CosmosDriverRuntimeBuilder`.
//!
//! The runtime is **opaque** at the FFI boundary — consumers get a
//! `cosmos_runtime_t *` and never look inside. See spec §3.1.1 + §4.1.

use std::sync::Arc;

use tokio::runtime::Runtime;

/// Internal storage of a `cosmos_runtime_t`.
///
/// Phase 1 only holds the Tokio runtime (used by `block_on` once Phase 2
/// wires real submit calls). Phase 2 will grow an
/// `Arc<azure_data_cosmos_driver::CosmosDriverRuntime>` field alongside it.
pub(crate) struct RuntimeContextInner {
    #[allow(
        dead_code,
        reason = "first caller arrives in Phase 2 (runtime builder + submit)"
    )]
    pub(crate) tokio: Runtime,
}

/// Opaque C ABI handle for the async runtime.
///
/// The struct body is intentionally opaque to cbindgen — the real state
/// lives behind a `Box<RuntimeContextInner>` accessed through the
/// `_unused` field's tag. Construction always goes through
/// [`RuntimeContext::new_default`] (Phase 1) or the public builder
/// (Phase 2+); never construct one directly.
#[repr(C)]
pub struct RuntimeContext {
    // Cbindgen-opaque marker: zero-sized array on the C side, real storage
    // lives in a separately-boxed `RuntimeContextInner` whose pointer is
    // recovered via the `Self::inner` accessor below. We use this trick
    // (rather than a plain Rust field of type `Arc<RuntimeContextInner>`)
    // so the public header doesn't grow `cosmos_Arc_*` typedef noise.
    _opaque: [u8; 0],
}

/// Internal box paired with each `RuntimeContext` allocation.
///
/// We pun the `RuntimeContext`'s heap allocation: the `Box<RuntimeContext>`
/// we hand across the FFI is *actually* a `Box<RuntimeContextStorage>`
/// whose first field is the empty `_opaque` marker. See
/// [`RuntimeContext::storage_from_ptr`].
#[repr(C)]
struct RuntimeContextStorage {
    _opaque: [u8; 0],
    inner: Arc<RuntimeContextInner>,
}

impl RuntimeContext {
    /// Constructs a Tokio-backed runtime suitable for Phase 1's stub /
    /// test-only use. The full builder surface lands in Phase 2.
    pub(crate) fn new_default() -> Result<*mut Self, std::io::Error> {
        let tokio = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .thread_name("azurecosmosdriver-worker")
            .build()?;
        let storage = Box::new(RuntimeContextStorage {
            _opaque: [],
            inner: Arc::new(RuntimeContextInner { tokio }),
        });
        // SAFETY: `RuntimeContextStorage` and `RuntimeContext` are both
        // `#[repr(C)]` with `_opaque: [u8; 0]` as the first field. The C
        // side only ever sees the `_opaque` field; the inner Arc lives in
        // the trailing bytes of the same allocation.
        Ok(Box::into_raw(storage).cast::<RuntimeContext>())
    }

    /// Returns a cloned `Arc` to the inner state, used by completion queues
    /// that need to keep the runtime alive for the duration of in-flight
    /// operations.
    pub(crate) fn inner_arc(this: *const RuntimeContext) -> Option<Arc<RuntimeContextInner>> {
        if this.is_null() {
            return None;
        }
        // SAFETY: caller guarantees `this` was obtained from
        // `new_default()` (i.e. a `Box<RuntimeContextStorage>` punned to
        // `*mut RuntimeContext`).
        let storage = unsafe { &*(this as *const RuntimeContextStorage) };
        Some(Arc::clone(&storage.inner))
    }

    /// Borrows the wrapper from a raw pointer for the duration of an FFI call.
    #[allow(
        dead_code,
        reason = "first non-test caller arrives in Phase 2 (runtime builder)"
    )]
    pub(crate) fn from_ptr<'a>(p: *const RuntimeContext) -> Option<&'a Self> {
        if p.is_null() {
            None
        } else {
            // SAFETY: caller guarantees `p` was obtained from
            // `new_default()` and has not been freed.
            Some(unsafe { &*p })
        }
    }

    /// Drops a runtime handle allocated by `new_default()`.
    pub(crate) fn drop_raw(this: *mut RuntimeContext) {
        if this.is_null() {
            return;
        }
        // SAFETY: pun back into the `Box<RuntimeContextStorage>` the
        // allocation actually came from.
        unsafe {
            drop(Box::from_raw(this.cast::<RuntimeContextStorage>()));
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
///
/// Phase 1 has no public `cosmos_runtime_create` (the builder lands in
/// Phase 2). This entry exists so the lifecycle contract is documented up
/// front and so internal test-only constructors can be paired with the same
/// `_free` consumers will use.
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
// Exposed via an internal Rust API (not `#[no_mangle]`) so Phase 1 integration
// tests can build a queue end-to-end. Phase 2 replaces this with the public
// builder.
// ─────────────────────────────────────────────────────────────────────────────

/// Test-only: construct a default `cosmos_runtime_t` and return a raw pointer.
///
/// Visible only to the workspace's own tests; not exported as `#[no_mangle]`.
/// Phase 2 replaces internal callers with the public `cosmos_runtime_builder_*`
/// surface.
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
}

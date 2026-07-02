// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! C ABI surface for `cosmos_driver_t` — wraps
//! [`azure_data_cosmos_driver::driver::CosmosDriver`].
//!
//! This ships the synchronous **convenience** entry point —
//! [`cosmos_driver_get_or_create_blocking`] — plus the matching free
//! function. The async submit-and-deliver-via-completion-queue variant
//! (`cosmos_driver_get_or_create_submit`) lives in the submit module so the
//! wrapper's generic `tokio::spawn` → `cq_enqueue` plumbing lands once,
//! with the operation submit pipeline.
//!
//! ## Cache-hit advisory (`OPTIONS_IGNORED_ON_CACHE_HIT`)
//!
//! Spec section 4.4.1 requires the wrapper to detect when the driver returns a
//! cached driver for an endpoint that already has an entry, and surface
//! a `5001` warning. The merged
//! `CosmosDriverRuntime::get_or_create_driver` API does not expose a
//! "was cached" signal, so detecting cache hits requires either a
//! driver-side enhancement (preferred) or wrapper-side cache shadowing
//! (hacky). The advisory is intentionally **not** implemented today
//! — `cosmos_driver_get_or_create_blocking` always returns `SUCCESS` on
//! a cached hit. This is tracked as a follow-up.
//!
//! See [`docs/NATIVE_WRAPPER_SPEC.md`] section 4.4.
//!
//! [`docs/NATIVE_WRAPPER_SPEC.md`]: https://github.com/Azure/azure-sdk-for-rust/blob/main/sdk/cosmos/azure_data_cosmos_driver/docs/NATIVE_WRAPPER_SPEC.md

use std::sync::Arc;

use azure_data_cosmos_driver::driver::CosmosDriver;
use azure_data_cosmos_driver::options::DriverOptions;

use crate::account_ref::AccountRefHandle;
use crate::driver_options::DriverOptionsHandle;
use crate::error::{CosmosErrorCode, CosmosErrorHandle};
use crate::runtime::RuntimeContext;

/// The C ABI handle for a [`CosmosDriver`] (`cosmos_driver_t`).
///
/// Reference-counted via `Arc` so the submit pipeline and a degenerate
/// response's stashed side payload can share it with only an atomic bump.
pub struct DriverHandle {
    /// Consumed by the submit pipeline (`cosmos_driver_*_submit`,
    /// `cosmos_driver_resolve_container_*`) which `Arc::clone`s it
    /// onto each spawned task.
    pub(crate) inner: Arc<CosmosDriver>,
}

impl DriverHandle {
    fn into_raw(inner: Arc<CosmosDriver>) -> *mut Self {
        Arc::into_raw(Arc::new(DriverHandle { inner })) as *mut Self
    }

    /// Allocates a fresh FFI handle that shares an existing
    /// [`DriverHandle`] `Arc`. Used by
    /// [`crate::completion::cosmos_completion_take_driver`] to mint a
    /// public `cosmos_driver_t *` from a degenerate completion's stashed
    /// side payload.
    pub(crate) fn from_arc_into_raw(this: Arc<DriverHandle>) -> *mut Self {
        Arc::into_raw(this) as *mut Self
    }

    pub(crate) fn inner_arc(p: *const DriverHandle) -> Option<Arc<DriverHandle>> {
        if p.is_null() {
            return None;
        }
        // SAFETY: caller guarantees `p` was obtained from `into_raw` and has
        // not been freed. Bumping the strong count before reconstructing the
        // `Arc` leaves the caller's reference intact.
        unsafe {
            Arc::increment_strong_count(p);
            Some(Arc::from_raw(p))
        }
    }

    fn drop_raw(p: *mut DriverHandle) {
        if p.is_null() {
            return;
        }
        // SAFETY: caller guarantees `p` was obtained from `into_raw` and has
        // not already been freed.
        unsafe {
            drop(Arc::from_raw(p as *const DriverHandle));
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// FFI: lifecycle
// ─────────────────────────────────────────────────────────────────────────────

/// Frees a driver handle. Drops the FFI-side `Arc` reference; the
/// underlying driver remains alive in the runtime's cache until the
/// owning `cosmos_runtime_t` is freed (spec section 4.4.1). NULL is a no-op.
#[no_mangle]
pub extern "C" fn cosmos_driver_free(driver: *mut DriverHandle) {
    if driver.is_null() {
        return;
    }
    tracing::trace!(?driver, "freeing cosmos_driver_t");
    DriverHandle::drop_raw(driver);
}

// ─────────────────────────────────────────────────────────────────────────────
// FFI: get_or_create (blocking)
// ─────────────────────────────────────────────────────────────────────────────

/// Synchronously gets or creates the driver for the supplied account.
///
/// Bridges
/// `CosmosDriverRuntime::get_or_create_driver` through the wrapper's
/// own multi-threaded Tokio runtime via `block_on`. Suitable for
/// startup-time initialization; for runtime use prefer the async
/// `_submit` variant.
///
/// # Cache behavior (spec section 4.4.1)
///
/// - The runtime caches drivers by endpoint URL. A second call with the
///   same endpoint returns the cached driver and **silently ignores**
///   `options`.
/// - Two `AccountReference`s with the same endpoint but different
///   credentials collide in the cache — first credential wins.
/// - Cache eviction happens only when the owning `cosmos_runtime_t` is
///   freed; freeing a `cosmos_driver_t` does not evict.
///
/// The `5001` `OPTIONS_IGNORED_ON_CACHE_HIT` advisory described in spec
/// Section 4.4.1 is not emitted today — see the module-level
/// `Cache-hit advisory` note for the rationale.
///
/// # Parameters
///
/// - `runtime` — non-NULL runtime handle.
/// - `account` — non-NULL account reference.
/// - `options` — optional driver options. NULL means "use the driver's
///   defaults" (matches passing `None` to the underlying API).
/// - `out_driver` — non-NULL slot that receives the new driver handle
///   on success.
/// - `out_error` — optional. Receives a rich `cosmos_error_t *` on
///   driver-side failure (e.g. network errors during
///   `CosmosDriver::initialize`). NULL silently drops it.
///
/// # Returns
///
/// - `SUCCESS` (0) with `*out_driver` populated.
/// - `INVALID_ARGUMENT` (1) when `runtime`, `account`, or `out_driver`
///   is NULL.
/// - One of the `2xxx` / `3xxx` codes derived from the driver-side
///   error per spec section 3.5.1 when the underlying
///   `get_or_create_driver` returns an error.
#[no_mangle]
pub extern "C" fn cosmos_driver_get_or_create_blocking(
    runtime: *const RuntimeContext,
    account: *const AccountRefHandle,
    options: *const DriverOptionsHandle,
    out_driver: *mut *mut DriverHandle,
    out_error: *mut *mut CosmosErrorHandle,
) -> i32 {
    if out_driver.is_null() {
        return CosmosErrorCode::CosmosErrorCodeInvalidArgument.as_i32();
    }
    let Some(runtime_inner) = RuntimeContext::inner_arc(runtime) else {
        return CosmosErrorCode::CosmosErrorCodeInvalidArgument.as_i32();
    };
    let Some(account_inner) = AccountRefHandle::from_ptr(account) else {
        return CosmosErrorCode::CosmosErrorCodeInvalidArgument.as_i32();
    };
    // Options is optional — NULL → None.
    let options_owned = if options.is_null() {
        None
    } else {
        DriverOptionsHandle::inner_arc(options).map(|arc| arc.inner.clone())
    };

    let account_for_call = account_inner.inner.clone();
    let driver_runtime = Arc::clone(&runtime_inner.driver);

    let result = runtime_inner.tokio.block_on(async move {
        // Since #4588 `create_driver` takes a single `DriverOptions` that
        // embeds the account. When the caller supplied options we use them
        // as-is (they already carry an account); otherwise build a default
        // `DriverOptions` from the account argument.
        let driver_options =
            options_owned.unwrap_or_else(|| DriverOptions::builder(account_for_call).build());
        driver_runtime.create_driver(driver_options).await
    });

    match result {
        Ok(driver_arc) => {
            let handle = DriverHandle::into_raw(driver_arc);
            // SAFETY: caller guarantees `out_driver` is writable for one
            // `*mut DriverHandle`.
            unsafe {
                *out_driver = handle;
            }
            CosmosErrorCode::CosmosErrorCodeSuccess.as_i32()
        }
        Err(driver_err) => {
            let coarse = CosmosErrorCode::from_driver_error(&driver_err);
            if !out_error.is_null() {
                // SAFETY: caller guarantees `out_error` is writable for
                // one `*mut CosmosErrorHandle`.
                unsafe {
                    *out_error = CosmosErrorHandle::into_raw(driver_err);
                }
            }
            coarse.as_i32()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ptr;

    fn make_runtime() -> *mut RuntimeContext {
        crate::runtime::__test_only_create_default_runtime()
    }

    fn make_account() -> *mut AccountRefHandle {
        crate::account_ref::tests::make_master_key_handle(
            "https://myaccount.documents.azure.com:443/",
            "fake-master-key",
        )
    }

    #[test]
    fn free_handles_null() {
        cosmos_driver_free(ptr::null_mut());
    }

    #[test]
    fn blocking_rejects_null_arguments() {
        let runtime = make_runtime();
        let account = make_account();
        let mut out: *mut DriverHandle = ptr::null_mut();
        let mut err: *mut CosmosErrorHandle = ptr::null_mut();
        assert_eq!(
            cosmos_driver_get_or_create_blocking(
                ptr::null(),
                account,
                ptr::null(),
                &mut out,
                &mut err,
            ),
            CosmosErrorCode::CosmosErrorCodeInvalidArgument.as_i32()
        );
        assert_eq!(
            cosmos_driver_get_or_create_blocking(
                runtime,
                ptr::null(),
                ptr::null(),
                &mut out,
                &mut err,
            ),
            CosmosErrorCode::CosmosErrorCodeInvalidArgument.as_i32()
        );
        assert_eq!(
            cosmos_driver_get_or_create_blocking(
                runtime,
                account,
                ptr::null(),
                ptr::null_mut(),
                &mut err,
            ),
            CosmosErrorCode::CosmosErrorCodeInvalidArgument.as_i32()
        );
        assert!(out.is_null());
        assert!(err.is_null());

        crate::account_ref::cosmos_account_ref_free(account);
        crate::runtime::cosmos_runtime_free(runtime);
    }

    /// End-to-end against a non-routable endpoint. We don't have an
    /// emulator wired up in Rust unit tests so this exercises the error
    /// path: bootstrap network metadata fails, the driver returns an
    /// error, and we surface it through the coarse code + rich error.
    ///
    /// Skipped in normal `cargo test` because the actual failure mode
    /// (transport error vs. DNS error vs. timeout) varies between OSes
    /// and CI environments. Run manually when validating the network
    /// path:
    ///
    /// ```text
    /// cargo test -p azure_data_cosmos_driver_native -- \
    ///     --ignored blocking_against_invalid_endpoint
    /// ```
    #[test]
    #[ignore = "exercises real network path; surface varies by environment"]
    fn blocking_against_invalid_endpoint() {
        let runtime = make_runtime();
        let account = crate::account_ref::tests::make_master_key_handle(
            "https://nonexistent.invalid.documents.azure.com:443/",
            "fake-master-key",
        );
        let mut out: *mut DriverHandle = ptr::null_mut();
        let mut err: *mut CosmosErrorHandle = ptr::null_mut();
        let rc =
            cosmos_driver_get_or_create_blocking(runtime, account, ptr::null(), &mut out, &mut err);
        assert_ne!(
            rc,
            CosmosErrorCode::CosmosErrorCodeSuccess.as_i32(),
            "unreachable endpoint must fail"
        );
        assert!(out.is_null());
        assert!(!err.is_null(), "rich error attached on failure");
        crate::error::cosmos_error_free(err);
        crate::account_ref::cosmos_account_ref_free(account);
        crate::runtime::cosmos_runtime_free(runtime);
    }
}

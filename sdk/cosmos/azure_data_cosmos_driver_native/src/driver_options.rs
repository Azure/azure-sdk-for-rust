// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! C ABI surface for `cosmos_driver_options_t` + `cosmos_driver_options_builder_t`
//! — wraps the driver's
//! [`azure_data_cosmos_driver::options::DriverOptions`] and its builder.
//!
//! `DriverOptions` itself is small (3 fields per spec §4.2): the bound
//! account, the per-driver `OperationOptions`, and a `Vec<Region>` of
//! preferred regions. The per-driver defaults are set through
//! `with_operation_options`, which now takes the flat
//! `cosmos_operation_options_t` struct
//! ([`crate::op_request::CosmosOperationOptions`]) directly rather than an
//! opaque options-builder handle. Drivers that don't configure operation
//! options use the driver's own defaults (the same defaults
//! `DriverOptionsBuilder::build()` populates when none are configured).
//!
//! The settings frequently confused with "per-driver" defaults
//! (excluded regions, consistency, content-response-on-write,
//! throughput control, retry counts, etc.) all live on
//! `OperationOptions` and reach this surface via `with_operation_options`
//! — not as additional setters on the driver-options builder.
//!
//! Transport-side knobs (connection pool, user-agent suffix, workload
//! id, correlation id) live on `cosmos_runtime_builder_t`, not here.
//!
//! See [`docs/NATIVE_WRAPPER_SPEC.md`] §4.2.
//!
//! [`docs/NATIVE_WRAPPER_SPEC.md`]: https://github.com/Azure/azure-sdk-for-rust/blob/main/sdk/cosmos/azure_data_cosmos_driver/docs/NATIVE_WRAPPER_SPEC.md

use std::ffi::{c_char, CStr};
use std::sync::Arc;

use azure_data_cosmos_driver::options::{DriverOptions, DriverOptionsBuilder, Region};

use crate::account_ref::AccountRefHandle;
use crate::error::CosmosErrorCode;

// ─────────────────────────────────────────────────────────────────────────────
// Built options handle
// ─────────────────────────────────────────────────────────────────────────────

pub(crate) struct DriverOptionsInner {
    pub(crate) inner: DriverOptions,
}

/// Opaque C ABI handle for a built [`DriverOptions`] value.
///
/// Storage pun: same shape as `AccountRefHandle`.
#[repr(C)]
pub struct DriverOptionsHandle {
    _opaque: [u8; 0],
}

#[repr(C)]
struct DriverOptionsStorage {
    _opaque: [u8; 0],
    inner: Arc<DriverOptionsInner>,
}

impl DriverOptionsHandle {
    fn into_raw(inner: DriverOptions) -> *mut Self {
        let storage = Box::new(DriverOptionsStorage {
            _opaque: [],
            inner: Arc::new(DriverOptionsInner { inner }),
        });
        Box::into_raw(storage).cast::<DriverOptionsHandle>()
    }

    /// Borrows the inner `Arc` for the lifetime of an FFI call.
    pub(crate) fn inner_arc(p: *const DriverOptionsHandle) -> Option<Arc<DriverOptionsInner>> {
        if p.is_null() {
            return None;
        }
        // SAFETY: caller guarantees `p` was obtained from `into_raw` and
        // has not been freed.
        let storage = unsafe { &*(p as *const DriverOptionsStorage) };
        Some(Arc::clone(&storage.inner))
    }

    fn drop_raw(p: *mut DriverOptionsHandle) {
        if p.is_null() {
            return;
        }
        // SAFETY: pun back into the `Box<DriverOptionsStorage>` we
        // originally allocated.
        unsafe {
            drop(Box::from_raw(p.cast::<DriverOptionsStorage>()));
        }
    }
}

/// Frees a built `cosmos_driver_options_t *`. NULL is a no-op.
#[no_mangle]
pub extern "C" fn cosmos_driver_options_free(options: *mut DriverOptionsHandle) {
    if options.is_null() {
        return;
    }
    tracing::trace!(?options, "freeing cosmos_driver_options_t");
    DriverOptionsHandle::drop_raw(options);
}

// ─────────────────────────────────────────────────────────────────────────────
// Builder handle
// ─────────────────────────────────────────────────────────────────────────────

pub(crate) struct DriverOptionsBuilderInner {
    /// Wrapped in `Option` so each setter can `.take()` the builder,
    /// invoke a consuming `with_*` setter, and put the new value back.
    /// `DriverOptionsBuilder` does not derive `Default` (it requires an
    /// account up-front), so `mem::take` is not available.
    pub(crate) builder: Option<DriverOptionsBuilder>,
}

/// Opaque C ABI handle for a `DriverOptionsBuilder`.
///
/// Setters mutate in place (the underlying `with_*` consume `self` so each
/// setter does a `mem::take` / call / store dance — mirrors
/// `cosmos_runtime_builder_t`).
#[repr(C)]
pub struct DriverOptionsBuilderHandle {
    _opaque: [u8; 0],
}

#[repr(C)]
struct DriverOptionsBuilderStorage {
    _opaque: [u8; 0],
    inner: DriverOptionsBuilderInner,
}

impl DriverOptionsBuilderHandle {
    fn new_raw(account: azure_data_cosmos_driver::models::AccountReference) -> *mut Self {
        let storage = Box::new(DriverOptionsBuilderStorage {
            _opaque: [],
            inner: DriverOptionsBuilderInner {
                builder: Some(DriverOptionsBuilder::new(account)),
            },
        });
        Box::into_raw(storage).cast::<DriverOptionsBuilderHandle>()
    }

    fn inner_mut<'a>(
        p: *mut DriverOptionsBuilderHandle,
    ) -> Option<&'a mut DriverOptionsBuilderInner> {
        if p.is_null() {
            return None;
        }
        // SAFETY: caller guarantees `p` was obtained from `new_raw` and
        // has not been freed. The borrow is scoped to a single FFI call.
        let storage = unsafe { &mut *(p.cast::<DriverOptionsBuilderStorage>()) };
        Some(&mut storage.inner)
    }

    fn into_owned_builder(p: *mut DriverOptionsBuilderHandle) -> Option<DriverOptionsBuilder> {
        if p.is_null() {
            return None;
        }
        // SAFETY: pun back into the storage and move the inner builder
        // out. The `Option` is always `Some` outside of a setter's
        // take/restore window, and setters never panic between take and
        // restore.
        let storage = unsafe { Box::from_raw(p.cast::<DriverOptionsBuilderStorage>()) };
        storage.inner.builder
    }

    fn drop_raw(p: *mut DriverOptionsBuilderHandle) {
        if p.is_null() {
            return;
        }
        // SAFETY: pun back into the storage we originally allocated.
        unsafe {
            drop(Box::from_raw(p.cast::<DriverOptionsBuilderStorage>()));
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// FFI: lifecycle
// ─────────────────────────────────────────────────────────────────────────────

/// Allocates a new builder bound to the supplied account reference.
///
/// The account is cloned into the builder, so freeing `account` after
/// this call does not invalidate the builder. Returns NULL if `account`
/// is NULL.
#[no_mangle]
pub extern "C" fn cosmos_driver_options_builder_new(
    account: *const AccountRefHandle,
) -> *mut DriverOptionsBuilderHandle {
    let Some(account_inner) = AccountRefHandle::inner_arc(account) else {
        return std::ptr::null_mut();
    };
    DriverOptionsBuilderHandle::new_raw(account_inner.inner.clone())
}

/// Frees a builder that was never consumed by `_build`. NULL is a no-op.
#[no_mangle]
pub extern "C" fn cosmos_driver_options_builder_free(builder: *mut DriverOptionsBuilderHandle) {
    if builder.is_null() {
        return;
    }
    tracing::trace!(?builder, "freeing cosmos_driver_options_builder_t");
    DriverOptionsBuilderHandle::drop_raw(builder);
}

// ─────────────────────────────────────────────────────────────────────────────
// FFI: setters
// ─────────────────────────────────────────────────────────────────────────────

/// Sets the preferred regions for routing.
///
/// Mirrors [`DriverOptionsBuilder::with_preferred_regions`].
///
/// # Parameters
///
/// - `builder` — non-NULL builder.
/// - `regions` — pointer to an array of NUL-terminated UTF-8 region
///   names. May be NULL when `regions_len == 0`.
/// - `regions_len` — number of entries in `regions`.
///
/// # Returns
///
/// - `SUCCESS` (0) on success.
/// - `INVALID_ARGUMENT` (1) when `builder` is NULL or `regions` is NULL
///   but `regions_len > 0`.
/// - `INVALID_UTF8` (2) when any region name is not valid UTF-8.
///
/// Each call replaces the previously configured list (mirrors the
/// driver's `with_*` semantics). Calling with `regions_len == 0` clears
/// the list.
#[no_mangle]
pub extern "C" fn cosmos_driver_options_builder_with_preferred_regions(
    builder: *mut DriverOptionsBuilderHandle,
    regions: *const *const c_char,
    regions_len: usize,
) -> i32 {
    let Some(inner) = DriverOptionsBuilderHandle::inner_mut(builder) else {
        return CosmosErrorCode::CosmosErrorCodeInvalidArgument.as_i32();
    };
    if regions.is_null() && regions_len > 0 {
        return CosmosErrorCode::CosmosErrorCodeInvalidArgument.as_i32();
    }

    let mut owned: Vec<Region> = Vec::with_capacity(regions_len);
    for i in 0..regions_len {
        // SAFETY: `regions` is non-NULL when `regions_len > 0` (checked
        // above) and the caller guarantees the array has at least
        // `regions_len` entries.
        let entry_ptr = unsafe { *regions.add(i) };
        if entry_ptr.is_null() {
            return CosmosErrorCode::CosmosErrorCodeInvalidArgument.as_i32();
        }
        // SAFETY: each entry is a NUL-terminated C string per the
        // caller's contract.
        let cstr = unsafe { CStr::from_ptr(entry_ptr) };
        let s = match cstr.to_str() {
            Ok(s) => s,
            Err(_) => return CosmosErrorCode::CosmosErrorCodeInvalidUtf8.as_i32(),
        };
        owned.push(Region::new(s.to_owned()));
    }

    let Some(taken) = inner.builder.take() else {
        // Builder was previously consumed by `_build` or a setter panic
        // — neither should be reachable from outside this module. Treat
        // as a programmer error.
        return CosmosErrorCode::CosmosErrorCodeInvalidArgument.as_i32();
    };
    inner.builder = Some(taken.with_preferred_regions(owned));
    CosmosErrorCode::CosmosErrorCodeSuccess.as_i32()
}

/// Sets the per-driver default operation options.
///
/// Mirrors [`DriverOptionsBuilder::with_operation_options`]. The supplied
/// flat `cosmos_operation_options_t` (see
/// [`crate::op_request::CosmosOperationOptions`]) is converted and **cloned**
/// into the builder; the caller retains ownership of the source struct. NULL
/// `options` is rejected with `INVALID_ARGUMENT` — pass a struct from
/// [`crate::op_request::cosmos_operation_options_default`] (mutated as
/// needed) or don't call this setter at all to inherit the driver defaults.
/// An out-of-range option value is rejected with `INVALID_OPTION_VALUE`.
#[no_mangle]
pub extern "C" fn cosmos_driver_options_builder_with_operation_options(
    builder: *mut DriverOptionsBuilderHandle,
    options: *const crate::op_request::CosmosOperationOptions,
) -> i32 {
    let Some(inner) = DriverOptionsBuilderHandle::inner_mut(builder) else {
        return CosmosErrorCode::CosmosErrorCodeInvalidArgument.as_i32();
    };
    if options.is_null() {
        return CosmosErrorCode::CosmosErrorCodeInvalidArgument.as_i32();
    }
    // SAFETY: `options` is non-NULL and the caller guarantees its pointer
    // fields per the `cosmos_operation_options_t` contract.
    let driver_opts = match unsafe { (*options).to_driver() } {
        Ok(o) => o,
        Err(code) => return code.as_i32(),
    };
    let Some(taken) = inner.builder.take() else {
        return CosmosErrorCode::CosmosErrorCodeInvalidArgument.as_i32();
    };
    inner.builder = Some(taken.with_operation_options(driver_opts));
    CosmosErrorCode::CosmosErrorCodeSuccess.as_i32()
}

// ─────────────────────────────────────────────────────────────────────────────
// FFI: build
// ─────────────────────────────────────────────────────────────────────────────

/// Consumes the builder and returns a fresh `cosmos_driver_options_t *`.
///
/// # Lifetime
///
/// `_build` consumes the builder regardless of success or failure.
/// Callers must NOT call [`cosmos_driver_options_builder_free`] on the
/// same pointer afterwards.
///
/// # Returns
///
/// - `SUCCESS` (0) with `*out_options` populated.
/// - `INVALID_ARGUMENT` (1) when `builder` or `out_options` is NULL. In
///   the NULL-`out_options` case the builder is still consumed to avoid
///   leaking the inner allocation.
#[no_mangle]
pub extern "C" fn cosmos_driver_options_builder_build(
    builder: *mut DriverOptionsBuilderHandle,
    out_options: *mut *mut DriverOptionsHandle,
) -> i32 {
    if builder.is_null() {
        return CosmosErrorCode::CosmosErrorCodeInvalidArgument.as_i32();
    }
    let Some(driver_builder) = DriverOptionsBuilderHandle::into_owned_builder(builder) else {
        return CosmosErrorCode::CosmosErrorCodeInvalidArgument.as_i32();
    };
    if out_options.is_null() {
        return CosmosErrorCode::CosmosErrorCodeInvalidArgument.as_i32();
    }
    let opts = driver_builder.build();
    let handle = DriverOptionsHandle::into_raw(opts);
    // SAFETY: caller guarantees `out_options` is writable for one
    // `*mut DriverOptionsHandle`.
    unsafe {
        *out_options = handle;
    }
    CosmosErrorCode::CosmosErrorCodeSuccess.as_i32()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CString;
    use std::ptr;

    fn ok_cstr(s: &str) -> CString {
        CString::new(s).expect("test inputs must be NUL-free")
    }

    fn make_account() -> *mut AccountRefHandle {
        crate::account_ref::tests::make_master_key_handle(
            "https://myaccount.documents.azure.com:443/",
            "fake-master-key",
        )
    }

    #[test]
    fn lifecycle_null_safe() {
        cosmos_driver_options_builder_free(ptr::null_mut());
        cosmos_driver_options_free(ptr::null_mut());
    }

    #[test]
    fn builder_new_rejects_null_account() {
        let b = cosmos_driver_options_builder_new(ptr::null());
        assert!(b.is_null());
    }

    #[test]
    fn builder_happy_path_no_setters() {
        let account = make_account();
        let b = cosmos_driver_options_builder_new(account);
        assert!(!b.is_null());

        let mut opts: *mut DriverOptionsHandle = ptr::null_mut();
        assert_eq!(
            cosmos_driver_options_builder_build(b, &mut opts),
            CosmosErrorCode::CosmosErrorCodeSuccess.as_i32()
        );
        assert!(!opts.is_null());

        let inner = DriverOptionsHandle::inner_arc(opts).unwrap();
        assert!(inner.inner.preferred_regions().is_empty());
        drop(inner);

        cosmos_driver_options_free(opts);
        crate::account_ref::cosmos_account_ref_free(account);
    }

    #[test]
    fn with_preferred_regions_roundtrips() {
        let account = make_account();
        let b = cosmos_driver_options_builder_new(account);
        let r1 = ok_cstr("East US");
        let r2 = ok_cstr("West US 3");
        let arr: [*const c_char; 2] = [r1.as_ptr(), r2.as_ptr()];
        assert_eq!(
            cosmos_driver_options_builder_with_preferred_regions(b, arr.as_ptr(), arr.len()),
            CosmosErrorCode::CosmosErrorCodeSuccess.as_i32()
        );
        let mut opts: *mut DriverOptionsHandle = ptr::null_mut();
        cosmos_driver_options_builder_build(b, &mut opts);
        let inner = DriverOptionsHandle::inner_arc(opts).unwrap();
        let names: Vec<&str> = inner
            .inner
            .preferred_regions()
            .iter()
            .map(|r| r.as_str())
            .collect();
        // Region::new normalizes (lowercased, spaces stripped) — assert
        // against the normalized form the driver actually stores.
        assert_eq!(names, vec!["eastus", "westus3"]);
        drop(inner);
        cosmos_driver_options_free(opts);
        crate::account_ref::cosmos_account_ref_free(account);
    }

    #[test]
    fn with_preferred_regions_zero_len_clears() {
        let account = make_account();
        let b = cosmos_driver_options_builder_new(account);
        // Pre-populate.
        let r1 = ok_cstr("East US");
        let arr: [*const c_char; 1] = [r1.as_ptr()];
        cosmos_driver_options_builder_with_preferred_regions(b, arr.as_ptr(), 1);
        // Clear with len=0 (NULL ptr accepted when len=0).
        assert_eq!(
            cosmos_driver_options_builder_with_preferred_regions(b, ptr::null(), 0),
            CosmosErrorCode::CosmosErrorCodeSuccess.as_i32()
        );
        let mut opts: *mut DriverOptionsHandle = ptr::null_mut();
        cosmos_driver_options_builder_build(b, &mut opts);
        let inner = DriverOptionsHandle::inner_arc(opts).unwrap();
        assert!(inner.inner.preferred_regions().is_empty());
        drop(inner);
        cosmos_driver_options_free(opts);
        crate::account_ref::cosmos_account_ref_free(account);
    }

    #[test]
    fn with_preferred_regions_rejects_nulls() {
        let account = make_account();
        let b = cosmos_driver_options_builder_new(account);
        // NULL builder.
        assert_eq!(
            cosmos_driver_options_builder_with_preferred_regions(ptr::null_mut(), ptr::null(), 0),
            CosmosErrorCode::CosmosErrorCodeInvalidArgument.as_i32()
        );
        // NULL ptr with non-zero len.
        assert_eq!(
            cosmos_driver_options_builder_with_preferred_regions(b, ptr::null(), 1),
            CosmosErrorCode::CosmosErrorCodeInvalidArgument.as_i32()
        );
        // Entry within the array is NULL.
        let arr: [*const c_char; 1] = [ptr::null()];
        assert_eq!(
            cosmos_driver_options_builder_with_preferred_regions(b, arr.as_ptr(), 1),
            CosmosErrorCode::CosmosErrorCodeInvalidArgument.as_i32()
        );
        cosmos_driver_options_builder_free(b);
        crate::account_ref::cosmos_account_ref_free(account);
    }

    #[test]
    fn build_rejects_null_builder() {
        let mut opts: *mut DriverOptionsHandle = ptr::null_mut();
        assert_eq!(
            cosmos_driver_options_builder_build(ptr::null_mut(), &mut opts),
            CosmosErrorCode::CosmosErrorCodeInvalidArgument.as_i32()
        );
        assert!(opts.is_null());
    }
}

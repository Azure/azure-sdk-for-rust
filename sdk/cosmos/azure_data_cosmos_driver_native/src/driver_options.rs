// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! C ABI surface for `cosmos_driver_options_t` + `cosmos_driver_options_builder_t`
//! — wraps the driver's
//! [`azure_data_cosmos_driver::options::DriverOptions`] and its builder.
//!
//! `DriverOptions` itself is small (3 fields per spec section 4.2): the bound
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
//! See [`docs/NATIVE_WRAPPER_SPEC.md`] section 4.2.
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

/// The C ABI handle for a built [`DriverOptions`] value
/// (`cosmos_driver_options_t`).
pub struct DriverOptionsHandle {
    pub(crate) inner: DriverOptions,
}

impl DriverOptionsHandle {
    fn into_raw(inner: DriverOptions) -> *mut Self {
        Arc::into_raw(Arc::new(DriverOptionsHandle { inner })) as *mut Self
    }

    /// Clones the `Arc` for the lifetime of an FFI call without consuming the
    /// caller's pointer.
    pub(crate) fn inner_arc(p: *const DriverOptionsHandle) -> Option<Arc<DriverOptionsHandle>> {
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

    fn drop_raw(p: *mut DriverOptionsHandle) {
        if p.is_null() {
            return;
        }
        // SAFETY: caller guarantees `p` was obtained from `into_raw` and has
        // not already been freed.
        unsafe {
            drop(Arc::from_raw(p as *const DriverOptionsHandle));
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

/// The C ABI handle for a `DriverOptionsBuilder`
/// (`cosmos_driver_options_builder_t`).
///
/// Single-owner and `Box`-managed: setters mutate in place (the underlying
/// `with_*` consume `self`, so each setter does an `Option::take` / call /
/// store dance).
pub struct DriverOptionsBuilderHandle {
    /// Wrapped in `Option` so each setter can `.take()` the builder,
    /// invoke a consuming `with_*` setter, and put the new value back.
    /// `DriverOptionsBuilder` does not derive `Default` (it requires an
    /// account up-front), so `mem::take` is not available.
    pub(crate) builder: Option<DriverOptionsBuilder>,
}

impl DriverOptionsBuilderHandle {
    fn new_raw(account: azure_data_cosmos_driver::models::AccountReference) -> *mut Self {
        Box::into_raw(Box::new(DriverOptionsBuilderHandle {
            builder: Some(DriverOptionsBuilder::new(account)),
        }))
    }

    fn inner_mut<'a>(
        p: *mut DriverOptionsBuilderHandle,
    ) -> Option<&'a mut DriverOptionsBuilderHandle> {
        if p.is_null() {
            return None;
        }
        // SAFETY: caller guarantees `p` was obtained from `new_raw` and
        // has not been freed. The borrow is scoped to a single FFI call.
        Some(unsafe { &mut *p })
    }

    fn into_owned_builder(p: *mut DriverOptionsBuilderHandle) -> Option<DriverOptionsBuilder> {
        if p.is_null() {
            return None;
        }
        // SAFETY: reclaim the `Box` and move the inner builder out. The
        // `Option` is always `Some` outside of a setter's take/restore
        // window, and setters never panic between take and restore.
        let handle = unsafe { Box::from_raw(p) };
        handle.builder
    }

    fn drop_raw(p: *mut DriverOptionsBuilderHandle) {
        if p.is_null() {
            return;
        }
        // SAFETY: caller guarantees `p` was obtained from `new_raw` and has
        // not already been freed.
        unsafe {
            drop(Box::from_raw(p));
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
    let Some(account_inner) = AccountRefHandle::from_ptr(account) else {
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
/// Decodes a `(ptr, len)` array of NUL-terminated UTF-8 region names into a
/// `Vec<Region>`, shared by the incremental setter and the flat
/// [`cosmos_driver_options_build`].
///
/// A NULL pointer with `len == 0` yields an empty list (clears the regions).
/// A NULL pointer with `len > 0`, a NULL entry, or non-UTF-8 input is an error.
///
/// # Safety
///
/// `regions` must be NULL or point at `len` valid NUL-terminated UTF-8 string
/// pointers for the duration of the call.
unsafe fn decode_preferred_regions(
    regions: *const *const c_char,
    regions_len: usize,
) -> Result<Vec<Region>, CosmosErrorCode> {
    if regions.is_null() {
        if regions_len > 0 {
            return Err(CosmosErrorCode::CosmosErrorCodeInvalidArgument);
        }
        return Ok(Vec::new());
    }
    let mut owned: Vec<Region> = Vec::with_capacity(regions_len);
    for i in 0..regions_len {
        // SAFETY: `regions` is non-NULL (checked above) and the caller
        // guarantees the array has at least `regions_len` entries.
        let entry_ptr = unsafe { *regions.add(i) };
        if entry_ptr.is_null() {
            return Err(CosmosErrorCode::CosmosErrorCodeInvalidArgument);
        }
        // SAFETY: each entry is a NUL-terminated C string per the caller's
        // contract.
        let cstr = unsafe { CStr::from_ptr(entry_ptr) };
        let s = cstr
            .to_str()
            .map_err(|_| CosmosErrorCode::CosmosErrorCodeInvalidUtf8)?;
        owned.push(Region::new(s.to_owned()));
    }
    Ok(owned)
}

#[no_mangle]
pub extern "C" fn cosmos_driver_options_builder_with_preferred_regions(
    builder: *mut DriverOptionsBuilderHandle,
    regions: *const *const c_char,
    regions_len: usize,
) -> i32 {
    let Some(inner) = DriverOptionsBuilderHandle::inner_mut(builder) else {
        return CosmosErrorCode::CosmosErrorCodeInvalidArgument.as_i32();
    };

    // SAFETY: caller contract on the region array pointer + length.
    let owned = match unsafe { decode_preferred_regions(regions, regions_len) } {
        Ok(v) => v,
        Err(code) => return code.as_i32(),
    };

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

// ─────────────────────────────────────────────────────────────────────────────
// Flat single-call construction (cosmos_driver_options_config_t /
// cosmos_driver_options_build)
//
// Per docs/DATA_MOVEMENT_MODEL.md: a host fills out one flat `#[repr(C)]`
// struct and hands it across the boundary in a single
// `cosmos_driver_options_build` call, instead of `_builder_new` + the
// per-field setters + `_builder_build`. The account stays a handle (it owns
// `Arc`-shared state and cannot round-trip as bytes). The incremental builder
// above is retained for back-compat; its removal is a sign-off decision (see
// the note's open questions).
// ─────────────────────────────────────────────────────────────────────────────

/// Flat C ABI config for building a `cosmos_driver_options_t` in a single call.
///
/// All fields are sentinel-encoded so a zeroed struct (or a NULL pointer passed
/// to [`cosmos_driver_options_build`]) means "no preferred regions and the
/// driver's default operation options":
///
/// - `preferred_regions` / `preferred_regions_len`: NULL / `0` = no preferred
///   regions. A non-NULL pointer with `0` length is treated as empty.
/// - `operation_options`: pointer to a flat
///   [`cosmos_operation_options_t`](crate::op_request::CosmosOperationOptions),
///   or NULL to inherit the driver defaults.
///
/// The account reference stays a separate handle parameter on
/// [`cosmos_driver_options_build`] — it owns `Arc`-shared state and cannot be
/// flattened into bytes.
///
/// Construct with [`cosmos_driver_options_config_default`] to obtain an
/// all-unset value, then set the fields you care about.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CosmosDriverOptionsConfig {
    /// Preferred regions for routing — array of NUL-terminated UTF-8 region
    /// names. NULL / `0` length = none.
    pub preferred_regions: *const *const c_char,
    /// Number of entries in `preferred_regions`.
    pub preferred_regions_len: usize,
    /// Per-driver default operation options, or NULL to inherit the driver
    /// defaults.
    pub operation_options: *const crate::op_request::CosmosOperationOptions,
}

/// Returns an all-unset [`CosmosDriverOptionsConfig`] by value. The host
/// mutates the fields it cares about and leaves the rest at their default
/// sentinels.
#[no_mangle]
pub extern "C" fn cosmos_driver_options_config_default() -> CosmosDriverOptionsConfig {
    CosmosDriverOptionsConfig {
        preferred_regions: std::ptr::null(),
        preferred_regions_len: 0,
        operation_options: std::ptr::null(),
    }
}

/// Builds a `cosmos_driver_options_t *` from an account reference and a flat
/// [`CosmosDriverOptionsConfig`] in a single call — the single-call
/// alternative to `cosmos_driver_options_builder_new` + the per-field setters +
/// `cosmos_driver_options_builder_build`.
///
/// # Parameters
///
/// - `account` — non-NULL account reference. Cloned into the options, so
///   freeing `account` afterwards does not invalidate the result.
/// - `config` — the flat config. A NULL pointer means "no preferred regions,
///   default operation options" (equivalent to
///   [`cosmos_driver_options_config_default`]).
/// - `out_options` — on success, receives the new options handle. Must be
///   non-NULL.
///
/// # Returns
///
/// - `SUCCESS` (0) with `*out_options` populated.
/// - `INVALID_ARGUMENT` (1) when `account` or `out_options` is NULL, or a
///   region entry is NULL.
/// - `INVALID_UTF8` (2) when a region name is not valid UTF-8.
/// - `INVALID_OPTION_VALUE` (4014) when an operation-option value is out of
///   range.
#[no_mangle]
pub extern "C" fn cosmos_driver_options_build(
    account: *const AccountRefHandle,
    config: *const CosmosDriverOptionsConfig,
    out_options: *mut *mut DriverOptionsHandle,
) -> i32 {
    if out_options.is_null() {
        return CosmosErrorCode::CosmosErrorCodeInvalidArgument.as_i32();
    }
    let Some(account_inner) = AccountRefHandle::from_ptr(account) else {
        return CosmosErrorCode::CosmosErrorCodeInvalidArgument.as_i32();
    };

    let mut builder = DriverOptionsBuilder::new(account_inner.inner.clone());

    if !config.is_null() {
        // SAFETY: caller guarantees `config` points at a valid
        // `CosmosDriverOptionsConfig` whose pointer fields satisfy their
        // documented contracts for the duration of the call.
        let cfg = unsafe { &*config };

        // SAFETY: caller contract on the region array pointer + length.
        let regions = match unsafe {
            decode_preferred_regions(cfg.preferred_regions, cfg.preferred_regions_len)
        } {
            Ok(v) => v,
            Err(code) => return code.as_i32(),
        };
        if !regions.is_empty() {
            builder = builder.with_preferred_regions(regions);
        }

        if !cfg.operation_options.is_null() {
            // SAFETY: caller guarantees the operation-options struct and its
            // pointer fields per the `cosmos_operation_options_t` contract.
            let driver_opts = match unsafe { (*cfg.operation_options).to_driver() } {
                Ok(o) => o,
                Err(code) => return code.as_i32(),
            };
            builder = builder.with_operation_options(driver_opts);
        }
    }

    let handle = DriverOptionsHandle::into_raw(builder.build());
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

    // ── Flat single-call construction (cosmos_driver_options_build) ──

    #[test]
    fn config_default_is_all_unset() {
        let c = cosmos_driver_options_config_default();
        assert!(c.preferred_regions.is_null());
        assert_eq!(c.preferred_regions_len, 0);
        assert!(c.operation_options.is_null());
    }

    #[test]
    fn flat_build_rejects_null_args() {
        let mut opts: *mut DriverOptionsHandle = ptr::null_mut();
        let cfg = cosmos_driver_options_config_default();
        // NULL account.
        assert_eq!(
            cosmos_driver_options_build(ptr::null(), &cfg, &mut opts),
            CosmosErrorCode::CosmosErrorCodeInvalidArgument.as_i32()
        );
        assert!(opts.is_null());
        // NULL out_options.
        let account = make_account();
        assert_eq!(
            cosmos_driver_options_build(account, &cfg, ptr::null_mut()),
            CosmosErrorCode::CosmosErrorCodeInvalidArgument.as_i32()
        );
        crate::account_ref::cosmos_account_ref_free(account);
    }

    #[test]
    fn flat_build_null_config_builds_empty() {
        let account = make_account();
        let mut opts: *mut DriverOptionsHandle = ptr::null_mut();
        // A NULL config means "no regions, default operation options".
        assert_eq!(
            cosmos_driver_options_build(account, ptr::null(), &mut opts),
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
    fn flat_build_with_regions_and_options_roundtrips() {
        let account = make_account();
        let r1 = ok_cstr("East US");
        let r2 = ok_cstr("West US 3");
        let arr: [*const c_char; 2] = [r1.as_ptr(), r2.as_ptr()];
        let op_opts = crate::op_request::cosmos_operation_options_default();

        let mut cfg = cosmos_driver_options_config_default();
        cfg.preferred_regions = arr.as_ptr();
        cfg.preferred_regions_len = arr.len();
        cfg.operation_options = &op_opts;

        let mut opts: *mut DriverOptionsHandle = ptr::null_mut();
        assert_eq!(
            cosmos_driver_options_build(account, &cfg, &mut opts),
            CosmosErrorCode::CosmosErrorCodeSuccess.as_i32()
        );
        let inner = DriverOptionsHandle::inner_arc(opts).unwrap();
        let names: Vec<&str> = inner
            .inner
            .preferred_regions()
            .iter()
            .map(|r| r.as_str())
            .collect();
        // Region::new normalizes (lowercased, spaces stripped) — same as the
        // incremental setter's roundtrip test.
        assert_eq!(names, vec!["eastus", "westus3"]);
        drop(inner);
        cosmos_driver_options_free(opts);
        crate::account_ref::cosmos_account_ref_free(account);
    }

    #[test]
    fn flat_build_rejects_null_region_entry() {
        let account = make_account();
        let arr: [*const c_char; 1] = [ptr::null()];
        let mut cfg = cosmos_driver_options_config_default();
        cfg.preferred_regions = arr.as_ptr();
        cfg.preferred_regions_len = 1;
        let mut opts: *mut DriverOptionsHandle = ptr::null_mut();
        assert_eq!(
            cosmos_driver_options_build(account, &cfg, &mut opts),
            CosmosErrorCode::CosmosErrorCodeInvalidArgument.as_i32()
        );
        assert!(opts.is_null());
        crate::account_ref::cosmos_account_ref_free(account);
    }
}

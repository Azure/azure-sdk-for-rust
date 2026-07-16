// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! C ABI surface for `cosmos_driver_options_t` — wraps the driver's
//! [`azure_data_cosmos_driver::options::DriverOptions`].
//!
//! `DriverOptions` itself is small (3 fields per spec section 4.2): the bound
//! account, the per-driver `OperationOptions`, and a `Vec<Region>` of
//! preferred regions. Construction is a single flat call: the host fills a
//! [`CosmosDriverOptionsConfig`] `#[repr(C)]` struct (preferred regions + a
//! pointer to the flat `cosmos_operation_options_t`) and passes it, together
//! with the account handle, to [`cosmos_driver_options_build`]. Drivers that
//! don't configure operation options inherit the driver's own defaults.
//!
//! The settings frequently confused with "per-driver" defaults
//! (excluded regions, consistency, content-response-on-write,
//! throughput control, retry counts, etc.) all live on
//! `OperationOptions` and reach this surface via the config's
//! `operation_options` pointer — not as additional fields on the driver
//! options.
//!
//! Transport-side knobs (connection pool, user-agent suffix, workload
//! id, correlation id) live on the runtime options, not here.
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
// Helpers
// ─────────────────────────────────────────────────────────────────────────────

/// Decodes a `(ptr, len)` array of NUL-terminated UTF-8 region names into a
/// `Vec<Region>` for the flat [`cosmos_driver_options_build`].
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

// ─────────────────────────────────────────────────────────────────────────────
// Flat single-call construction (cosmos_driver_options_config_t /
// cosmos_driver_options_build)
//
// Per docs/DATA_MOVEMENT_MODEL.md: a host fills out one flat `#[repr(C)]`
// struct and hands it across the boundary in a single
// `cosmos_driver_options_build` call. The account stays a handle (it owns
// `Arc`-shared state and cannot round-trip as bytes). This is the only
// driver-options-construction surface — the per-field incremental builder was
// removed in P5 (no back-compat).
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
/// [`CosmosDriverOptionsConfig`] in a single call.
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
        cosmos_driver_options_free(ptr::null_mut());
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

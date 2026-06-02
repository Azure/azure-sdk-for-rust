// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! C ABI surface for `cosmos_account_ref_t` — wraps the driver's
//! [`azure_data_cosmos_driver::models::AccountReference`].
//!
//! Phase 3 ships only the master-key path. Token-credential
//! (`AccountReference::with_credential`) and resource-token paths require an
//! FFI bridge for `Arc<dyn TokenCredential>` (an async trait whose
//! implementations live in `azure_identity`) — bridging an arbitrary C-side
//! async credential through FFI is non-trivial and is intentionally
//! deferred to a Phase 3+ follow-up.
//!
//! Construction validates the endpoint URL up-front; a parse failure
//! surfaces [`CosmosErrorCode::CosmosErrorCodeInvalidAccountReference`]
//! (4003) and populates the optional `out_error` slot with a rich
//! description for diagnostics.
//!
//! See [`docs/NATIVE_WRAPPER_SPEC.md`] §4.3.
//!
//! [`docs/NATIVE_WRAPPER_SPEC.md`]: https://github.com/Azure/azure-sdk-for-rust/blob/main/sdk/cosmos/azure_data_cosmos_driver/docs/NATIVE_WRAPPER_SPEC.md

use std::ffi::{c_char, CStr};
use std::sync::Arc;

use azure_core::credentials::Secret;
use azure_data_cosmos_driver::models::AccountReference as DriverAccountReference;
use url::Url;

use crate::error::{CosmosErrorCode, CosmosErrorHandle};

/// Internal storage shared between cloned FFI handles.
///
/// The driver's `AccountReference` is itself cheap to clone (the inner
/// state is `Arc`-shared on its side too), but holding it through our own
/// `Arc` lets `cosmos_account_ref_clone` mint a sibling FFI handle without
/// touching the driver type. It also keeps the FFI handle's storage shape
/// consistent with the other reference types (`DatabaseReference`,
/// `ContainerReference` to come) that don't carry their own internal Arc.
pub(crate) struct AccountRefInner {
    pub(crate) inner: DriverAccountReference,
}

/// Opaque C ABI handle for [`AccountRefInner`].
///
/// Storage pun: see the matching pattern on `RuntimeContext` and
/// `RuntimeBuilderHandle`. The public `#[repr(C)]` struct only carries the
/// `_opaque` marker; the real `Arc` lives in the trailing
/// `AccountRefStorage` field allocated by [`AccountRefHandle::into_raw`].
#[repr(C)]
pub struct AccountRefHandle {
    _opaque: [u8; 0],
}

#[repr(C)]
struct AccountRefStorage {
    _opaque: [u8; 0],
    inner: Arc<AccountRefInner>,
}

impl AccountRefHandle {
    /// Allocates a fresh FFI handle wrapping the supplied driver reference.
    fn into_raw(inner: DriverAccountReference) -> *mut Self {
        let storage = Box::new(AccountRefStorage {
            _opaque: [],
            inner: Arc::new(AccountRefInner { inner }),
        });
        Box::into_raw(storage).cast::<AccountRefHandle>()
    }

    fn from_arc_into_raw(inner: Arc<AccountRefInner>) -> *mut Self {
        let storage = Box::new(AccountRefStorage { _opaque: [], inner });
        Box::into_raw(storage).cast::<AccountRefHandle>()
    }

    /// Borrows the inner `Arc` for the duration of an FFI call. Returns
    /// `None` for a NULL pointer; the borrow is otherwise valid for the
    /// lifetime the caller scopes it to.
    pub(crate) fn inner_arc(p: *const AccountRefHandle) -> Option<Arc<AccountRefInner>> {
        if p.is_null() {
            return None;
        }
        // SAFETY: caller guarantees `p` was obtained from `into_raw` and
        // has not been freed.
        let storage = unsafe { &*(p as *const AccountRefStorage) };
        Some(Arc::clone(&storage.inner))
    }

    fn drop_raw(p: *mut AccountRefHandle) {
        if p.is_null() {
            return;
        }
        // SAFETY: pun back into the `Box<AccountRefStorage>` we originally
        // allocated.
        unsafe {
            drop(Box::from_raw(p.cast::<AccountRefStorage>()));
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// FFI helpers
// ─────────────────────────────────────────────────────────────────────────────

/// Borrows a NUL-terminated UTF-8 string from C. Returns
/// `Err(INVALID_ARGUMENT)` for NULL and `Err(INVALID_UTF8)` for non-UTF-8.
fn try_cstr_to_str<'a>(p: *const c_char) -> Result<&'a str, CosmosErrorCode> {
    if p.is_null() {
        return Err(CosmosErrorCode::CosmosErrorCodeInvalidArgument);
    }
    // SAFETY: caller contract on every public setter.
    let cstr = unsafe { CStr::from_ptr(p) };
    cstr.to_str()
        .map_err(|_| CosmosErrorCode::CosmosErrorCodeInvalidUtf8)
}

/// Populates `*out_error` with a driver `CosmosError` when the caller
/// supplied a non-NULL slot. NULL slots silently drop the error so the
/// `out_error` parameter remains optional from the C side.
fn write_optional_error(
    out_error: *mut *mut CosmosErrorHandle,
    err: azure_data_cosmos_driver::error::CosmosError,
) {
    if out_error.is_null() {
        return;
    }
    // SAFETY: caller contract — `out_error` is writable for one `*mut
    // CosmosErrorHandle` when non-NULL.
    unsafe {
        *out_error = CosmosErrorHandle::into_raw(err);
    }
}

/// Parses the endpoint URL and populates `out_error` on failure, returning
/// `Err(INVALID_ACCOUNT_REFERENCE)` so callers map directly to the C code.
fn parse_endpoint(
    endpoint_str: &str,
    out_error: *mut *mut CosmosErrorHandle,
) -> Result<Url, CosmosErrorCode> {
    match Url::parse(endpoint_str) {
        Ok(u) => Ok(u),
        Err(e) => {
            let driver_err = azure_data_cosmos_driver::error::CosmosError::builder()
                .with_status(
                    azure_data_cosmos_driver::error::CosmosStatus::CLIENT_INVALID_ACCOUNT_ENDPOINT_URL,
                )
                .with_message(format!("failed to parse account endpoint URL: {e}"))
                .build();
            write_optional_error(out_error, driver_err);
            Err(CosmosErrorCode::CosmosErrorCodeInvalidAccountReference)
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// FFI: constructors
// ─────────────────────────────────────────────────────────────────────────────

/// Creates an account reference authenticated by a Cosmos master key.
///
/// Mirrors
/// [`azure_data_cosmos_driver::models::AccountReference::with_master_key`].
///
/// # Parameters
///
/// - `endpoint` — NUL-terminated UTF-8 service endpoint URL (e.g.
///   `https://myaccount.documents.azure.com:443/`). Must be non-NULL.
/// - `key` — NUL-terminated UTF-8 master key. Must be non-NULL. The
///   key is copied into a [`Secret`] on the Rust side; the caller may
///   free its copy immediately after this call returns.
/// - `out_account` — receives the new FFI handle on success. Must be
///   non-NULL.
/// - `out_error` — optional. On `INVALID_*` failures receives a rich
///   `cosmos_error_t *` describing the failure. NULL silently drops it.
///
/// # Returns
///
/// - `SUCCESS` (0) with `*out_account` populated.
/// - `INVALID_ARGUMENT` (1) when `endpoint`, `key`, or `out_account` is
///   NULL.
/// - `INVALID_UTF8` (2) when `endpoint` or `key` is not valid UTF-8.
/// - `INVALID_ACCOUNT_REFERENCE` (4003) when `endpoint` is not a parsable
///   URL. `*out_error` is populated when non-NULL.
#[no_mangle]
pub extern "C" fn cosmos_account_ref_with_master_key(
    endpoint: *const c_char,
    key: *const c_char,
    out_account: *mut *mut AccountRefHandle,
    out_error: *mut *mut CosmosErrorHandle,
) -> i32 {
    if out_account.is_null() {
        return CosmosErrorCode::CosmosErrorCodeInvalidArgument.as_i32();
    }
    let endpoint_str = match try_cstr_to_str(endpoint) {
        Ok(s) => s,
        Err(code) => return code.as_i32(),
    };
    let key_str = match try_cstr_to_str(key) {
        Ok(s) => s,
        Err(code) => return code.as_i32(),
    };

    let url = match parse_endpoint(endpoint_str, out_error) {
        Ok(u) => u,
        Err(code) => return code.as_i32(),
    };

    // Copy the key into a `String` so the resulting `Secret` owns its
    // bytes (the C caller may free its copy after this call returns).
    let secret = Secret::from(key_str.to_owned());
    let driver_ref = DriverAccountReference::with_master_key(url, secret);
    let handle = AccountRefHandle::into_raw(driver_ref);
    // SAFETY: caller guarantees `out_account` is writable for one
    // `*mut AccountRefHandle`.
    unsafe {
        *out_account = handle;
    }
    CosmosErrorCode::CosmosErrorCodeSuccess.as_i32()
}

/// Clones an existing account reference into a fresh FFI handle that
/// shares the underlying state.
///
/// Cheap — an atomic refcount bump on a single `Arc`. Never touches the
/// network.
#[no_mangle]
pub extern "C" fn cosmos_account_ref_clone(
    account: *const AccountRefHandle,
    out_clone: *mut *mut AccountRefHandle,
) -> i32 {
    if out_clone.is_null() {
        return CosmosErrorCode::CosmosErrorCodeInvalidArgument.as_i32();
    }
    let Some(arc) = AccountRefHandle::inner_arc(account) else {
        return CosmosErrorCode::CosmosErrorCodeInvalidArgument.as_i32();
    };
    let cloned = AccountRefHandle::from_arc_into_raw(arc);
    // SAFETY: caller guarantees `out_clone` is writable for one
    // `*mut AccountRefHandle`.
    unsafe {
        *out_clone = cloned;
    }
    CosmosErrorCode::CosmosErrorCodeSuccess.as_i32()
}

/// Frees an account-reference handle. NULL is a no-op.
#[no_mangle]
pub extern "C" fn cosmos_account_ref_free(account: *mut AccountRefHandle) {
    if account.is_null() {
        return;
    }
    tracing::trace!(?account, "freeing cosmos_account_ref_t");
    AccountRefHandle::drop_raw(account);
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use std::ffi::CString;
    use std::ptr;

    fn ok_cstr(s: &str) -> CString {
        CString::new(s).expect("test inputs must be NUL-free")
    }

    /// Constructs a default master-key handle and returns it via the same
    /// FFI surface external callers use. Panics on failure so the test
    /// short-circuits early.
    pub(crate) fn make_master_key_handle(endpoint: &str, key: &str) -> *mut AccountRefHandle {
        let ep = ok_cstr(endpoint);
        let k = ok_cstr(key);
        let mut out: *mut AccountRefHandle = ptr::null_mut();
        let mut err: *mut CosmosErrorHandle = ptr::null_mut();
        let rc = cosmos_account_ref_with_master_key(ep.as_ptr(), k.as_ptr(), &mut out, &mut err);
        assert_eq!(rc, CosmosErrorCode::CosmosErrorCodeSuccess.as_i32());
        assert!(!out.is_null());
        assert!(err.is_null());
        out
    }

    #[test]
    fn free_handles_null() {
        cosmos_account_ref_free(ptr::null_mut());
    }

    #[test]
    fn with_master_key_happy_path() {
        let handle = make_master_key_handle(
            "https://myaccount.documents.azure.com:443/",
            "fake-master-key",
        );
        cosmos_account_ref_free(handle);
    }

    #[test]
    fn with_master_key_rejects_null_arguments() {
        let s = ok_cstr("https://x.documents.azure.com:443/");
        let k = ok_cstr("k");
        let mut out: *mut AccountRefHandle = ptr::null_mut();
        let mut err: *mut CosmosErrorHandle = ptr::null_mut();

        assert_eq!(
            cosmos_account_ref_with_master_key(ptr::null(), k.as_ptr(), &mut out, &mut err),
            CosmosErrorCode::CosmosErrorCodeInvalidArgument.as_i32()
        );
        assert_eq!(
            cosmos_account_ref_with_master_key(s.as_ptr(), ptr::null(), &mut out, &mut err),
            CosmosErrorCode::CosmosErrorCodeInvalidArgument.as_i32()
        );
        assert_eq!(
            cosmos_account_ref_with_master_key(s.as_ptr(), k.as_ptr(), ptr::null_mut(), &mut err),
            CosmosErrorCode::CosmosErrorCodeInvalidArgument.as_i32()
        );
        assert!(out.is_null());
        assert!(err.is_null());
    }

    #[test]
    fn with_master_key_rejects_invalid_endpoint() {
        let bad = ok_cstr("not a url");
        let k = ok_cstr("k");
        let mut out: *mut AccountRefHandle = ptr::null_mut();
        let mut err: *mut CosmosErrorHandle = ptr::null_mut();
        let rc = cosmos_account_ref_with_master_key(bad.as_ptr(), k.as_ptr(), &mut out, &mut err);
        assert_eq!(
            rc,
            CosmosErrorCode::CosmosErrorCodeInvalidAccountReference.as_i32()
        );
        assert!(out.is_null(), "no handle on failure");
        assert!(!err.is_null(), "rich error populated on parse failure");
        crate::error::cosmos_error_free(err);
    }

    #[test]
    fn with_master_key_tolerates_null_out_error() {
        // out_error == NULL must silently drop the rich error on failure.
        let bad = ok_cstr("not a url");
        let k = ok_cstr("k");
        let mut out: *mut AccountRefHandle = ptr::null_mut();
        let rc =
            cosmos_account_ref_with_master_key(bad.as_ptr(), k.as_ptr(), &mut out, ptr::null_mut());
        assert_eq!(
            rc,
            CosmosErrorCode::CosmosErrorCodeInvalidAccountReference.as_i32()
        );
        assert!(out.is_null());
    }

    #[test]
    fn clone_rejects_null_arguments() {
        let h = make_master_key_handle("https://x.documents.azure.com:443/", "k");
        let mut out: *mut AccountRefHandle = ptr::null_mut();
        assert_eq!(
            cosmos_account_ref_clone(ptr::null(), &mut out),
            CosmosErrorCode::CosmosErrorCodeInvalidArgument.as_i32()
        );
        assert_eq!(
            cosmos_account_ref_clone(h, ptr::null_mut()),
            CosmosErrorCode::CosmosErrorCodeInvalidArgument.as_i32()
        );
        cosmos_account_ref_free(h);
    }

    #[test]
    fn clone_shares_inner_arc() {
        let h = make_master_key_handle("https://x.documents.azure.com:443/", "k");
        let mut h2: *mut AccountRefHandle = ptr::null_mut();
        assert_eq!(
            cosmos_account_ref_clone(h, &mut h2),
            CosmosErrorCode::CosmosErrorCodeSuccess.as_i32()
        );
        assert!(!h2.is_null());

        // Both handles see the same underlying Arc (strong count >= 2).
        let arc1 = AccountRefHandle::inner_arc(h).unwrap();
        let arc2 = AccountRefHandle::inner_arc(h2).unwrap();
        assert!(Arc::ptr_eq(&arc1, &arc2), "clone shares underlying Arc");
        drop(arc1);
        drop(arc2);

        cosmos_account_ref_free(h);
        cosmos_account_ref_free(h2);
    }
}

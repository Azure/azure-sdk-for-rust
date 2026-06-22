// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! C ABI surface for `cosmos_database_ref_t` — wraps the driver's
//! [`azure_data_cosmos_driver::models::DatabaseReference`].
//!
//! The wrapper currently ships only the name-based constructor
//! (`DatabaseReference::from_name`). The RID-based constructor (`from_rid`)
//! is mechanically identical but is intentionally deferred — host SDKs that
//! hold a RID invariably obtained it through a resolved container response.
//!
//! `cosmos_container_ref_*` is built from the response surface, not here. The
//! driver's `ContainerReference::new` is `pub(crate)`-only and demands the
//! container's RID + partition-key definition, which are obtained via
//! `CosmosDriver::resolve_container` — an async, network-touching call.
//! Container handles therefore arrive alongside the response surface that
//! delivers a resolved container.
//!
//! See [`docs/NATIVE_WRAPPER_SPEC.md`] section 4.3.
//!
//! [`docs/NATIVE_WRAPPER_SPEC.md`]: https://github.com/Azure/azure-sdk-for-rust/blob/main/sdk/cosmos/azure_data_cosmos_driver/docs/NATIVE_WRAPPER_SPEC.md

use std::ffi::{c_char, CStr};
use std::sync::Arc;

use azure_data_cosmos_driver::models::DatabaseReference as DriverDatabaseReference;

use crate::account_ref::AccountRefHandle;
use crate::error::CosmosErrorCode;

pub(crate) struct DatabaseRefInner {
    /// Consumed by the operation request builder when it takes a database
    /// reference. Tests read it directly via `DatabaseRefHandle::inner_arc`
    /// to assert the wire shape.
    #[allow(
        dead_code,
        reason = "first non-test caller is the operation request builder"
    )]
    pub(crate) inner: DriverDatabaseReference,
}

/// Opaque C ABI handle for `DatabaseRefInner`.
///
/// Storage pun: see the matching pattern on `AccountRefHandle`.
#[repr(C)]
pub struct DatabaseRefHandle {
    _opaque: [u8; 0],
}

#[repr(C)]
struct DatabaseRefStorage {
    _opaque: [u8; 0],
    inner: Arc<DatabaseRefInner>,
}

impl DatabaseRefHandle {
    fn into_raw(inner: DriverDatabaseReference) -> *mut Self {
        let storage = Box::new(DatabaseRefStorage {
            _opaque: [],
            inner: Arc::new(DatabaseRefInner { inner }),
        });
        Box::into_raw(storage).cast::<DatabaseRefHandle>()
    }

    fn from_arc_into_raw(inner: Arc<DatabaseRefInner>) -> *mut Self {
        let storage = Box::new(DatabaseRefStorage { _opaque: [], inner });
        Box::into_raw(storage).cast::<DatabaseRefHandle>()
    }

    #[allow(
        dead_code,
        reason = "first non-test caller is the operation request builder"
    )]
    pub(crate) fn inner_arc(p: *const DatabaseRefHandle) -> Option<Arc<DatabaseRefInner>> {
        if p.is_null() {
            return None;
        }
        // SAFETY: caller guarantees `p` was obtained from `into_raw` and
        // has not been freed.
        let storage = unsafe { &*(p as *const DatabaseRefStorage) };
        Some(Arc::clone(&storage.inner))
    }

    fn drop_raw(p: *mut DatabaseRefHandle) {
        if p.is_null() {
            return;
        }
        // SAFETY: pun back into the `Box<DatabaseRefStorage>` we
        // originally allocated.
        unsafe {
            drop(Box::from_raw(p.cast::<DatabaseRefStorage>()));
        }
    }
}

fn try_cstr_to_str<'a>(p: *const c_char) -> Result<&'a str, CosmosErrorCode> {
    if p.is_null() {
        return Err(CosmosErrorCode::CosmosErrorCodeInvalidArgument);
    }
    // SAFETY: caller contract on every public setter.
    let cstr = unsafe { CStr::from_ptr(p) };
    cstr.to_str()
        .map_err(|_| CosmosErrorCode::CosmosErrorCodeInvalidUtf8)
}

/// Creates a name-based database reference parented to `account`.
///
/// Mirrors `DatabaseReference::from_name`. Pure value-type construction;
/// never touches the network. The supplied `account` is cloned into the
/// new database reference, so freeing `account` after this call does not
/// invalidate the database handle.
///
/// # Parameters
///
/// - `account` — parent account reference. Must be non-NULL.
/// - `database_id` — NUL-terminated UTF-8 database name. Must be
///   non-NULL.
/// - `out_database` — receives the new FFI handle on success. Must be
///   non-NULL.
///
/// # Returns
///
/// - `SUCCESS` (0) with `*out_database` populated.
/// - `INVALID_ARGUMENT` (1) when `account`, `database_id`, or
///   `out_database` is NULL.
/// - `INVALID_UTF8` (2) when `database_id` is not valid UTF-8.
#[no_mangle]
pub extern "C" fn cosmos_database_ref_create(
    account: *const AccountRefHandle,
    database_id: *const c_char,
    out_database: *mut *mut DatabaseRefHandle,
) -> i32 {
    if out_database.is_null() {
        return CosmosErrorCode::CosmosErrorCodeInvalidArgument.as_i32();
    }
    let Some(account_inner) = AccountRefHandle::inner_arc(account) else {
        return CosmosErrorCode::CosmosErrorCodeInvalidArgument.as_i32();
    };
    let name = match try_cstr_to_str(database_id) {
        Ok(s) => s,
        Err(code) => return code.as_i32(),
    };

    // `from_name` accepts any `Into<Cow<'static, str>>` — owned `String`
    // (cloned from the C buffer) becomes `Cow::Owned` and keeps the
    // database reference independent of the caller's buffer.
    let driver_ref =
        DriverDatabaseReference::from_name(account_inner.inner.clone(), name.to_owned());
    let handle = DatabaseRefHandle::into_raw(driver_ref);
    // SAFETY: caller guarantees `out_database` is writable for one
    // `*mut DatabaseRefHandle`.
    unsafe {
        *out_database = handle;
    }
    CosmosErrorCode::CosmosErrorCodeSuccess.as_i32()
}

/// Clones an existing database reference into a fresh FFI handle that
/// shares the underlying state.
///
/// Cheap — an atomic refcount bump on a single `Arc`.
#[no_mangle]
pub extern "C" fn cosmos_database_ref_clone(
    database: *const DatabaseRefHandle,
    out_clone: *mut *mut DatabaseRefHandle,
) -> i32 {
    if out_clone.is_null() {
        return CosmosErrorCode::CosmosErrorCodeInvalidArgument.as_i32();
    }
    let Some(arc) = DatabaseRefHandle::inner_arc(database) else {
        return CosmosErrorCode::CosmosErrorCodeInvalidArgument.as_i32();
    };
    let cloned = DatabaseRefHandle::from_arc_into_raw(arc);
    // SAFETY: caller guarantees `out_clone` is writable for one
    // `*mut DatabaseRefHandle`.
    unsafe {
        *out_clone = cloned;
    }
    CosmosErrorCode::CosmosErrorCodeSuccess.as_i32()
}

/// Frees a database-reference handle. NULL is a no-op.
#[no_mangle]
pub extern "C" fn cosmos_database_ref_free(database: *mut DatabaseRefHandle) {
    if database.is_null() {
        return;
    }
    tracing::trace!(?database, "freeing cosmos_database_ref_t");
    DatabaseRefHandle::drop_raw(database);
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
    fn free_handles_null() {
        cosmos_database_ref_free(ptr::null_mut());
    }

    #[test]
    fn create_happy_path() {
        let account = make_account();
        let db_id = ok_cstr("mydb");
        let mut out: *mut DatabaseRefHandle = ptr::null_mut();
        let rc = cosmos_database_ref_create(account, db_id.as_ptr(), &mut out);
        assert_eq!(rc, CosmosErrorCode::CosmosErrorCodeSuccess.as_i32());
        assert!(!out.is_null());

        // Inner reference has the expected name.
        let inner = DatabaseRefHandle::inner_arc(out).unwrap();
        assert_eq!(inner.inner.name(), Some("mydb"));
        drop(inner);

        cosmos_database_ref_free(out);
        cosmos_account_ref_free_for_tests(account);
    }

    #[test]
    fn create_rejects_null_arguments() {
        let account = make_account();
        let db_id = ok_cstr("mydb");
        let mut out: *mut DatabaseRefHandle = ptr::null_mut();
        assert_eq!(
            cosmos_database_ref_create(ptr::null(), db_id.as_ptr(), &mut out),
            CosmosErrorCode::CosmosErrorCodeInvalidArgument.as_i32()
        );
        assert_eq!(
            cosmos_database_ref_create(account, ptr::null(), &mut out),
            CosmosErrorCode::CosmosErrorCodeInvalidArgument.as_i32()
        );
        assert_eq!(
            cosmos_database_ref_create(account, db_id.as_ptr(), ptr::null_mut()),
            CosmosErrorCode::CosmosErrorCodeInvalidArgument.as_i32()
        );
        cosmos_account_ref_free_for_tests(account);
    }

    #[test]
    fn clone_shares_inner_arc() {
        let account = make_account();
        let db_id = ok_cstr("mydb");
        let mut h: *mut DatabaseRefHandle = ptr::null_mut();
        cosmos_database_ref_create(account, db_id.as_ptr(), &mut h);

        let mut h2: *mut DatabaseRefHandle = ptr::null_mut();
        assert_eq!(
            cosmos_database_ref_clone(h, &mut h2),
            CosmosErrorCode::CosmosErrorCodeSuccess.as_i32()
        );
        let a = DatabaseRefHandle::inner_arc(h).unwrap();
        let b = DatabaseRefHandle::inner_arc(h2).unwrap();
        assert!(Arc::ptr_eq(&a, &b));
        drop((a, b));

        cosmos_database_ref_free(h);
        cosmos_database_ref_free(h2);
        cosmos_account_ref_free_for_tests(account);
    }

    /// Local alias for the cross-module test helper so each test can read
    /// top-to-bottom without ducking back into `account_ref::tests`.
    fn cosmos_account_ref_free_for_tests(p: *mut AccountRefHandle) {
        crate::account_ref::cosmos_account_ref_free(p);
    }
}

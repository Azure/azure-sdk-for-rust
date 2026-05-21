// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Account reference handles (Phase 3 — partial skeleton).
//!
//! Wraps [`azure_data_cosmos_driver::models::AccountReference`].

use std::ffi::c_char;
use std::os::raw::c_void;

use azure_data_cosmos_driver::models::AccountReference;
use url::Url;

use crate::error::{messages, CosmosError, CosmosErrorCode, Error};
use crate::string::parse_cstr;

/// Opaque handle to an [`AccountReference`].
///
/// cbindgen:ignore
#[allow(non_camel_case_types)]
pub struct cosmos_account_ref(pub(crate) AccountReference);

/// Creates a new account reference authenticated by a master key.
///
/// On failure returns null and writes the error to `out_error` (if non-null).
/// The caller releases the returned handle via [`cosmos_account_ref_free`].
///
/// # Safety
/// `endpoint` and `key` must be non-null, NUL-terminated UTF-8 C strings.
#[no_mangle]
pub unsafe extern "C" fn cosmos_account_ref_with_master_key(
    endpoint: *const c_char,
    key: *const c_char,
    out_error: *mut CosmosError,
) -> *mut cosmos_account_ref {
    let r: Result<cosmos_account_ref, Error> = (|| {
        let endpoint = parse_cstr(endpoint, messages::INVALID_ENDPOINT)?;
        let key = parse_cstr(key, messages::INVALID_KEY)?.to_string();
        let url = Url::parse(endpoint).map_err(|e| {
            Error::with_detail(
                CosmosErrorCode::InvalidAccountReference,
                messages::INVALID_ENDPOINT,
                e,
            )
        })?;
        Ok(cosmos_account_ref(AccountReference::with_master_key(
            url, key,
        )))
    })();
    match r {
        Ok(h) => Box::into_raw(Box::new(h)),
        Err(e) => {
            if !out_error.is_null() {
                *out_error = e.into_ffi(true);
            }
            std::ptr::null_mut()
        }
    }
}

/// Releases an account reference handle.
///
/// # Safety
/// `account` must be null or a pointer previously returned by
/// `cosmos_account_ref_with_*`.
#[no_mangle]
pub unsafe extern "C" fn cosmos_account_ref_free(account: *mut cosmos_account_ref) {
    if !account.is_null() {
        drop(Box::from_raw(account));
    }
}

// Suppress unused-import warning when no other call sites exist yet.
#[allow(dead_code)]
fn _phantom(_: *mut c_void) {}

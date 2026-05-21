// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Partition key handles (Phase 4).
//!
//! Provides a fluent builder for constructing single and hierarchical
//! partition keys spanning all five JSON value types.

use std::os::raw::c_char;

use azure_data_cosmos_driver::models::{PartitionKey, PartitionKeyValue};

use crate::error::{messages, CosmosError, CosmosErrorCode};
use crate::string::parse_cstr;

/// Opaque, owned partition key handle. Built via
/// [`cosmos_partition_key_builder_new`] + appends + `_build`, or via the
/// convenience [`cosmos_partition_key_from_string`].
///
/// cbindgen:ignore
#[allow(non_camel_case_types)]
pub struct cosmos_partition_key(pub(crate) PartitionKey);

/// Opaque builder. A successful [`cosmos_partition_key_builder_build`]
/// consumes the builder; the caller must NOT free the builder afterwards.
///
/// cbindgen:ignore
#[allow(non_camel_case_types)]
pub struct cosmos_partition_key_builder(Vec<PartitionKeyValue>);

/// Creates an empty partition-key builder.
#[no_mangle]
pub extern "C" fn cosmos_partition_key_builder_new() -> *mut cosmos_partition_key_builder {
    Box::into_raw(Box::new(cosmos_partition_key_builder(Vec::new())))
}

/// Releases a builder that was never built.
///
/// # Safety
/// `b` must be null or a pointer returned by [`cosmos_partition_key_builder_new`].
#[no_mangle]
pub unsafe extern "C" fn cosmos_partition_key_builder_free(b: *mut cosmos_partition_key_builder) {
    if !b.is_null() {
        drop(Box::from_raw(b));
    }
}

fn with_builder(
    b: *mut cosmos_partition_key_builder,
    f: impl FnOnce(&mut Vec<PartitionKeyValue>),
) -> CosmosErrorCode {
    if b.is_null() {
        return CosmosErrorCode::InvalidArgument;
    }
    let b = unsafe { &mut *b };
    f(&mut b.0);
    CosmosErrorCode::Success
}

/// Appends a string component to the partition key.
///
/// # Safety
/// `value` must be a non-null, NUL-terminated UTF-8 C string.
#[no_mangle]
pub unsafe extern "C" fn cosmos_partition_key_builder_append_string(
    b: *mut cosmos_partition_key_builder,
    value: *const c_char,
) -> CosmosErrorCode {
    let s = match parse_cstr(value, messages::INVALID_PARTITION_KEY) {
        Ok(s) => s.to_string(),
        Err(_) => return CosmosErrorCode::InvalidArgument,
    };
    with_builder(b, |v| v.push(PartitionKeyValue::from(s)))
}

/// Appends a numeric component.
#[no_mangle]
pub extern "C" fn cosmos_partition_key_builder_append_number(
    b: *mut cosmos_partition_key_builder,
    value: f64,
) -> CosmosErrorCode {
    with_builder(b, |v| v.push(PartitionKeyValue::from(value)))
}

/// Appends a boolean component.
#[no_mangle]
pub extern "C" fn cosmos_partition_key_builder_append_bool(
    b: *mut cosmos_partition_key_builder,
    value: bool,
) -> CosmosErrorCode {
    with_builder(b, |v| v.push(PartitionKeyValue::from(value)))
}

/// Appends a null component.
#[no_mangle]
pub extern "C" fn cosmos_partition_key_builder_append_null(
    b: *mut cosmos_partition_key_builder,
) -> CosmosErrorCode {
    // `PartitionKeyValue::from(Option::<String>::None)` produces a Null value
    // (see the `From<Option<T>>` impl in the driver).
    with_builder(b, |v| {
        v.push(PartitionKeyValue::from(Option::<String>::None))
    })
}

/// Appends an "undefined" component, representing an item with no value at
/// the partition key path.
#[no_mangle]
pub extern "C" fn cosmos_partition_key_builder_append_undefined(
    b: *mut cosmos_partition_key_builder,
) -> CosmosErrorCode {
    with_builder(b, |v| v.push(PartitionKeyValue::undefined()))
}

/// Builds a partition key from the accumulated components, consuming the
/// builder. The builder pointer is freed by this call and must NOT be used or
/// re-freed by the caller.
///
/// Returns [`CosmosErrorCode::InvalidPartitionKey`] if the builder is empty.
///
/// # Safety
/// `b` must be a pointer returned by [`cosmos_partition_key_builder_new`].
/// `out_pk` must be non-null.
#[no_mangle]
pub unsafe extern "C" fn cosmos_partition_key_builder_build(
    b: *mut cosmos_partition_key_builder,
    out_pk: *mut *mut cosmos_partition_key,
) -> CosmosErrorCode {
    if b.is_null() || out_pk.is_null() {
        return CosmosErrorCode::InvalidArgument;
    }
    let values = Box::from_raw(b).0;
    if values.is_empty() {
        return CosmosErrorCode::InvalidPartitionKey;
    }
    let pk = PartitionKey::from(values);
    *out_pk = Box::into_raw(Box::new(cosmos_partition_key(pk)));
    CosmosErrorCode::Success
}

/// Convenience: build a single-string partition key in one call.
///
/// # Safety
/// `value` must be a non-null, NUL-terminated UTF-8 C string. `out_pk` must
/// be non-null. `out_error` may be null.
#[no_mangle]
pub unsafe extern "C" fn cosmos_partition_key_from_string(
    value: *const c_char,
    out_pk: *mut *mut cosmos_partition_key,
    out_error: *mut CosmosError,
) -> CosmosErrorCode {
    if out_pk.is_null() {
        return CosmosErrorCode::InvalidArgument;
    }
    match parse_cstr(value, messages::INVALID_PARTITION_KEY) {
        Ok(s) => {
            let pk = PartitionKey::from(s.to_string());
            *out_pk = Box::into_raw(Box::new(cosmos_partition_key(pk)));
            CosmosErrorCode::Success
        }
        Err(e) => {
            if !out_error.is_null() {
                *out_error = e.into_ffi(true);
            }
            CosmosErrorCode::InvalidPartitionKey
        }
    }
}

/// Releases a partition key handle.
///
/// # Safety
/// `pk` must be null or a pointer returned by `cosmos_partition_key_*`.
#[no_mangle]
pub unsafe extern "C" fn cosmos_partition_key_free(pk: *mut cosmos_partition_key) {
    if !pk.is_null() {
        drop(Box::from_raw(pk));
    }
}

// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! C ABI surface for `cosmos_partition_key_t` + `cosmos_partition_key_builder_t`
//! — wraps the driver's [`azure_data_cosmos_driver::models::PartitionKey`]
//! and [`azure_data_cosmos_driver::models::PartitionKeyValue`].
//!
//! Hierarchical partition keys are constructed incrementally: callers
//! create a builder via `cosmos_partition_key_builder_new`, append up to
//! three components with `_add_string` / `_add_number` / `_add_bool` /
//! `_add_null` / `_add_undefined` (in path order), then call
//! `_build` to produce an immutable `PartitionKey` handle. The
//! incremental shape exists for the FFI even though the driver itself
//! offers `From<T>` / `From<(T1, T2)>` / `From<(T1, T2, T3)>` impls —
//! C callers can't construct Rust tuples and need an incremental API.
//!
//! The driver's `From<Vec<PartitionKeyValue>>` impl panics if the
//! vector contains more than 3 elements (Cosmos DB supports at most
//! 3 hierarchical levels); the FFI surface pre-validates the component
//! count on every `_add_*` so callers see a deterministic
//! `INVALID_PARTITION_KEY` (4004) instead of an abort. Likewise,
//! `From<f64>` for `PartitionKeyValue` panics on non-finite values —
//! `_add_number` rejects NaN / ±∞ up-front with
//! `INVALID_OPTION_VALUE` (4014).
//!
//! See [`docs/NATIVE_WRAPPER_SPEC.md`] section 4.5.
//!
//! [`docs/NATIVE_WRAPPER_SPEC.md`]: https://github.com/Azure/azure-sdk-for-rust/blob/main/sdk/cosmos/azure_data_cosmos_driver/docs/NATIVE_WRAPPER_SPEC.md

use std::ffi::{c_char, CStr};

use azure_data_cosmos_driver::models::{PartitionKey as DriverPartitionKey, PartitionKeyValue};

use crate::error::CosmosErrorCode;

// ─────────────────────────────────────────────────────────────────────────────
// Constants
// ─────────────────────────────────────────────────────────────────────────────

/// Cosmos DB supports at most 3 hierarchical partition-key levels.
/// Enforced by `_add_*` pre-flight checks and again on `_build`.
const MAX_COMPONENTS: usize = 3;

// ─────────────────────────────────────────────────────────────────────────────
// Inline tagged-union partition key (cosmos_partition_key_component_t)
// ─────────────────────────────────────────────────────────────────────────────

/// Discriminant for a [`CosmosPartitionKeyComponent`].
///
/// Stored on the component as a raw `i32` (validated, never transmuted), so an
/// out-of-range host value yields `INVALID_OPTION_VALUE` instead of UB.
#[repr(i32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CosmosPartitionKeyComponentKind {
    /// String component — read from `string_value`.
    CosmosPartitionKeyComponentKindString = 0,
    /// Numeric component — read from `number_value` (must be finite).
    CosmosPartitionKeyComponentKindNumber = 1,
    /// Boolean component — read from `bool_value` (`0` = false, else true).
    CosmosPartitionKeyComponentKindBool = 2,
    /// Explicit JSON `null` component — no value field is read.
    CosmosPartitionKeyComponentKindNull = 3,
    /// `undefined` (missing-value) component — no value field is read.
    CosmosPartitionKeyComponentKindUndefined = 4,
}

impl CosmosPartitionKeyComponentKind {
    fn from_i32(raw: i32) -> Result<Self, CosmosErrorCode> {
        Ok(match raw {
            0 => Self::CosmosPartitionKeyComponentKindString,
            1 => Self::CosmosPartitionKeyComponentKindNumber,
            2 => Self::CosmosPartitionKeyComponentKindBool,
            3 => Self::CosmosPartitionKeyComponentKindNull,
            4 => Self::CosmosPartitionKeyComponentKindUndefined,
            _ => return Err(CosmosErrorCode::CosmosErrorCodeInvalidOptionValue),
        })
    }
}

/// One component of a hierarchical partition key, assembled inline by the host
/// (a C-style tagged union: a `kind` tag plus all possible value fields).
///
/// This lets a calling SDK assemble a whole partition key in a single array
/// and drop it straight into [`CosmosOperationRequest`](crate::op_request::CosmosOperationRequest),
/// avoiding the per-component builder FFI round-trips. Only the field selected
/// by `kind` is read; the others are ignored.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CosmosPartitionKeyComponent {
    /// Which value field to read, as a [`CosmosPartitionKeyComponentKind`]
    /// discriminant.
    pub kind: i32,
    /// String payload (NUL-terminated UTF-8). Read iff `kind` is `String`.
    pub string_value: *const c_char,
    /// Numeric payload. Read iff `kind` is `Number`. Must be finite.
    pub number_value: f64,
    /// Boolean payload (`0` = false, non-zero = true). Read iff `kind` is
    /// `Bool`. Taken as `u8` so an arbitrary host byte cannot form an invalid
    /// `bool` (which would be undefined behavior).
    pub bool_value: u8,
}

/// Builds a driver [`PartitionKey`](DriverPartitionKey) from an inline
/// component array carried directly on a request, applying the same
/// validation as the incremental builder (at most [`MAX_COMPONENTS`]
/// components, finite numbers, valid UTF-8 strings).
///
/// # Safety
///
/// `components` must point to `len` initialized [`CosmosPartitionKeyComponent`]
/// values, and each `String` component's `string_value` must be a valid
/// NUL-terminated UTF-8 string that outlives the call.
pub(crate) unsafe fn partition_key_from_components(
    components: *const CosmosPartitionKeyComponent,
    len: usize,
) -> Result<DriverPartitionKey, CosmosErrorCode> {
    if components.is_null() || len == 0 {
        return Err(CosmosErrorCode::CosmosErrorCodeInvalidPartitionKey);
    }
    if len > MAX_COMPONENTS {
        // Mirrors the builder's per-`_add_*` cap so `From<Vec<...>>` (which
        // panics above 3 levels) is never reached with an over-cap vector.
        return Err(CosmosErrorCode::CosmosErrorCodeInvalidPartitionKey);
    }
    // SAFETY: caller guarantees `components` points to `len` initialized values.
    let slice = unsafe { std::slice::from_raw_parts(components, len) };
    let mut values = Vec::with_capacity(len);
    for component in slice {
        let value = match CosmosPartitionKeyComponentKind::from_i32(component.kind)? {
            CosmosPartitionKeyComponentKind::CosmosPartitionKeyComponentKindString => {
                let s = try_cstr_to_str(component.string_value)?;
                PartitionKeyValue::from(s.to_owned())
            }
            CosmosPartitionKeyComponentKind::CosmosPartitionKeyComponentKindNumber => {
                if !component.number_value.is_finite() {
                    // The driver's `From<f64>` panics on NaN / ±∞; reject early.
                    return Err(CosmosErrorCode::CosmosErrorCodeInvalidOptionValue);
                }
                PartitionKeyValue::from(component.number_value)
            }
            CosmosPartitionKeyComponentKind::CosmosPartitionKeyComponentKindBool => {
                PartitionKeyValue::from(component.bool_value != 0)
            }
            CosmosPartitionKeyComponentKind::CosmosPartitionKeyComponentKindNull => {
                PartitionKeyValue::NULL
            }
            CosmosPartitionKeyComponentKind::CosmosPartitionKeyComponentKindUndefined => {
                PartitionKeyValue::UNDEFINED
            }
        };
        values.push(value);
    }
    // Length is capped at <= MAX_COMPONENTS above, so `From<Vec<...>>` will not
    // panic here.
    Ok(DriverPartitionKey::from(values))
}

// ─────────────────────────────────────────────────────────────────────────────
// PartitionKeyBuilderHandle
// ─────────────────────────────────────────────────────────────────────────────

/// The C ABI handle for an incrementally-populated partition-key builder
/// (`cosmos_partition_key_builder_t`).
///
/// Single-owner and `Box`-managed.
pub struct PartitionKeyBuilderHandle {
    pub(crate) components: Vec<PartitionKeyValue>,
}

impl PartitionKeyBuilderHandle {
    fn new_raw() -> *mut Self {
        Box::into_raw(Box::new(PartitionKeyBuilderHandle {
            components: Vec::with_capacity(MAX_COMPONENTS),
        }))
    }

    fn inner_mut<'a>(
        p: *mut PartitionKeyBuilderHandle,
    ) -> Option<&'a mut PartitionKeyBuilderHandle> {
        if p.is_null() {
            return None;
        }
        // SAFETY: caller guarantees `p` was obtained from `new_raw` and
        // has not been freed.
        Some(unsafe { &mut *p })
    }

    fn into_owned(p: *mut PartitionKeyBuilderHandle) -> Option<Vec<PartitionKeyValue>> {
        if p.is_null() {
            return None;
        }
        // SAFETY: reclaim the `Box` and move the components vector out.
        let handle = unsafe { Box::from_raw(p) };
        Some(handle.components)
    }

    fn drop_raw(p: *mut PartitionKeyBuilderHandle) {
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
// PartitionKeyHandle
// ─────────────────────────────────────────────────────────────────────────────

/// The C ABI handle for an immutable partition key (`cosmos_partition_key_t`).
///
/// Owned by the SDK via `Box` single-ownership; freed with
/// `cosmos_partition_key_free`.
pub struct PartitionKeyHandle {
    /// Consumed by the operation factories that take a partition key. Tests
    /// read it directly via `PartitionKeyHandle::from_ptr` to assert the
    /// wire shape.
    pub(crate) inner: DriverPartitionKey,
}

impl PartitionKeyHandle {
    fn into_raw(pk: DriverPartitionKey) -> *mut Self {
        Box::into_raw(Box::new(PartitionKeyHandle { inner: pk }))
    }

    /// Borrows the handle for the duration of an FFI call without taking
    /// ownership. Returns `None` for a NULL pointer.
    pub(crate) fn from_ptr<'a>(p: *const PartitionKeyHandle) -> Option<&'a PartitionKeyHandle> {
        if p.is_null() {
            return None;
        }
        // SAFETY: caller guarantees `p` was obtained from `into_raw` and is
        // not freed for the duration of the borrow.
        Some(unsafe { &*p })
    }

    fn drop_raw(p: *mut PartitionKeyHandle) {
        if p.is_null() {
            return;
        }
        // SAFETY: caller guarantees `p` was obtained from `into_raw` and has
        // not already been freed.
        unsafe {
            drop(Box::from_raw(p));
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// FFI helpers
// ─────────────────────────────────────────────────────────────────────────────

fn try_cstr_to_str<'a>(p: *const c_char) -> Result<&'a str, CosmosErrorCode> {
    if p.is_null() {
        return Err(CosmosErrorCode::CosmosErrorCodeInvalidArgument);
    }
    // SAFETY: caller contract on every public setter.
    let cstr = unsafe { CStr::from_ptr(p) };
    cstr.to_str()
        .map_err(|_| CosmosErrorCode::CosmosErrorCodeInvalidUtf8)
}

/// Common pre-flight: borrow the builder, reject NULL, reject overflow.
fn push_pre_flight<'a>(
    builder: *mut PartitionKeyBuilderHandle,
) -> Result<&'a mut PartitionKeyBuilderHandle, CosmosErrorCode> {
    let Some(inner) = PartitionKeyBuilderHandle::inner_mut(builder) else {
        return Err(CosmosErrorCode::CosmosErrorCodeInvalidArgument);
    };
    if inner.components.len() >= MAX_COMPONENTS {
        // Cosmos DB caps hierarchical partition keys at 3 levels. Reject
        // the over-cap append up-front so callers see a deterministic
        // error code rather than an abort from the driver's
        // `From<Vec<...>>::from` assertion later.
        return Err(CosmosErrorCode::CosmosErrorCodeInvalidOptionValue);
    }
    Ok(inner)
}

// ─────────────────────────────────────────────────────────────────────────────
// FFI: builder lifecycle
// ─────────────────────────────────────────────────────────────────────────────

/// Allocates a new partition-key builder. Always succeeds; the returned
/// handle holds an empty component list.
#[no_mangle]
pub extern "C" fn cosmos_partition_key_builder_new() -> *mut PartitionKeyBuilderHandle {
    PartitionKeyBuilderHandle::new_raw()
}

/// Frees a builder that was never consumed by [`cosmos_partition_key_builder_build`].
/// NULL is a no-op.
#[no_mangle]
pub extern "C" fn cosmos_partition_key_builder_free(builder: *mut PartitionKeyBuilderHandle) {
    if builder.is_null() {
        return;
    }
    tracing::trace!(?builder, "freeing cosmos_partition_key_builder_t");
    PartitionKeyBuilderHandle::drop_raw(builder);
}

// ─────────────────────────────────────────────────────────────────────────────
// FFI: builder setters
//
// Common contract:
//
// - `INVALID_ARGUMENT` (1) — `builder` is NULL.
// - `INVALID_UTF8`     (2) — string setters with non-UTF-8 input.
// - `INVALID_OPTION_VALUE` (4014) — the builder already has the maximum
//   3 components, or `_add_number` was called with a non-finite value
//   (NaN or ±∞).
//
// On any non-`SUCCESS` return the builder is left unchanged.
// ─────────────────────────────────────────────────────────────────────────────

/// Appends a string component to the partition key.
#[no_mangle]
pub extern "C" fn cosmos_partition_key_builder_add_string(
    builder: *mut PartitionKeyBuilderHandle,
    value: *const c_char,
) -> i32 {
    let inner = match push_pre_flight(builder) {
        Ok(i) => i,
        Err(code) => return code.as_i32(),
    };
    let s = match try_cstr_to_str(value) {
        Ok(s) => s,
        Err(code) => return code.as_i32(),
    };
    inner.components.push(PartitionKeyValue::from(s.to_owned()));
    CosmosErrorCode::CosmosErrorCodeSuccess.as_i32()
}

/// Appends a numeric component to the partition key. Rejects NaN and
/// ±∞ with `INVALID_OPTION_VALUE`.
#[no_mangle]
pub extern "C" fn cosmos_partition_key_builder_add_number(
    builder: *mut PartitionKeyBuilderHandle,
    value: f64,
) -> i32 {
    let inner = match push_pre_flight(builder) {
        Ok(i) => i,
        Err(code) => return code.as_i32(),
    };
    if !value.is_finite() {
        // The driver's `From<f64>` for `PartitionKeyValue` routes through
        // `FiniteF64::new_strict` which panics on NaN / ±∞. Reject
        // up-front for a clean error path.
        return CosmosErrorCode::CosmosErrorCodeInvalidOptionValue.as_i32();
    }
    inner.components.push(PartitionKeyValue::from(value));
    CosmosErrorCode::CosmosErrorCodeSuccess.as_i32()
}

/// Appends a boolean component to the partition key.
///
/// `value` is a C boolean (`0` = false, non-zero = true). It is taken as a
/// `u8` rather than a Rust `bool` so an arbitrary host-supplied byte cannot
/// produce an invalid `bool` (which would be undefined behavior).
#[no_mangle]
pub extern "C" fn cosmos_partition_key_builder_add_bool(
    builder: *mut PartitionKeyBuilderHandle,
    value: u8,
) -> i32 {
    let inner = match push_pre_flight(builder) {
        Ok(i) => i,
        Err(code) => return code.as_i32(),
    };
    inner.components.push(PartitionKeyValue::from(value != 0));
    CosmosErrorCode::CosmosErrorCodeSuccess.as_i32()
}

/// Appends an explicit `null` component to the partition key.
#[no_mangle]
pub extern "C" fn cosmos_partition_key_builder_add_null(
    builder: *mut PartitionKeyBuilderHandle,
) -> i32 {
    let inner = match push_pre_flight(builder) {
        Ok(i) => i,
        Err(code) => return code.as_i32(),
    };
    inner.components.push(PartitionKeyValue::NULL);
    CosmosErrorCode::CosmosErrorCodeSuccess.as_i32()
}

/// Appends an `undefined` (missing-value) component to the partition
/// key.
#[no_mangle]
pub extern "C" fn cosmos_partition_key_builder_add_undefined(
    builder: *mut PartitionKeyBuilderHandle,
) -> i32 {
    let inner = match push_pre_flight(builder) {
        Ok(i) => i,
        Err(code) => return code.as_i32(),
    };
    inner.components.push(PartitionKeyValue::UNDEFINED);
    CosmosErrorCode::CosmosErrorCodeSuccess.as_i32()
}

// ─────────────────────────────────────────────────────────────────────────────
// FFI: build
// ─────────────────────────────────────────────────────────────────────────────

/// Consumes the builder and returns an immutable
/// `cosmos_partition_key_t *`.
///
/// # Lifetime
///
/// `_build` consumes the builder regardless of success or failure.
/// Callers must NOT call [`cosmos_partition_key_builder_free`] on the
/// same pointer afterwards.
///
/// # Returns
///
/// - `SUCCESS` (0) with `*out_pk` populated.
/// - `INVALID_ARGUMENT` (1) when `builder` or `out_pk` is NULL. In
///   the NULL-`out_pk` case the builder is still consumed to avoid
///   leaking the inner allocation.
/// - `INVALID_PARTITION_KEY` (4004) when no components were added.
///   The driver's `EMPTY` partition key has a specific meaning
///   (cross-partition fan-out) and host SDKs cannot construct it via
///   the builder by accident; if you need it explicitly, use
///   `cosmos_partition_key_empty`.
#[no_mangle]
pub extern "C" fn cosmos_partition_key_builder_build(
    builder: *mut PartitionKeyBuilderHandle,
    out_pk: *mut *mut PartitionKeyHandle,
) -> i32 {
    if builder.is_null() {
        return CosmosErrorCode::CosmosErrorCodeInvalidArgument.as_i32();
    }
    let Some(components) = PartitionKeyBuilderHandle::into_owned(builder) else {
        return CosmosErrorCode::CosmosErrorCodeInvalidArgument.as_i32();
    };
    if out_pk.is_null() {
        return CosmosErrorCode::CosmosErrorCodeInvalidArgument.as_i32();
    }
    if components.is_empty() {
        return CosmosErrorCode::CosmosErrorCodeInvalidPartitionKey.as_i32();
    }
    // Length is already capped at <= MAX_COMPONENTS by every `_add_*`
    // setter, so `From<Vec<...>>` will not panic here.
    let pk = DriverPartitionKey::from(components);
    let handle = PartitionKeyHandle::into_raw(pk);
    // SAFETY: caller guarantees `out_pk` is writable for one
    // `*mut PartitionKeyHandle`.
    unsafe {
        *out_pk = handle;
    }
    CosmosErrorCode::CosmosErrorCodeSuccess.as_i32()
}

// ─────────────────────────────────────────────────────────────────────────────
// FFI: PartitionKey lifecycle + accessors
// ─────────────────────────────────────────────────────────────────────────────

/// Returns a fresh handle for the special cross-partition / "empty"
/// partition key (driver constant
/// `azure_data_cosmos_driver::models::PartitionKey::EMPTY`).
///
/// This is the only way to obtain an empty key through the FFI — the
/// builder rejects empty-build with `INVALID_PARTITION_KEY` to catch
/// accidental misuse.
#[no_mangle]
pub extern "C" fn cosmos_partition_key_empty() -> *mut PartitionKeyHandle {
    PartitionKeyHandle::into_raw(DriverPartitionKey::from(Vec::<PartitionKeyValue>::new()))
}

/// Frees a partition-key handle. NULL is a no-op.
#[no_mangle]
pub extern "C" fn cosmos_partition_key_free(pk: *mut PartitionKeyHandle) {
    if pk.is_null() {
        return;
    }
    tracing::trace!(?pk, "freeing cosmos_partition_key_t");
    PartitionKeyHandle::drop_raw(pk);
}

/// Returns the number of components in this partition key.
///
/// Returns `0` for NULL (matches the driver's `EMPTY` semantics, but
/// also serves as a safe default for callers that pass a freed or
/// uninitialized pointer — distinguish from a genuinely-empty key by
/// checking for NULL up-front).
#[no_mangle]
pub extern "C" fn cosmos_partition_key_component_count(pk: *const PartitionKeyHandle) -> usize {
    PartitionKeyHandle::from_ptr(pk)
        .map(|h| h.inner.len())
        .unwrap_or(0)
}

/// Returns `true` when this partition key has zero components.
///
/// Returns `true` for NULL (a NULL handle has no components by
/// definition; the contract mirrors `cosmos_partition_key_component_count`
/// returning `0`).
#[no_mangle]
pub extern "C" fn cosmos_partition_key_is_empty(pk: *const PartitionKeyHandle) -> bool {
    PartitionKeyHandle::from_ptr(pk)
        .map(|h| h.inner.is_empty())
        .unwrap_or(true)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CString;
    use std::ptr;

    fn ok_cstr(s: &str) -> CString {
        CString::new(s).expect("test inputs must be NUL-free")
    }

    #[test]
    fn lifecycle_null_safe() {
        cosmos_partition_key_builder_free(ptr::null_mut());
        cosmos_partition_key_free(ptr::null_mut());
    }

    #[test]
    fn empty_pk_accessor_round_trip() {
        let pk = cosmos_partition_key_empty();
        assert!(!pk.is_null());
        assert_eq!(cosmos_partition_key_component_count(pk), 0);
        assert!(cosmos_partition_key_is_empty(pk));
        cosmos_partition_key_free(pk);
    }

    #[test]
    fn build_empty_rejected() {
        let b = cosmos_partition_key_builder_new();
        let mut out: *mut PartitionKeyHandle = ptr::null_mut();
        let rc = cosmos_partition_key_builder_build(b, &mut out);
        assert_eq!(
            rc,
            CosmosErrorCode::CosmosErrorCodeInvalidPartitionKey.as_i32()
        );
        assert!(out.is_null());
        // Builder is consumed regardless — do NOT free.
    }

    #[test]
    fn single_string_component() {
        let b = cosmos_partition_key_builder_new();
        let s = ok_cstr("tenant-42");
        assert_eq!(
            cosmos_partition_key_builder_add_string(b, s.as_ptr()),
            CosmosErrorCode::CosmosErrorCodeSuccess.as_i32()
        );
        let mut out: *mut PartitionKeyHandle = ptr::null_mut();
        assert_eq!(
            cosmos_partition_key_builder_build(b, &mut out),
            CosmosErrorCode::CosmosErrorCodeSuccess.as_i32()
        );
        assert!(!out.is_null());
        assert_eq!(cosmos_partition_key_component_count(out), 1);
        assert!(!cosmos_partition_key_is_empty(out));

        // Compare against a driver-side equivalent.
        let driver_built = DriverPartitionKey::from("tenant-42");
        let our_built = PartitionKeyHandle::from_ptr(out).unwrap();
        assert_eq!(our_built.inner, driver_built);

        cosmos_partition_key_free(out);
    }

    #[test]
    fn hierarchical_all_value_kinds() {
        let b = cosmos_partition_key_builder_new();
        let s = ok_cstr("region-1");
        assert_eq!(
            cosmos_partition_key_builder_add_string(b, s.as_ptr()),
            CosmosErrorCode::CosmosErrorCodeSuccess.as_i32()
        );
        assert_eq!(
            cosmos_partition_key_builder_add_number(b, 42.0),
            CosmosErrorCode::CosmosErrorCodeSuccess.as_i32()
        );
        assert_eq!(
            cosmos_partition_key_builder_add_bool(b, 1),
            CosmosErrorCode::CosmosErrorCodeSuccess.as_i32()
        );
        let mut out: *mut PartitionKeyHandle = ptr::null_mut();
        assert_eq!(
            cosmos_partition_key_builder_build(b, &mut out),
            CosmosErrorCode::CosmosErrorCodeSuccess.as_i32()
        );
        assert_eq!(cosmos_partition_key_component_count(out), 3);

        // Equivalent driver-side construction.
        let driver_built = DriverPartitionKey::from(("region-1", 42, true));
        let our_built = PartitionKeyHandle::from_ptr(out).unwrap();
        assert_eq!(our_built.inner, driver_built);

        cosmos_partition_key_free(out);
    }

    #[test]
    fn fourth_component_rejected() {
        let b = cosmos_partition_key_builder_new();
        let s = ok_cstr("x");
        // Three accepted appends.
        for _ in 0..3 {
            assert_eq!(
                cosmos_partition_key_builder_add_string(b, s.as_ptr()),
                CosmosErrorCode::CosmosErrorCodeSuccess.as_i32()
            );
        }
        // Fourth rejected with INVALID_OPTION_VALUE — every kind.
        assert_eq!(
            cosmos_partition_key_builder_add_string(b, s.as_ptr()),
            CosmosErrorCode::CosmosErrorCodeInvalidOptionValue.as_i32()
        );
        assert_eq!(
            cosmos_partition_key_builder_add_number(b, 1.0),
            CosmosErrorCode::CosmosErrorCodeInvalidOptionValue.as_i32()
        );
        assert_eq!(
            cosmos_partition_key_builder_add_bool(b, 0),
            CosmosErrorCode::CosmosErrorCodeInvalidOptionValue.as_i32()
        );
        assert_eq!(
            cosmos_partition_key_builder_add_null(b),
            CosmosErrorCode::CosmosErrorCodeInvalidOptionValue.as_i32()
        );
        assert_eq!(
            cosmos_partition_key_builder_add_undefined(b),
            CosmosErrorCode::CosmosErrorCodeInvalidOptionValue.as_i32()
        );
        // Build still succeeds with the 3 accepted components.
        let mut out: *mut PartitionKeyHandle = ptr::null_mut();
        assert_eq!(
            cosmos_partition_key_builder_build(b, &mut out),
            CosmosErrorCode::CosmosErrorCodeSuccess.as_i32()
        );
        assert_eq!(cosmos_partition_key_component_count(out), 3);
        cosmos_partition_key_free(out);
    }

    #[test]
    fn null_and_undefined_components() {
        let b = cosmos_partition_key_builder_new();
        assert_eq!(
            cosmos_partition_key_builder_add_null(b),
            CosmosErrorCode::CosmosErrorCodeSuccess.as_i32()
        );
        assert_eq!(
            cosmos_partition_key_builder_add_undefined(b),
            CosmosErrorCode::CosmosErrorCodeSuccess.as_i32()
        );
        let mut out: *mut PartitionKeyHandle = ptr::null_mut();
        cosmos_partition_key_builder_build(b, &mut out);
        let our_built = PartitionKeyHandle::from_ptr(out).unwrap();
        // The driver's `PartitionKey::EMPTY` is _not_ the same as a
        // 2-component (null, undefined) key — assert wire shape directly.
        assert_eq!(our_built.inner.len(), 2);
        assert_eq!(our_built.inner.values()[0], PartitionKeyValue::NULL);
        assert_eq!(our_built.inner.values()[1], PartitionKeyValue::UNDEFINED);
        cosmos_partition_key_free(out);
    }

    #[test]
    fn number_rejects_non_finite() {
        let b = cosmos_partition_key_builder_new();
        assert_eq!(
            cosmos_partition_key_builder_add_number(b, f64::NAN),
            CosmosErrorCode::CosmosErrorCodeInvalidOptionValue.as_i32()
        );
        assert_eq!(
            cosmos_partition_key_builder_add_number(b, f64::INFINITY),
            CosmosErrorCode::CosmosErrorCodeInvalidOptionValue.as_i32()
        );
        assert_eq!(
            cosmos_partition_key_builder_add_number(b, f64::NEG_INFINITY),
            CosmosErrorCode::CosmosErrorCodeInvalidOptionValue.as_i32()
        );
        // Verify the builder is left unmodified.
        let mut out: *mut PartitionKeyHandle = ptr::null_mut();
        let rc = cosmos_partition_key_builder_build(b, &mut out);
        assert_eq!(
            rc,
            CosmosErrorCode::CosmosErrorCodeInvalidPartitionKey.as_i32(),
            "builder must still be empty after rejected appends"
        );
    }

    #[test]
    fn setters_reject_null_builder() {
        let s = ok_cstr("x");
        assert_eq!(
            cosmos_partition_key_builder_add_string(ptr::null_mut(), s.as_ptr()),
            CosmosErrorCode::CosmosErrorCodeInvalidArgument.as_i32()
        );
        assert_eq!(
            cosmos_partition_key_builder_add_number(ptr::null_mut(), 1.0),
            CosmosErrorCode::CosmosErrorCodeInvalidArgument.as_i32()
        );
        assert_eq!(
            cosmos_partition_key_builder_add_bool(ptr::null_mut(), 0),
            CosmosErrorCode::CosmosErrorCodeInvalidArgument.as_i32()
        );
        assert_eq!(
            cosmos_partition_key_builder_add_null(ptr::null_mut()),
            CosmosErrorCode::CosmosErrorCodeInvalidArgument.as_i32()
        );
        assert_eq!(
            cosmos_partition_key_builder_add_undefined(ptr::null_mut()),
            CosmosErrorCode::CosmosErrorCodeInvalidArgument.as_i32()
        );
    }

    #[test]
    fn add_string_rejects_invalid_inputs() {
        let b = cosmos_partition_key_builder_new();
        // NULL string.
        assert_eq!(
            cosmos_partition_key_builder_add_string(b, ptr::null()),
            CosmosErrorCode::CosmosErrorCodeInvalidArgument.as_i32()
        );
        cosmos_partition_key_builder_free(b);
    }

    #[test]
    fn accessors_handle_null() {
        assert_eq!(cosmos_partition_key_component_count(ptr::null()), 0);
        assert!(cosmos_partition_key_is_empty(ptr::null()));
    }

    #[test]
    fn build_rejects_null_arguments() {
        let mut out: *mut PartitionKeyHandle = ptr::null_mut();
        // NULL builder.
        assert_eq!(
            cosmos_partition_key_builder_build(ptr::null_mut(), &mut out),
            CosmosErrorCode::CosmosErrorCodeInvalidArgument.as_i32()
        );
        // NULL out_pk (builder is consumed regardless).
        let b = cosmos_partition_key_builder_new();
        let s = ok_cstr("x");
        cosmos_partition_key_builder_add_string(b, s.as_ptr());
        assert_eq!(
            cosmos_partition_key_builder_build(b, ptr::null_mut()),
            CosmosErrorCode::CosmosErrorCodeInvalidArgument.as_i32()
        );
        // Do NOT free `b` — it has been consumed.
    }

    #[test]
    fn number_zero_and_negative_zero_normalize() {
        // The driver normalizes -0.0 to +0.0; the FFI must pass through
        // both forms cleanly.
        let b = cosmos_partition_key_builder_new();
        assert_eq!(
            cosmos_partition_key_builder_add_number(b, -0.0),
            CosmosErrorCode::CosmosErrorCodeSuccess.as_i32()
        );
        let mut out: *mut PartitionKeyHandle = ptr::null_mut();
        cosmos_partition_key_builder_build(b, &mut out);
        let our_built = PartitionKeyHandle::from_ptr(out).unwrap();
        // Driver normalizes to +0.0; ours does the same via the same
        // code path.
        assert_eq!(our_built.inner, DriverPartitionKey::from(0.0));
        cosmos_partition_key_free(out);
    }

    // ── Inline tagged-union components ───────────────────────────────────

    /// Helper: a component of a given kind with default value fields.
    fn component(kind: CosmosPartitionKeyComponentKind) -> CosmosPartitionKeyComponent {
        CosmosPartitionKeyComponent {
            kind: kind as i32,
            string_value: ptr::null(),
            number_value: 0.0,
            bool_value: 0,
        }
    }

    #[test]
    fn inline_components_match_builder_for_hierarchical_key() {
        let s = ok_cstr("tenant-42");
        let comps = [
            CosmosPartitionKeyComponent {
                kind: CosmosPartitionKeyComponentKind::CosmosPartitionKeyComponentKindString as i32,
                string_value: s.as_ptr(),
                number_value: 0.0,
                bool_value: 0,
            },
            CosmosPartitionKeyComponent {
                kind: CosmosPartitionKeyComponentKind::CosmosPartitionKeyComponentKindNumber as i32,
                string_value: ptr::null(),
                number_value: 7.0,
                bool_value: 0,
            },
            CosmosPartitionKeyComponent {
                kind: CosmosPartitionKeyComponentKind::CosmosPartitionKeyComponentKindBool as i32,
                string_value: ptr::null(),
                number_value: 0.0,
                bool_value: 1,
            },
        ];
        // SAFETY: `comps` is a live, fully-initialized array.
        let built = unsafe { partition_key_from_components(comps.as_ptr(), comps.len()) }
            .expect("inline build succeeds");
        let expected = DriverPartitionKey::from(("tenant-42", 7.0, true));
        assert_eq!(built, expected);
    }

    #[test]
    fn inline_components_null_and_undefined() {
        let comps = [
            component(CosmosPartitionKeyComponentKind::CosmosPartitionKeyComponentKindNull),
            component(CosmosPartitionKeyComponentKind::CosmosPartitionKeyComponentKindUndefined),
        ];
        // SAFETY: live array.
        let built = unsafe { partition_key_from_components(comps.as_ptr(), comps.len()) }
            .expect("inline build succeeds");
        let expected =
            DriverPartitionKey::from(vec![PartitionKeyValue::NULL, PartitionKeyValue::UNDEFINED]);
        assert_eq!(built, expected);
    }

    #[test]
    fn inline_empty_or_null_rejected() {
        // SAFETY: NULL pointer / zero length is explicitly handled.
        let rc = unsafe { partition_key_from_components(ptr::null(), 0) };
        assert_eq!(rc, Err(CosmosErrorCode::CosmosErrorCodeInvalidPartitionKey));

        let comps = [component(
            CosmosPartitionKeyComponentKind::CosmosPartitionKeyComponentKindNull,
        )];
        // SAFETY: live array, but len 0 forces the empty-rejection path.
        let rc = unsafe { partition_key_from_components(comps.as_ptr(), 0) };
        assert_eq!(rc, Err(CosmosErrorCode::CosmosErrorCodeInvalidPartitionKey));
    }

    #[test]
    fn inline_over_cap_rejected() {
        let comps = [
            component(CosmosPartitionKeyComponentKind::CosmosPartitionKeyComponentKindNull),
            component(CosmosPartitionKeyComponentKind::CosmosPartitionKeyComponentKindNull),
            component(CosmosPartitionKeyComponentKind::CosmosPartitionKeyComponentKindNull),
            component(CosmosPartitionKeyComponentKind::CosmosPartitionKeyComponentKindNull),
        ];
        // SAFETY: live 4-element array; the cap check rejects before any
        // driver call.
        let rc = unsafe { partition_key_from_components(comps.as_ptr(), comps.len()) };
        assert_eq!(rc, Err(CosmosErrorCode::CosmosErrorCodeInvalidPartitionKey));
    }

    #[test]
    fn inline_non_finite_number_rejected() {
        let comps = [CosmosPartitionKeyComponent {
            kind: CosmosPartitionKeyComponentKind::CosmosPartitionKeyComponentKindNumber as i32,
            string_value: ptr::null(),
            number_value: f64::NAN,
            bool_value: 0,
        }];
        // SAFETY: live array.
        let rc = unsafe { partition_key_from_components(comps.as_ptr(), comps.len()) };
        assert_eq!(rc, Err(CosmosErrorCode::CosmosErrorCodeInvalidOptionValue));
    }

    #[test]
    fn inline_invalid_kind_rejected() {
        let comps = [CosmosPartitionKeyComponent {
            kind: 99,
            string_value: ptr::null(),
            number_value: 0.0,
            bool_value: 0,
        }];
        // SAFETY: live array.
        let rc = unsafe { partition_key_from_components(comps.as_ptr(), comps.len()) };
        assert_eq!(rc, Err(CosmosErrorCode::CosmosErrorCodeInvalidOptionValue));
    }
}

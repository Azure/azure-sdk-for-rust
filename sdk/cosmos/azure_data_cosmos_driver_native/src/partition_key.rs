// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! C ABI surface for `cosmos_partition_key_t` — wraps the driver's
//! [`azure_data_cosmos_driver::models::PartitionKey`] and
//! [`azure_data_cosmos_driver::models::PartitionKeyValue`].
//!
//! Hierarchical partition keys are constructed in a single flat call: the host
//! fills an array of up to three [`CosmosPartitionKeyComponent`] tagged-union
//! values (in path order) and passes it to [`cosmos_partition_key_create`],
//! which returns an immutable `cosmos_partition_key_t` handle. The same
//! component array is also accepted inline on
//! [`CosmosOperationRequest`](crate::op_request::CosmosOperationRequest) via its
//! `partition_key_components` field. The flat shape exists because C callers
//! can't construct the driver's Rust `From<T>` / `From<(T1, T2)>` tuples.
//!
//! The driver's `From<Vec<PartitionKeyValue>>` impl panics if the
//! vector contains more than 3 elements (Cosmos DB supports at most
//! 3 hierarchical levels); the FFI surface pre-validates the component
//! count so callers see a deterministic `INVALID_PARTITION_KEY` (4004)
//! instead of an abort. Likewise, `From<f64>` for `PartitionKeyValue` panics
//! on non-finite values — a numeric component with NaN / ±∞ is rejected
//! up-front with `INVALID_OPTION_VALUE` (4014).
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
/// and drop it straight into [`CosmosOperationRequest`](crate::op_request::CosmosOperationRequest)
/// or [`cosmos_partition_key_create`]. Only the field selected
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
/// component array, applying the partition-key validation rules (at most
/// [`MAX_COMPONENTS`] components, finite numbers, valid UTF-8 strings).
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
        // Cosmos DB caps hierarchical keys at 3 levels; reject before
        // `From<Vec<...>>` (which panics above 3 levels) is reached.
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

// ─────────────────────────────────────────────────────────────────────────────
// FFI: PartitionKey lifecycle + accessors
// ─────────────────────────────────────────────────────────────────────────────

/// Creates an immutable partition key from an inline component array in a
/// single call — the flat, standalone counterpart to the
/// `partition_key_components` array carried on
/// [`CosmosOperationRequest`](crate::op_request::CosmosOperationRequest).
///
/// Applies the same validation as the operation-request path: at most 3
/// components, finite numbers, and valid UTF-8 strings.
/// For the special cross-partition / "empty" key use
/// [`cosmos_partition_key_empty`] instead — an empty array here is rejected so
/// the empty key is never constructed by accident.
///
/// # Parameters
///
/// - `components` — array of `len` [`CosmosPartitionKeyComponent`] values.
///   Each `String` component's `string_value` must be valid NUL-terminated
///   UTF-8 for the duration of the call; the wrapper copies what it needs.
/// - `len` — number of components (`1..=3`).
/// - `out_pk` — receives the new handle on success. Must be non-NULL.
///
/// # Returns
///
/// - `SUCCESS` (0) with `*out_pk` populated.
/// - `INVALID_ARGUMENT` (1) when `out_pk` is NULL.
/// - `INVALID_PARTITION_KEY` (4004) when `components` is NULL, `len` is `0`,
///   or `len` exceeds 3.
/// - `INVALID_OPTION_VALUE` (4014) when a numeric component is non-finite or a
///   component `kind` is out of range.
/// - `INVALID_UTF8` (2) when a `String` component is not valid UTF-8.
#[no_mangle]
pub extern "C" fn cosmos_partition_key_create(
    components: *const CosmosPartitionKeyComponent,
    len: usize,
    out_pk: *mut *mut PartitionKeyHandle,
) -> i32 {
    if out_pk.is_null() {
        return CosmosErrorCode::CosmosErrorCodeInvalidArgument.as_i32();
    }
    // SAFETY: caller guarantees `components` points at `len` initialized
    // components whose string payloads are valid NUL-terminated UTF-8 for the
    // duration of the call (documented contract above).
    let pk = match unsafe { partition_key_from_components(components, len) } {
        Ok(pk) => pk,
        Err(code) => return code.as_i32(),
    };
    let handle = PartitionKeyHandle::into_raw(pk);
    // SAFETY: caller guarantees `out_pk` is writable for one
    // `*mut PartitionKeyHandle`.
    unsafe {
        *out_pk = handle;
    }
    CosmosErrorCode::CosmosErrorCodeSuccess.as_i32()
}

/// Returns a fresh handle for the special cross-partition / "empty"
/// partition key (driver constant
/// `azure_data_cosmos_driver::models::PartitionKey::EMPTY`).
///
/// This is the only way to obtain an empty key through the FFI —
/// [`cosmos_partition_key_create`] rejects an empty array with
/// `INVALID_PARTITION_KEY` to catch accidental misuse.
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
    fn accessors_handle_null() {
        assert_eq!(cosmos_partition_key_component_count(ptr::null()), 0);
        assert!(cosmos_partition_key_is_empty(ptr::null()));
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

    // ── Flat FFI constructor (cosmos_partition_key_create) ───────────────

    #[test]
    fn create_produces_handle_matching_driver() {
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
        ];
        let mut out: *mut PartitionKeyHandle = ptr::null_mut();
        assert_eq!(
            cosmos_partition_key_create(comps.as_ptr(), comps.len(), &mut out),
            CosmosErrorCode::CosmosErrorCodeSuccess.as_i32()
        );
        assert!(!out.is_null());
        assert_eq!(cosmos_partition_key_component_count(out), 2);
        assert!(!cosmos_partition_key_is_empty(out));

        let built = PartitionKeyHandle::from_ptr(out).unwrap();
        assert_eq!(built.inner, DriverPartitionKey::from(("tenant-42", 7.0)));
        cosmos_partition_key_free(out);
    }

    #[test]
    fn create_rejects_null_out() {
        let comps = [component(
            CosmosPartitionKeyComponentKind::CosmosPartitionKeyComponentKindNull,
        )];
        assert_eq!(
            cosmos_partition_key_create(comps.as_ptr(), comps.len(), ptr::null_mut()),
            CosmosErrorCode::CosmosErrorCodeInvalidArgument.as_i32()
        );
    }

    #[test]
    fn create_rejects_empty_and_over_cap() {
        let mut out: *mut PartitionKeyHandle = ptr::null_mut();
        // Empty (NULL / zero length) → INVALID_PARTITION_KEY, out untouched.
        assert_eq!(
            cosmos_partition_key_create(ptr::null(), 0, &mut out),
            CosmosErrorCode::CosmosErrorCodeInvalidPartitionKey.as_i32()
        );
        assert!(out.is_null());

        // Four components exceed the 3-level cap.
        let comps = [
            component(CosmosPartitionKeyComponentKind::CosmosPartitionKeyComponentKindNull),
            component(CosmosPartitionKeyComponentKind::CosmosPartitionKeyComponentKindNull),
            component(CosmosPartitionKeyComponentKind::CosmosPartitionKeyComponentKindNull),
            component(CosmosPartitionKeyComponentKind::CosmosPartitionKeyComponentKindNull),
        ];
        assert_eq!(
            cosmos_partition_key_create(comps.as_ptr(), comps.len(), &mut out),
            CosmosErrorCode::CosmosErrorCodeInvalidPartitionKey.as_i32()
        );
        assert!(out.is_null());
    }
}

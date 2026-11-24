// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! C ABI surface for `cosmos_feed_range_t` — wraps the driver's
//! [`azure_data_cosmos_driver::models::FeedRange`].
//!
//! Returned handles are independent owners. Serialized ranges preserve effective
//! bounds, not logical partition-key identity or cursor continuation state.

use azure_data_cosmos_driver::models::{
    effective_partition_key::EffectivePartitionKey, FeedRange as DriverFeedRange,
};
use std::ffi::{c_char, CString};

use crate::container_ref::ContainerRefHandle;
use crate::error::{CosmosErrorCode, CosmosStatusCode};
use crate::partition_key::PartitionKeyHandle;
use crate::string::{required_text, CosmosStringView};

/// The C ABI handle for a feed range (`cosmos_feed_range_t`).
pub struct FeedRangeHandle {
    /// Consumed by the `query_items` request path.
    #[allow(dead_code, reason = "consumed by the query_items request path")]
    pub(crate) inner: DriverFeedRange,
}

impl FeedRangeHandle {
    pub(crate) fn into_raw(inner: DriverFeedRange) -> *mut Self {
        Box::into_raw(Box::new(FeedRangeHandle { inner }))
    }
}

/// Owned discovery result. Free with [`cosmos_feed_range_list_free()`].
pub struct FeedRangeListHandle {
    pub(crate) ranges: Vec<DriverFeedRange>,
}

/// Returns the number of ranges, or zero for NULL.
#[no_mangle]
pub extern "C" fn cosmos_feed_range_list_len(list: *const FeedRangeListHandle) -> usize {
    // SAFETY: a non-NULL list is a live handle for this call.
    unsafe { list.as_ref() }.map_or(0, |list| list.ranges.len())
}

/// Copies one range into an independent handle; an invalid index returns an invalid-option status.
#[no_mangle]
pub extern "C" fn cosmos_feed_range_list_get(
    list: *const FeedRangeListHandle,
    index: usize,
    out: *mut *mut FeedRangeHandle,
) -> CosmosStatusCode {
    if list.is_null() || out.is_null() {
        return CosmosErrorCode::CosmosErrorCodeInvalidArgument.as_status_code();
    }
    // SAFETY: caller supplies a live list and writable output.
    unsafe {
        *out = std::ptr::null_mut();
        let Some(range) = (&(*list).ranges).get(index) else {
            return CosmosErrorCode::CosmosErrorCodeInvalidOptionValue.as_status_code();
        };
        *out = FeedRangeHandle::into_raw(range.clone());
    }
    CosmosErrorCode::CosmosErrorCodeSuccess.as_status_code()
}

/// Frees a discovery list without invalidating previously copied range handles. NULL is a no-op.
#[no_mangle]
pub extern "C" fn cosmos_feed_range_list_free(list: *mut FeedRangeListHandle) {
    if !list.is_null() {
        // SAFETY: list is an owned handle that has not been freed.
        unsafe { drop(Box::from_raw(list)) };
    }
}

fn write_range(
    out: *mut *mut FeedRangeHandle,
    build: impl FnOnce() -> Result<DriverFeedRange, CosmosStatusCode>,
) -> CosmosStatusCode {
    if out.is_null() {
        return CosmosErrorCode::CosmosErrorCodeInvalidArgument.as_status_code();
    }
    // SAFETY: caller supplies a writable output slot.
    unsafe { *out = std::ptr::null_mut() };
    match build() {
        Ok(range) => {
            // SAFETY: output slot was checked above.
            unsafe { *out = FeedRangeHandle::into_raw(range) };
            CosmosErrorCode::CosmosErrorCodeSuccess.as_status_code()
        }
        Err(status) => status,
    }
}

/// Constructs a range from even-length hexadecimal EPK bounds, including the empty minimum.
///
/// Inverted or malformed bounds fail. Equal bounds represent one EPK, matching the driver.
#[no_mangle]
pub extern "C" fn cosmos_feed_range_from_epk(
    min: CosmosStringView,
    max: CosmosStringView,
    out: *mut *mut FeedRangeHandle,
) -> CosmosStatusCode {
    write_range(out, || {
        let invalid = CosmosErrorCode::CosmosErrorCodeInvalidOptionValue;
        // SAFETY: both counted views remain readable for this call.
        let (min, max) = unsafe { (required_text(min, invalid), required_text(max, invalid)) };
        let min = EffectivePartitionKey::try_from(min.map_err(|e| e.as_status_code())?)
            .map_err(|e| CosmosStatusCode::from_driver_error(&e))?;
        let max = EffectivePartitionKey::try_from(max.map_err(|e| e.as_status_code())?)
            .map_err(|e| CosmosStatusCode::from_driver_error(&e))?;
        DriverFeedRange::new(min, max).map_err(|e| CosmosStatusCode::from_driver_error(&e))
    })
}

/// Parses the driver's base64-encoded JSON range representation, copying the input.
///
/// Parsing restores effective bounds only, not logical partition identity.
#[no_mangle]
pub extern "C" fn cosmos_feed_range_parse(
    serialized: CosmosStringView,
    out: *mut *mut FeedRangeHandle,
) -> CosmosStatusCode {
    write_range(out, || {
        // SAFETY: counted input remains readable for this call.
        let text = unsafe {
            required_text(
                serialized,
                CosmosErrorCode::CosmosErrorCodeInvalidOptionValue,
            )
        }
        .map_err(|e| e.as_status_code())?;
        text.parse()
            .map_err(|e| CosmosStatusCode::from_driver_error(&e))
    })
}

fn write_string(text: String, out: *mut *mut c_char) -> CosmosStatusCode {
    if out.is_null() {
        return CosmosErrorCode::CosmosErrorCodeInvalidArgument.as_status_code();
    }
    // All callers produce base64 or hexadecimal text.
    match CString::new(text) {
        Ok(text) => {
            // SAFETY: caller supplies a writable output slot.
            unsafe { *out = text.into_raw() };
            CosmosErrorCode::CosmosErrorCodeSuccess.as_status_code()
        }
        Err(_) => CosmosErrorCode::CosmosErrorCodeInternalError.as_status_code(),
    }
}

/// Serializes effective bounds as base64 JSON, setting `*out` to NULL on failure.
/// Free the owned string with `cosmos_string_free`.
#[no_mangle]
pub extern "C" fn cosmos_feed_range_serialize(
    range: *const FeedRangeHandle,
    out: *mut *mut c_char,
) -> CosmosStatusCode {
    if !out.is_null() {
        // SAFETY: caller supplies a writable output slot.
        unsafe { *out = std::ptr::null_mut() };
    }
    let Some(range) = FeedRangeHandle::from_ptr(range) else {
        return CosmosErrorCode::CosmosErrorCodeInvalidArgument.as_status_code();
    };
    write_string(range.inner.to_string(), out)
}

/// Owned feed-range bounds and logical partition-key identity.
///
/// Free each string independently with `cosmos_string_free`. Neither string
/// borrows from the range handle; no separate result-record free is needed.
#[repr(C)]
#[derive(Default)]
pub struct CosmosFeedRangeInfo {
    /// Owned hexadecimal inclusive minimum, including the empty minimum.
    pub min_inclusive: *mut c_char,
    /// Owned hexadecimal exclusive maximum.
    pub max_exclusive: *mut c_char,
    /// 1 if the handle retains logical partition-key identity; otherwise 0.
    pub is_logical_partition: u8,
}

/// Returns owned bounds and logical identity. Zeroes the output on failure.
///
/// `out` must point to writable, aligned storage for one [`CosmosFeedRangeInfo`].
/// Free any previous strings before reusing the output storage.
#[no_mangle]
pub extern "C" fn cosmos_feed_range_inspect(
    range: *const FeedRangeHandle,
    out: *mut CosmosFeedRangeInfo,
) -> CosmosStatusCode {
    if out.is_null() {
        return CosmosErrorCode::CosmosErrorCodeInvalidArgument.as_status_code();
    }
    // SAFETY: caller provides writable, aligned result storage.
    unsafe { *out = CosmosFeedRangeInfo::default() };
    let Some(range) = FeedRangeHandle::from_ptr(range) else {
        return CosmosErrorCode::CosmosErrorCodeInvalidArgument.as_status_code();
    };
    let (Ok(min), Ok(max)) = (
        CString::new(range.inner.min_inclusive().to_hex()),
        CString::new(range.inner.max_exclusive().to_hex()),
    ) else {
        return CosmosErrorCode::CosmosErrorCodeInternalError.as_status_code();
    };
    // SAFETY: both owned strings transfer to the caller's result storage.
    unsafe {
        *out = CosmosFeedRangeInfo {
            min_inclusive: min.into_raw(),
            max_exclusive: max.into_raw(),
            is_logical_partition: u8::from(range.inner.is_logical_partition()),
        };
    }
    CosmosErrorCode::CosmosErrorCodeSuccess.as_status_code()
}

/// The driver's effective-bound subset and overlap results, encoded as 0 or 1.
#[repr(C)]
#[derive(Default)]
pub struct CosmosFeedRangeRelation {
    /// Whether the first range is a subset of the second range.
    pub is_subset: u8,
    /// Whether the ranges overlap.
    pub overlaps: u8,
}

/// Compares effective bounds using the driver's existing subset and overlap semantics.
///
/// `out` must point to writable, aligned storage for one [`CosmosFeedRangeRelation`].
/// Failure zeroes the output; neither input handle is consumed.
#[no_mangle]
pub extern "C" fn cosmos_feed_range_relate(
    range: *const FeedRangeHandle,
    other: *const FeedRangeHandle,
    out: *mut CosmosFeedRangeRelation,
) -> CosmosStatusCode {
    if out.is_null() {
        return CosmosErrorCode::CosmosErrorCodeInvalidArgument.as_status_code();
    }
    // SAFETY: caller provides writable, aligned result storage.
    unsafe { *out = CosmosFeedRangeRelation::default() };
    let (Some(range), Some(other)) = (
        FeedRangeHandle::from_ptr(range),
        FeedRangeHandle::from_ptr(other),
    ) else {
        return CosmosErrorCode::CosmosErrorCodeInvalidArgument.as_status_code();
    };
    // SAFETY: caller provides writable, aligned result storage.
    unsafe {
        *out = CosmosFeedRangeRelation {
            is_subset: u8::from(range.inner.is_subset_of(&other.inner)),
            overlaps: u8::from(range.inner.overlaps(&other.inner)),
        };
    }
    CosmosErrorCode::CosmosErrorCodeSuccess.as_status_code()
}

impl FeedRangeHandle {
    /// Borrows the handle for the duration of an FFI call without taking
    /// ownership. Returns `None` for a NULL pointer.
    pub(crate) fn from_ptr<'a>(p: *const FeedRangeHandle) -> Option<&'a FeedRangeHandle> {
        if p.is_null() {
            return None;
        }
        // SAFETY: caller guarantees `p` was obtained from `into_raw` and is
        // not freed for the duration of the borrow.
        Some(unsafe { &*p })
    }

    fn drop_raw(p: *mut FeedRangeHandle) {
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

/// Constructs a feed range covering the entire EPK key space. Mirrors
/// [`azure_data_cosmos_driver::models::FeedRange::full`].
///
/// # Returns
///
/// A packed [`crate::error::CosmosStatusCode`] (`(http << 16) | sub_status`;
/// decode with `COSMOS_STATUS_HTTP` / `COSMOS_STATUS_SUB`):
///
/// - `COSMOS_STATUS_SUCCESS` (`0`) with `*out_fr` populated.
/// - `400` / `CLIENT_FFI_NULL_ARGUMENT` when `out_fr` is NULL.
#[no_mangle]
pub extern "C" fn cosmos_feed_range_full(out_fr: *mut *mut FeedRangeHandle) -> CosmosStatusCode {
    if out_fr.is_null() {
        return CosmosErrorCode::CosmosErrorCodeInvalidArgument.as_status_code();
    }
    let handle = FeedRangeHandle::into_raw(DriverFeedRange::full());
    // SAFETY: caller guarantees `out_fr` is writable.
    unsafe {
        *out_fr = handle;
    }
    CosmosErrorCode::CosmosErrorCodeSuccess.as_status_code()
}

/// Constructs a feed range that targets a single logical partition
/// key. Mirrors `FeedRange::for_partition(pk, container.partition_key_definition())`.
///
/// The partition-key definition is extracted from `container`, so
/// callers do not have to plumb it manually.
///
/// # Returns
///
/// A packed [`crate::error::CosmosStatusCode`] (`(http << 16) | sub_status`;
/// decode with `COSMOS_STATUS_HTTP` / `COSMOS_STATUS_SUB`):
///
/// - `COSMOS_STATUS_SUCCESS` (`0`) with `*out_fr` populated.
/// - `400` / `CLIENT_FFI_NULL_ARGUMENT` when any pointer is NULL.
#[no_mangle]
pub extern "C" fn cosmos_feed_range_for_partition_key(
    container: *const ContainerRefHandle,
    pk: *const PartitionKeyHandle,
    out_fr: *mut *mut FeedRangeHandle,
) -> CosmosStatusCode {
    if out_fr.is_null() {
        return CosmosErrorCode::CosmosErrorCodeInvalidArgument.as_status_code();
    }
    let Some(container_inner) = ContainerRefHandle::from_ptr(container) else {
        return CosmosErrorCode::CosmosErrorCodeInvalidArgument.as_status_code();
    };
    let Some(pk_inner) = PartitionKeyHandle::from_ptr(pk) else {
        return CosmosErrorCode::CosmosErrorCodeInvalidArgument.as_status_code();
    };
    let fr = DriverFeedRange::for_partition(
        pk_inner.inner.clone(),
        container_inner.inner.partition_key_definition(),
    );
    let handle = FeedRangeHandle::into_raw(fr);
    // SAFETY: caller guarantees `out_fr` is writable.
    unsafe {
        *out_fr = handle;
    }
    CosmosErrorCode::CosmosErrorCodeSuccess.as_status_code()
}

/// Frees a feed-range handle. NULL is a no-op.
#[no_mangle]
pub extern "C" fn cosmos_feed_range_free(fr: *mut FeedRangeHandle) {
    if fr.is_null() {
        return;
    }
    tracing::trace!(?fr, "freeing cosmos_feed_range_t");
    FeedRangeHandle::drop_raw(fr);
}

#[cfg(test)]
mod tests {
    use super::{
        cosmos_feed_range_for_partition_key, cosmos_feed_range_free, cosmos_feed_range_from_epk,
        cosmos_feed_range_full, cosmos_feed_range_inspect, cosmos_feed_range_parse,
        cosmos_feed_range_relate, cosmos_feed_range_serialize, CosmosErrorCode,
        CosmosFeedRangeInfo, CosmosFeedRangeRelation, DriverFeedRange, FeedRangeHandle,
    };
    use crate::string::{cosmos_string_free, view};
    use azure_data_cosmos_driver::models::{PartitionKey, PartitionKeyDefinition};
    use std::ffi::CStr;
    use std::ptr;

    #[test]
    fn epk_roundtrip_owns_strings_and_rejects_malformed_inputs() {
        let mut range = ptr::null_mut();
        let mut text = ptr::null_mut();
        let mut info = CosmosFeedRangeInfo::default();
        let success = CosmosErrorCode::CosmosErrorCodeSuccess.as_status_code();
        assert_eq!(
            cosmos_feed_range_from_epk(view(b"10"), view(b"80"), &mut range),
            success
        );
        assert_eq!(cosmos_feed_range_serialize(range, &mut text), success);
        assert_eq!(cosmos_feed_range_inspect(range, &mut info), success);
        assert_eq!(info.is_logical_partition, 0);
        cosmos_feed_range_free(range);
        // SAFETY: strings are independent live owned outputs.
        unsafe {
            assert_eq!(CStr::from_ptr(info.min_inclusive).to_bytes(), b"10");
            assert_eq!(CStr::from_ptr(info.max_exclusive).to_bytes(), b"80");
            assert_eq!(
                cosmos_feed_range_parse(view(CStr::from_ptr(text).to_bytes()), &mut range),
                success
            );
        }
        cosmos_string_free(text);
        cosmos_string_free(info.min_inclusive);
        cosmos_string_free(info.max_exclusive);
        let mut full = ptr::null_mut();
        cosmos_feed_range_full(&mut full);
        let mut result = CosmosFeedRangeRelation::default();
        assert_eq!(cosmos_feed_range_relate(range, full, &mut result), success);
        assert_eq!((result.is_subset, result.overlaps), (1, 1));
        cosmos_feed_range_free(range);
        cosmos_feed_range_free(full);
        for (min, max) in [
            (b"80".as_slice(), b"10".as_slice()),
            (b"0", b"FF"),
            (b"GG", b"FF"),
        ] {
            assert_ne!(
                cosmos_feed_range_from_epk(view(min), view(max), &mut range),
                success
            );
            assert!(range.is_null());
        }
        for input in [b"not base64".as_slice(), b"e30="] {
            assert_ne!(cosmos_feed_range_parse(view(input), &mut range), success);
            assert!(range.is_null());
        }
    }

    #[test]
    fn comparisons_respect_ordinary_half_open_boundaries() {
        for (min, max, other_min, other_max, subset, overlap) in [
            ("10", "20", "10", "20", true, true),
            ("10", "20", "00", "20", true, true),
            ("00", "20", "10", "20", false, true),
            ("10", "20", "00", "10", false, false),
            ("00", "10", "10", "20", false, false),
            ("10", "30", "20", "40", false, true),
            ("10", "10", "10", "10", true, true),
            ("10", "10", "10", "20", true, true),
            ("20", "20", "10", "20", false, false),
            ("10", "20", "10", "10", false, true),
            ("10", "20", "20", "20", false, false),
        ] {
            let mut range = ptr::null_mut();
            let mut other = ptr::null_mut();
            let success = CosmosErrorCode::CosmosErrorCodeSuccess.as_status_code();
            assert_eq!(
                cosmos_feed_range_from_epk(view(min.as_bytes()), view(max.as_bytes()), &mut range),
                success
            );
            assert_eq!(
                cosmos_feed_range_from_epk(
                    view(other_min.as_bytes()),
                    view(other_max.as_bytes()),
                    &mut other
                ),
                success
            );
            let mut result = CosmosFeedRangeRelation::default();
            assert_eq!(cosmos_feed_range_relate(range, other, &mut result), success);
            assert_eq!(result.is_subset, u8::from(subset));
            assert_eq!(result.overlaps, u8::from(overlap));
            cosmos_feed_range_free(range);
            cosmos_feed_range_free(other);
        }
    }

    #[test]
    fn logical_partition_points_relate_by_their_effective_key() {
        let success = CosmosErrorCode::CosmosErrorCodeSuccess.as_status_code();
        let logical = FeedRangeHandle::into_raw(DriverFeedRange::for_partition(
            PartitionKey::from("tenant"),
            &PartitionKeyDefinition::from("/pk"),
        ));
        let mut info = CosmosFeedRangeInfo::default();
        assert_eq!(cosmos_feed_range_inspect(logical, &mut info), success);
        // SAFETY: inspection returns live owned strings.
        let point = unsafe {
            let min = CStr::from_ptr(info.min_inclusive).to_bytes().to_vec();
            assert_eq!(min, CStr::from_ptr(info.max_exclusive).to_bytes());
            min
        };
        cosmos_string_free(info.min_inclusive);
        cosmos_string_free(info.max_exclusive);
        let relate = |other: *const FeedRangeHandle| {
            let mut result = CosmosFeedRangeRelation::default();
            assert_eq!(
                cosmos_feed_range_relate(logical, other, &mut result),
                success
            );
            (result.is_subset, result.overlaps)
        };
        assert_eq!(relate(logical), (1, 1));
        for (min, max, expected) in [
            (point.as_slice(), point.as_slice(), (1, 1)),
            (point.as_slice(), b"FF".as_slice(), (1, 1)),
            (b"".as_slice(), point.as_slice(), (0, 0)),
        ] {
            let mut other = ptr::null_mut();
            assert_eq!(
                cosmos_feed_range_from_epk(view(min), view(max), &mut other),
                success
            );
            assert_eq!(relate(other), expected);
            cosmos_feed_range_free(other);
        }
        cosmos_feed_range_free(logical);
    }

    #[test]
    fn inspection_retains_logical_identity_but_serialization_does_not() {
        let logical = FeedRangeHandle::into_raw(DriverFeedRange::for_partition(
            PartitionKey::from("tenant"),
            &PartitionKeyDefinition::from("/pk"),
        ));
        let mut info = CosmosFeedRangeInfo::default();
        let mut serialized = ptr::null_mut();
        let success = CosmosErrorCode::CosmosErrorCodeSuccess.as_status_code();
        assert_eq!(cosmos_feed_range_inspect(logical, &mut info), success);
        assert_eq!(info.is_logical_partition, 1);
        assert_eq!(
            cosmos_feed_range_serialize(logical, &mut serialized),
            success
        );
        cosmos_feed_range_free(logical);
        let mut parsed = ptr::null_mut();
        // SAFETY: serialization and inspection strings independently outlive the source.
        unsafe {
            assert!(!CStr::from_ptr(info.min_inclusive).to_bytes().is_empty());
            assert_eq!(
                cosmos_feed_range_parse(view(CStr::from_ptr(serialized).to_bytes()), &mut parsed),
                success
            );
        }
        cosmos_string_free(serialized);
        cosmos_string_free(info.min_inclusive);
        cosmos_string_free(info.max_exclusive);
        assert_eq!(cosmos_feed_range_inspect(parsed, &mut info), success);
        assert_eq!(info.is_logical_partition, 0);
        cosmos_string_free(info.min_inclusive);
        cosmos_string_free(info.max_exclusive);
        cosmos_feed_range_free(parsed);
    }

    #[test]
    fn inspection_and_relation_failures_clear_outputs() {
        let mut info = CosmosFeedRangeInfo {
            min_inclusive: ptr::dangling_mut(),
            max_exclusive: ptr::dangling_mut(),
            is_logical_partition: 1,
        };
        let mut relation = CosmosFeedRangeRelation {
            is_subset: 1,
            overlaps: 1,
        };
        let invalid = CosmosErrorCode::CosmosErrorCodeInvalidArgument.as_status_code();
        assert_eq!(cosmos_feed_range_inspect(ptr::null(), &mut info), invalid);
        assert!(info.min_inclusive.is_null() && info.max_exclusive.is_null());
        assert_eq!(info.is_logical_partition, 0);
        assert_eq!(
            cosmos_feed_range_relate(ptr::null(), ptr::null(), &mut relation),
            invalid
        );
        assert_eq!((relation.is_subset, relation.overlaps), (0, 0));
        assert_eq!(
            cosmos_feed_range_inspect(ptr::null(), ptr::null_mut()),
            invalid
        );
        assert_eq!(
            cosmos_feed_range_relate(ptr::null(), ptr::null(), ptr::null_mut()),
            invalid
        );
    }

    #[test]
    fn empty_minimum_is_accepted_and_serialize_failure_clears_output() {
        let success = CosmosErrorCode::CosmosErrorCodeSuccess.as_status_code();
        let invalid = CosmosErrorCode::CosmosErrorCodeInvalidArgument.as_status_code();
        let mut range = ptr::null_mut();
        assert_eq!(
            cosmos_feed_range_from_epk(view(b""), view(b"FF"), &mut range),
            success
        );
        let mut info = CosmosFeedRangeInfo::default();
        assert_eq!(cosmos_feed_range_inspect(range, &mut info), success);
        // SAFETY: both strings are live owned outputs.
        unsafe {
            assert_eq!(CStr::from_ptr(info.min_inclusive).to_bytes(), b"");
            assert_eq!(CStr::from_ptr(info.max_exclusive).to_bytes(), b"FF");
        }
        cosmos_string_free(info.min_inclusive);
        cosmos_string_free(info.max_exclusive);
        cosmos_feed_range_free(range);

        let mut text = ptr::dangling_mut();
        assert_eq!(cosmos_feed_range_serialize(ptr::null(), &mut text), invalid);
        assert!(text.is_null());
    }

    #[test]
    fn free_handles_null() {
        cosmos_feed_range_free(ptr::null_mut());
    }

    #[test]
    fn full_roundtrip() {
        let mut fr: *mut FeedRangeHandle = ptr::null_mut();
        assert_eq!(
            cosmos_feed_range_full(&mut fr),
            CosmosErrorCode::CosmosErrorCodeSuccess.as_status_code()
        );
        assert!(!fr.is_null());
        cosmos_feed_range_free(fr);
    }

    #[test]
    fn full_rejects_null_out() {
        assert_eq!(
            cosmos_feed_range_full(ptr::null_mut()),
            CosmosErrorCode::CosmosErrorCodeInvalidArgument.as_status_code()
        );
    }

    #[test]
    fn for_partition_key_rejects_nulls() {
        let mut fr: *mut FeedRangeHandle = ptr::null_mut();
        assert_eq!(
            cosmos_feed_range_for_partition_key(ptr::null(), ptr::null(), &mut fr),
            CosmosErrorCode::CosmosErrorCodeInvalidArgument.as_status_code()
        );
        assert_eq!(
            cosmos_feed_range_for_partition_key(ptr::null(), ptr::null(), ptr::null_mut()),
            CosmosErrorCode::CosmosErrorCodeInvalidArgument.as_status_code()
        );
    }
}

// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! C ABI surface for `cosmos_feed_range_t` — wraps the driver's
//! [`azure_data_cosmos_driver::models::FeedRange`].
//!
//! Returned handles are independent owners that expose their range information
//! directly. Serialized ranges preserve effective bounds, not logical
//! partition-key identity or cursor continuation state.

use azure_data_cosmos_driver::models::{
    effective_partition_key::EffectivePartitionKey, FeedRange as DriverFeedRange,
};
use std::ffi::{c_char, CString, NulError};

use crate::container_ref::ContainerRefHandle;
use crate::error::{CosmosErrorCode, CosmosStatusCode};
use crate::partition_key::PartitionKeyHandle;
use crate::string::{required_text, CosmosStringView};

/// The C ABI handle for a feed range (`cosmos_feed_range_t`).
///
/// `info` and its strings are read-only and valid until [`cosmos_feed_range_free()`].
/// Copy any needed string before freeing; never modify or free the strings.
///
/// Use only pointers returned by this library. The struct is the prefix of a larger
/// allocation: never allocate it, copy it by value, or pass a copy to any API.
#[repr(C)]
pub struct FeedRangeHandle {
    /// Bounds, logical partition-key identity, and canonical serialization.
    pub info: CosmosFeedRangeInfo,
}

/// Private owner behind every public [`FeedRangeHandle`]; its view must stay first.
#[repr(C)]
pub(crate) struct OwnedFeedRange {
    view: FeedRangeHandle,
    pub(crate) inner: DriverFeedRange,
    text: RangeText,
}

impl OwnedFeedRange {
    fn new(inner: DriverFeedRange) -> Result<Self, NulError> {
        let text = RangeText::new(&inner)?;
        Ok(Self {
            view: FeedRangeHandle {
                info: text.info(&inner),
            },
            inner,
            text,
        })
    }
}

impl FeedRangeHandle {
    pub(crate) fn into_raw(inner: DriverFeedRange) -> Result<*mut Self, NulError> {
        // The repr(C) owner starts with its public view.
        Ok(Box::into_raw(Box::new(OwnedFeedRange::new(inner)?)).cast())
    }

    /// Borrows the owner for the duration of an FFI call without taking
    /// ownership. Returns `None` for a NULL pointer.
    pub(crate) fn from_ptr<'a>(p: *const FeedRangeHandle) -> Option<&'a OwnedFeedRange> {
        if p.is_null() {
            return None;
        }
        // SAFETY: caller guarantees `p` was obtained from `into_raw` and is
        // not freed for the duration of the borrow.
        Some(unsafe { &*p.cast::<OwnedFeedRange>() })
    }

    fn drop_raw(p: *mut FeedRangeHandle) {
        if p.is_null() {
            return;
        }
        // SAFETY: caller guarantees `p` was obtained from `into_raw` and has
        // not already been freed.
        unsafe {
            drop(Box::from_raw(p.cast::<OwnedFeedRange>()));
        }
    }
}

/// NUL-terminated strings referenced by a [`CosmosFeedRangeInfo`].
struct RangeText {
    min: CString,
    max: CString,
    serialized: CString,
}

impl RangeText {
    fn new(range: &DriverFeedRange) -> Result<Self, NulError> {
        Ok(Self {
            min: CString::new(range.min_inclusive().to_hex())?,
            max: CString::new(range.max_exclusive().to_hex())?,
            serialized: CString::new(range.to_string())?,
        })
    }

    /// Heap buffers stay put when the text moves into its owner.
    fn info(&self, range: &DriverFeedRange) -> CosmosFeedRangeInfo {
        CosmosFeedRangeInfo {
            min_inclusive: self.min.as_ptr(),
            max_exclusive: self.max.as_ptr(),
            is_logical_partition: u8::from(range.is_logical_partition()),
            serialized: self.serialized.as_ptr(),
        }
    }
}

/// Feed-range bounds, logical partition-key identity, and bounds-only serialization.
///
/// Read directly from a feed-range handle's `info` field or a list entry.
/// Strings are read-only and borrowed from their owner; copy them before it is freed.
///
/// Bounds are uppercase hexadecimal with trailing `00` padding preserved. Compare ranges
/// with `cosmos_feed_range_relate()`, not string equality: `3A00` and `3A0000` are one EPK.
#[repr(C)]
pub struct CosmosFeedRangeInfo {
    /// NUL-terminated hexadecimal inclusive minimum, including the empty minimum.
    pub min_inclusive: *const c_char,
    /// NUL-terminated hexadecimal exclusive maximum. Bounds equal after ignoring trailing
    /// `00` bytes denote one EPK.
    pub max_exclusive: *const c_char,
    /// 1 if the handle retains logical partition-key identity; otherwise 0.
    pub is_logical_partition: u8,
    /// NUL-terminated canonical serialization for [`cosmos_feed_range_parse()`].
    /// Preserves bounds only, not logical partition-key identity or continuation state.
    pub serialized: *const c_char,
}

/// Rust-owned discovery result with a contiguous array of range information.
///
/// The array and its strings are read-only and valid until [`cosmos_feed_range_list_free()`].
/// Copy every needed string before freeing the list; do not free entries or strings separately.
#[repr(C)]
pub struct FeedRangeListHandle {
    /// Borrowed entries, or NULL when `count` is zero.
    pub ranges: *const CosmosFeedRangeInfo,
    /// Number of entries addressable from `ranges`.
    pub count: usize,
}

#[repr(C)]
pub(crate) struct OwnedFeedRangeList {
    view: FeedRangeListHandle,
    entries: Box<[CosmosFeedRangeInfo]>,
    text: Vec<RangeText>,
}

// SAFETY: all pointers reference this owner's heap allocations, which are read-only after construction.
unsafe impl Send for OwnedFeedRangeList {}
unsafe impl Sync for OwnedFeedRangeList {}

impl OwnedFeedRangeList {
    pub(crate) fn new(ranges: Vec<DriverFeedRange>) -> Result<Self, NulError> {
        let text = ranges
            .iter()
            .map(RangeText::new)
            .collect::<Result<Vec<_>, _>>()?;
        let entries = ranges
            .iter()
            .zip(&text)
            .map(|(range, text)| text.info(range))
            .collect::<Box<[_]>>();
        Ok(Self {
            view: FeedRangeListHandle {
                ranges: if entries.is_empty() {
                    std::ptr::null()
                } else {
                    entries.as_ptr()
                },
                count: entries.len(),
            },
            entries,
            text,
        })
    }

    pub(crate) fn into_raw(self: Box<Self>) -> *mut FeedRangeListHandle {
        // The repr(C) owner starts with its public view.
        Box::into_raw(self).cast()
    }
}

/// Frees a discovery list, its entries, and all referenced strings. NULL is a no-op.
///
/// `list` must be an unmodified list returned by `cosmos_completion_take_feed_ranges`
/// that has not already been freed. Any pointers into it become invalid.
#[no_mangle]
pub extern "C" fn cosmos_feed_range_list_free(list: *mut FeedRangeListHandle) {
    if !list.is_null() {
        // SAFETY: the public view is the first field of the repr(C) owner returned by into_raw.
        unsafe { drop(Box::from_raw(list.cast::<OwnedFeedRangeList>())) };
    }
}

fn publish(range: DriverFeedRange, out: *mut *mut FeedRangeHandle) -> CosmosStatusCode {
    // SAFETY: callers verified that `out` is a writable output slot.
    unsafe { *out = std::ptr::null_mut() };
    match FeedRangeHandle::into_raw(range) {
        Ok(handle) => {
            // SAFETY: output slot was checked by the caller.
            unsafe { *out = handle };
            CosmosErrorCode::CosmosErrorCodeSuccess.as_status_code()
        }
        // Hexadecimal and base64 text never contains NUL.
        Err(_) => CosmosErrorCode::CosmosErrorCodeInternalError.as_status_code(),
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
        Ok(range) => publish(range, out),
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
    publish(DriverFeedRange::full(), out_fr)
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
    publish(fr, out_fr)
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
        cosmos_feed_range_full, cosmos_feed_range_list_free, cosmos_feed_range_parse,
        cosmos_feed_range_relate, CosmosErrorCode, CosmosFeedRangeInfo, CosmosFeedRangeRelation,
        DriverFeedRange, FeedRangeHandle, OwnedFeedRangeList,
    };
    use crate::string::view;
    use azure_data_cosmos_driver::models::{
        PartitionKey, PartitionKeyDefinition, PartitionKeyValue,
    };
    use std::ffi::CStr;
    use std::ptr;

    #[test]
    fn flat_list_copies_out_bounds_identity_and_serialization() {
        let logical = DriverFeedRange::for_partition(
            PartitionKey::from("tenant"),
            &PartitionKeyDefinition::from("/pk"),
        );
        let input = vec![DriverFeedRange::full(), logical];
        let expected = input
            .iter()
            .map(|range| {
                (
                    range.min_inclusive().to_hex().to_owned(),
                    range.max_exclusive().to_hex().to_owned(),
                    u8::from(range.is_logical_partition()),
                    range.to_string(),
                )
            })
            .collect::<Vec<_>>();
        let list = Box::new(OwnedFeedRangeList::new(input).unwrap()).into_raw();
        // SAFETY: the list owns all entries and strings until list_free.
        let copied = unsafe {
            std::slice::from_raw_parts((*list).ranges, (*list).count)
                .iter()
                .map(|info| {
                    (
                        CStr::from_ptr(info.min_inclusive)
                            .to_str()
                            .unwrap()
                            .to_owned(),
                        CStr::from_ptr(info.max_exclusive)
                            .to_str()
                            .unwrap()
                            .to_owned(),
                        info.is_logical_partition,
                        CStr::from_ptr(info.serialized).to_str().unwrap().to_owned(),
                    )
                })
                .collect::<Vec<_>>()
        };
        cosmos_feed_range_list_free(list);
        assert_eq!(copied, expected);
        for (min, max, _, serialized) in copied {
            let parsed: DriverFeedRange = serialized.parse().unwrap();
            assert_eq!(parsed.min_inclusive().to_hex(), min);
            assert_eq!(parsed.max_exclusive().to_hex(), max);
            assert!(!parsed.is_logical_partition());
        }
    }

    #[test]
    fn empty_flat_list_has_null_entries_and_zero_count() {
        let list = Box::new(OwnedFeedRangeList::new(Vec::new()).unwrap()).into_raw();
        // SAFETY: into_raw returned a live list, including for an empty result.
        unsafe {
            assert!((*list).ranges.is_null());
            assert_eq!((*list).count, 0);
        }
        cosmos_feed_range_list_free(list);
        cosmos_feed_range_list_free(ptr::null_mut());
    }

    /// Copies the strings readable from a live handle.
    fn read_info(range: *const FeedRangeHandle) -> (String, String, u8, String) {
        // SAFETY: callers pass a live handle; its strings are valid until free.
        unsafe { copy_info(&(*range).info) }
    }

    /// SAFETY: `info` strings must be live NUL-terminated text.
    unsafe fn copy_info(info: &CosmosFeedRangeInfo) -> (String, String, u8, String) {
        let text = |p| CStr::from_ptr(p).to_str().unwrap().to_owned();
        (
            text(info.min_inclusive),
            text(info.max_exclusive),
            info.is_logical_partition,
            text(info.serialized),
        )
    }

    #[test]
    fn epk_handle_exposes_info_that_matches_its_serialization() {
        let success = CosmosErrorCode::CosmosErrorCodeSuccess.as_status_code();
        let mut range = ptr::null_mut();
        assert_eq!(
            cosmos_feed_range_from_epk(view(b"10"), view(b"80"), &mut range),
            success
        );
        let (min, max, logical, serialized) = read_info(range);
        assert_eq!((min.as_str(), max.as_str(), logical), ("10", "80", 0));
        let expected: DriverFeedRange = serialized.parse().unwrap();
        assert_eq!(expected.to_string(), serialized);
        cosmos_feed_range_free(range);

        let mut parsed = ptr::null_mut();
        assert_eq!(
            cosmos_feed_range_parse(view(serialized.as_bytes()), &mut parsed),
            success
        );
        assert_eq!(read_info(parsed), (min, max, 0, serialized));
        let mut full = ptr::null_mut();
        assert_eq!(cosmos_feed_range_full(&mut full), success);
        let mut result = CosmosFeedRangeRelation::default();
        assert_eq!(cosmos_feed_range_relate(parsed, full, &mut result), success);
        assert_eq!((result.is_subset, result.overlaps), (1, 1));
        cosmos_feed_range_free(parsed);
        cosmos_feed_range_free(full);
    }

    #[test]
    fn info_bounds_equal_parsed_serialization_for_every_range_shape() {
        let success = CosmosErrorCode::CosmosErrorCodeSuccess.as_status_code();
        let multi: PartitionKeyDefinition = serde_json::from_value(
            serde_json::json!({"paths":["/tenant","/pk"],"kind":"MultiHash","version":2}),
        )
        .unwrap();
        let components = |n: usize| {
            PartitionKey::try_from(
                ["tenant", "user"][..n]
                    .iter()
                    .map(|v| PartitionKeyValue::from(*v))
                    .collect::<Vec<_>>(),
            )
            .unwrap()
        };
        let single = DriverFeedRange::for_partition(
            PartitionKey::from("tenant"),
            &PartitionKeyDefinition::from("/pk"),
        );
        let prefix = DriverFeedRange::for_partition(components(1), &multi);
        let full_key = DriverFeedRange::for_partition(components(2), &multi);
        let epk = |min: &str, max: &str| {
            let (min, max) = (min.try_into().unwrap(), max.try_into().unwrap());
            DriverFeedRange::new(min, max).unwrap()
        };
        let cases = [
            (single, true),
            (prefix, true),
            (full_key, true),
            (DriverFeedRange::full(), false),
            (epk("3a0000", "3a00ff"), false),
            (epk("3A00", "3a0000"), false),
            (epk("", "ff"), false),
        ];
        let mut kinds = Vec::new();
        for (range, logical) in cases {
            let (min, max) = (
                range.min_inclusive().to_hex(),
                range.max_exclusive().to_hex(),
            );
            kinds.push(min == max);
            let handle = FeedRangeHandle::into_raw(range).unwrap();
            let (info_min, info_max, flag, serialized) = read_info(handle);
            cosmos_feed_range_free(handle);
            assert_eq!(
                ((&info_min, &info_max), flag),
                ((&min, &max), u8::from(logical))
            );
            assert_eq!(info_min, info_min.to_uppercase());
            let mut parsed = ptr::null_mut();
            assert_eq!(
                cosmos_feed_range_parse(view(serialized.as_bytes()), &mut parsed),
                success
            );
            assert_eq!(read_info(parsed), (info_min, info_max, 0, serialized));
            cosmos_feed_range_free(parsed);
        }
        // Both a prefix interval and equal-bound points were exercised.
        assert!(kinds.contains(&true) && kinds.contains(&false));
    }

    #[test]
    fn constructors_reject_malformed_inputs_without_a_handle() {
        let success = CosmosErrorCode::CosmosErrorCodeSuccess.as_status_code();
        let mut range = ptr::dangling_mut();
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
            range = ptr::dangling_mut();
        }
        for input in [b"not base64".as_slice(), b"e30="] {
            assert_ne!(cosmos_feed_range_parse(view(input), &mut range), success);
            assert!(range.is_null());
            range = ptr::dangling_mut();
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

    fn logical_handle() -> *mut FeedRangeHandle {
        FeedRangeHandle::into_raw(DriverFeedRange::for_partition(
            PartitionKey::from("tenant"),
            &PartitionKeyDefinition::from("/pk"),
        ))
        .unwrap()
    }

    #[test]
    fn logical_partition_points_relate_by_their_effective_key() {
        let success = CosmosErrorCode::CosmosErrorCodeSuccess.as_status_code();
        let logical = logical_handle();
        let (min, max, _, _) = read_info(logical);
        assert_eq!(min, max);
        let point = min.into_bytes();
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
    fn logical_identity_is_in_info_but_not_in_serialization() {
        let success = CosmosErrorCode::CosmosErrorCodeSuccess.as_status_code();
        let logical = logical_handle();
        let (min, max, flag, serialized) = read_info(logical);
        assert_eq!(flag, 1);
        assert!(!min.is_empty());
        cosmos_feed_range_free(logical);
        let mut parsed = ptr::null_mut();
        assert_eq!(
            cosmos_feed_range_parse(view(serialized.as_bytes()), &mut parsed),
            success
        );
        assert_eq!(read_info(parsed), (min, max, 0, serialized));
        cosmos_feed_range_free(parsed);
    }

    #[test]
    fn relation_failures_clear_outputs() {
        let mut relation = CosmosFeedRangeRelation {
            is_subset: 1,
            overlaps: 1,
        };
        let invalid = CosmosErrorCode::CosmosErrorCodeInvalidArgument.as_status_code();
        assert_eq!(
            cosmos_feed_range_relate(ptr::null(), ptr::null(), &mut relation),
            invalid
        );
        assert_eq!((relation.is_subset, relation.overlaps), (0, 0));
        assert_eq!(
            cosmos_feed_range_relate(ptr::null(), ptr::null(), ptr::null_mut()),
            invalid
        );
    }

    #[test]
    fn empty_minimum_is_accepted() {
        let success = CosmosErrorCode::CosmosErrorCodeSuccess.as_status_code();
        let mut range = ptr::null_mut();
        assert_eq!(
            cosmos_feed_range_from_epk(view(b""), view(b"FF"), &mut range),
            success
        );
        let (min, max, ..) = read_info(range);
        assert_eq!((min.as_str(), max.as_str()), ("", "FF"));
        cosmos_feed_range_free(range);
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

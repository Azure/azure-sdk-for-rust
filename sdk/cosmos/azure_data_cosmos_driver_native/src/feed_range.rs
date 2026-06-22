// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! C ABI surface for `cosmos_feed_range_t` — wraps the driver's
//! [`azure_data_cosmos_driver::models::FeedRange`].
//!
//! Two constructors per spec section 4.6.4:
//!
//! - [`cosmos_feed_range_full`] — entire EPK key space, equivalent to
//!   passing NULL to `cosmos_operation_query_items`.
//! - [`cosmos_feed_range_for_partition_key`] — single logical partition
//!   key (the driver needs the container's partition-key definition,
//!   which the wrapper extracts from the supplied container reference).
//!
//! Plus `_clone` / `_free`. EPK-range and PKRangeId variants are
//! deferred (driver does not expose strongly-typed public constructors
//! for either yet — see spec section 4.6.4 deferred-list).

use std::sync::Arc;

use azure_data_cosmos_driver::models::FeedRange as DriverFeedRange;

use crate::container_ref::ContainerRefHandle;
use crate::error::CosmosErrorCode;
use crate::partition_key::PartitionKeyHandle;

/// The C ABI handle for a feed range.
///
/// A real Rust struct, not a `#[repr(C)]` layout: cbindgen emits it as an
/// opaque type (`cosmos_feed_range_t`) because C cannot see its fields. The
/// handle is reference-counted via `Arc`.
pub struct FeedRangeHandle {
    /// Consumed by the `query_items` request path.
    #[allow(dead_code, reason = "consumed by the query_items request path")]
    pub(crate) inner: DriverFeedRange,
}

impl FeedRangeHandle {
    fn into_raw(inner: DriverFeedRange) -> *mut Self {
        Arc::into_raw(Arc::new(FeedRangeHandle { inner })) as *mut Self
    }

    fn from_arc_into_raw(this: Arc<FeedRangeHandle>) -> *mut Self {
        Arc::into_raw(this) as *mut Self
    }

    pub(crate) fn inner_arc(p: *const FeedRangeHandle) -> Option<Arc<FeedRangeHandle>> {
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

    fn drop_raw(p: *mut FeedRangeHandle) {
        if p.is_null() {
            return;
        }
        // SAFETY: caller guarantees `p` was obtained from `into_raw` and has
        // not already been freed.
        unsafe {
            drop(Arc::from_raw(p as *const FeedRangeHandle));
        }
    }
}

/// Constructs a feed range covering the entire EPK key space. Mirrors
/// [`azure_data_cosmos_driver::models::FeedRange::full`].
///
/// # Returns
///
/// - `SUCCESS` (0) with `*out_fr` populated.
/// - `INVALID_ARGUMENT` (1) when `out_fr` is NULL.
#[no_mangle]
pub extern "C" fn cosmos_feed_range_full(out_fr: *mut *mut FeedRangeHandle) -> i32 {
    if out_fr.is_null() {
        return CosmosErrorCode::CosmosErrorCodeInvalidArgument.as_i32();
    }
    let handle = FeedRangeHandle::into_raw(DriverFeedRange::full());
    // SAFETY: caller guarantees `out_fr` is writable.
    unsafe {
        *out_fr = handle;
    }
    CosmosErrorCode::CosmosErrorCodeSuccess.as_i32()
}

/// Constructs a feed range that targets a single logical partition
/// key. Mirrors `FeedRange::for_partition(pk, container.partition_key_definition())`.
///
/// The partition-key definition is extracted from `container`, so
/// callers do not have to plumb it manually.
///
/// # Returns
///
/// - `SUCCESS` (0) with `*out_fr` populated.
/// - `INVALID_ARGUMENT` (1) when any pointer is NULL.
#[no_mangle]
pub extern "C" fn cosmos_feed_range_for_partition_key(
    container: *const ContainerRefHandle,
    pk: *const PartitionKeyHandle,
    out_fr: *mut *mut FeedRangeHandle,
) -> i32 {
    if out_fr.is_null() {
        return CosmosErrorCode::CosmosErrorCodeInvalidArgument.as_i32();
    }
    let Some(container_inner) = ContainerRefHandle::inner_arc(container) else {
        return CosmosErrorCode::CosmosErrorCodeInvalidArgument.as_i32();
    };
    let Some(pk_inner) = PartitionKeyHandle::inner_arc(pk) else {
        return CosmosErrorCode::CosmosErrorCodeInvalidArgument.as_i32();
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
    CosmosErrorCode::CosmosErrorCodeSuccess.as_i32()
}

/// Clones an existing feed-range handle. Cheap atomic refcount bump.
#[no_mangle]
pub extern "C" fn cosmos_feed_range_clone(
    fr: *const FeedRangeHandle,
    out_clone: *mut *mut FeedRangeHandle,
) -> i32 {
    if out_clone.is_null() {
        return CosmosErrorCode::CosmosErrorCodeInvalidArgument.as_i32();
    }
    let Some(arc) = FeedRangeHandle::inner_arc(fr) else {
        return CosmosErrorCode::CosmosErrorCodeInvalidArgument.as_i32();
    };
    let cloned = FeedRangeHandle::from_arc_into_raw(arc);
    // SAFETY: caller guarantees `out_clone` is writable.
    unsafe {
        *out_clone = cloned;
    }
    CosmosErrorCode::CosmosErrorCodeSuccess.as_i32()
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
    use super::*;
    use std::ptr;

    #[test]
    fn free_handles_null() {
        cosmos_feed_range_free(ptr::null_mut());
    }

    #[test]
    fn full_roundtrip() {
        let mut fr: *mut FeedRangeHandle = ptr::null_mut();
        assert_eq!(
            cosmos_feed_range_full(&mut fr),
            CosmosErrorCode::CosmosErrorCodeSuccess.as_i32()
        );
        assert!(!fr.is_null());
        cosmos_feed_range_free(fr);
    }

    #[test]
    fn full_rejects_null_out() {
        assert_eq!(
            cosmos_feed_range_full(ptr::null_mut()),
            CosmosErrorCode::CosmosErrorCodeInvalidArgument.as_i32()
        );
    }

    #[test]
    fn clone_full_round_trip() {
        let mut fr: *mut FeedRangeHandle = ptr::null_mut();
        cosmos_feed_range_full(&mut fr);
        let mut cloned: *mut FeedRangeHandle = ptr::null_mut();
        assert_eq!(
            cosmos_feed_range_clone(fr, &mut cloned),
            CosmosErrorCode::CosmosErrorCodeSuccess.as_i32()
        );
        assert!(!cloned.is_null());

        let a = FeedRangeHandle::inner_arc(fr).unwrap();
        let b = FeedRangeHandle::inner_arc(cloned).unwrap();
        assert!(Arc::ptr_eq(&a, &b));
        drop((a, b));

        cosmos_feed_range_free(fr);
        cosmos_feed_range_free(cloned);
    }

    #[test]
    fn for_partition_key_rejects_nulls() {
        let mut fr: *mut FeedRangeHandle = ptr::null_mut();
        assert_eq!(
            cosmos_feed_range_for_partition_key(ptr::null(), ptr::null(), &mut fr),
            CosmosErrorCode::CosmosErrorCodeInvalidArgument.as_i32()
        );
        assert_eq!(
            cosmos_feed_range_for_partition_key(ptr::null(), ptr::null(), ptr::null_mut()),
            CosmosErrorCode::CosmosErrorCodeInvalidArgument.as_i32()
        );
    }
}

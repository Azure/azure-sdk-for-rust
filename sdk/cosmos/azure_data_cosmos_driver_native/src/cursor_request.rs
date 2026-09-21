// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Versioned inputs for retained feed cursors.

use crate::{
    container_ref::ContainerRefHandle,
    error::CosmosErrorCode,
    feed_range::FeedRangeHandle,
    op_request::{build_request_with_operation, BuiltRequest, CosmosOperationRequest},
    string::{optional_text, CosmosStringView},
};
use azure_data_cosmos_driver::models::{ChangeFeedStartFrom, CosmosOperation};
use time::{format_description::well_known::Rfc3339, OffsetDateTime};

/// Retained feed input. Initialize with `cosmos_cursor_request_init`.
#[repr(C)]
pub struct CosmosCursorRequest {
    /// Readable input size, including any zero-filled extension bytes.
    pub struct_size_bytes: u32,
    /// Must be 1.
    pub abi_version: u32,
    /// Frozen legacy common fields; `kind` must be zero for change feeds.
    pub operation: CosmosOperationRequest,
    /// 0: common query/read kind; 1: LatestVersion; 2: AllVersionsAndDeletes.
    pub change_feed_mode: u32,
    /// 0: unset (resume only); 1: beginning; 2: now; 3: point in time.
    pub start_from: u32,
    /// Counted RFC 3339 timestamp, only with `start_from == 3`.
    pub start_time: CosmosStringView,
    /// Must be zero.
    pub reserved: [u32; 4],
}

/// Initialize a caller-owned full-size cursor request.
#[no_mangle]
pub extern "C" fn cosmos_cursor_request_init(out: *mut CosmosCursorRequest) {
    if out.is_null() {
        return;
    }
    // SAFETY: caller supplies writable storage for the complete record.
    unsafe {
        out.write(CosmosCursorRequest {
            struct_size_bytes: std::mem::size_of::<CosmosCursorRequest>() as u32,
            abi_version: 1,
            operation: std::mem::zeroed(),
            change_feed_mode: 0,
            start_from: 0,
            start_time: CosmosStringView::default(),
            reserved: [0; 4],
        });
        (*out).operation.max_item_count = -1;
    }
}

pub(crate) fn is_feed_kind(kind: i32) -> bool {
    matches!(kind, 2 | 3 | 4 | 10 | 11 | 15 | 16 | 17)
}

pub(crate) unsafe fn build_cursor_request(
    input: *const CosmosCursorRequest,
) -> Result<BuiltRequest, CosmosErrorCode> {
    let invalid = CosmosErrorCode::CosmosErrorCodeInvalidOptionValue;
    if input.is_null() {
        return Err(CosmosErrorCode::CosmosErrorCodeInvalidArgument);
    }
    // SAFETY: caller supplies at least the size/version prefix, even for short records.
    let size = unsafe { std::ptr::addr_of!((*input).struct_size_bytes).read() } as usize;
    if size < 8 {
        return Err(invalid);
    }
    // SAFETY: the prefix is readable, as checked above.
    let version = unsafe { std::ptr::addr_of!((*input).abi_version).read() };
    let known_size = std::mem::size_of::<CosmosCursorRequest>();
    if version != 1 || size < known_size || size > isize::MAX as usize {
        return Err(invalid);
    }
    // SAFETY: caller guarantees the declared size is readable.
    let extension = unsafe {
        std::slice::from_raw_parts(input.cast::<u8>().add(known_size), size - known_size)
    };
    if extension.iter().any(|b| *b != 0) {
        return Err(invalid);
    }
    // SAFETY: the full known record is readable; all discriminants are integers.
    let request = unsafe { &*input };
    if request.reserved != [0; 4] {
        return Err(invalid);
    }
    // SAFETY: counted input view is readable for this call.
    let timestamp = unsafe { optional_text(request.start_time, invalid)? };
    let start = match (request.start_from, timestamp) {
        (0, None) => None,
        (1, None) => Some(ChangeFeedStartFrom::Beginning),
        (2, None) => Some(ChangeFeedStartFrom::Now),
        (3, Some(text)) => Some(ChangeFeedStartFrom::PointInTime(
            OffsetDateTime::parse(&text, &Rfc3339).map_err(|_| invalid)?,
        )),
        _ => return Err(invalid),
    };
    let common = &request.operation;
    let operation = match request.change_feed_mode {
        0 if is_feed_kind(common.kind) && start.is_none() => None,
        1 | 2 if common.kind == 0 => {
            let container = ContainerRefHandle::from_ptr(common.container)
                .ok_or(CosmosErrorCode::CosmosErrorCodeInvalidArgument)?
                .inner
                .clone();
            let range = if common.feed_range.is_null() {
                Some(azure_data_cosmos_driver::models::FeedRange::full())
            } else {
                Some(
                    FeedRangeHandle::from_ptr(common.feed_range)
                        .ok_or(CosmosErrorCode::CosmosErrorCodeInvalidArgument)?
                        .inner
                        .clone(),
                )
            };
            let operation = if request.change_feed_mode == 1 {
                CosmosOperation::change_feed(container, range)
            } else {
                CosmosOperation::change_feed_all_versions_and_deletes(container, range)
            };
            if start.is_none() && common.continuation_token.is_unset() {
                return Err(invalid);
            }
            Some(match start {
                Some(start) => operation.with_change_feed_start(start),
                None => operation,
            })
        }
        _ => return Err(invalid),
    };
    // SAFETY: common fields retain the legacy input allocation contract.
    unsafe { build_request_with_operation(common, operation) }
}

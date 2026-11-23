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
use azure_data_cosmos_driver::models::MaxItemCountHint;
use azure_data_cosmos_driver::models::{ChangeFeedStartFrom, CosmosOperation};
use std::num::NonZeroU32;
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

/// Additive retained-feed options. Initialize with [`cosmos_feed_options_init()`].
///
/// Unused payloads must be zero. All values are copied before submit returns.
#[repr(C)]
pub struct CosmosFeedOptions {
    /// Readable record size, including zero-filled future extension bytes.
    pub struct_size_bytes: u32,
    /// Must be 1.
    pub abi_version: u32,
    /// 0 inherits the planner default; 1 applies the full `u64` value, including zero.
    pub has_max_buffered_query_window: u8,
    /// Maximum finite buffered query window. Ignored only when unset and zero.
    pub max_buffered_query_window: u64,
    /// 0 preserves the legacy hint; 1 is ServerDecides; 2 is a positive limit.
    pub max_item_count_kind: u32,
    /// 1..=u32::MAX for kind 2, otherwise zero. Explicit kinds require a negative legacy hint.
    ///
    /// Sent as a decimal `x-ms-max-item-count` header. Values above `i32::MAX` are forwarded
    /// as-is, but transports may treat them as server-decided or unset: Gateway 2.0 encodes
    /// `u32::MAX` as its -1 sentinel, and the in-memory emulator pages them as unset.
    pub max_item_count_limit: u32,
}

/// Initializes additive feed options to inherited defaults.
#[no_mangle]
pub extern "C" fn cosmos_feed_options_init(out: *mut CosmosFeedOptions) {
    if !out.is_null() {
        // SAFETY: caller supplies writable storage for the complete record.
        unsafe {
            out.write(CosmosFeedOptions {
                struct_size_bytes: std::mem::size_of::<CosmosFeedOptions>() as u32,
                abi_version: 1,
                has_max_buffered_query_window: 0,
                max_buffered_query_window: 0,
                max_item_count_kind: 0,
                max_item_count_limit: 0,
            });
        }
    }
}

pub(crate) unsafe fn build_cursor_request_v2(
    input: *const CosmosCursorRequest,
    options: *const CosmosFeedOptions,
) -> Result<BuiltRequest, CosmosErrorCode> {
    let invalid = CosmosErrorCode::CosmosErrorCodeInvalidOptionValue;
    if options.is_null() {
        // SAFETY: forwarded cursor input allocation contract.
        return unsafe { build_cursor_request(input) };
    }
    if !options.is_aligned() {
        return Err(CosmosErrorCode::CosmosErrorCodeInvalidArgument);
    }
    // SAFETY: caller supplies the readable size/version prefix.
    let size = unsafe { std::ptr::addr_of!((*options).struct_size_bytes).read() } as usize;
    let known = std::mem::size_of::<CosmosFeedOptions>();
    if size < known || size > isize::MAX as usize {
        return Err(invalid);
    }
    // SAFETY: declared size covers the full record and extension bytes.
    let (options, extension) = unsafe {
        (
            &*options,
            std::slice::from_raw_parts(options.cast::<u8>().add(known), size - known),
        )
    };
    if options.abi_version != 1 || extension.iter().any(|byte| *byte != 0) {
        return Err(invalid);
    }
    match options.has_max_buffered_query_window {
        0 if options.max_buffered_query_window == 0 => {}
        1 => {}
        _ => return Err(invalid),
    }
    let hint = match (options.max_item_count_kind, options.max_item_count_limit) {
        (0, 0) => None,
        (1, 0) => Some(MaxItemCountHint::ServerDecides),
        (2, limit) => Some(MaxItemCountHint::Limit(
            NonZeroU32::new(limit).ok_or(invalid)?,
        )),
        _ => return Err(invalid),
    };
    // SAFETY: all legacy input is copied and validated synchronously.
    let mut built = unsafe { build_cursor_request(input)? };
    if let Some(hint) = hint {
        // SAFETY: build_cursor_request validated the full input allocation.
        if unsafe { (*input).operation.max_item_count } >= 0 {
            return Err(invalid);
        }
        built.operation = built.operation.with_max_item_count(hint);
    }
    if options.has_max_buffered_query_window == 1 {
        // A window affects planning, never paging or checkpoint eligibility.
        built.plan_options = built
            .plan_options
            .with_max_buffered_query_window(options.max_buffered_query_window);
    }
    Ok(built)
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

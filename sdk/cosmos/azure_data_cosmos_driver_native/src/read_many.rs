// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Versioned read-many inputs using the retained feed-cursor lifecycle.

use crate::{
    completion::{CompletionQueue, OperationHandle},
    container_ref::ContainerRefHandle,
    cursor::open_cursor,
    driver::DriverHandle,
    error::{CosmosErrorCode, CosmosStatusCode},
    op_request::{build_request_with_operation, BuiltRequest, CosmosOperationRequest},
    partition_key::{partition_key_from_components, CosmosPartitionKeyComponent},
    string::{optional_text, required_text, validate_array, CosmosStringView},
};
use azure_data_cosmos_driver::{
    models::CosmosOperation,
    read_many::{ReadManyFilter, ReadManySelection},
};

/// One selected logical key, with an item id only for item selection.
#[repr(C)]
pub struct CosmosReadManyIdentity {
    /// Counted components in container path order; must describe a complete key.
    pub partition_key: *const CosmosPartitionKeyComponent,
    /// Component count (one to three).
    pub partition_key_len: usize,
    /// Required for Items; must be NULL/0 for Partitions.
    pub item_id: CosmosStringView,
}

/// A named filter parameter containing one JSON value.
#[repr(C)]
pub struct CosmosReadManyParameter {
    /// Parameter name including `@`.
    pub name: CosmosStringView,
    /// UTF-8 JSON encoding of the value.
    pub json_value: CosmosStringView,
}

/// Read-many cursor input. Initialize with [`cosmos_read_many_request_init()`].
///
/// Pointers are borrowed only until submit returns. Results use the existing
/// cursor completion format and ownership rules. No durable resume is supported.
#[repr(C)]
pub struct CosmosReadManyRequest {
    /// Readable record size, including zero-filled extensions.
    pub struct_size_bytes: u32,
    /// Must be 1.
    pub abi_version: u32,
    /// Container, session, activity, page hints, and options; kind must be zero.
    pub operation: CosmosOperationRequest,
    /// 1: exact items; 2: complete logical partitions.
    pub selection_kind: u32,
    /// Selected identities. NULL/0 selects nothing, never the full container.
    pub identities: *const CosmosReadManyIdentity,
    /// Number of selected identities.
    pub identities_len: usize,
    /// Optional scalar predicate using alias `c`, without WHERE.
    pub filter: CosmosStringView,
    /// Named JSON parameters for the predicate.
    pub parameters: *const CosmosReadManyParameter,
    /// Number of parameters.
    pub parameters_len: usize,
    /// Must be zero.
    pub reserved: [u32; 4],
}

/// Initializes a caller-owned request. Set selection kind and container before use.
///
/// # Safety
///
/// `out` must be NULL or point to writable, aligned storage for the full record.
#[no_mangle]
pub unsafe extern "C" fn cosmos_read_many_request_init(out: *mut CosmosReadManyRequest) {
    if out.is_null() {
        return;
    }
    // SAFETY: caller provides full writable storage; all fields permit zero bits.
    unsafe {
        out.write(CosmosReadManyRequest {
            struct_size_bytes: std::mem::size_of::<CosmosReadManyRequest>() as u32,
            abi_version: 1,
            operation: std::mem::zeroed(),
            selection_kind: 0,
            identities: std::ptr::null(),
            identities_len: 0,
            filter: CosmosStringView::default(),
            parameters: std::ptr::null(),
            parameters_len: 0,
            reserved: [0; 4],
        });
        (*out).operation.max_item_count = -1;
    }
}

/// Opens a read-many cursor without consuming its first page.
///
/// Use the existing cursor Next, Free, and completion functions. Checkpoint is
/// unsupported but does not terminate live iteration.
///
/// # Safety
///
/// Handles must be live and synchronized against release. `request` must
/// describe an aligned readable size/version prefix and the declared allocation;
/// nested arrays and strings must remain readable until this call returns.
/// `out_pre_error`, when non-NULL, must be writable.
#[no_mangle]
pub unsafe extern "C" fn cosmos_read_many_open_submit(
    driver: *const DriverHandle,
    request: *const CosmosReadManyRequest,
    queue: *mut CompletionQueue,
    user_data: isize,
    out_pre_error: *mut CosmosStatusCode,
) -> *mut OperationHandle {
    open_cursor(driver, queue, user_data, out_pre_error, || {
        // SAFETY: request satisfies the caller's counted allocation contract.
        unsafe { build_read_many(request) }
    })
}

unsafe fn build_read_many(
    input: *const CosmosReadManyRequest,
) -> Result<BuiltRequest, CosmosErrorCode> {
    let invalid = CosmosErrorCode::CosmosErrorCodeInvalidOptionValue;
    if input.is_null() {
        return Err(CosmosErrorCode::CosmosErrorCodeInvalidArgument);
    }
    // SAFETY: caller supplies the size prefix even for short records.
    let size = unsafe { std::ptr::addr_of!((*input).struct_size_bytes).read() } as usize;
    if size < 8 {
        return Err(invalid);
    }
    // SAFETY: the caller-declared prefix contains the version.
    let version = unsafe { std::ptr::addr_of!((*input).abi_version).read() };
    let known = std::mem::size_of::<CosmosReadManyRequest>();
    if version != 1 || size < known || size > isize::MAX as usize {
        return Err(invalid);
    }
    // SAFETY: caller guarantees that the declared allocation is readable.
    let extension =
        unsafe { std::slice::from_raw_parts(input.cast::<u8>().add(known), size - known) };
    if extension.iter().any(|b| *b != 0) {
        return Err(invalid);
    }
    // SAFETY: full known record lies within the validated readable size.
    let input = unsafe { &*input };
    if input.reserved != [0; 4] || !matches!(input.selection_kind, 1 | 2) {
        return Err(invalid);
    }
    let common = &input.operation;
    if common.kind != 0
        || !common.continuation_token.is_unset()
        || !common.account.is_null()
        || !common.database.is_null()
        || !common.item_id.is_unset()
        || !common.resource_link.is_unset()
        || !common.partition_key.is_null()
        || !common.partition_key_components.is_null()
        || common.partition_key_len != 0
        || !common.feed_range.is_null()
        || !common.body.is_null()
        || common.body_len != 0
        || common.precondition_kind != 0
        || !common.precondition_etag.is_unset()
        || common.patch_max_attempts != 0
        || !common.patch_tracking_id.is_unset()
        || common.patch_tracking_capacity != 0
        || common.patch_tracking_retention_seconds != 0
    {
        return Err(invalid);
    }
    let container = ContainerRefHandle::from_ptr(common.container)
        .ok_or(CosmosErrorCode::CosmosErrorCodeInvalidArgument)?
        .inner
        .clone();
    validate_array(input.identities, input.identities_len)?;
    let identities = if input.identities_len == 0 {
        &[]
    } else {
        // SAFETY: metadata is validated; allocation validity is the caller's contract.
        unsafe { std::slice::from_raw_parts(input.identities, input.identities_len) }
    };
    let mut items = Vec::new();
    let mut partitions = Vec::new();
    for identity in identities {
        validate_array(identity.partition_key, identity.partition_key_len)?;
        // SAFETY: nested component arrays are readable for this submit.
        let key = unsafe {
            partition_key_from_components(identity.partition_key, identity.partition_key_len)?
        };
        if input.selection_kind == 1 {
            // SAFETY: counted item id is readable for this submit.
            let id = unsafe { required_text(identity.item_id, invalid)? };
            items.push((key, id));
        } else {
            if !identity.item_id.is_unset() {
                return Err(invalid);
            }
            partitions.push(key);
        }
    }
    // SAFETY: predicate view is readable for this submit.
    let text = unsafe { optional_text(input.filter, invalid)? };
    let mut filter = text
        .map(|text| text.parse::<ReadManyFilter>().map_err(|_| invalid))
        .transpose()?;
    validate_array(input.parameters, input.parameters_len)?;
    if input.parameters_len != 0 {
        let filter = filter.as_mut().ok_or(invalid)?;
        // SAFETY: metadata is validated and caller supplies readable parameter records.
        for parameter in
            unsafe { std::slice::from_raw_parts(input.parameters, input.parameters_len) }
        {
            // SAFETY: nested text views remain readable for this submit.
            let (name, value) = unsafe {
                (
                    required_text(parameter.name, invalid)?,
                    required_text(parameter.json_value, invalid)?,
                )
            };
            let value = serde_json::from_str(&value).map_err(|_| invalid)?;
            *filter = filter
                .clone()
                .with_parameter(name, value)
                .map_err(|_| invalid)?;
        }
    }
    let selection = if input.selection_kind == 1 {
        ReadManySelection::Items(items)
    } else {
        ReadManySelection::Partitions(partitions)
    };
    let operation = CosmosOperation::read_many(container, selection, filter);
    // SAFETY: common fields inherit the request allocation contract.
    unsafe { build_request_with_operation(common, Some(operation)) }
}

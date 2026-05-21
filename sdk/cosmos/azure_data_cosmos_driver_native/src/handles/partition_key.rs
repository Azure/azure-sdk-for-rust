// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Partition-key handles (Phase 4 — stub).
//!
//! Will wrap [`azure_data_cosmos_driver::models::PartitionKey`] with a
//! builder supporting all five value types and hierarchical keys.

use azure_data_cosmos_driver::models::PartitionKey;

/// Opaque handle to a built [`PartitionKey`].
///
/// cbindgen:ignore
#[allow(non_camel_case_types, dead_code)]
pub struct cosmos_partition_key(pub(crate) PartitionKey);

/// Releases a partition-key handle.
///
/// # Safety
/// `pk` must be null or a pointer previously returned by a
/// `cosmos_partition_key_*` constructor.
#[no_mangle]
pub unsafe extern "C" fn cosmos_partition_key_free(pk: *mut cosmos_partition_key) {
    if !pk.is_null() {
        drop(Box::from_raw(pk));
    }
}

// TODO(phase-4): cosmos_partition_key_builder_{new,append_string,append_number,
// append_bool,append_null,append_none,build,free} and the
// cosmos_partition_key_from_string convenience helper.

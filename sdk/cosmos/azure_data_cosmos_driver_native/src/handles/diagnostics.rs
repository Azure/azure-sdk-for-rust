// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Diagnostics handles (Phase 7 — stub).
//!
//! Will wrap `Arc<azure_data_cosmos_driver::DiagnosticsContext>` and expose
//! aggregate metrics, region iteration, and the optional JSON snapshot.

use std::sync::Arc;

use azure_data_cosmos_driver::DiagnosticsContext;

/// Opaque diagnostics handle (an `Arc` clone of the response's diagnostics).
///
/// cbindgen:ignore
#[allow(non_camel_case_types, dead_code)]
pub struct cosmos_diagnostics(pub(crate) Arc<DiagnosticsContext>);

/// Releases the diagnostics handle, dropping its `Arc` strong count.
///
/// # Safety
/// `d` must be null or a pointer returned by `cosmos_response_diagnostics`.
#[no_mangle]
pub unsafe extern "C" fn cosmos_diagnostics_free(d: *mut cosmos_diagnostics) {
    if !d.is_null() {
        drop(Box::from_raw(d));
    }
}

// TODO(phase-7): cosmos_diagnostics_total_request_charge,
// cosmos_diagnostics_total_elapsed_micros, cosmos_diagnostics_retry_count,
// cosmos_diagnostics_iter_regions_contacted, cosmos_diagnostics_to_json.

// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Response handles (Phase 6 — stub).
//!
//! Will wrap [`azure_data_cosmos_driver::models::CosmosResponse`] with
//! accessors for status, RU charge, activity id, ETag, session token,
//! continuation token, headers, body, and diagnostics.

use azure_data_cosmos_driver::models::CosmosResponse;

/// Opaque handle to a `CosmosResponse`.
///
/// cbindgen:ignore
#[allow(non_camel_case_types, dead_code)]
pub struct cosmos_response(pub(crate) CosmosResponse);

/// Releases a response handle.
///
/// # Safety
/// `r` must be null or a pointer returned by `cosmos_driver_execute`.
#[no_mangle]
pub unsafe extern "C" fn cosmos_response_free(r: *mut cosmos_response) {
    if !r.is_null() {
        drop(Box::from_raw(r));
    }
}

// TODO(phase-6): cosmos_response_status_code, cosmos_response_request_charge,
// cosmos_response_activity_id, cosmos_response_etag,
// cosmos_response_session_token, cosmos_response_continuation_token,
// cosmos_response_iter_headers, cosmos_response_body,
// cosmos_response_into_body, cosmos_response_diagnostics.

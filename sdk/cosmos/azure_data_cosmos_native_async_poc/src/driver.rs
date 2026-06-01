// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Opaque driver handle wrapping a built [`azure_data_cosmos::CosmosClient`]
//! plus an `Arc` reference to the runtime that owns the Tokio thread pool
//! the driver was constructed against.
//!
//! For the spike, every driver is built with shared-key authentication
//! against the well-known emulator master key. The endpoint and key are
//! passed in as caller-owned C strings. Building the client is itself an
//! `async` operation (the driver primes its region cache), so the call is
//! `block_on`'d against the host-supplied runtime — this is the **only**
//! place the FFI ever blocks. All subsequent operations submit non-blocking.

use std::ffi::{c_char, CStr};
use std::sync::Arc;

use azure_core::credentials::Secret;
use azure_data_cosmos::{
    AccountEndpoint, AccountReference, CosmosClient, CosmosClientBuilder, Region, RoutingStrategy,
};
use tokio::runtime::Runtime;

use crate::error::CosmosStatusCode;
use crate::ffi_guard;
use crate::runtime::CosmosRuntime;

/// Public opaque type.
pub struct CosmosDriver {
    pub(crate) client: CosmosClient,
    pub(crate) runtime: Arc<Runtime>,
}

/// Creates a Cosmos driver bound to `runtime`. `endpoint` and `master_key`
/// must be NUL-terminated UTF-8 strings owned by the caller. The driver
/// keeps a strong reference to the runtime, so the host may free the
/// runtime handle while drivers are still alive — the runtime will live
/// until the last driver is freed.
///
/// On success, writes the handle to `*out` and returns `Ok`. On failure
/// writes `null` to `*out` and returns one of the error codes.
///
/// # Safety
///
/// `runtime`, `endpoint`, `master_key`, and `out` must be non-null. The
/// strings must remain valid for the duration of the call (they are copied
/// into Rust-owned storage before the call returns).
#[no_mangle]
pub unsafe extern "C" fn cosmos_driver_new(
    runtime: *mut CosmosRuntime,
    endpoint: *const c_char,
    master_key: *const c_char,
    out: *mut *mut CosmosDriver,
) -> i32 {
    ffi_guard!(CosmosStatusCode::InternalError as i32, {
        if runtime.is_null() || endpoint.is_null() || master_key.is_null() || out.is_null() {
            if !out.is_null() {
                *out = std::ptr::null_mut();
            }
            return CosmosStatusCode::InvalidArg as i32;
        }
        *out = std::ptr::null_mut();

        let endpoint_str = match CStr::from_ptr(endpoint).to_str() {
            Ok(s) => s.to_owned(),
            Err(_) => return CosmosStatusCode::InvalidArg as i32,
        };
        let key_str = match CStr::from_ptr(master_key).to_str() {
            Ok(s) => s.to_owned(),
            Err(_) => return CosmosStatusCode::InvalidArg as i32,
        };

        let endpoint_parsed: AccountEndpoint = match endpoint_str.parse() {
            Ok(e) => e,
            Err(_) => return CosmosStatusCode::InvalidArg as i32,
        };

        let rt_arc = (*runtime).inner.clone();

        let build_result = rt_arc.block_on(async {
            let account =
                AccountReference::with_authentication_key(endpoint_parsed, Secret::from(key_str));
            CosmosClientBuilder::new()
                .with_allow_emulator_invalid_certificates(true)
                .build(account, RoutingStrategy::ProximityTo(Region::EAST_US))
                .await
        });

        match build_result {
            Ok(client) => {
                let handle = Box::new(CosmosDriver {
                    client,
                    runtime: rt_arc,
                });
                *out = Box::into_raw(handle);
                CosmosStatusCode::Ok as i32
            }
            Err(_) => CosmosStatusCode::InternalError as i32,
        }
    })
}

/// Releases the driver. Cancels nothing — pending operations submitted
/// against this driver continue to drain into the CQ. The host should
/// drain or shut down the CQ first.
///
/// # Safety
///
/// `driver` must have been returned by [`cosmos_driver_new`] and not yet
/// freed. `null` is a no-op.
#[no_mangle]
pub unsafe extern "C" fn cosmos_driver_free(driver: *mut CosmosDriver) {
    if driver.is_null() {
        return;
    }
    ffi_guard!((), {
        let _ = Box::from_raw(driver);
    })
}

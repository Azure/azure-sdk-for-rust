// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Internal error type for the spike. The C ABI surface uses
//! [`cosmos_status_t`] (a plain integer) — `FfiError` exists only so internal
//! helpers can return a typed `Result<T, FfiError>` and the FFI wrappers can
//! map it to a status code at the boundary.

/// FFI status codes returned by the spike's entry points. Matches the
/// `cosmos_status_t` enum in `include/cosmos_async_poc.h`. Keep these in
/// sync (the spike is small enough that hand-syncing is cheaper than wiring
/// `cbindgen`; the production crate will use `cbindgen`).
#[repr(i32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum CosmosStatusCode {
    Ok = 0,
    InvalidArg = 1,
    Cancelled = 2,
    QueueShutdown = 3,
    ServiceError = 4,
    InternalError = 5,
}

/// Internal Rust-side error used by helpers; never crosses the FFI boundary
/// as a Rust value — the wrapper translates it to a `CosmosStatusCode` and
/// (for the operation-completion path) packages the message into a
/// `cosmos_response_t` whose status field carries the service status code
/// (e.g. 404) and whose body carries any service-supplied error JSON.
#[derive(Debug)]
pub enum FfiError {
    InvalidArg(&'static str),
    Cancelled,
    /// Service replied but with a non-2xx status. The HTTP status code and
    /// the response body (which may contain a Cosmos error envelope) are
    /// preserved so the host can surface a typed exception.
    Service {
        status: u16,
        body: Vec<u8>,
    },
    /// Transport, TLS, serialization, or any other client-side failure.
    Internal(String),
}

impl FfiError {
    pub fn code(&self) -> CosmosStatusCode {
        match self {
            FfiError::InvalidArg(_) => CosmosStatusCode::InvalidArg,
            FfiError::Cancelled => CosmosStatusCode::Cancelled,
            FfiError::Service { .. } => CosmosStatusCode::ServiceError,
            FfiError::Internal(_) => CosmosStatusCode::InternalError,
        }
    }
}

impl From<azure_data_cosmos::CosmosError> for FfiError {
    fn from(err: azure_data_cosmos::CosmosError) -> Self {
        // Pull the HTTP status as a u16. The driver populates a synthetic
        // TRANSPORT_GENERATED_503 for non-service failures, so we treat
        // anything with a wire response as a Service error and everything
        // else as Internal. For the spike we don't pull the wire-error
        // body out (the production crate's CosmosResponse accessor surface
        // is still in flight — see PR #4461 §4.7); a textual summary is
        // enough to prove the F-checks.
        let status_u16: u16 = err.status().status_code().into();
        if err.response().is_some() {
            FfiError::Service {
                status: status_u16,
                body: format!("service error (status {status_u16}): {err}").into_bytes(),
            }
        } else {
            FfiError::Internal(format!("cosmos error (status {status_u16}): {err}"))
        }
    }
}

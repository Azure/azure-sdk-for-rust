// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! # azure_data_cosmos_native_async_poc
//!
//! Async-FFI feasibility spike for the planned `azure_data_cosmos_driver_native`
//! C ABI (PR #4461). The committed spec only covers a synchronous C ABI, which
//! forces every L3 language SDK to burn a host thread per in-flight operation —
//! a non-starter for .NET, Java, Go, or Python at production load.
//!
//! This crate validates the hypothesis that a **completion-queue model**
//! (analogous to gRPC C-core, libuv, IOCP, kqueue, io_uring) lets a small
//! number of host threads drain many completions from Rust without ever
//! blocking inside the FFI boundary. It is a 15-hour spike, not production
//! code. See `INVARIANTS.md` and `include/cosmos_async_poc.h` for the full
//! contract.
//!
//! ## ABI shape (mirrors the C header in `include/cosmos_async_poc.h`)
//!
//! ```text
//!   cosmos_runtime_new / _free      ── one Tokio multi-thread runtime
//!   cosmos_driver_new  / _free      ── one CosmosClient bound to runtime
//!   cosmos_cq_new / _free / _shutdown / _wait ── completion queue (1 waiter)
//!   cosmos_read_item               ── non-blocking submit
//!   cosmos_cancel / cosmos_op_release ── separate cancel + release (I5)
//!   cosmos_response_status / _body / _free
//! ```
//!
//! ## Module layout
//!
//! - [`error`]   internal `FfiError` enum and host-visible status mapping.
//! - [`runtime`] Tokio runtime handle.
//! - [`driver`]  `CosmosClient` + cached runtime handle.
//! - [`cq`]      completion queue (single-waiter for the spike).
//! - [`op`]      operation handle with cancel flag and abort handle.
//! - [`response`] success response with status code + raw body bytes.
//! - [`read_item`] the one operation factory implemented in the spike.
//!
//! Every `extern "C" fn` in this crate is wrapped in [`ffi_guard!`] which
//! installs `std::panic::catch_unwind` so a Rust panic can never unwind across
//! the FFI boundary into a host runtime — that would be undefined behavior on
//! every platform we care about.

#![allow(clippy::missing_safety_doc)]

pub mod cq;
pub mod driver;
pub mod error;
pub mod op;
pub mod read_item;
pub mod response;
pub mod runtime;

/// Wraps an FFI entry point body in `catch_unwind` and converts the unwind
/// payload into the supplied error value. Invariant I3 from `INVARIANTS.md`.
///
/// Usage:
/// ```ignore
/// pub unsafe extern "C" fn cosmos_foo() -> i32 {
///     ffi_guard!(-1, { /* body returning i32 */ })
/// }
/// ```
#[macro_export]
macro_rules! ffi_guard {
    ($on_panic:expr, $body:block) => {{
        match ::std::panic::catch_unwind(::std::panic::AssertUnwindSafe(|| $body)) {
            Ok(v) => v,
            Err(_) => $on_panic,
        }
    }};
}

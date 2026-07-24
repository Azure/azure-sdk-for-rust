// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

#![doc = include_str!("../README.md")]
// Cosmos-wide lint policy (see sdk/cosmos/lints.rs for documentation).
#![deny(clippy::allow_attributes)]
#![deny(clippy::missing_panics_doc)]
#![deny(clippy::should_panic_without_expect)]
#![deny(clippy::unwrap_in_result)]
#![cfg_attr(not(test), deny(clippy::unwrap_used))]
#![deny(clippy::wildcard_imports)]
// For the driver, every `.expect()` call in non-test code must be intentional
// and annotated with `#[expect(clippy::expect_used, reason = "...")]`.
#![cfg_attr(not(test), deny(clippy::expect_used))]

pub mod diagnostics;
pub mod driver;
pub mod error;
#[cfg(feature = "fault_injection")]
pub mod fault_injection;
#[cfg(feature = "__internal_in_memory_emulator")]
pub mod in_memory_emulator;
pub mod models;
pub mod options;
// The `query` module is local-plan scaffolding. Many helpers (gateway response
// envelope, value comparison helpers, etc.) are temporarily unused in the driver
// proper because no production caller wires the local plan generator in yet. The
// `#[allow(dead_code)]` annotation is intentional and should be removed once the
// driver pipeline starts consuming the local plan output. Until then, individual
// per-item `#[allow(dead_code)]` would mean ~50 annotations across lexer/parser/
// eval/plan scaffolding without changing what the compiler actually checks.
//
// The two `mod query;` declarations differ only in visibility, which is gated on
// the `__internal_testing` feature: when that feature is on we expose a small,
// `#[doc(hidden)]` test-only surface (`__test_only_generate_query_plan_for_pk_paths`,
// `__TEST_ONLY_SUPPORTED_QUERY_FEATURES`) so cross-crate gateway-comparison
// tests can drive the local plan generator without depending on internal types;
// otherwise the module is `pub(crate)` and nothing leaks out of the crate.
// Keep both arms in sync if you add another item under `mod query`.
//
// TODO(local-plan-wire-up): drop `allow(dead_code)` once the driver wires the
// local plan generator into the query execution path.
#[cfg(any(test, feature = "__internal_testing"))]
#[allow(dead_code)]
pub mod query;
#[cfg(not(any(test, feature = "__internal_testing")))]
#[allow(dead_code)]
pub(crate) mod query;
#[allow(dead_code)]
pub(crate) mod query_plan_native;
pub(crate) mod system;
#[cfg(feature = "__internal_mocking")]
pub mod testing;

// Re-export key types at crate root
pub use diagnostics::{DiagnosticsContext, ExecutionContext, RequestDiagnostics, RequestHandle};
pub use driver::{CosmosDriver, CosmosDriverRuntime, CosmosDriverRuntimeBuilder, OperationPlan};
pub use error::{CosmosError, CosmosErrorBuilder, CosmosStatus, Result, SubStatusCode};
pub use models::{ActivityId, CosmosResponse, RequestCharge, ResponseBody};
pub use options::{DiagnosticsOptions, DiagnosticsVerbosity, DriverOptions};

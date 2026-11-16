// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

#![doc = include_str!("../README.md")]
// Nightly sanitizer builds need deeper trait evaluation for container-recreation futures.
#![cfg_attr(fuzzing, recursion_limit = "256")]

//! Azure Cosmos DB Driver - Core Implementation Layer
//!
//! This crate provides the core transport, routing, and protocol handling for Azure Cosmos DB.
//! It is designed to be reused across multiple language SDKs.
//!
//! # Support Model
//!
//! This crate has a **public API** but receives **community/GitHub support only** (no 24x7 Microsoft Support).
//! For production Rust applications, use [`azure_data_cosmos`](https://docs.rs/azure_data_cosmos) instead,
//! which provides full Microsoft support.
//!
//! # Schema-Agnostic Design
//!
//! The driver is intentionally ignorant of document/item schemas. Data plane operations accept
//! raw bytes (`&[u8]`) and return buffered responses (`Vec<u8>`). Serialization is handled by
//! the consuming SDK in its native language.

pub mod binary_json;
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
// otherwise the module is `pub(crate)` and nothing leaks out of the crate. The
// module itself is also `#[doc(hidden)]` since its only public members are
// already `#[doc(hidden)]`, so it would otherwise appear as an empty public
// module in generated API surfaces (e.g. APIView with `--all-features`).
// Keep both arms in sync if you add another item under `mod query`.
//
// TODO(local-plan-wire-up): drop `allow(dead_code)` once the driver wires the
// local plan generator into the query execution path.
#[cfg(any(test, feature = "__internal_testing"))]
#[doc(hidden)]
#[allow(dead_code)]
pub mod query;
#[cfg(not(any(test, feature = "__internal_testing")))]
#[allow(dead_code)]
pub(crate) mod query;
#[allow(dead_code)]
pub(crate) mod query_plan_native;
pub(crate) mod system;
#[cfg(feature = "__internal_mocking")]
pub mod test;

// Re-export key types at crate root
pub use diagnostics::{DiagnosticsContext, ExecutionContext, RequestDiagnostics, RequestHandle};
pub use driver::{CosmosDriver, CosmosDriverRuntime, CosmosDriverRuntimeBuilder, OperationPlan};
pub use error::{CosmosError, CosmosErrorBuilder, CosmosStatus, Result, SubStatusCode};
pub use models::{ActivityId, CosmosResponse, RequestCharge, ResponseBody};
pub use options::{DiagnosticsOptions, DiagnosticsThresholds, DiagnosticsVerbosity, DriverOptions};

#[cfg(test)]
mod tests {
    use std::{fs, path::Path};

    #[test]
    fn docs_rs_includes_public_features() {
        // These features are already enabled by the default feature set.
        const IGNORE: &[&str] = &["default", "reqwest", "rustls", "tokio"];

        let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("Cargo.toml");
        let manifest: toml::Table = fs::read_to_string(path)
            .expect("read Cargo.toml")
            .parse()
            .expect("parse Cargo.toml");
        let features = manifest
            .get("features")
            .map(|value| value.as_table().expect("features must be a table"));
        let docs_metadata =
            ["package", "metadata", "docs", "rs"]
                .iter()
                .try_fold(&manifest, |table, key| {
                    table
                        .get(*key)
                        .map(|value| value.as_table().expect("docs.rs metadata must be tables"))
                });
        let docs_features: Vec<&str> = docs_metadata
            .and_then(|table| table.get("features"))
            .map(|value| {
                value
                    .as_array()
                    .expect("docs.rs features must be an array")
                    .iter()
                    .map(|value| value.as_str().expect("feature names must be strings"))
                    .collect()
            })
            .unwrap_or_default();
        let missing: Vec<&str> = features
            .into_iter()
            .flat_map(|features| features.keys())
            .map(String::as_str)
            .filter(|feature| {
                !feature.starts_with("__")
                    && !IGNORE.contains(feature)
                    && !docs_features.contains(feature)
            })
            .collect();
        assert!(
            missing.is_empty(),
            "Add these features to package.metadata.docs.rs.features: {missing:?}"
        );
    }
}

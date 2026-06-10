// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Native FFI query plan provider using the QueryPlanInterop C++ library.
//!
//! This module provides a safe Rust wrapper around `Cosmos.QueryPlanInterop.dll`
//! (Windows) / `libqueryplaninterop.so` (Linux), which generates partitioned
//! query execution plans for Cosmos DB SQL queries.
//!
//! Gated behind the `__internal_native_query_plan` feature flag. When disabled,
//! the driver uses the Gateway for all query plans. The module itself always
//! compiles so that tests can run independently of the feature flag.
//!
//! # Library loading
//!
//! The native library is loaded lazily on first query plan request.
//! If the DLL/.so is not found, the result is cached and the driver
//! falls back to the Gateway for all subsequent calls.
//!
//! Search order:
//! 1. `AZURE_COSMOS_QUERYPLANINTEROP_DIR` environment variable (absolute path)
//! 2. OS default (`PATH` on Windows, `LD_LIBRARY_PATH` on Linux)
//!
//! | Platform | Library name |
//! |----------|-------------|
//! | Windows  | `Cosmos.QueryPlanInterop.dll` |
//! | Linux    | `libqueryplaninterop.so` |
//! | macOS    | `libqueryplaninterop.dylib` |
//!
//! # Call chain
//!
//! ```text
//! cosmos_driver::plan_operation()
//!   -> NativeQueryPlanProvider::get_query_plan()
//!      -> OnceLock: cached QueryPlanProvider (created once, reused)
//!         -> QueryPlanProvider::new(config)
//!            -> query_plan_native_lib()  [OnceLock: loaded once per process]
//!               -> platform::load_library(LIB_NAME) -> LoadLibraryA / dlopen
//!               -> GetProcAddress / dlsym for each export
//!            -> calls CreateServiceProvider(config)
//!         -> provider.get_partition_key_ranges(query, pk_paths, options)
//!            -> calls GetPartitionKeyRangesFromQuery4(...)
//!            -> deserialize JSON -> QueryPlan
//!   -> if Err: fall through to gateway_query_plan()
//! ```
//!
//! # Reusing the driver's `QueryPlan` model
//!
//! The native DLL outputs the same JSON format as the Cosmos DB Gateway's
//! query plan endpoint. The provider deserializes the DLL's output directly
//! into `crate::driver::dataflow::query_plan::QueryPlan`, avoiding type
//! duplication.
//!
//! The following deserialization differences between Gateway JSON and native
//! DLL JSON are handled in `query_plan.rs`:
//!
//! | Field | Gateway format | Native DLL format | Resolution |
//! |---|---|---|---|
//! | `hasSelectValue`, `hasNonStreamingOrderBy`, `requiresGlobalStatistics`, `isMinInclusive`, `isMaxInclusive` | `true`/`false` | `0`/`1` integer | `bool_from_int_or_bool` custom deserializer |
//! | `rewrittenQuery` | `"SELECT ..."` | `null` when absent | Changed from `String` to `Option<String>` |
//! | `groupByAliasToAggregateType` values | `"Average"` | `""`, `null`, or `"Average"` | Changed from `HashMap<String, String>` to `HashMap<String, serde_json::Value>` |
//! | `queryRanges[].min/max` | hex EPK strings (`"05C1..."`) | structured JSON values (arrays/objects) | `string_or_json` custom deserializer (converts non-strings to JSON text at deserialization time) |
//! | `partitionedQueryExecutionInfoVersion` | always present | omitted by native DLL | Added `#[serde(default)]` on `QueryPlan` struct |
//! | `aggregates` (non-VALUE queries) | populated | empty (info in `groupByAliasToAggregateType` instead) | Tests use `has_aggregates()` helper that checks both fields |
//!
//! # Running tests
//!
//! ```bash
//! # Unit tests (no native DLL needed)
//! cargo test -p azure_data_cosmos_driver --lib query_plan_native
//!
//! # Integration tests (requires native DLL)
//! AZURE_COSMOS_QUERYPLANINTEROP_DIR=/path/to/lib \
//! RUSTFLAGS='--cfg test_category="native_query_plan"' \
//!     cargo test -p azure_data_cosmos_driver --lib query_plan_native \
//!         --features __internal_native_query_plan
//! ```
//!
//! # Regenerating bindgen bindings
//!
//! Requires `bindgen-cli` and `libclang`:
//!
//! ```bash
//! cargo install bindgen-cli
//! cd sdk/cosmos/azure_data_cosmos_driver/src/query_plan_native
//! LIBCLANG_PATH=/path/to/libclang bindgen bindgen_wrapper.h \
//!     --use-core --no-layout-tests \
//!     --allowlist-type "QueryPlanInterop.*" \
//!     --allowlist-function "CreateServiceProvider|UpdateServiceProvider|GetPartitionKeyRangesFromQuery4" \
//!     --output generated/native_bindings.rs \
//!     -- -x c++ -D_WIN32
//! ```

// ABI contract items (HRESULT constants, enum variants, free_library) are
// defined for completeness but not all are consumed by the driver yet.
#[allow(dead_code)]
pub(crate) mod error;
#[path = "generated/native_bindings.rs"]
#[allow(dead_code)]
// NOTE: Generated bindings assume Windows (WCHAR = u16). On Linux/macOS
// WCHAR is u32. These bindings are only used for struct layout validation,
// not for runtime FFI calls. See native.rs for the platform-correct WChar type.
pub(crate) mod generated_bindings;
#[allow(dead_code)]
pub(crate) mod native;
#[allow(dead_code)]
pub(crate) mod provider;

#[cfg(test)]
mod native_dll_tests;

// Re-export the driver's query plan model types for use by integration tests.
#[cfg(test)]
pub(crate) use crate::driver::dataflow::query_plan::{
    DistinctType, QueryInfo, QueryPlan, SortOrder,
};

/// High-level entry point for native query plan generation.
///
/// Encapsulates all lazy-initialization logic: the inner
/// [`QueryPlanProvider`](provider::QueryPlanProvider) is created on first use
/// and cached for the process lifetime. If the native library is unavailable,
/// `None` is cached so subsequent calls fail fast without retrying.
///
/// The query engine configuration is accepted per-call and the provider is
/// updated when it changes (e.g. after an account metadata refresh).
///
/// The driver holds an instance of this type and calls
/// [`get_query_plan`](NativeQueryPlanProvider::get_query_plan) -- no `OnceLock`
/// or `Option` handling leaks into the driver code.
pub(crate) struct NativeQueryPlanProvider {
    inner: std::sync::OnceLock<Result<provider::QueryPlanProvider, error::QueryPlanError>>,
    last_config: std::sync::Mutex<String>,
}

impl std::fmt::Debug for NativeQueryPlanProvider {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("NativeQueryPlanProvider")
            .field("initialized", &self.inner.get().is_some())
            .finish()
    }
}

impl NativeQueryPlanProvider {
    pub fn new() -> Self {
        Self {
            inner: std::sync::OnceLock::new(),
            last_config: std::sync::Mutex::new(String::new()),
        }
    }

    /// Generates a query plan using the native FFI provider.
    ///
    /// On first call, lazily loads the native library and creates the
    /// service provider with the given `query_engine_config`. On subsequent
    /// calls, if the config has changed the provider is updated via
    /// `UpdateServiceProvider`.
    pub fn get_query_plan(
        &self,
        query_spec_json: &str,
        partition_key_paths: &[&str],
        partition_key_kind: crate::models::PartitionKeyKind,
        query_engine_config: &str,
    ) -> Result<crate::driver::dataflow::query_plan::QueryPlan, error::QueryPlanError> {
        let provider = self
            .inner
            .get_or_init(|| provider::QueryPlanProvider::new(query_engine_config))
            .as_ref()
            .map_err(|e| error::QueryPlanError::LibraryNotAvailable {
                message: format!("{e}"),
            })?;

        // Update the provider if the query engine configuration changed
        // (e.g. after an account metadata refresh).
        {
            let mut last = self.last_config.lock().unwrap_or_else(|e| e.into_inner());
            if *last != query_engine_config {
                if let Err(e) = provider.update(query_engine_config) {
                    tracing::warn!("failed to update native query plan config: {e}");
                } else {
                    *last = query_engine_config.to_string();
                }
            }
        }

        let partition_kind = match partition_key_kind {
            crate::models::PartitionKeyKind::MultiHash => native::PartitionKind::MultiHash,
            _ => native::PartitionKind::Hash,
        };

        let options = provider::QueryPlanOptions {
            partition_kind,
            ..provider::QueryPlanOptions::default()
        };

        provider.get_partition_key_ranges(query_spec_json, partition_key_paths, &options, None)
    }
}

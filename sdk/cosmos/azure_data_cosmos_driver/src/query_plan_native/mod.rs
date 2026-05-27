// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Native FFI query plan provider using the QueryPlanInterop C++ library.
//!
//! This module provides a safe Rust wrapper around `Cosmos.QueryPlanInterop.dll`
//! (Windows) / `libQueryPlanInterop.so` (Linux), which generates partitioned
//! query execution plans for Cosmos DB SQL queries.
//!
//! The native library is loaded dynamically at runtime -- no compile-time
//! dependency is required.
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

#[cfg(all(test, feature = "__query_plan_native_integration"))]
mod integration_tests;

// Re-export the driver's query plan model types for use by integration tests
// and future callers within the crate. The native DLL outputs the same JSON
// format as the Gateway, so both deserialize into these types.
#[cfg(all(test, feature = "__query_plan_native_integration"))]
pub(crate) use crate::driver::dataflow::query_plan::{
    DistinctType, QueryInfo, QueryPlan, SortOrder,
};

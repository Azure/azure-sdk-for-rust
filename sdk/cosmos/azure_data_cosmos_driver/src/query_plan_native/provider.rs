// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Safe wrapper around the QueryPlanInterop native library.
//!
//! [`QueryPlanProvider`] manages the lifecycle of the native service
//! provider and exposes a safe Rust API for generating query plans.
//! The library is loaded dynamically at runtime on first use.

use std::ffi::CString;

use crate::query_plan_native::error::QueryPlanError;
use crate::query_plan_native::native::{self as query_plan_native, GeospatialType, PartitionKeyRangesApiOptions, PartitionKind, WChar};
use crate::driver::dataflow::query_plan::QueryPlan;

/// Initial buffer size for the serialized query plan output (8 KiB).
const INITIAL_BUFFER_SIZE: u32 = 8 * 1024;

/// Safe wrapper around the native `IUnknown`-based service provider.
///
/// # Lifecycle
///
/// 1. Create with [`QueryPlanProvider::new`] (loads the DLL on first call).
/// 2. Generate query plans with [`QueryPlanProvider::get_partition_key_ranges`].
/// 3. The DLL stays loaded for the process lifetime (no explicit unload needed).
pub struct QueryPlanProvider {
    handle: query_plan_native::ServiceProviderHandle,
}

impl std::fmt::Debug for QueryPlanProvider {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("QueryPlanProvider")
            .field("handle", &(!self.handle.is_null()))
            .finish()
    }
}

unsafe impl Send for QueryPlanProvider {}
unsafe impl Sync for QueryPlanProvider {}

impl QueryPlanProvider {
    /// Creates a new service provider from a JSON configuration string.
    ///
    /// On the first call, this dynamically loads the native library.
    pub fn new(config_json: &str) -> Result<Self, QueryPlanError> {
        let lib = query_plan_native::query_plan_native_lib()?;
        let c_config = CString::new(config_json).map_err(|_| QueryPlanError::InvalidArgument {
            context: "config_json".to_string(),
        })?;

        let mut handle: query_plan_native::ServiceProviderHandle = std::ptr::null_mut();
        // SAFETY: c_config is a valid null-terminated UTF-8 string, and
        // handle is a valid writable pointer. The function is resolved
        // from the native library with a matching ABI signature.
        let hr = unsafe {
            (lib.create_service_provider)(c_config.as_ptr().cast(), &mut handle)
        };

        if query_plan_native::failed(hr) {
            return Err(QueryPlanError::from_hresult(hr));
        }

        Ok(Self { handle })
    }

    /// Updates the service provider with new configuration.
    #[allow(dead_code)] // Will be used when driver passes real query engine config.
    pub fn update(&self, config_json: &str) -> Result<(), QueryPlanError> {
        let lib = query_plan_native::query_plan_native_lib()?;
        let c_config = CString::new(config_json).map_err(|_| QueryPlanError::InvalidArgument {
            context: "config_json".to_string(),
        })?;

        let hr = unsafe {
            (lib.update_service_provider)(self.handle, c_config.as_ptr().cast())
        };

        if query_plan_native::failed(hr) {
            return Err(QueryPlanError::from_hresult(hr));
        }

        Ok(())
    }

    /// Generates a partitioned query execution plan.
    ///
    /// Returns the driver's [`QueryPlan`] type, which is the same JSON format
    /// that the Cosmos DB Gateway returns. The native DLL serializes to
    /// identical JSON, so we deserialize directly into the driver's model.
    pub fn get_partition_key_ranges(
        &self,
        query_spec_json: &str,
        partition_key_paths: &[&str],
        options: &QueryPlanOptions,
        vector_embedding_policy_json: Option<&str>,
    ) -> Result<QueryPlan, QueryPlanError> {
        let lib = query_plan_native::query_plan_native_lib()?;

        let query_spec_native = to_native_string(query_spec_json);

        let (token_segments, token_counts) = tokenize_partition_key_paths(partition_key_paths);
        let all_token_ptrs: Vec<*const WChar> =
            token_segments.iter().map(|w| w.as_ptr()).collect();

        let vector_policy = vector_embedding_policy_json.map(to_native_string);
        let (vec_policy_ptr, vec_policy_len) = match &vector_policy {
            Some(w) => (w.as_ptr(), w.len() as u32),
            None => (std::ptr::null(), 0),
        };

        let native_options = PartitionKeyRangesApiOptions {
            require_formattable_order_by_query: options.require_formattable_order_by_query as i32,
            is_continuation_expected: options.is_continuation_expected as i32,
            allow_non_value_aggregate_query: options.allow_non_value_aggregate_query as i32,
            has_logical_partition_key: options.has_logical_partition_key as i32,
            allow_dcount: options.allow_dcount as i32,
            use_system_prefix: options.use_system_prefix as i32,
            partition_kind: options.partition_kind,
            geospatial_type: options.geospatial_type,
            hybrid_search_skip_order_by_rewrite: options.hybrid_search_skip_order_by_rewrite
                as i32,
            reserved: [0u8; 28],
        };

        let mut buffer = vec![0u8; INITIAL_BUFFER_SIZE as usize];
        let mut result_length: u32 = 0;
        let pk_count = partition_key_paths.len() as u32;

        // Closure to avoid duplicating the 11-argument FFI call.
        // SAFETY: all pointers are valid for the duration of the closure call.
        // The native ABI is documented in QueryPlanInterop.h.
        let call_native = |buf: &mut Vec<u8>, out_len: &mut u32| unsafe {
            (lib.get_partition_key_ranges_from_query4)(
                self.handle,
                query_spec_native.as_ptr(),
                native_options,
                all_token_ptrs.as_ptr(),
                token_counts.as_ptr(),
                pk_count,
                vec_policy_ptr,
                vec_policy_len,
                buf.as_mut_ptr(),
                buf.len() as u32,
                out_len,
            )
        };

        let mut hr = call_native(&mut buffer, &mut result_length);

        if hr == query_plan_native::DISP_E_BUFFERTOOSMALL && result_length as usize > buffer.len() {
            buffer.resize(result_length as usize, 0);
            hr = call_native(&mut buffer, &mut result_length);
        }

        let payload = String::from_utf8(buffer[..result_length as usize].to_vec())
            .map_err(|e| QueryPlanError::InvalidArgument {
                context: format!("native library returned invalid UTF-8: {e}"),
            })?;

        if query_plan_native::failed(hr) {
            return if payload.is_empty() {
                Err(QueryPlanError::from_hresult(hr))
            } else {
                Err(QueryPlanError::from_hresult_with_payload(hr, payload))
            };
        }

        let info: QueryPlan = serde_json::from_str(&payload)?;
        Ok(info)
    }
}

// -------------------------------------------------------------------------
// Options
// -------------------------------------------------------------------------

/// High-level options for query plan generation.
#[derive(Debug, Clone)]
pub struct QueryPlanOptions {
    pub require_formattable_order_by_query: bool,
    pub is_continuation_expected: bool,
    pub allow_non_value_aggregate_query: bool,
    pub has_logical_partition_key: bool,
    pub allow_dcount: bool,
    pub use_system_prefix: bool,
    pub partition_kind: PartitionKind,
    pub geospatial_type: GeospatialType,
    pub hybrid_search_skip_order_by_rewrite: bool,
}

impl Default for QueryPlanOptions {
    fn default() -> Self {
        Self {
            require_formattable_order_by_query: false,
            is_continuation_expected: true,
            allow_non_value_aggregate_query: false,
            has_logical_partition_key: false,
            allow_dcount: false,
            use_system_prefix: false,
            partition_kind: PartitionKind::Hash,
            geospatial_type: GeospatialType::Geography,
            hybrid_search_skip_order_by_rewrite: false,
        }
    }
}

// -------------------------------------------------------------------------
// Helpers
// -------------------------------------------------------------------------

/// Encodes a Rust `&str` as a null-terminated wide string for the native API.
/// Windows: UTF-16 (`u16`). Linux/macOS: UTF-32 (`u32`).
#[cfg(target_os = "windows")]
fn to_native_string(s: &str) -> Vec<WChar> {
    s.encode_utf16().chain(std::iter::once(0)).collect()
}

#[cfg(not(target_os = "windows"))]
fn to_native_string(s: &str) -> Vec<WChar> {
    s.chars().map(|c| c as WChar).chain(std::iter::once(0)).collect()
}

/// Splits partition key paths into individual segments (tokens) and encodes
/// them as null-terminated wide strings for the native API.
///
/// The native API expects a flat array of segment pointers and a parallel
/// array of segment counts per path. For example, `["/tenantId", "/a/b"]`
/// becomes tokens `["tenantId", "a", "b"]` with counts `[1, 2]`.
///
/// This matches the .NET SDK's `PathParser.GetPathParts()` behavior.
fn tokenize_partition_key_paths(paths: &[&str]) -> (Vec<Vec<WChar>>, Vec<u32>) {
    let mut token_segments: Vec<Vec<WChar>> = Vec::new();
    let mut token_counts: Vec<u32> = Vec::with_capacity(paths.len());

    for path in paths {
        let segments: Vec<&str> = path.split('/').filter(|s| !s.is_empty()).collect();
        token_counts.push(segments.len() as u32);
        for seg in &segments {
            token_segments.push(to_native_string(seg));
        }
    }

    (token_segments, token_counts)
}

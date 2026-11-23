// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Safe wrapper around the QueryPlanInterop native library.
//!
//! [`QueryPlanProvider`] manages the lifecycle of the native service
//! provider and exposes a safe Rust API for generating query plans.
//! The library is loaded dynamically at runtime on first use.

use std::ffi::CString;

use crate::driver::dataflow::query_plan::{QueryPlan, RawQueryPlan};
use crate::models::{PartitionKeyDefinition, PartitionKeyKind};
use crate::query_plan_native::error::QueryPlanError;
use crate::query_plan_native::native::{
    self as query_plan_native, GeospatialType, PartitionKeyRangesApiOptions, PartitionKind, WChar,
};

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

// SAFETY: The native library's service provider handle is thread-safe.
// All exported functions are documented as safe for concurrent use across
// threads and internally synchronize their state.
unsafe impl Send for QueryPlanProvider {}
unsafe impl Sync for QueryPlanProvider {}

impl QueryPlanProvider {
    /// Creates a new service provider from a JSON configuration string.
    ///
    /// On the first call, this dynamically loads the native library.
    pub fn new(config_json: &str) -> Result<Self, QueryPlanError> {
        let lib = query_plan_native::query_plan_native_lib()?;
        let c_config = CString::new(config_json).map_err(|_| QueryPlanError::ConfigContainsNull)?;

        let (hr, handle) = lib.create_service_provider(&c_config);

        if query_plan_native::failed(hr) {
            return Err(QueryPlanError::from_hresult(hr));
        }

        Ok(Self { handle })
    }

    /// Updates the service provider with new configuration.
    pub fn update(&self, config_json: &str) -> Result<(), QueryPlanError> {
        let lib = query_plan_native::query_plan_native_lib()?;
        let c_config = CString::new(config_json).map_err(|_| QueryPlanError::ConfigContainsNull)?;

        let hr = lib.update_service_provider(self.handle, &c_config);

        if query_plan_native::failed(hr) {
            return Err(QueryPlanError::from_hresult(hr));
        }

        Ok(())
    }

    /// Generates a partitioned query execution plan for a container with the
    /// given partition key.
    ///
    /// The native library reports `queryRanges` as partition-key values; they
    /// are hashed into EPK hex with the definition's kind and version, so the
    /// returned [`QueryPlan`] matches what the Gateway path produces.
    pub fn get_partition_key_ranges(
        &self,
        query_spec_json: &str,
        partition_key: &PartitionKeyDefinition,
        options: &QueryPlanOptions,
        vector_embedding_policy_json: Option<&str>,
    ) -> Result<QueryPlan, QueryPlanError> {
        let lib = query_plan_native::query_plan_native_lib()?;

        let query_spec_native = to_native_string(query_spec_json);

        let partition_key_paths: Vec<&str> = partition_key
            .paths()
            .iter()
            .map(|path| path.as_ref())
            .collect();
        let (flat_tokens, token_offsets, token_counts) =
            tokenize_partition_key_paths(&partition_key_paths);
        let all_token_ptrs: Vec<*const WChar> = token_offsets
            .iter()
            .map(|&off| unsafe { flat_tokens.as_ptr().add(off) })
            .collect();

        let vector_policy = vector_embedding_policy_json.map(to_native_string);
        let (vec_policy_ptr, vec_policy_len) = match &vector_policy {
            Some(w) => (w.as_ptr(), w.len() as u32),
            None => (std::ptr::null(), 0),
        };

        let native_options = options.to_native(partition_key.kind());

        let mut buffer = vec![0u8; INITIAL_BUFFER_SIZE as usize];
        let mut result_length: u32 = 0;
        let pk_count = partition_key_paths.len() as u32;

        // Closure to avoid duplicating the FFI call.
        // SAFETY: all pointers are valid for the duration of the closure call.
        let call_native = |buf: &mut Vec<u8>, out_len: &mut u32| unsafe {
            lib.get_partition_key_ranges(
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

        // Check for failure before attempting UTF-8 validation so the true
        // HRESULT error is not masked by a misleading UTF-8 error.
        if query_plan_native::failed(hr) {
            let payload = std::str::from_utf8(&buffer[..result_length as usize]).unwrap_or("");
            return if payload.is_empty() {
                Err(QueryPlanError::from_hresult(hr))
            } else {
                Err(QueryPlanError::from_hresult_with_payload(
                    hr,
                    payload.to_string(),
                ))
            };
        }

        let payload = std::str::from_utf8(&buffer[..result_length as usize]).map_err(|e| {
            QueryPlanError::InvalidUtf8 {
                message: e.to_string(),
            }
        })?;

        let raw: RawQueryPlan = serde_json::from_str(payload)?;
        raw.resolve(partition_key)
            .map_err(|source| QueryPlanError::InvalidQueryRanges { source })
    }
}

// -------------------------------------------------------------------------
// Options
// -------------------------------------------------------------------------

/// High-level options for query plan generation. The partition kind comes
/// from the container's [`PartitionKeyDefinition`].
#[derive(Debug, Clone)]
pub struct QueryPlanOptions {
    pub require_formattable_order_by_query: bool,
    pub is_continuation_expected: bool,
    pub allow_non_value_aggregate_query: bool,
    pub has_logical_partition_key: bool,
    pub allow_dcount: bool,
    pub use_system_prefix: bool,
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
            geospatial_type: GeospatialType::Geography,
            hybrid_search_skip_order_by_rewrite: false,
        }
    }
}

impl QueryPlanOptions {
    fn to_native(&self, kind: PartitionKeyKind) -> PartitionKeyRangesApiOptions {
        PartitionKeyRangesApiOptions {
            require_formattable_order_by_query: self.require_formattable_order_by_query as i32,
            is_continuation_expected: self.is_continuation_expected as i32,
            allow_non_value_aggregate_query: self.allow_non_value_aggregate_query as i32,
            has_logical_partition_key: self.has_logical_partition_key as i32,
            allow_dcount: self.allow_dcount as i32,
            use_system_prefix: self.use_system_prefix as i32,
            partition_kind: match kind {
                PartitionKeyKind::MultiHash => PartitionKind::MultiHash,
                _ => PartitionKind::Hash,
            },
            geospatial_type: self.geospatial_type,
            hybrid_search_skip_order_by_rewrite: self.hybrid_search_skip_order_by_rewrite as i32,
            reserved: [0u8; 28],
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
    s.chars()
        .map(|c| c as WChar)
        .chain(std::iter::once(0))
        .collect()
}

/// Splits partition key paths into individual segments (tokens) and encodes
/// them as null-terminated wide strings for the native API.
///
/// Returns a single flat buffer of all segments (each null-terminated,
/// concatenated), the byte offset of each segment within that buffer,
/// and per-path segment counts. For example, `["/tenantId", "/a/b"]`
/// produces a flat buffer with `"tenantId\0a\0b\0"`, offsets pointing to
/// the start of each segment, and counts `[1, 2]`.
///
/// This matches the .NET SDK's `PathParser.GetPathParts()` behavior.
fn tokenize_partition_key_paths(paths: &[&str]) -> (Vec<WChar>, Vec<usize>, Vec<u32>) {
    let mut flat_buf: Vec<WChar> = Vec::new();
    let mut offsets: Vec<usize> = Vec::new();
    let mut counts: Vec<u32> = Vec::with_capacity(paths.len());

    for path in paths {
        let segments: Vec<&str> = path.split('/').filter(|s| !s.is_empty()).collect();
        counts.push(segments.len() as u32);
        for seg in &segments {
            offsets.push(flat_buf.len());
            flat_buf.extend_from_slice(&to_native_string(seg));
        }
    }

    (flat_buf, offsets, counts)
}

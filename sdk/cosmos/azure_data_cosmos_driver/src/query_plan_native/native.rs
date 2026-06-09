// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! FFI bindings to the QueryPlanInterop native library.
//!
//! Uses runtime dynamic loading (`LoadLibrary`/`dlopen`) so the consumer
//! controls when the library is loaded and unloaded. This avoids
//! process-exit crashes caused by DLL static destructor ordering and
//! provides graceful error handling when the library is missing.
//!
//! # Library search order
//!
//! 1. `QUERY_PLAN_INTEROP_LIB_DIR` environment variable (full path)
//! 2. OS default search (PATH on Windows, LD_LIBRARY_PATH on Linux)

use std::os::raw::c_void;

/// HRESULT type matching the native ABI (signed 32-bit).
pub type HResult = i32;

/// Opaque handle to an `IUnknown`-based service provider.
pub type ServiceProviderHandle = *mut c_void;

/// Wide character type matching the native ABI.
/// Windows: `wchar_t` is 16-bit (UTF-16).
/// Linux/macOS: `wchar_t` is 32-bit (UTF-32) per QueryPlanInterop.h
/// (`-fshort-wchar` is NOT used).
#[cfg(target_os = "windows")]
pub type WChar = u16;
#[cfg(not(target_os = "windows"))]
pub type WChar = u32;

// -------------------------------------------------------------------------
// HRESULT constants
// -------------------------------------------------------------------------

pub const S_OK: HResult = 0x0000_0000_u32 as i32;
pub const E_FAIL: HResult = 0x8000_4005_u32 as i32;
pub const E_POINTER: HResult = 0x8000_4003_u32 as i32;
pub const E_INVALIDARG: HResult = 0x8007_0057_u32 as i32;
pub const E_OUTOFMEMORY: HResult = 0x8007_000E_u32 as i32;
pub const E_UNEXPECTED: HResult = 0x8000_FFFF_u32 as i32;
pub const DISP_E_BUFFERTOOSMALL: HResult = 0x8002_0013_u32 as i32;

#[inline]
pub fn succeeded(hr: HResult) -> bool {
    hr >= 0
}

#[inline]
pub fn failed(hr: HResult) -> bool {
    hr < 0
}

// -------------------------------------------------------------------------
// Enums
// -------------------------------------------------------------------------

/// Partition key kind. Matches `QueryPlanInteropPartitionKind`.
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PartitionKind {
    Hash = 0,
    Range = 1,
    MultiHash = 2,
}

/// Geospatial type. Matches `QueryPlanInteropGeospatialType`.
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GeospatialType {
    Geography = 0,
    Geometry = 1,
}

// -------------------------------------------------------------------------
// Options struct (64 bytes, ABI-pinned)
// -------------------------------------------------------------------------

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PartitionKeyRangesApiOptions {
    pub require_formattable_order_by_query: i32,
    pub is_continuation_expected: i32,
    pub allow_non_value_aggregate_query: i32,
    pub has_logical_partition_key: i32,
    pub allow_dcount: i32,
    pub use_system_prefix: i32,
    pub partition_kind: PartitionKind,
    pub geospatial_type: GeospatialType,
    pub hybrid_search_skip_order_by_rewrite: i32,
    pub reserved: [u8; 28],
}

// Compile-time ABI guard matching the C++ `static_assert` in QueryPlanInterop.h.
// If any field changes size or offset, the build fails immediately.
const _: () = {
    assert!(
        std::mem::size_of::<PartitionKeyRangesApiOptions>() == 64,
        "ABI size must remain 64 bytes"
    );
    // Field offset checks matching QueryPlanInterop.h static_assert block.
    // These use a const fn since offset_of! is not yet stable in const context,
    // so we verify in the test module below.
};

impl Default for PartitionKeyRangesApiOptions {
    fn default() -> Self {
        // SAFETY: All-zeros is a valid representation for this #[repr(C)]
        // struct: i32 zeros are valid, enum variants start at 0, and the
        // reserved tail is required to be zeroed per the C++ ABI contract.
        unsafe { std::mem::zeroed() }
    }
}

// -------------------------------------------------------------------------
// Function pointer types
// -------------------------------------------------------------------------

type CreateServiceProviderFn =
    unsafe extern "C" fn(*const u8, *mut ServiceProviderHandle) -> HResult;

type UpdateServiceProviderFn = unsafe extern "C" fn(ServiceProviderHandle, *const u8) -> HResult;

#[allow(clippy::type_complexity)]
type GetPartitionKeyRangesFromQuery4Fn = unsafe extern "C" fn(
    ServiceProviderHandle,
    *const WChar,
    PartitionKeyRangesApiOptions,
    *const *const WChar,
    *const u32,
    u32,
    *const WChar,
    u32,
    *mut u8,
    u32,
    *mut u32,
) -> HResult;

// -------------------------------------------------------------------------
// Platform-specific library loading
// -------------------------------------------------------------------------

#[cfg(target_os = "windows")]
mod platform {
    use std::ffi::CString;
    use std::os::raw::c_void;

    pub type LibHandle = *mut c_void;

    #[link(name = "kernel32")]
    unsafe extern "system" {
        fn LoadLibraryA(name: *const u8) -> *mut c_void;
        fn GetProcAddress(module: *mut c_void, name: *const u8) -> *mut c_void;
        fn FreeLibrary(module: *mut c_void) -> i32;
    }

    /// # Safety
    ///
    /// The library name must refer to a valid shared library on the system.
    ///
    /// Search order:
    /// 1. QUERY_PLAN_INTEROP_LIB_DIR environment variable (loads by absolute path)
    /// 2. OS default search (PATH)
    pub unsafe fn load_library(name: &str) -> Option<LibHandle> {
        // Try QUERY_PLAN_INTEROP_LIB_DIR if set — build an absolute path
        // to avoid mutating the process-wide DLL search directory.
        if let Ok(dir) = std::env::var("QUERY_PLAN_INTEROP_LIB_DIR") {
            let full_path = format!("{}\\{}", dir.trim_end_matches('\\'), name);
            if let Ok(c_path) = CString::new(full_path) {
                // SAFETY: CString guarantees a valid nul-terminated string.
                let h = unsafe { LoadLibraryA(c_path.as_ptr().cast()) };
                if !h.is_null() {
                    return Some(h);
                }
            }
        }

        // Fall back to OS default search (PATH).
        let c_name = CString::new(name).ok()?;
        // SAFETY: CString guarantees a valid nul-terminated string.
        let h = unsafe { LoadLibraryA(c_name.as_ptr().cast()) };
        if h.is_null() {
            None
        } else {
            Some(h)
        }
    }

    /// # Safety
    ///
    /// `lib` must be a valid library handle from `load_library`.
    pub unsafe fn get_proc(lib: LibHandle, name: &str) -> Option<*mut c_void> {
        let c_name = CString::new(name).ok()?;
        // SAFETY: lib is a valid module handle from load_library;
        // CString guarantees a valid nul-terminated string.
        let p = unsafe { GetProcAddress(lib, c_name.as_ptr().cast()) };
        if p.is_null() {
            None
        } else {
            Some(p)
        }
    }

    /// # Safety
    ///
    /// `lib` must be a valid library handle. No code from this library
    /// may be executing on any thread.
    pub unsafe fn free_library(lib: LibHandle) {
        // SAFETY: Caller guarantees lib is valid and no code is executing.
        unsafe {
            FreeLibrary(lib);
        }
    }

    pub const LIB_NAME: &str = "Cosmos.QueryPlanInterop.dll";
}

#[cfg(not(target_os = "windows"))]
mod platform {
    use std::ffi::CString;
    use std::os::raw::{c_char, c_int, c_void};

    pub type LibHandle = *mut c_void;

    unsafe extern "C" {
        fn dlopen(filename: *const c_char, flags: c_int) -> *mut c_void;
        fn dlsym(handle: *mut c_void, symbol: *const c_char) -> *mut c_void;
        fn dlclose(handle: *mut c_void) -> c_int;
    }

    const RTLD_NOW: c_int = 0x2;
    const RTLD_LOCAL: c_int = 0x0;

    /// # Safety
    ///
    /// The library name must refer to a valid shared library on the system.
    ///
    /// Search order:
    /// 1. QUERY_PLAN_INTEROP_LIB_DIR environment variable
    /// 2. OS default search (LD_LIBRARY_PATH / DYLD_LIBRARY_PATH)
    pub unsafe fn load_library(name: &str) -> Option<LibHandle> {
        // Try QUERY_PLAN_INTEROP_LIB_DIR if set.
        if let Ok(dir) = std::env::var("QUERY_PLAN_INTEROP_LIB_DIR") {
            let full_path = format!("{}/{}", dir.trim_end_matches('/'), name);
            if let Ok(c_path) = CString::new(full_path) {
                // SAFETY: CString guarantees a valid nul-terminated string.
                let h = unsafe { dlopen(c_path.as_ptr(), RTLD_NOW | RTLD_LOCAL) };
                if !h.is_null() {
                    return Some(h);
                }
            }
        }

        // Fall back to OS default search.
        let c_name = CString::new(name).ok()?;
        // SAFETY: CString guarantees a valid nul-terminated string.
        let h = unsafe { dlopen(c_name.as_ptr(), RTLD_NOW | RTLD_LOCAL) };
        if h.is_null() {
            None
        } else {
            Some(h)
        }
    }

    /// # Safety
    ///
    /// `lib` must be a valid library handle from `load_library`.
    pub unsafe fn get_proc(lib: LibHandle, name: &str) -> Option<*mut c_void> {
        let c_name = CString::new(name).ok()?;
        // SAFETY: lib is a valid handle from load_library;
        // CString guarantees a valid nul-terminated string.
        let p = unsafe { dlsym(lib, c_name.as_ptr()) };
        if p.is_null() {
            None
        } else {
            Some(p)
        }
    }

    /// # Safety
    ///
    /// `lib` must be a valid library handle. No code from this library
    /// may be executing on any thread.
    pub unsafe fn free_library(lib: LibHandle) {
        // SAFETY: Caller guarantees lib is valid and no code is executing.
        unsafe {
            dlclose(lib);
        }
    }

    #[cfg(target_os = "linux")]
    pub const LIB_NAME: &str = "libqueryplaninterop.so";
    #[cfg(target_os = "macos")]
    pub const LIB_NAME: &str = "libqueryplaninterop.dylib";
}

// -------------------------------------------------------------------------
// Resolved function table
// -------------------------------------------------------------------------

/// Dynamically loaded native library with resolved function pointers.
pub(crate) struct QueryPlanNativeLibrary {
    pub create_service_provider: CreateServiceProviderFn,
    pub update_service_provider: UpdateServiceProviderFn,
    pub get_partition_key_ranges_from_query4: GetPartitionKeyRangesFromQuery4Fn,
}

// SAFETY: The native library's functions are thread-safe per its
// documentation. All internal state is synchronized by the library.
unsafe impl Send for QueryPlanNativeLibrary {}
unsafe impl Sync for QueryPlanNativeLibrary {}

impl QueryPlanNativeLibrary {
    fn load() -> Result<Self, String> {
        // SAFETY: We load the library and resolve symbols whose signatures
        // match the C ABI declared in QueryPlanInterop.h. The transmutes
        // convert raw function pointers to typed function pointers with
        // signatures matching the header's extern "C" declarations.
        unsafe {
            let handle = platform::load_library(platform::LIB_NAME)
                .ok_or_else(|| format!("failed to load {}", platform::LIB_NAME))?;

            let resolve = |name: &str| -> Result<*mut c_void, String> {
                platform::get_proc(handle, name)
                    .ok_or_else(|| format!("symbol '{}' not found in {}", name, platform::LIB_NAME))
            };

            Ok(Self {
                create_service_provider: std::mem::transmute::<*mut c_void, CreateServiceProviderFn>(
                    resolve("CreateServiceProvider")?,
                ),
                update_service_provider: std::mem::transmute::<*mut c_void, UpdateServiceProviderFn>(
                    resolve("UpdateServiceProvider")?,
                ),
                get_partition_key_ranges_from_query4: std::mem::transmute::<
                    *mut c_void,
                    GetPartitionKeyRangesFromQuery4Fn,
                >(resolve(
                    "GetPartitionKeyRangesFromQuery4",
                )?),
            })
        }
    }
}

// -------------------------------------------------------------------------
// Global singleton -- load once, no mutex around calls
// -------------------------------------------------------------------------

static QUERY_PLAN_NATIVE_LIB: std::sync::OnceLock<Result<QueryPlanNativeLibrary, String>> =
    std::sync::OnceLock::new();

/// Returns a reference to the loaded native library.
/// Loads it on first call. Returns a `LibraryNotAvailable` error if loading fails.
pub(crate) fn query_plan_native_lib(
) -> Result<&'static QueryPlanNativeLibrary, super::error::QueryPlanError> {
    use super::error::QueryPlanError;
    QUERY_PLAN_NATIVE_LIB
        .get_or_init(QueryPlanNativeLibrary::load)
        .as_ref()
        .map_err(|e| QueryPlanError::LibraryNotAvailable { message: e.clone() })
}

#[cfg(test)]
mod tests {
    use super::*;

    // -----------------------------------------------------------------
    // ABI contract tests -- mirror QueryPlanInterop.h static_asserts.
    // If the C++ header changes struct layout, these tests fail.
    // -----------------------------------------------------------------

    #[test]
    fn abi_options_field_offsets() {
        assert_eq!(
            std::mem::offset_of!(
                PartitionKeyRangesApiOptions,
                require_formattable_order_by_query
            ),
            0
        );
        assert_eq!(
            std::mem::offset_of!(PartitionKeyRangesApiOptions, is_continuation_expected),
            4
        );
        assert_eq!(
            std::mem::offset_of!(
                PartitionKeyRangesApiOptions,
                allow_non_value_aggregate_query
            ),
            8
        );
        assert_eq!(
            std::mem::offset_of!(PartitionKeyRangesApiOptions, has_logical_partition_key),
            12
        );
        assert_eq!(
            std::mem::offset_of!(PartitionKeyRangesApiOptions, allow_dcount),
            16
        );
        assert_eq!(
            std::mem::offset_of!(PartitionKeyRangesApiOptions, use_system_prefix),
            20
        );
        assert_eq!(
            std::mem::offset_of!(PartitionKeyRangesApiOptions, partition_kind),
            24
        );
        assert_eq!(
            std::mem::offset_of!(PartitionKeyRangesApiOptions, geospatial_type),
            28
        );
        assert_eq!(
            std::mem::offset_of!(
                PartitionKeyRangesApiOptions,
                hybrid_search_skip_order_by_rewrite
            ),
            32
        );
        assert_eq!(
            std::mem::offset_of!(PartitionKeyRangesApiOptions, reserved),
            36
        );
    }

    #[test]
    fn abi_enum_values_match_header() {
        assert_eq!(PartitionKind::Hash as i32, 0);
        assert_eq!(PartitionKind::Range as i32, 1);
        assert_eq!(PartitionKind::MultiHash as i32, 2);
        assert_eq!(GeospatialType::Geography as i32, 0);
        assert_eq!(GeospatialType::Geometry as i32, 1);
    }

    #[test]
    fn abi_hresult_constants_match_header() {
        assert_eq!(S_OK, 0x0000_0000_u32 as i32);
        assert_eq!(E_FAIL, 0x8000_4005_u32 as i32);
        assert_eq!(E_POINTER, 0x8000_4003_u32 as i32);
        assert_eq!(E_INVALIDARG, 0x8007_0057_u32 as i32);
        assert_eq!(E_OUTOFMEMORY, 0x8007_000E_u32 as i32);
        assert_eq!(E_UNEXPECTED, 0x8000_FFFF_u32 as i32);
        assert_eq!(DISP_E_BUFFERTOOSMALL, 0x8002_0013_u32 as i32);
    }

    #[test]
    fn hresult_success_failure() {
        assert!(succeeded(S_OK));
        assert!(!failed(S_OK));
        assert!(failed(E_FAIL));
        assert!(!succeeded(E_FAIL));
        assert!(failed(DISP_E_BUFFERTOOSMALL));
    }

    #[test]
    fn options_default_is_zeroed() {
        let opts = PartitionKeyRangesApiOptions::default();
        assert_eq!(opts.require_formattable_order_by_query, 0);
        assert_eq!(opts.is_continuation_expected, 0);
        assert_eq!(opts.allow_non_value_aggregate_query, 0);
        assert_eq!(opts.has_logical_partition_key, 0);
        assert_eq!(opts.allow_dcount, 0);
        assert_eq!(opts.use_system_prefix, 0);
        assert_eq!(opts.hybrid_search_skip_order_by_rewrite, 0);
        assert_eq!(opts.reserved, [0u8; 28]);
    }

    #[test]
    fn native_lib_returns_error_when_dll_missing() {
        use crate::query_plan_native::error::QueryPlanError;
        // When the DLL is not on PATH, loading should return Err, not panic.
        // This test may pass (Ok) if the DLL happens to be available.
        if let Err(QueryPlanError::LibraryNotAvailable { message }) = query_plan_native_lib() {
            assert!(message.contains("failed to load"));
        }
    }

    // -----------------------------------------------------------------
    // Cross-validation against bindgen-generated types.
    // -----------------------------------------------------------------

    #[test]
    fn generated_options_struct_size_matches_handwritten() {
        use crate::query_plan_native::generated_bindings::QueryPlanInteropPartitionKeyRangesApiOptions as Gen;
        assert_eq!(
            std::mem::size_of::<Gen>(),
            std::mem::size_of::<PartitionKeyRangesApiOptions>(),
        );
    }

    #[test]
    fn generated_field_offsets_match_handwritten() {
        use crate::query_plan_native::generated_bindings::QueryPlanInteropPartitionKeyRangesApiOptions as Gen;
        type Hw = PartitionKeyRangesApiOptions;
        assert_eq!(
            std::mem::offset_of!(Gen, bRequireFormattableOrderByQuery),
            std::mem::offset_of!(Hw, require_formattable_order_by_query)
        );
        assert_eq!(
            std::mem::offset_of!(Gen, bIsContinuationExpected),
            std::mem::offset_of!(Hw, is_continuation_expected)
        );
        assert_eq!(
            std::mem::offset_of!(Gen, bAllowNonValueAggregateQuery),
            std::mem::offset_of!(Hw, allow_non_value_aggregate_query)
        );
        assert_eq!(
            std::mem::offset_of!(Gen, bHasLogicalPartitionKey),
            std::mem::offset_of!(Hw, has_logical_partition_key)
        );
        assert_eq!(
            std::mem::offset_of!(Gen, bAllowDCount),
            std::mem::offset_of!(Hw, allow_dcount)
        );
        assert_eq!(
            std::mem::offset_of!(Gen, bUseSystemPrefix),
            std::mem::offset_of!(Hw, use_system_prefix)
        );
        assert_eq!(
            std::mem::offset_of!(Gen, ePartitionKind),
            std::mem::offset_of!(Hw, partition_kind)
        );
        assert_eq!(
            std::mem::offset_of!(Gen, eGeospatialType),
            std::mem::offset_of!(Hw, geospatial_type)
        );
        assert_eq!(
            std::mem::offset_of!(Gen, bHybridSearchSkipOrderByRewrite),
            std::mem::offset_of!(Hw, hybrid_search_skip_order_by_rewrite)
        );
        assert_eq!(
            std::mem::offset_of!(Gen, rgbyReserved),
            std::mem::offset_of!(Hw, reserved)
        );
    }

    #[test]
    fn generated_enum_values_match_handwritten() {
        use crate::query_plan_native::generated_bindings::*;
        assert_eq!(
            QueryPlanInteropPartitionKind_Hash,
            PartitionKind::Hash as i32
        );
        assert_eq!(
            QueryPlanInteropPartitionKind_Range,
            PartitionKind::Range as i32
        );
        assert_eq!(
            QueryPlanInteropPartitionKind_MultiHash,
            PartitionKind::MultiHash as i32
        );
        assert_eq!(
            QueryPlanInteropGeospatialType_Geography,
            GeospatialType::Geography as i32
        );
        assert_eq!(
            QueryPlanInteropGeospatialType_Geometry,
            GeospatialType::Geometry as i32
        );
    }
}

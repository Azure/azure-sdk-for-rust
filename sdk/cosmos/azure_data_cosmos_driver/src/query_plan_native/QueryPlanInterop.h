//------------------------------------------------------------------------------
// <copyright file="QueryPlanInterop.h" company="Microsoft">
//      Copyright (c) Microsoft Corporation.  All rights reserved.
// </copyright>
//
// Public ABI header for the QueryPlanInterop native library
// (libQueryPlanInterop.so on Linux, QueryPlanInterop.dll on Windows).
//
// This header is the ONLY supported include surface for external consumers
// of QueryPlanInterop. It is intentionally self-contained: it does not pull
// in any QueryEngineCore, ServiceFabric, or Cosmos backend headers, and
// does not require any QPI-internal preprocessor defines to be set on the
// consumer's command line.
//
// The library exports additional legacy C entry points that are not part
// of the public surface; new consumers must call the V4 entry point only.
//------------------------------------------------------------------------------

#pragma once

// QueryPlanInterop.h is the public ABI surface of libQueryPlanInterop. It
// uses C++-only constructs (enum class, IUnknown via virtual methods) and
// must be compiled as C++. The functions inside the extern "C" block are
// callable from C, but consumers wanting a pure-C interface should wrap
// this header in their own C++ shim.
#ifndef __cplusplus
    #error "QueryPlanInterop.h must be compiled as C++."
#endif

#include <stddef.h>
#include <stdint.h>

#if defined(_WIN32)
    #include <unknwn.h>

    // QUERY_PLAN_INTEROP_API:
    //   - When BUILDING libQueryPlanInterop itself, the build system
    //     defines BUILDING_QUERY_PLAN_INTEROP, which selects dllexport
    //     so the symbols are emitted into the produced .dll.
    //   - When CONSUMING the library, that macro is absent, so QUERY_PLAN_INTEROP_API
    //     resolves to dllimport, telling the linker to resolve these
    //     symbols against the import library instead of attempting to
    //     re-export them from the consumer.
    // Defining the wrong direction (import vs. export) on either side
    // leads to "duplicate export" / "unresolved external" link errors.
    #ifdef BUILDING_QUERY_PLAN_INTEROP
        #define QUERY_PLAN_INTEROP_API __declspec(dllexport)
    #else
        #define QUERY_PLAN_INTEROP_API __declspec(dllimport)
    #endif
#else
    // Linux/POSIX type-compat layer. Matches the QPI build settings:
    //   -fshort-wchar is NOT used; wchar_t is 32-bit on Linux (gcc/clang default).
    typedef int32_t  HRESULT;
    typedef uint32_t ULONG;
    typedef uint8_t  BYTE;
    typedef int32_t  BOOL;
    typedef const wchar_t* LPCWSTR;
    typedef const char*    LPCSTR;
    typedef const wchar_t* LPCWCH;

    #ifndef TRUE
        #define TRUE  1
    #endif
    #ifndef FALSE
        #define FALSE 0
    #endif

    #ifndef S_OK
        #define S_OK            ((HRESULT)0x00000000L)
    #endif
    #ifndef E_FAIL
        #define E_FAIL          ((HRESULT)0x80004005L)
    #endif
    #ifndef E_POINTER
        #define E_POINTER       ((HRESULT)0x80004003L)
    #endif
    #ifndef E_INVALIDARG
        #define E_INVALIDARG    ((HRESULT)0x80070057L)
    #endif
    #ifndef E_OUTOFMEMORY
        #define E_OUTOFMEMORY   ((HRESULT)0x8007000EL)
    #endif
    #ifndef E_UNEXPECTED
        #define E_UNEXPECTED    ((HRESULT)0x8000FFFFL)
    #endif

    // Minimal IUnknown definition. The vtable layout (QueryInterface, AddRef,
    // Release) and calling convention are the standard COM layout that
    // libQueryPlanInterop.so produces; consumers only ever call Release() on
    // handles returned by CreateServiceProvider.
    struct IID;

    struct IUnknown
    {
        virtual HRESULT QueryInterface(const IID& riid, void** ppvObject) = 0;
        virtual ULONG   AddRef() = 0;
        virtual ULONG   Release() = 0;
    };

    #define QUERY_PLAN_INTEROP_API __attribute__((visibility("default")))
#endif

#ifndef SUCCEEDED
    #define SUCCEEDED(hr) (((HRESULT)(hr)) >= 0)
#endif

#ifndef FAILED
    #define FAILED(hr) (((HRESULT)(hr)) < 0)
#endif

//------------------------------------------------------------------------------
// Public enums and structs
//
// These are intentionally prefixed with "QueryPlanInterop" so that a single
// translation unit may include both this header and the legacy
// ServiceInterop.h header (which declares the same logical types under
// shorter names) without ODR violations. Layouts and underlying integer
// types are byte-identical to the internal types, so values cross the
// extern "C" ABI boundary unchanged.
//------------------------------------------------------------------------------

enum class QueryPlanInteropPartitionKind : int32_t
{
    Hash      = 0,
    Range     = 1,
    MultiHash = 2,
};

enum class QueryPlanInteropGeospatialType : int32_t
{
    Geography = 0,
    Geometry  = 1,
};

// Layout MUST match Microsoft.Azure.Documents/ServiceInterop/PartitionKeyRangesApiOptions.h
// byte-for-byte. The reserved tail keeps the size at 64 bytes so future
// fields can be added without breaking ABI.
struct QueryPlanInteropPartitionKeyRangesApiOptions
{
    BOOL bRequireFormattableOrderByQuery;
    BOOL bIsContinuationExpected;
    BOOL bAllowNonValueAggregateQuery;
    BOOL bHasLogicalPartitionKey;
    BOOL bAllowDCount;
    BOOL bUseSystemPrefix;

    QueryPlanInteropPartitionKind   ePartitionKind;
    QueryPlanInteropGeospatialType  eGeospatialType;

    BOOL bHybridSearchSkipOrderByRewrite;

    // Reserved for future expansion. Callers MUST zero-initialize the entire
    // struct (e.g. via aggregate-init or memset) so future fields default to
    // a known value when the header is older than the .so/.dll.
    BYTE rgbyReserved[28];
};

// ABI guards: this struct crosses the extern "C" boundary and any layout
// drift (toolchain default packing, alignment, member reorder) silently
// corrupts caller/callee communication. Pin both the total size and the
// exact offset of every field so the build fails immediately if anything
// shifts.
static_assert(sizeof(QueryPlanInteropPartitionKeyRangesApiOptions) == 64,
    "QueryPlanInteropPartitionKeyRangesApiOptions ABI size must remain 64 bytes");
static_assert(offsetof(QueryPlanInteropPartitionKeyRangesApiOptions, bRequireFormattableOrderByQuery)   ==  0, "ABI offset drift");
static_assert(offsetof(QueryPlanInteropPartitionKeyRangesApiOptions, bIsContinuationExpected)            ==  4, "ABI offset drift");
static_assert(offsetof(QueryPlanInteropPartitionKeyRangesApiOptions, bAllowNonValueAggregateQuery)       ==  8, "ABI offset drift");
static_assert(offsetof(QueryPlanInteropPartitionKeyRangesApiOptions, bHasLogicalPartitionKey)            == 12, "ABI offset drift");
static_assert(offsetof(QueryPlanInteropPartitionKeyRangesApiOptions, bAllowDCount)                       == 16, "ABI offset drift");
static_assert(offsetof(QueryPlanInteropPartitionKeyRangesApiOptions, bUseSystemPrefix)                   == 20, "ABI offset drift");
static_assert(offsetof(QueryPlanInteropPartitionKeyRangesApiOptions, ePartitionKind)                     == 24, "ABI offset drift");
static_assert(offsetof(QueryPlanInteropPartitionKeyRangesApiOptions, eGeospatialType)                    == 28, "ABI offset drift");
static_assert(offsetof(QueryPlanInteropPartitionKeyRangesApiOptions, bHybridSearchSkipOrderByRewrite)    == 32, "ABI offset drift");
static_assert(offsetof(QueryPlanInteropPartitionKeyRangesApiOptions, rgbyReserved)                       == 36, "ABI offset drift");

#ifdef __cplusplus
extern "C" {
#endif

//------------------------------------------------------------------------------
// Service provider lifecycle
//------------------------------------------------------------------------------

QUERY_PLAN_INTEROP_API HRESULT CreateServiceProvider(
    LPCSTR     pszConfigJson,
    IUnknown** ppServiceProviderOut);

QUERY_PLAN_INTEROP_API HRESULT UpdateServiceProvider(
    IUnknown* pServiceProvider,
    LPCSTR    pszConfigJson);

//------------------------------------------------------------------------------
// Query plan generation
//
// The only public entry point. Earlier numbered variants
// (GetPartitionKeyRangesFromQuery1/2/3) remain exported by the library for
// legacy in-tree consumers but are not part of the supported public surface
// and may be removed in a future release.
//------------------------------------------------------------------------------

QUERY_PLAN_INTEROP_API HRESULT GetPartitionKeyRangesFromQuery4(
    IUnknown* pServiceProvider,
    LPCWSTR   pwszQuerySpec,
    const QueryPlanInteropPartitionKeyRangesApiOptions options,
    LPCWSTR*  pPartitionKeyPathTokens,
    ULONG     rgPartitionKeyPathTokensLengths[],
    ULONG     nPartitionKeyCount,
    LPCWCH    pwchVectorEmbeddingPolicy,
    ULONG     nVectorEmbeddingPolicyLength,
    BYTE*     pbySerializedPartitionedQueryExecutionInfoBuffer,
    ULONG     nSerializedPartitionedQueryExecutionInfoBufferLength,
    ULONG*    pnSerializedPartitionedQueryExecutionInfoResultLength);

#ifdef __cplusplus
} // extern "C"
#endif

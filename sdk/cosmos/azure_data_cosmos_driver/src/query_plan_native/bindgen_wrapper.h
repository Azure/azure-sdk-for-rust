// bindgen_wrapper.h -- Thin wrapper for bindgen compatibility.
// The original QueryPlanInterop.h uses MSVC's offsetof() in static_assert,
// which clang rejects in constexpr context. This wrapper suppresses those
// assertions while preserving all type and function declarations.

#define static_assert(...) /* suppressed for bindgen */
#include "QueryPlanInterop.h"

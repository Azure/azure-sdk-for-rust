// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

#include "test_common.h"
#include <stddef.h>
#include <stdint.h>

_Static_assert(offsetof(cosmos_diagnostics_info_v2_t, struct_size_bytes) == 0,
               "diagnostics info size prefix");
_Static_assert(offsetof(cosmos_diagnostics_info_v2_t, schema_version) == 4,
               "diagnostics info version prefix");
_Static_assert(offsetof(cosmos_diagnostics_attempt_v2_t, struct_size_bytes) == 0,
               "diagnostics attempt size prefix");
_Static_assert(offsetof(cosmos_diagnostics_attempt_v2_t, schema_version) == 4,
               "diagnostics attempt version prefix");
_Static_assert(COSMOS_DIAGNOSTICS_ABI_VERSION_2 == 2, "diagnostics schema version");
_Static_assert(COSMOS_DIAGNOSTICS_EXECUTION_CONTEXT_UNKNOWN == 0,
               "unknown execution context is stable");
_Static_assert(COSMOS_DIAGNOSTICS_REQUEST_SENT_UNKNOWN == 0,
               "unknown request-sent state is stable");

#if UINTPTR_MAX == UINT64_MAX
_Static_assert(sizeof(cosmos_borrowed_string_view_t) == 16, "borrowed view layout");
_Static_assert(sizeof(cosmos_diagnostics_info_v2_t) == 64, "diagnostics info layout");
_Static_assert(sizeof(cosmos_diagnostics_attempt_v2_t) == 192,
               "diagnostics attempt layout");
_Static_assert(offsetof(cosmos_diagnostics_attempt_v2_t, endpoint) == 80,
               "diagnostics attempt string offset");
#endif

static void count_attempt(void *user_data,
                          const cosmos_diagnostics_attempt_v2_t *attempt)
{
    (void)attempt;
    (*(uint32_t *)user_data)++;
}

static int test_null_diagnostics_are_inert(void)
{
    int result = TEST_PASS;
    cosmos_diagnostics_info_v2_t info = {0};
    uint32_t attempts = 0;
    ASSERT(cosmos_diagnostics_get_info_v2(NULL, &info) != COSMOS_STATUS_SUCCESS,
           "NULL diagnostics rejected");
    ASSERT(cosmos_diagnostics_get_info_v2(NULL, NULL) != COSMOS_STATUS_SUCCESS,
           "NULL output rejected");
    cosmos_diagnostics_iter_attempts_v2(NULL, count_attempt, &attempts);
    ASSERT(attempts == 0, "NULL diagnostics did not invoke callback");
    cosmos_diagnostics_iter_attempts_v2(NULL, NULL, NULL);
    return result;
}

TEST_SUITE_BEGIN("Diagnostics ABI")
TEST_REGISTER(null_diagnostics_are_inert)
TEST_SUITE_END("Diagnostics ABI")

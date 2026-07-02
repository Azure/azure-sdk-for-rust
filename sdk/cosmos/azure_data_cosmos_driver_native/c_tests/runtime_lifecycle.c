// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

// Runtime construction & lifecycle.
//
// Validates the C surface of the flat `cosmos_runtime_build` path end-to-end:
//
//   1. NULL-safety / argument validation on `cosmos_runtime_build`.
//   2. Range / charset validation on the flat `cosmos_runtime_options_t`
//      fields (workload id, correlation id, user-agent suffix, cpu refresh
//      interval) — surfaced at build time.
//   3. The happy path — build a runtime from a flat options struct, then prove
//      the resulting `cosmos_runtime_t *` is usable by handing it to
//      `cosmos_completion_queue_create` / `cosmos_completion_queue_free` /
//      `cosmos_runtime_free`.
//
// The per-field incremental builder was removed in P5; construction is now a
// single `cosmos_runtime_build(&opts, ...)` call. Validation happens at build
// time, so each field test sets exactly one invalid field (leaving the rest at
// their unset sentinels) and asserts the build fails with the runtime slot
// left NULL.

#include "test_common.h"

static int test_options_default_all_unset(void)
{
    int result = TEST_PASS;
    cosmos_runtime_options_t opts = cosmos_runtime_options_default();
    ASSERT(opts.workload_id == 0, "workload_id defaults to 0 (unset)");
    ASSERT(opts.correlation_id == NULL, "correlation_id defaults to NULL");
    ASSERT(opts.user_agent_suffix == NULL, "user_agent_suffix defaults to NULL");
    ASSERT(opts.wrapping_sdk_identifier == NULL,
           "wrapping_sdk_identifier defaults to NULL");
    ASSERT(opts.cpu_refresh_interval_ms == 0,
           "cpu_refresh_interval_ms defaults to 0 (unset)");
    return result;
}

static int test_build_rejects_null_out_runtime(void)
{
    int result = TEST_PASS;
    cosmos_runtime_options_t opts = cosmos_runtime_options_default();
    cosmos_error_t *err = NULL;
    int32_t rc = cosmos_runtime_build(&opts, NULL, &err);
    ASSERT(rc == COSMOS_ERROR_CODE_INVALID_ARGUMENT,
           "NULL out_runtime rejected (rc=%d)", rc);
    ASSERT(err == NULL, "out_error untouched when out_runtime is NULL");
    return result;
}

static int test_workload_id_range_validation(void)
{
    int result = TEST_PASS;
    cosmos_runtime_t *runtime = NULL;
    cosmos_error_t *err = NULL;

    // 51 is one over the 1..=50 cap; 0 is the "unset" sentinel (not an error).
    cosmos_runtime_options_t opts = cosmos_runtime_options_default();
    opts.workload_id = 51;
    int32_t rc = cosmos_runtime_build(&opts, &runtime, &err);
    ASSERT(rc == COSMOS_ERROR_CODE_INVALID_OPTION_VALUE,
           "workload_id=51 rejected (rc=%d)", rc);
    ASSERT(runtime == NULL, "out_runtime untouched on invalid workload_id");
    return result;
}

static int test_string_field_validation(void)
{
    int result = TEST_PASS;
    cosmos_runtime_t *runtime = NULL;
    cosmos_error_t *err = NULL;

    // Correlation id: 51 chars — one over the cap.
    cosmos_runtime_options_t opts = cosmos_runtime_options_default();
    opts.correlation_id = "xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx";
    int32_t rc = cosmos_runtime_build(&opts, &runtime, &err);
    ASSERT(rc == COSMOS_ERROR_CODE_INVALID_OPTION_VALUE,
           "correlation_id too-long rejected (rc=%d)", rc);
    ASSERT(runtime == NULL, "out_runtime untouched on invalid correlation_id");

    // Correlation id: invalid charset (space).
    opts = cosmos_runtime_options_default();
    opts.correlation_id = "has space";
    rc = cosmos_runtime_build(&opts, &runtime, &err);
    ASSERT(rc == COSMOS_ERROR_CODE_INVALID_OPTION_VALUE,
           "correlation_id invalid charset rejected (rc=%d)", rc);
    ASSERT(runtime == NULL, "out_runtime untouched on bad-charset correlation_id");

    // User-agent suffix: 26 chars — one over the cap.
    opts = cosmos_runtime_options_default();
    opts.user_agent_suffix = "xxxxxxxxxxxxxxxxxxxxxxxxxx";
    rc = cosmos_runtime_build(&opts, &runtime, &err);
    ASSERT(rc == COSMOS_ERROR_CODE_INVALID_OPTION_VALUE,
           "user_agent_suffix too-long rejected (rc=%d)", rc);
    ASSERT(runtime == NULL, "out_runtime untouched on invalid user_agent_suffix");

    return result;
}

static int test_cpu_refresh_interval_range(void)
{
    int result = TEST_PASS;
    cosmos_runtime_t *runtime = NULL;
    cosmos_error_t *err = NULL;

    // 999 ms — one under the 1000..=60000 range.
    cosmos_runtime_options_t opts = cosmos_runtime_options_default();
    opts.cpu_refresh_interval_ms = 999;
    int32_t rc = cosmos_runtime_build(&opts, &runtime, &err);
    ASSERT(rc == COSMOS_ERROR_CODE_INVALID_OPTION_VALUE,
           "999 ms rejected (rc=%d)", rc);
    ASSERT(runtime == NULL, "out_runtime untouched on too-small interval");

    // 60001 ms — one over the range.
    opts = cosmos_runtime_options_default();
    opts.cpu_refresh_interval_ms = 60001;
    rc = cosmos_runtime_build(&opts, &runtime, &err);
    ASSERT(rc == COSMOS_ERROR_CODE_INVALID_OPTION_VALUE,
           "60001 ms rejected (rc=%d)", rc);
    ASSERT(runtime == NULL, "out_runtime untouched on too-large interval");

    return result;
}

static int test_build_happy_path(void)
{
    int result = TEST_PASS;
    cosmos_runtime_t *runtime = NULL;
    cosmos_error_t *err = NULL;

    // Configure a couple of valid fields and build.
    cosmos_runtime_options_t opts = cosmos_runtime_options_default();
    opts.workload_id = 7;
    opts.user_agent_suffix = "c-tests";
    opts.cpu_refresh_interval_ms = 5000;

    int32_t rc = cosmos_runtime_build(&opts, &runtime, &err);
    REQUIRE(rc == COSMOS_ERROR_CODE_SUCCESS,
            "cosmos_runtime_build returned SUCCESS (rc=%d)", rc);
    REQUIRE(runtime != NULL, "build produced a non-NULL runtime");
    ASSERT(err == NULL, "no rich error returned on success");

    cosmos_completion_queue_t *cq = cosmos_completion_queue_create(runtime, NULL);
    REQUIRE(cq != NULL, "completion_queue_create(runtime) returned non-NULL");

    // Sanity: queue state is RUNNING.
    ASSERT(cosmos_completion_queue_state(cq) == COSMOS_COMPLETION_QUEUE_STATE_RUNNING,
           "queue starts in RUNNING state");

    cosmos_completion_queue_free(cq);

cleanup:
    cosmos_runtime_free(runtime);
    return result;
}

static int test_build_null_options_uses_defaults(void)
{
    int result = TEST_PASS;
    cosmos_runtime_t *runtime = NULL;
    cosmos_error_t *err = NULL;

    // A NULL options pointer means "all driver defaults".
    int32_t rc = cosmos_runtime_build(NULL, &runtime, &err);
    REQUIRE(rc == COSMOS_ERROR_CODE_SUCCESS,
            "build(NULL options) returned SUCCESS (rc=%d)", rc);
    REQUIRE(runtime != NULL, "build produced a non-NULL runtime");
    ASSERT(err == NULL, "no rich error returned on success");

cleanup:
    cosmos_runtime_free(runtime);
    return result;
}

TEST_SUITE_BEGIN("Runtime Construction & Lifecycle")
TEST_REGISTER(options_default_all_unset)
TEST_REGISTER(build_rejects_null_out_runtime)
TEST_REGISTER(workload_id_range_validation)
TEST_REGISTER(string_field_validation)
TEST_REGISTER(cpu_refresh_interval_range)
TEST_REGISTER(build_happy_path)
TEST_REGISTER(build_null_options_uses_defaults)
TEST_SUITE_END("Runtime Construction & Lifecycle")

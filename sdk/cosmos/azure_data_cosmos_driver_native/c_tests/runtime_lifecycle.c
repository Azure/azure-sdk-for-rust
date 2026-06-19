// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

// Runtime builder & lifecycle.
//
// Validates the C surface of `cosmos_runtime_builder_*` end-to-end:
//
//   1. NULL-safety on `_free` and on every setter that takes a builder
//      pointer.
//   2. Range / charset validation on the typed setters (workload id,
//      correlation id, user-agent suffix, cpu refresh interval).
//   3. The happy path — build a runtime via the builder, then prove the
//      resulting `cosmos_runtime_t *` is usable by handing it to
//      `cosmos_cq_create` / `cosmos_cq_free` / `cosmos_runtime_free`.
//
// The multi-producer / single-consumer scenario lives in the Rust-side
// completion_t tests (which already exercise it via the internal
// `__test_only_enqueue_completion` helper). The lifecycle path validated
// here is the contract the submit path builds on top of.

#include "test_common.h"

static int test_builder_lifecycle_null_safe(void)
{
    int result = TEST_PASS;
    cosmos_runtime_builder_free(NULL);
    ASSERT(1, "cosmos_runtime_builder_free(NULL) returned without crashing");
    return result;
}

static int test_builder_setters_reject_null(void)
{
    int result = TEST_PASS;

    int32_t rc;
    rc = cosmos_runtime_builder_with_workload_id(NULL, 1);
    ASSERT(rc == COSMOS_ERROR_CODE_INVALID_ARGUMENT,
           "with_workload_id rejects NULL builder (rc=%d)", rc);

    rc = cosmos_runtime_builder_with_correlation_id(NULL, "x");
    ASSERT(rc == COSMOS_ERROR_CODE_INVALID_ARGUMENT,
           "with_correlation_id rejects NULL builder (rc=%d)", rc);

    rc = cosmos_runtime_builder_with_user_agent_suffix(NULL, "x");
    ASSERT(rc == COSMOS_ERROR_CODE_INVALID_ARGUMENT,
           "with_user_agent_suffix rejects NULL builder (rc=%d)", rc);

    rc = cosmos_runtime_builder_with_wrapping_sdk_identifier(NULL, "x");
    ASSERT(rc == COSMOS_ERROR_CODE_INVALID_ARGUMENT,
           "with_wrapping_sdk_identifier rejects NULL builder (rc=%d)", rc);

    rc = cosmos_runtime_builder_with_cpu_refresh_interval_ms(NULL, 5000);
    ASSERT(rc == COSMOS_ERROR_CODE_INVALID_ARGUMENT,
           "with_cpu_refresh_interval_ms rejects NULL builder (rc=%d)", rc);

    return result;
}

static int test_workload_id_range_validation(void)
{
    int result = TEST_PASS;
    cosmos_runtime_builder_t *b = cosmos_runtime_builder_new();
    REQUIRE(b != NULL, "builder allocated");

    int32_t rc = cosmos_runtime_builder_with_workload_id(b, 1);
    ASSERT(rc == COSMOS_ERROR_CODE_SUCCESS, "workload_id=1 accepted (rc=%d)", rc);

    rc = cosmos_runtime_builder_with_workload_id(b, 50);
    ASSERT(rc == COSMOS_ERROR_CODE_SUCCESS, "workload_id=50 accepted (rc=%d)", rc);

    rc = cosmos_runtime_builder_with_workload_id(b, 0);
    ASSERT(rc == COSMOS_ERROR_CODE_INVALID_OPTION_VALUE,
           "workload_id=0 rejected (rc=%d)", rc);

    rc = cosmos_runtime_builder_with_workload_id(b, 51);
    ASSERT(rc == COSMOS_ERROR_CODE_INVALID_OPTION_VALUE,
           "workload_id=51 rejected (rc=%d)", rc);

cleanup:
    cosmos_runtime_builder_free(b);
    return result;
}

static int test_string_setters_validation(void)
{
    int result = TEST_PASS;
    cosmos_runtime_builder_t *b = cosmos_runtime_builder_new();
    REQUIRE(b != NULL, "builder allocated");

    int32_t rc;

    rc = cosmos_runtime_builder_with_correlation_id(b, "aks-prod-eastus-001");
    ASSERT(rc == COSMOS_ERROR_CODE_SUCCESS, "correlation_id accepted (rc=%d)", rc);

    // 51 characters — one over the cap.
    rc = cosmos_runtime_builder_with_correlation_id(
        b, "xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx");
    ASSERT(rc == COSMOS_ERROR_CODE_INVALID_OPTION_VALUE,
           "correlation_id too-long rejected (rc=%d)", rc);

    rc = cosmos_runtime_builder_with_correlation_id(b, "has space");
    ASSERT(rc == COSMOS_ERROR_CODE_INVALID_OPTION_VALUE,
           "correlation_id invalid charset rejected (rc=%d)", rc);

    rc = cosmos_runtime_builder_with_correlation_id(b, NULL);
    ASSERT(rc == COSMOS_ERROR_CODE_INVALID_ARGUMENT,
           "correlation_id NULL string rejected (rc=%d)", rc);

    rc = cosmos_runtime_builder_with_user_agent_suffix(b, "myapp-westus2");
    ASSERT(rc == COSMOS_ERROR_CODE_SUCCESS, "user_agent_suffix accepted (rc=%d)", rc);

    // 26 characters — one over the cap.
    rc = cosmos_runtime_builder_with_user_agent_suffix(b, "xxxxxxxxxxxxxxxxxxxxxxxxxx");
    ASSERT(rc == COSMOS_ERROR_CODE_INVALID_OPTION_VALUE,
           "user_agent_suffix too-long rejected (rc=%d)", rc);

    rc = cosmos_runtime_builder_with_wrapping_sdk_identifier(b, "azsdk-rust-cosmos/0.34.0");
    ASSERT(rc == COSMOS_ERROR_CODE_SUCCESS,
           "wrapping_sdk_identifier accepted (rc=%d)", rc);

cleanup:
    cosmos_runtime_builder_free(b);
    return result;
}

static int test_cpu_refresh_interval_range(void)
{
    int result = TEST_PASS;
    cosmos_runtime_builder_t *b = cosmos_runtime_builder_new();
    REQUIRE(b != NULL, "builder allocated");

    int32_t rc;

    rc = cosmos_runtime_builder_with_cpu_refresh_interval_ms(b, 1000);
    ASSERT(rc == COSMOS_ERROR_CODE_SUCCESS, "1000 ms accepted (rc=%d)", rc);

    rc = cosmos_runtime_builder_with_cpu_refresh_interval_ms(b, 60000);
    ASSERT(rc == COSMOS_ERROR_CODE_SUCCESS, "60000 ms accepted (rc=%d)", rc);

    rc = cosmos_runtime_builder_with_cpu_refresh_interval_ms(b, 999);
    ASSERT(rc == COSMOS_ERROR_CODE_INVALID_OPTION_VALUE,
           "999 ms rejected (rc=%d)", rc);

    rc = cosmos_runtime_builder_with_cpu_refresh_interval_ms(b, 60001);
    ASSERT(rc == COSMOS_ERROR_CODE_INVALID_OPTION_VALUE,
           "60001 ms rejected (rc=%d)", rc);

    rc = cosmos_runtime_builder_with_cpu_refresh_interval_ms(b, 0);
    ASSERT(rc == COSMOS_ERROR_CODE_INVALID_OPTION_VALUE,
           "0 ms rejected (rc=%d)", rc);

cleanup:
    cosmos_runtime_builder_free(b);
    return result;
}

static int test_build_happy_path(void)
{
    int result = TEST_PASS;
    cosmos_runtime_builder_t *b = cosmos_runtime_builder_new();
    REQUIRE(b != NULL, "builder allocated");

    int32_t rc = cosmos_runtime_builder_with_user_agent_suffix(b, "c-tests");
    REQUIRE(rc == COSMOS_ERROR_CODE_SUCCESS,
            "with_user_agent_suffix returned SUCCESS (rc=%d)", rc);

    cosmos_runtime_t *runtime = NULL;
    cosmos_error_t *err = NULL;
    rc = cosmos_runtime_builder_build(b, &runtime, &err);
    REQUIRE(rc == COSMOS_ERROR_CODE_SUCCESS,
            "cosmos_runtime_builder_build returned SUCCESS (rc=%d)", rc);
    REQUIRE(runtime != NULL, "build produced a non-NULL runtime");
    ASSERT(err == NULL, "no rich error returned on success");

    // The builder is consumed by `_build`; do NOT free it.

    cosmos_cq_t *cq = cosmos_cq_create(runtime, NULL);
    REQUIRE(cq != NULL, "cq_create(runtime) returned non-NULL");

    // Sanity: queue state is RUNNING and runtime accessor round-trips.
    ASSERT(cosmos_cq_state(cq) == COSMOS_CQ_STATE_RUNNING,
           "queue starts in RUNNING state");
    ASSERT(cosmos_cq_runtime(cq) != NULL,
           "cq_runtime() returns the runtime");

    cosmos_cq_free(cq);
    cosmos_runtime_free(runtime);
    return result;

cleanup:
    /* `b` is only freed when `_build` was never called successfully. */
    cosmos_runtime_builder_free(b);
    return result;
}

static int test_build_rejects_null_arguments(void)
{
    int result = TEST_PASS;
    cosmos_runtime_t *runtime = NULL;
    cosmos_error_t *err = NULL;

    /* NULL builder is rejected without writing the out-slots. */
    int32_t rc = cosmos_runtime_builder_build(NULL, &runtime, &err);
    ASSERT(rc == COSMOS_ERROR_CODE_INVALID_ARGUMENT,
           "NULL builder rejected (rc=%d)", rc);
    ASSERT(runtime == NULL, "out_runtime untouched on NULL-builder failure");
    ASSERT(err == NULL, "out_error untouched on NULL-builder failure");

    /* NULL out_runtime is rejected (the builder is consumed regardless). */
    cosmos_runtime_builder_t *b = cosmos_runtime_builder_new();
    REQUIRE(b != NULL, "builder allocated");
    rc = cosmos_runtime_builder_build(b, NULL, &err);
    ASSERT(rc == COSMOS_ERROR_CODE_INVALID_ARGUMENT,
           "NULL out_runtime rejected (rc=%d)", rc);
    ASSERT(err == NULL, "out_error untouched when out_runtime is NULL");
    /* Do NOT free `b` — it has been consumed. */
    return result;

cleanup:
    return result;
}

TEST_SUITE_BEGIN("Runtime Builder Lifecycle")
TEST_REGISTER(builder_lifecycle_null_safe)
TEST_REGISTER(builder_setters_reject_null)
TEST_REGISTER(workload_id_range_validation)
TEST_REGISTER(string_setters_validation)
TEST_REGISTER(cpu_refresh_interval_range)
TEST_REGISTER(build_happy_path)
TEST_REGISTER(build_rejects_null_arguments)
TEST_SUITE_END("Runtime Builder Lifecycle")

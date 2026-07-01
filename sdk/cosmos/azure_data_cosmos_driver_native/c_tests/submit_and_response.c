// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

// Submit pipeline, completion queue, container refs, feed ranges,
// container/item factories.
//
// The full emulator-backed CRUD test requires a running Cosmos emulator
// and is exercised via CI when the emulator endpoint is reachable. This
// harness covers the pure lifecycle / NULL-safety / pre-flight rejection
// contracts that do not touch the network so external SDKs can validate
// their bindings before they ever start an emulator.

#include "test_common.h"

// ─────────────────────────────────────────────────────────────────────
// Section 1 — lifecycle & NULL-safety
// ─────────────────────────────────────────────────────────────────────

static int test_lifecycle_null_safe(void)
{
    int result = TEST_PASS;
    cosmos_completion_queue_free(NULL);
    cosmos_completion_queue_free_completions(NULL, 0);
    cosmos_container_ref_free(NULL);
    cosmos_feed_range_free(NULL);
    ASSERT(1, "_free entry points NULL-safe");
    return result;
}

// ─────────────────────────────────────────────────────────────────────
// Section 2 — completion queue NULL-safety + header-id mapping
// ─────────────────────────────────────────────────────────────────────

static int test_completion_queue_wait_handles_null(void)
{
    int result = TEST_PASS;
    cosmos_completion_t out;
    // NULL queue drains nothing.
    ASSERT(cosmos_completion_queue_wait(NULL, &out, 1, 0) == 0,
           "wait(NULL queue) drains 0");
    // NULL out slot / zero max drain nothing (no queue needed).
    ASSERT(cosmos_completion_queue_wait(NULL, NULL, 0, 0) == 0,
           "wait(NULL out, max 0) drains 0");
    // create(NULL runtime) returns NULL.
    ASSERT(cosmos_completion_queue_create(NULL, NULL) == NULL,
           "create(NULL runtime) returns NULL");
    return result;
}

static int test_header_name_mapping(void)
{
    int result = TEST_PASS;
    // The forward-compat sentinel has no wire name.
    ASSERT(cosmos_header_name(COSMOS_HEADER_ID_UNKNOWN) == NULL,
           "header_name(UNKNOWN) == NULL");
    // A known id maps to its canonical wire name.
    const char *activity = cosmos_header_name(COSMOS_HEADER_ID_ACTIVITY_ID);
    REQUIRE(activity != NULL, "header_name(ACTIVITY_ID) non-NULL");
    ASSERT(strcmp(activity, "x-ms-activity-id") == 0,
           "header_name(ACTIVITY_ID) == x-ms-activity-id (got %s)", activity);
    const char *etag = cosmos_header_name(COSMOS_HEADER_ID_ETAG);
    REQUIRE(etag != NULL, "header_name(ETAG) non-NULL");
    ASSERT(strcmp(etag, "etag") == 0,
           "header_name(ETAG) == etag (got %s)", etag);
cleanup:
    return result;
}

// ─────────────────────────────────────────────────────────────────────
// Section 3 — feed range builders
// ─────────────────────────────────────────────────────────────────────

static int test_feed_range_full_roundtrip(void)
{
    int result = TEST_PASS;
    cosmos_feed_range_t *fr = NULL;
    int32_t rc = cosmos_feed_range_full(&fr);
    ASSERT(rc == COSMOS_ERROR_CODE_SUCCESS, "full (rc=%d)", rc);
    REQUIRE(fr != NULL, "full produced non-NULL");

cleanup:
    cosmos_feed_range_free(fr);
    return result;
}

static int test_feed_range_full_rejects_null_out(void)
{
    int result = TEST_PASS;
    int32_t rc = cosmos_feed_range_full(NULL);
    ASSERT(rc == COSMOS_ERROR_CODE_INVALID_ARGUMENT,
           "full(NULL) rejected (rc=%d)", rc);
    return result;
}

static int test_feed_range_for_pk_rejects_nulls(void)
{
    int result = TEST_PASS;
    cosmos_feed_range_t *fr = NULL;
    int32_t rc = cosmos_feed_range_for_partition_key(NULL, NULL, &fr);
    ASSERT(rc == COSMOS_ERROR_CODE_INVALID_ARGUMENT, "rejects NULL inputs (rc=%d)", rc);
    rc = cosmos_feed_range_for_partition_key(NULL, NULL, NULL);
    ASSERT(rc == COSMOS_ERROR_CODE_INVALID_ARGUMENT, "rejects NULL out (rc=%d)", rc);
    return result;
}

// ─────────────────────────────────────────────────────────────────────
// Section 4 — container ref / resolve preflight
// ─────────────────────────────────────────────────────────────────────


static int test_resolve_container_blocking_rejects_nulls(void)
{
    int result = TEST_PASS;
    cosmos_container_ref_t *out = NULL;
    cosmos_error_t *err = NULL;
    int32_t rc = cosmos_driver_resolve_container_blocking(
        NULL, NULL, "db", "c", &out, &err);
    ASSERT(rc == COSMOS_ERROR_CODE_INVALID_ARGUMENT,
           "rejects NULL runtime/driver (rc=%d)", rc);
    return result;
}

// ─────────────────────────────────────────────────────────────────────
// Section 5 — submit pipeline preflight
// ─────────────────────────────────────────────────────────────────────

static int test_execute_operation_submit_rejects_null_driver(void)
{
    int result = TEST_PASS;
    cosmos_error_code_t err = COSMOS_ERROR_CODE_SUCCESS;
    cosmos_operation_handle_t *h =
        cosmos_submit_operation(NULL, NULL, NULL, 0, &err);
    ASSERT(h == NULL, "submit returned NULL");
    ASSERT(err == COSMOS_ERROR_CODE_INVALID_ARGUMENT,
           "submit set INVALID_ARGUMENT (err=%d)", err);
    return result;
}

static int test_execute_singleton_operation_submit_rejects_null_driver(void)
{
    int result = TEST_PASS;
    cosmos_error_code_t err = COSMOS_ERROR_CODE_SUCCESS;
    cosmos_operation_handle_t *h =
        cosmos_submit_singleton_operation(NULL, NULL, NULL, 0, &err);
    ASSERT(h == NULL, "submit returned NULL");
    ASSERT(err == COSMOS_ERROR_CODE_INVALID_ARGUMENT,
           "submit set INVALID_ARGUMENT (err=%d)", err);
    return result;
}

static int test_get_or_create_submit_rejects_null_runtime(void)
{
    int result = TEST_PASS;
    cosmos_error_code_t err = COSMOS_ERROR_CODE_SUCCESS;
    cosmos_operation_handle_t *h =
        cosmos_driver_get_or_create_submit(NULL, NULL, NULL, NULL, 0, &err);
    ASSERT(h == NULL, "submit returned NULL");
    ASSERT(err == COSMOS_ERROR_CODE_INVALID_ARGUMENT,
           "set INVALID_ARGUMENT (err=%d)", err);
    return result;
}

static int test_resolve_container_submit_rejects_null_driver(void)
{
    int result = TEST_PASS;
    cosmos_error_code_t err = COSMOS_ERROR_CODE_SUCCESS;
    cosmos_operation_handle_t *h =
        cosmos_driver_resolve_container_submit(NULL, "db", "c", NULL, 0, &err);
    ASSERT(h == NULL, "submit returned NULL");
    ASSERT(err == COSMOS_ERROR_CODE_INVALID_ARGUMENT,
           "set INVALID_ARGUMENT (err=%d)", err);
    return result;
}

// ─────────────────────────────────────────────────────────────────────
// Section 6 — singleton submit preflight with a populated request
// ─────────────────────────────────────────────────────────────────────

static int test_singleton_submit_with_request_rejects_null_driver(void)
{
    int result = TEST_PASS;
    cosmos_operation_request_t req = {0};
    req.kind = COSMOS_OPERATION_KIND_READ_ITEM;
    req.item_id = "id-1";
    req.max_item_count = -1;

    cosmos_error_code_t err = COSMOS_ERROR_CODE_SUCCESS;
    cosmos_operation_handle_t *h =
        cosmos_submit_singleton_operation(NULL, &req, NULL, 0, &err);
    ASSERT(h == NULL, "submit returned NULL on NULL driver");
    ASSERT(err == COSMOS_ERROR_CODE_INVALID_ARGUMENT,
           "set INVALID_ARGUMENT (err=%d)", err);
    return result;
}

TEST_SUITE_BEGIN("Submit + Completion Queue + Container + Feed Range")
TEST_REGISTER(lifecycle_null_safe)
TEST_REGISTER(completion_queue_wait_handles_null)
TEST_REGISTER(header_name_mapping)
TEST_REGISTER(feed_range_full_roundtrip)
TEST_REGISTER(feed_range_full_rejects_null_out)
TEST_REGISTER(feed_range_for_pk_rejects_nulls)
TEST_REGISTER(resolve_container_blocking_rejects_nulls)
TEST_REGISTER(execute_operation_submit_rejects_null_driver)
TEST_REGISTER(execute_singleton_operation_submit_rejects_null_driver)
TEST_REGISTER(get_or_create_submit_rejects_null_runtime)
TEST_REGISTER(resolve_container_submit_rejects_null_driver)
TEST_REGISTER(singleton_submit_with_request_rejects_null_driver)
TEST_SUITE_END("Submit + Completion Queue + Container + Feed Range")

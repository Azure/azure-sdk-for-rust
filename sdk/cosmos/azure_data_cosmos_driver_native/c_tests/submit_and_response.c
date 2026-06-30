// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

// Submit pipeline, response surface, container refs, feed ranges,
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
    cosmos_response_free(NULL);
    cosmos_container_ref_free(NULL);
    cosmos_feed_range_free(NULL);
    ASSERT(1, "_free entry points NULL-safe");
    return result;
}

// ─────────────────────────────────────────────────────────────────────
// Section 2 — response accessors handle NULL
// ─────────────────────────────────────────────────────────────────────

static int test_response_accessors_handle_null(void)
{
    int result = TEST_PASS;
    ASSERT(cosmos_response_status_code(NULL) == 0,
           "status_code(NULL) returns 0");
    ASSERT(cosmos_response_request_charge(NULL) == 0.0,
           "request_charge(NULL) returns 0.0");
    ASSERT(cosmos_response_activity_id(NULL) == NULL,
           "activity_id(NULL) returns NULL");
    ASSERT(cosmos_response_session_token(NULL) == NULL,
           "session_token(NULL) returns NULL");
    ASSERT(cosmos_response_etag(NULL) == NULL, "etag(NULL) returns NULL");
    ASSERT(cosmos_response_continuation_token(NULL) == NULL,
           "continuation_token(NULL) returns NULL");

    const uint8_t *data = (const uint8_t *)0xDEADBEEF;
    size_t len = 999;
    int32_t rc = cosmos_response_body(NULL, &data, &len);
    ASSERT(rc == COSMOS_ERROR_CODE_INVALID_ARGUMENT,
           "body(NULL,...) rejected (rc=%d)", rc);

    ASSERT(cosmos_response_take_driver(NULL) == NULL,
           "take_driver(NULL) returns NULL");
    ASSERT(cosmos_response_take_container(NULL) == NULL,
           "take_container(NULL) returns NULL");
    return result;
}

static int test_response_view_rejects_null(void)
{
    int result = TEST_PASS;
    cosmos_response_view_t view;
    int32_t rc = cosmos_response_view(NULL, &view);
    ASSERT(rc == COSMOS_ERROR_CODE_INVALID_ARGUMENT,
           "response_view(NULL, &view) rejected (rc=%d)", rc);
    rc = cosmos_response_view(NULL, NULL);
    ASSERT(rc == COSMOS_ERROR_CODE_INVALID_ARGUMENT,
           "response_view(NULL, NULL) rejected (rc=%d)", rc);
    return result;
}

static int test_completion_view_rejects_null(void)
{
    int result = TEST_PASS;
    cosmos_completion_view_t view;
    cosmos_error_code_t rc = cosmos_completion_view(NULL, &view);
    ASSERT(rc == COSMOS_ERROR_CODE_INVALID_ARGUMENT,
           "completion_view(NULL, &view) rejected (rc=%d)", rc);
    rc = cosmos_completion_view(NULL, NULL);
    ASSERT(rc == COSMOS_ERROR_CODE_INVALID_ARGUMENT,
           "completion_view(NULL, NULL) rejected (rc=%d)", rc);
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

TEST_SUITE_BEGIN("Submit + Response + Container + Feed Range")
TEST_REGISTER(lifecycle_null_safe)
TEST_REGISTER(response_accessors_handle_null)
TEST_REGISTER(response_view_rejects_null)
TEST_REGISTER(completion_view_rejects_null)
TEST_REGISTER(feed_range_full_roundtrip)
TEST_REGISTER(feed_range_full_rejects_null_out)
TEST_REGISTER(feed_range_for_pk_rejects_nulls)
TEST_REGISTER(resolve_container_blocking_rejects_nulls)
TEST_REGISTER(execute_operation_submit_rejects_null_driver)
TEST_REGISTER(execute_singleton_operation_submit_rejects_null_driver)
TEST_REGISTER(get_or_create_submit_rejects_null_runtime)
TEST_REGISTER(resolve_container_submit_rejects_null_driver)
TEST_REGISTER(singleton_submit_with_request_rejects_null_driver)
TEST_SUITE_END("Submit + Response + Container + Feed Range")

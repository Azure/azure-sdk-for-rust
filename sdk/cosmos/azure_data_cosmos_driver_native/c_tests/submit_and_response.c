// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

// Phase 6 — submit pipeline, response surface, container refs, feed
// ranges, container/item factories.
//
// The full emulator-backed CRUD test (spec section 8 Phase 6 done-when)
// requires a running Cosmos emulator and is exercised via CI when the
// emulator endpoint is reachable. This harness covers the pure
// lifecycle / NULL-safety / pre-flight rejection contracts that do not
// touch the network so external SDKs can validate their bindings
// before they ever start an emulator.

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
    ASSERT(1, "Phase 6 _free entry points NULL-safe");
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

    cosmos_feed_range_t *cloned = NULL;
    rc = cosmos_feed_range_clone(fr, &cloned);
    ASSERT(rc == COSMOS_ERROR_CODE_SUCCESS, "clone (rc=%d)", rc);
    ASSERT(cloned != NULL, "clone produced non-NULL");

cleanup:
    cosmos_feed_range_free(cloned);
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

static int test_container_ref_clone_rejects_null(void)
{
    int result = TEST_PASS;
    cosmos_container_ref_t *out = NULL;
    int32_t rc = cosmos_container_ref_clone(NULL, &out);
    ASSERT(rc == COSMOS_ERROR_CODE_INVALID_ARGUMENT, "clone(NULL,...) rejected (rc=%d)", rc);
    return result;
}

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

static int test_driver_submit_rejects_null_driver(void)
{
    int result = TEST_PASS;
    cosmos_error_code_t err = COSMOS_ERROR_CODE_SUCCESS;
    cosmos_operation_handle_t *h =
        cosmos_driver_submit(NULL, NULL, NULL, NULL, NULL, &err);
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
        cosmos_driver_get_or_create_submit(NULL, NULL, NULL, NULL, NULL, &err);
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
        cosmos_driver_resolve_container_submit(NULL, "db", "c", NULL, NULL, &err);
    ASSERT(h == NULL, "submit returned NULL");
    ASSERT(err == COSMOS_ERROR_CODE_INVALID_ARGUMENT,
           "set INVALID_ARGUMENT (err=%d)", err);
    return result;
}

// ─────────────────────────────────────────────────────────────────────
// Section 6 — container/item factories preflight
// ─────────────────────────────────────────────────────────────────────

static int test_container_factories_reject_null_container(void)
{
    int result = TEST_PASS;
    cosmos_operation_t *op = NULL;
    int32_t rc;

    rc = cosmos_operation_read_container(NULL, &op);
    ASSERT(rc == COSMOS_ERROR_CODE_INVALID_ARGUMENT, "read_container(NULL) (rc=%d)", rc);

    rc = cosmos_operation_read_all_items_cross_partition(NULL, &op);
    ASSERT(rc == COSMOS_ERROR_CODE_INVALID_ARGUMENT,
           "read_all_items_cross_partition(NULL) (rc=%d)", rc);

    rc = cosmos_operation_query_items(NULL, NULL, &op);
    ASSERT(rc == COSMOS_ERROR_CODE_INVALID_ARGUMENT,
           "query_items(NULL,NULL,&op) (rc=%d)", rc);

    rc = cosmos_operation_read_all_items(NULL, NULL, &op);
    ASSERT(rc == COSMOS_ERROR_CODE_INVALID_ARGUMENT,
           "read_all_items(NULL,NULL,&op) (rc=%d)", rc);

    rc = cosmos_operation_batch(NULL, NULL, &op);
    ASSERT(rc == COSMOS_ERROR_CODE_INVALID_ARGUMENT, "batch(NULL,NULL,&op) (rc=%d)", rc);

    rc = cosmos_operation_create_item(NULL, "id", NULL, &op);
    ASSERT(rc == COSMOS_ERROR_CODE_INVALID_ARGUMENT,
           "create_item(NULL,...) (rc=%d)", rc);

    rc = cosmos_operation_read_item(NULL, "id", NULL, &op);
    ASSERT(rc == COSMOS_ERROR_CODE_INVALID_ARGUMENT,
           "read_item(NULL,...) (rc=%d)", rc);

    return result;
}

// ─────────────────────────────────────────────────────────────────────
// Section 7 — patch-max-attempts mutator
// ─────────────────────────────────────────────────────────────────────

static int test_patch_max_attempts_rejects_zero(void)
{
    int result = TEST_PASS;
    int32_t rc = cosmos_operation_with_patch_max_attempts(NULL, 0);
    ASSERT(rc == COSMOS_ERROR_CODE_INVALID_ARGUMENT,
           "with_patch_max_attempts(NULL, 0) rejected (rc=%d)", rc);
    return result;
}

TEST_SUITE_BEGIN("Phase 6 — Submit + Response + Container + Feed Range")
TEST_REGISTER(lifecycle_null_safe)
TEST_REGISTER(response_accessors_handle_null)
TEST_REGISTER(feed_range_full_roundtrip)
TEST_REGISTER(feed_range_full_rejects_null_out)
TEST_REGISTER(feed_range_for_pk_rejects_nulls)
TEST_REGISTER(container_ref_clone_rejects_null)
TEST_REGISTER(resolve_container_blocking_rejects_nulls)
TEST_REGISTER(driver_submit_rejects_null_driver)
TEST_REGISTER(get_or_create_submit_rejects_null_runtime)
TEST_REGISTER(resolve_container_submit_rejects_null_driver)
TEST_REGISTER(container_factories_reject_null_container)
TEST_REGISTER(patch_max_attempts_rejects_zero)
TEST_SUITE_END("Phase 6 — Submit + Response + Container + Feed Range")

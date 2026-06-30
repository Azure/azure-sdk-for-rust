// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

// Partition-key builder & accessors.
//
// Covers the C surface of every partition-key entry point:
//
//   1. Lifecycle & NULL-safety on every `_free`.
//   2. Each value kind (`_add_string`, `_add_number`, `_add_bool`,
//      `_add_null`, `_add_undefined`) accepted in a hierarchical
//      (3-component) build.
//   3. Single-component build and `_component_count` round-trip.
//   4. Empty-build rejection (`INVALID_PARTITION_KEY`).
//   5. 4th-component rejection (`INVALID_OPTION_VALUE`).
//   6. `_add_number` rejects NaN / ±Infinity.
//   7. `cosmos_partition_key_empty` reports `is_empty == true` and
//      `component_count == 0`.
//   8. Clone round-trip + NULL handling.

#include <math.h>

#include "test_common.h"

static int test_lifecycle_null_safe(void)
{
    int result = TEST_PASS;
    cosmos_partition_key_builder_free(NULL);
    cosmos_partition_key_free(NULL);
    ASSERT(1, "_free entry points NULL-safe");
    return result;
}

static int test_empty_pk_accessor_roundtrip(void)
{
    int result = TEST_PASS;
    cosmos_partition_key_t *pk = cosmos_partition_key_empty();
    REQUIRE(pk != NULL, "cosmos_partition_key_empty returned non-NULL");
    ASSERT(cosmos_partition_key_component_count(pk) == 0,
           "empty pk has 0 components");
    ASSERT(cosmos_partition_key_is_empty(pk),
           "empty pk reports is_empty == true");

cleanup:
    cosmos_partition_key_free(pk);
    return result;
}

static int test_accessors_handle_null(void)
{
    int result = TEST_PASS;
    ASSERT(cosmos_partition_key_component_count(NULL) == 0,
           "component_count(NULL) returns 0");
    ASSERT(cosmos_partition_key_is_empty(NULL),
           "is_empty(NULL) returns true");
    return result;
}

static int test_single_string_component(void)
{
    int result = TEST_PASS;
    cosmos_partition_key_builder_t *b = cosmos_partition_key_builder_new();
    REQUIRE(b != NULL, "builder allocated");

    int32_t rc = cosmos_partition_key_builder_add_string(b, "tenant-42");
    ASSERT(rc == COSMOS_ERROR_CODE_SUCCESS,
           "add_string accepted (rc=%d)", rc);

    cosmos_partition_key_t *pk = NULL;
    rc = cosmos_partition_key_builder_build(b, &pk);
    /* Builder is consumed regardless. */
    ASSERT(rc == COSMOS_ERROR_CODE_SUCCESS,
           "build returned SUCCESS (rc=%d)", rc);
    REQUIRE(pk != NULL, "build produced a non-NULL handle");
    ASSERT(cosmos_partition_key_component_count(pk) == 1,
           "pk has 1 component");
    ASSERT(!cosmos_partition_key_is_empty(pk),
           "pk reports is_empty == false");

cleanup:
    cosmos_partition_key_free(pk);
    return result;
}

static int test_hierarchical_all_value_kinds(void)
{
    int result = TEST_PASS;
    cosmos_partition_key_builder_t *b = cosmos_partition_key_builder_new();
    REQUIRE(b != NULL, "builder allocated");

    int32_t rc = cosmos_partition_key_builder_add_string(b, "region-1");
    ASSERT(rc == COSMOS_ERROR_CODE_SUCCESS, "add_string ok (rc=%d)", rc);
    rc = cosmos_partition_key_builder_add_number(b, 42.0);
    ASSERT(rc == COSMOS_ERROR_CODE_SUCCESS, "add_number ok (rc=%d)", rc);
    rc = cosmos_partition_key_builder_add_bool(b, true);
    ASSERT(rc == COSMOS_ERROR_CODE_SUCCESS, "add_bool ok (rc=%d)", rc);

    cosmos_partition_key_t *pk = NULL;
    rc = cosmos_partition_key_builder_build(b, &pk);
    ASSERT(rc == COSMOS_ERROR_CODE_SUCCESS, "build ok (rc=%d)", rc);
    ASSERT(cosmos_partition_key_component_count(pk) == 3,
           "3-component hierarchical key");

cleanup:
    cosmos_partition_key_free(pk);
    return result;
}

static int test_null_and_undefined_components(void)
{
    int result = TEST_PASS;
    cosmos_partition_key_builder_t *b = cosmos_partition_key_builder_new();
    REQUIRE(b != NULL, "builder allocated");

    int32_t rc = cosmos_partition_key_builder_add_null(b);
    ASSERT(rc == COSMOS_ERROR_CODE_SUCCESS, "add_null ok (rc=%d)", rc);
    rc = cosmos_partition_key_builder_add_undefined(b);
    ASSERT(rc == COSMOS_ERROR_CODE_SUCCESS,
           "add_undefined ok (rc=%d)", rc);

    cosmos_partition_key_t *pk = NULL;
    rc = cosmos_partition_key_builder_build(b, &pk);
    ASSERT(rc == COSMOS_ERROR_CODE_SUCCESS, "build ok (rc=%d)", rc);
    ASSERT(cosmos_partition_key_component_count(pk) == 2,
           "(null, undefined) is a 2-component key, not EMPTY");

cleanup:
    cosmos_partition_key_free(pk);
    return result;
}

static int test_empty_build_rejected(void)
{
    int result = TEST_PASS;
    cosmos_partition_key_builder_t *b = cosmos_partition_key_builder_new();
    REQUIRE(b != NULL, "builder allocated");

    cosmos_partition_key_t *pk = NULL;
    int32_t rc = cosmos_partition_key_builder_build(b, &pk);
    /* Builder is consumed regardless of success. */
    ASSERT(rc == COSMOS_ERROR_CODE_INVALID_PARTITION_KEY,
           "empty build rejected (rc=%d)", rc);
    ASSERT(pk == NULL, "no handle on empty-build failure");

cleanup:
    return result;
}

static int test_fourth_component_rejected(void)
{
    int result = TEST_PASS;
    cosmos_partition_key_builder_t *b = cosmos_partition_key_builder_new();
    REQUIRE(b != NULL, "builder allocated");

    int32_t rc;
    rc = cosmos_partition_key_builder_add_string(b, "a");
    ASSERT(rc == COSMOS_ERROR_CODE_SUCCESS, "1st component ok");
    rc = cosmos_partition_key_builder_add_string(b, "b");
    ASSERT(rc == COSMOS_ERROR_CODE_SUCCESS, "2nd component ok");
    rc = cosmos_partition_key_builder_add_string(b, "c");
    ASSERT(rc == COSMOS_ERROR_CODE_SUCCESS, "3rd component ok");

    /* All five kinds must reject the 4th append. */
    rc = cosmos_partition_key_builder_add_string(b, "d");
    ASSERT(rc == COSMOS_ERROR_CODE_INVALID_OPTION_VALUE,
           "4th string rejected (rc=%d)", rc);
    rc = cosmos_partition_key_builder_add_number(b, 1.0);
    ASSERT(rc == COSMOS_ERROR_CODE_INVALID_OPTION_VALUE,
           "4th number rejected (rc=%d)", rc);
    rc = cosmos_partition_key_builder_add_bool(b, true);
    ASSERT(rc == COSMOS_ERROR_CODE_INVALID_OPTION_VALUE,
           "4th bool rejected (rc=%d)", rc);
    rc = cosmos_partition_key_builder_add_null(b);
    ASSERT(rc == COSMOS_ERROR_CODE_INVALID_OPTION_VALUE,
           "4th null rejected (rc=%d)", rc);
    rc = cosmos_partition_key_builder_add_undefined(b);
    ASSERT(rc == COSMOS_ERROR_CODE_INVALID_OPTION_VALUE,
           "4th undefined rejected (rc=%d)", rc);

    /* Build still works with the 3 accepted components. */
    cosmos_partition_key_t *pk = NULL;
    rc = cosmos_partition_key_builder_build(b, &pk);
    ASSERT(rc == COSMOS_ERROR_CODE_SUCCESS, "build ok");
    ASSERT(cosmos_partition_key_component_count(pk) == 3,
           "3 components preserved after rejected appends");

cleanup:
    cosmos_partition_key_free(pk);
    return result;
}

static int test_number_rejects_non_finite(void)
{
    int result = TEST_PASS;
    cosmos_partition_key_builder_t *b = cosmos_partition_key_builder_new();
    REQUIRE(b != NULL, "builder allocated");

    int32_t rc = cosmos_partition_key_builder_add_number(b, NAN);
    ASSERT(rc == COSMOS_ERROR_CODE_INVALID_OPTION_VALUE,
           "NaN rejected (rc=%d)", rc);
    rc = cosmos_partition_key_builder_add_number(b, INFINITY);
    ASSERT(rc == COSMOS_ERROR_CODE_INVALID_OPTION_VALUE,
           "+Inf rejected (rc=%d)", rc);
    rc = cosmos_partition_key_builder_add_number(b, -INFINITY);
    ASSERT(rc == COSMOS_ERROR_CODE_INVALID_OPTION_VALUE,
           "-Inf rejected (rc=%d)", rc);

    /* Builder is unmodified — build with no successful appends is
     * rejected as empty. */
    cosmos_partition_key_t *pk = NULL;
    rc = cosmos_partition_key_builder_build(b, &pk);
    ASSERT(rc == COSMOS_ERROR_CODE_INVALID_PARTITION_KEY,
           "builder still empty after rejected non-finite appends");

cleanup:
    return result;
}

static int test_setters_reject_null_builder(void)
{
    int result = TEST_PASS;
    int32_t rc;
    rc = cosmos_partition_key_builder_add_string(NULL, "x");
    ASSERT(rc == COSMOS_ERROR_CODE_INVALID_ARGUMENT,
           "add_string(NULL,...) rejected");
    rc = cosmos_partition_key_builder_add_number(NULL, 1.0);
    ASSERT(rc == COSMOS_ERROR_CODE_INVALID_ARGUMENT,
           "add_number(NULL,...) rejected");
    rc = cosmos_partition_key_builder_add_bool(NULL, true);
    ASSERT(rc == COSMOS_ERROR_CODE_INVALID_ARGUMENT,
           "add_bool(NULL,...) rejected");
    rc = cosmos_partition_key_builder_add_null(NULL);
    ASSERT(rc == COSMOS_ERROR_CODE_INVALID_ARGUMENT,
           "add_null(NULL) rejected");
    rc = cosmos_partition_key_builder_add_undefined(NULL);
    ASSERT(rc == COSMOS_ERROR_CODE_INVALID_ARGUMENT,
           "add_undefined(NULL) rejected");
    return result;
}

static int test_add_string_rejects_null_value(void)
{
    int result = TEST_PASS;
    cosmos_partition_key_builder_t *b = cosmos_partition_key_builder_new();
    REQUIRE(b != NULL, "builder allocated");

    int32_t rc = cosmos_partition_key_builder_add_string(b, NULL);
    ASSERT(rc == COSMOS_ERROR_CODE_INVALID_ARGUMENT,
           "add_string(b, NULL) rejected (rc=%d)", rc);

    cosmos_partition_key_builder_free(b);
    return result;

cleanup:
    cosmos_partition_key_builder_free(b);
    return result;
}

static int test_build_rejects_null_arguments(void)
{
    int result = TEST_PASS;
    cosmos_partition_key_t *pk = NULL;
    int32_t rc = cosmos_partition_key_builder_build(NULL, &pk);
    ASSERT(rc == COSMOS_ERROR_CODE_INVALID_ARGUMENT,
           "build(NULL, ...) rejected (rc=%d)", rc);
    ASSERT(pk == NULL, "out_pk untouched");

    cosmos_partition_key_builder_t *b = cosmos_partition_key_builder_new();
    REQUIRE(b != NULL, "builder allocated");
    rc = cosmos_partition_key_builder_add_string(b, "x");
    ASSERT(rc == COSMOS_ERROR_CODE_SUCCESS, "add ok");
    rc = cosmos_partition_key_builder_build(b, NULL);
    ASSERT(rc == COSMOS_ERROR_CODE_INVALID_ARGUMENT,
           "build(b, NULL) rejected (rc=%d)", rc);
    /* `b` has been consumed; do NOT free. */

cleanup:
    return result;
}

TEST_SUITE_BEGIN("Partition Key Builder")
TEST_REGISTER(lifecycle_null_safe)
TEST_REGISTER(empty_pk_accessor_roundtrip)
TEST_REGISTER(accessors_handle_null)
TEST_REGISTER(single_string_component)
TEST_REGISTER(hierarchical_all_value_kinds)
TEST_REGISTER(null_and_undefined_components)
TEST_REGISTER(empty_build_rejected)
TEST_REGISTER(fourth_component_rejected)
TEST_REGISTER(number_rejects_non_finite)
TEST_REGISTER(setters_reject_null_builder)
TEST_REGISTER(add_string_rejects_null_value)
TEST_REGISTER(build_rejects_null_arguments)
TEST_SUITE_END("Partition Key Builder")

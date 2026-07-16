// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

// Partition-key construction & accessors.
//
// Covers the C surface of every partition-key entry point:
//
//   1. Lifecycle & NULL-safety on `_free`.
//   2. Each value kind (string, number, bool, null, undefined) assembled in a
//      flat `cosmos_partition_key_component_t[]` array and built in one call
//      via `cosmos_partition_key_create`.
//   3. Single-component build and `_component_count` round-trip.
//   4. Empty-array rejection (`INVALID_PARTITION_KEY`).
//   5. Over-cap (4-component) rejection (`INVALID_PARTITION_KEY`).
//   6. Non-finite number rejection (`INVALID_OPTION_VALUE`).
//   7. NULL string-value rejection (`INVALID_ARGUMENT`).
//   8. NULL `out_pk` rejection (`INVALID_ARGUMENT`).
//   9. `cosmos_partition_key_empty` reports `is_empty == true` and
//      `component_count == 0`.
//
// The per-component incremental builder was removed in P5; construction is now
// a single `cosmos_partition_key_create(components, len, &out)` call using the
// same tagged-union component array the operation request accepts inline.

#include <math.h>

#include "test_common.h"

// Convenience: a component of a given kind with default value fields.
static cosmos_partition_key_component_t pk_component(
    cosmos_partition_key_component_kind_t kind)
{
    cosmos_partition_key_component_t c = {0};
    c.kind = kind;
    return c;
}

static int test_lifecycle_null_safe(void)
{
    int result = TEST_PASS;
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
    cosmos_partition_key_component_t comps[1];
    comps[0] = pk_component(COSMOS_PARTITION_KEY_COMPONENT_KIND_STRING);
    comps[0].string_value = "tenant-42";

    cosmos_partition_key_t *pk = NULL;
    int32_t rc = cosmos_partition_key_create(comps, 1, &pk);
    ASSERT(rc == COSMOS_ERROR_CODE_SUCCESS,
           "create returned SUCCESS (rc=%d)", rc);
    REQUIRE(pk != NULL, "create produced a non-NULL handle");
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
    cosmos_partition_key_component_t comps[3];
    comps[0] = pk_component(COSMOS_PARTITION_KEY_COMPONENT_KIND_STRING);
    comps[0].string_value = "region-1";
    comps[1] = pk_component(COSMOS_PARTITION_KEY_COMPONENT_KIND_NUMBER);
    comps[1].number_value = 42.0;
    comps[2] = pk_component(COSMOS_PARTITION_KEY_COMPONENT_KIND_BOOL);
    comps[2].bool_value = 1;

    cosmos_partition_key_t *pk = NULL;
    int32_t rc = cosmos_partition_key_create(comps, 3, &pk);
    ASSERT(rc == COSMOS_ERROR_CODE_SUCCESS, "create ok (rc=%d)", rc);
    ASSERT(cosmos_partition_key_component_count(pk) == 3,
           "3-component hierarchical key");

cleanup:
    cosmos_partition_key_free(pk);
    return result;
}

static int test_null_and_undefined_components(void)
{
    int result = TEST_PASS;
    cosmos_partition_key_component_t comps[2];
    comps[0] = pk_component(COSMOS_PARTITION_KEY_COMPONENT_KIND_NULL);
    comps[1] = pk_component(COSMOS_PARTITION_KEY_COMPONENT_KIND_UNDEFINED);

    cosmos_partition_key_t *pk = NULL;
    int32_t rc = cosmos_partition_key_create(comps, 2, &pk);
    ASSERT(rc == COSMOS_ERROR_CODE_SUCCESS, "create ok (rc=%d)", rc);
    ASSERT(cosmos_partition_key_component_count(pk) == 2,
           "(null, undefined) is a 2-component key, not EMPTY");

cleanup:
    cosmos_partition_key_free(pk);
    return result;
}

static int test_empty_create_rejected(void)
{
    int result = TEST_PASS;
    cosmos_partition_key_t *pk = NULL;
    // NULL array / zero length is rejected (use cosmos_partition_key_empty
    // for the deliberate cross-partition key).
    int32_t rc = cosmos_partition_key_create(NULL, 0, &pk);
    ASSERT(rc == COSMOS_ERROR_CODE_INVALID_PARTITION_KEY,
           "empty create rejected (rc=%d)", rc);
    ASSERT(pk == NULL, "no handle on empty-create failure");
    return result;
}

static int test_over_cap_rejected(void)
{
    int result = TEST_PASS;
    // Four components exceed the 3-level hierarchical cap.
    cosmos_partition_key_component_t comps[4];
    for (int i = 0; i < 4; i++) {
        comps[i] = pk_component(COSMOS_PARTITION_KEY_COMPONENT_KIND_NULL);
    }
    cosmos_partition_key_t *pk = NULL;
    int32_t rc = cosmos_partition_key_create(comps, 4, &pk);
    ASSERT(rc == COSMOS_ERROR_CODE_INVALID_PARTITION_KEY,
           "4-component create rejected (rc=%d)", rc);
    ASSERT(pk == NULL, "no handle on over-cap failure");
    return result;
}

static int test_number_rejects_non_finite(void)
{
    int result = TEST_PASS;
    cosmos_partition_key_t *pk = NULL;

    cosmos_partition_key_component_t comps[1];
    comps[0] = pk_component(COSMOS_PARTITION_KEY_COMPONENT_KIND_NUMBER);

    comps[0].number_value = NAN;
    int32_t rc = cosmos_partition_key_create(comps, 1, &pk);
    ASSERT(rc == COSMOS_ERROR_CODE_INVALID_OPTION_VALUE, "NaN rejected (rc=%d)", rc);
    ASSERT(pk == NULL, "no handle on NaN");

    comps[0].number_value = INFINITY;
    rc = cosmos_partition_key_create(comps, 1, &pk);
    ASSERT(rc == COSMOS_ERROR_CODE_INVALID_OPTION_VALUE, "+Inf rejected (rc=%d)", rc);

    comps[0].number_value = -INFINITY;
    rc = cosmos_partition_key_create(comps, 1, &pk);
    ASSERT(rc == COSMOS_ERROR_CODE_INVALID_OPTION_VALUE, "-Inf rejected (rc=%d)", rc);

    return result;
}

static int test_string_null_value_rejected(void)
{
    int result = TEST_PASS;
    cosmos_partition_key_component_t comps[1];
    comps[0] = pk_component(COSMOS_PARTITION_KEY_COMPONENT_KIND_STRING);
    comps[0].string_value = NULL;

    cosmos_partition_key_t *pk = NULL;
    int32_t rc = cosmos_partition_key_create(comps, 1, &pk);
    ASSERT(rc == COSMOS_ERROR_CODE_INVALID_ARGUMENT,
           "string component with NULL value rejected (rc=%d)", rc);
    ASSERT(pk == NULL, "no handle on NULL string value");
    return result;
}

static int test_create_rejects_null_out(void)
{
    int result = TEST_PASS;
    cosmos_partition_key_component_t comps[1];
    comps[0] = pk_component(COSMOS_PARTITION_KEY_COMPONENT_KIND_NULL);
    int32_t rc = cosmos_partition_key_create(comps, 1, NULL);
    ASSERT(rc == COSMOS_ERROR_CODE_INVALID_ARGUMENT,
           "create(comps, 1, NULL) rejected (rc=%d)", rc);
    return result;
}

TEST_SUITE_BEGIN("Partition Key Construction")
TEST_REGISTER(lifecycle_null_safe)
TEST_REGISTER(empty_pk_accessor_roundtrip)
TEST_REGISTER(accessors_handle_null)
TEST_REGISTER(single_string_component)
TEST_REGISTER(hierarchical_all_value_kinds)
TEST_REGISTER(null_and_undefined_components)
TEST_REGISTER(empty_create_rejected)
TEST_REGISTER(over_cap_rejected)
TEST_REGISTER(number_rejects_non_finite)
TEST_REGISTER(string_null_value_rejected)
TEST_REGISTER(create_rejects_null_out)
TEST_SUITE_END("Partition Key Construction")

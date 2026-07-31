// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

// End-to-end C ABI test for the tagged-union header list on a completion.
//
// Rust-side unit tests validate that `synthesize_response_headers` produces
// the expected `CosmosValue` variants, but they read the values through the
// Rust type. That path cannot catch a struct-layout, discriminant, or
// padding mismatch between the wrapper's Rust C-layout definition and
// cbindgen's generated C header. A .NET, Go, or Python binding that reads
// `header.value.kind` through the generated header and dispatches to the
// matching `header.value.payload.<leg>` walks the exact marshalling path
// this test exercises — if any of the following ever drift, this test
// fails immediately:
//
//   * the layout of `cosmos_response_header_t`
//     (offsets of `id`, `value.kind`, `value.payload`),
//   * the numeric encoding of each `cosmos_value_kind_t` discriminant,
//   * the ability to read each `cosmos_value_payload_t` union leg
//     (string / i64 / f64 / bool / u64) without alignment/padding UB.
//
// The Rust side exposes a `__test_only_`-prefixed enqueue helper that
// synthesizes a completion with exactly one header per `CosmosValueKind`
// discriminant so this file can walk it in a single pass. The symbol is
// excluded from the checked-in public header (via `build.rs`'s
// `export.exclude`); we forward-declare it locally so binding authors
// vendoring the header never see it.

#include "test_common.h"

#include <inttypes.h>
#include <stdbool.h>
#include <stdint.h>

// Test-only enqueue helper. Not part of the public ABI, forward-declared
// so the auto-discovered CMake target can link against it.
extern cosmos_error_code_t
__test_only_enqueue_ok_completion_with_all_value_kinds(cosmos_completion_queue_t *queue);

// Small helper: build a runtime + queue, or return non-zero on failure so
// the caller can SKIP cleanly (mirrors the pattern in `cancellation.c`).
static int make_runtime_and_cq(cosmos_runtime_t **out_runtime,
                               cosmos_completion_queue_t **out_cq)
{
    *out_runtime = NULL;
    *out_cq = NULL;

    cosmos_runtime_options_t opts = cosmos_runtime_options_default();
    opts.user_agent_suffix = "abi-headers-c-tests";

    cosmos_runtime_t *runtime = NULL;
    cosmos_error_t *err = NULL;
    int32_t rc = cosmos_runtime_build(&opts, &runtime, &err);
    if (rc != COSMOS_ERROR_CODE_SUCCESS || runtime == NULL) {
        cosmos_error_free(err);
        return 1;
    }

    cosmos_completion_queue_options_t queue_options = {
        .capacity_hint = 0,
        .max_capacity = 0,
        .include_error_details = true,
    };
    cosmos_completion_queue_t *cq = cosmos_completion_queue_create(runtime, &queue_options);
    if (cq == NULL) {
        cosmos_runtime_free(runtime);
        return 1;
    }

    *out_runtime = runtime;
    *out_cq = cq;
    return 0;
}

// Walks the header list of the synthesized completion and asserts every
// `CosmosValueKind` variant was produced and readable through the union.
// This is the whole point of the file — if the ABI ever drifts, exactly
// one of the case branches fails and pinpoints which variant broke.
static int test_completion_headers_dispatch_by_kind(void)
{
    int result = TEST_PASS;
    cosmos_runtime_t *runtime = NULL;
    cosmos_completion_queue_t *cq = NULL;
    cosmos_completion_t out;
    size_t drained = 0;
    int freed_completion = 0;

    if (make_runtime_and_cq(&runtime, &cq) != 0) {
        printf("    SKIP: could not build runtime/cq in this environment\n");
        return TEST_SKIP;
    }

    cosmos_error_code_t enq =
        __test_only_enqueue_ok_completion_with_all_value_kinds(cq);
    REQUIRE(enq == COSMOS_ERROR_CODE_SUCCESS,
            "enqueue helper succeeded (rc=%d)", (int)enq);

    // Drain the single completion (100ms is comfortably more than needed
    // since the enqueue is already resolved by the time we call wait).
    drained = cosmos_completion_queue_wait(cq, &out, 1, 100);
    REQUIRE(drained == 1, "wait drained 1 completion (got %zu)", drained);

    ASSERT(out.outcome == COSMOS_COMPLETION_OUTCOME_OK,
           "outcome == OK (got %d)", (int)out.outcome);
    ASSERT(out.http_status_code == 200,
           "http_status_code == 200 (got %u)", (unsigned)out.http_status_code);
    REQUIRE(out.headers != NULL, "headers list non-NULL");
    REQUIRE(out.headers_len == 5,
            "headers_len == 5 (got %zu)", (size_t)out.headers_len);

    // Track which variants we observed so a missing / duplicated variant
    // fails the test even if the individual asserts inside each arm pass.
    bool saw_string = false, saw_i64 = false, saw_f64 = false;
    bool saw_bool = false, saw_u64 = false;

    for (size_t i = 0; i < out.headers_len; i++) {
        const cosmos_response_header_t *h = &out.headers[i];
        switch (h->value.kind) {
            case COSMOS_VALUE_KIND_STRING: {
                ASSERT(h->id == COSMOS_HEADER_ID_ACTIVITY_ID,
                       "String leg carries ACTIVITY_ID id (got %d)", (int)h->id);
                REQUIRE(h->value.payload.string_value != NULL,
                        "string payload non-NULL");
                ASSERT(strcmp(h->value.payload.string_value, "abi-test-activity") == 0,
                       "string value == 'abi-test-activity' (got '%s')",
                       h->value.payload.string_value);
                saw_string = true;
                break;
            }
            case COSMOS_VALUE_KIND_I64: {
                ASSERT(h->id == COSMOS_HEADER_ID_ITEM_COUNT,
                       "I64 leg carries ITEM_COUNT id (got %d)", (int)h->id);
                ASSERT(h->value.payload.i64_value == 42,
                       "i64 value == 42 (got %" PRId64 ")",
                       h->value.payload.i64_value);
                saw_i64 = true;
                break;
            }
            case COSMOS_VALUE_KIND_F64: {
                ASSERT(h->id == COSMOS_HEADER_ID_SERVER_DURATION_MS,
                       "F64 leg carries SERVER_DURATION_MS id (got %d)", (int)h->id);
                ASSERT(h->value.payload.f64_value == 12.5,
                       "f64 value == 12.5 (got %f)", h->value.payload.f64_value);
                saw_f64 = true;
                break;
            }
            case COSMOS_VALUE_KIND_BOOL: {
                ASSERT(h->id == COSMOS_HEADER_ID_OFFER_REPLACE_PENDING,
                       "Bool leg carries OFFER_REPLACE_PENDING id (got %d)", (int)h->id);
                ASSERT(h->value.payload.bool_value == true,
                       "bool value == true");
                saw_bool = true;
                break;
            }
            case COSMOS_VALUE_KIND_U64: {
                ASSERT(h->id == COSMOS_HEADER_ID_LSN,
                       "U64 leg carries LSN id (got %d)", (int)h->id);
                // The Rust helper populates `lsn` with `u64::MAX - 1` so the
                // C-side read observes the full unsigned range — a saturated
                // `i64::MAX` read would land two orders of magnitude below.
                ASSERT(h->value.payload.u64_value == UINT64_MAX - 1,
                       "u64 value == UINT64_MAX-1 (got %" PRIu64 ")",
                       h->value.payload.u64_value);
                saw_u64 = true;
                break;
            }
            default:
                ASSERT(0, "unexpected value.kind %u", (unsigned)h->value.kind);
                break;
        }
    }

    ASSERT(saw_string, "String variant observed");
    ASSERT(saw_i64, "I64 variant observed");
    ASSERT(saw_f64, "F64 variant observed");
    ASSERT(saw_bool, "Bool variant observed");
    ASSERT(saw_u64, "U64 variant observed");

    cosmos_completion_queue_free_completions(&out, 1);
    freed_completion = 1;

cleanup:
    if (!freed_completion && drained == 1) {
        cosmos_completion_queue_free_completions(&out, 1);
    }
    if (cq != NULL) {
        cosmos_completion_queue_free(cq);
    }
    if (runtime != NULL) {
        cosmos_runtime_free(runtime);
    }
    return result;
}

TEST_SUITE_BEGIN("completion_headers_abi")
    TEST_REGISTER(completion_headers_dispatch_by_kind)
TEST_SUITE_END("completion_headers_abi")

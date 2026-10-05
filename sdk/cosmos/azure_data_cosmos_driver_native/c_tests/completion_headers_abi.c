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
#include <stddef.h>
#include <stdbool.h>
#include <stdint.h>

// Pin the `cosmos_completion_t` layout so an accidental field add/remove or a
// padding mistake (e.g. a host binding mirroring the struct) is caught at
// compile time instead of corrupting every pointer past the change. Offsets
// assume 64-bit pointers; the 32-bit ABI is not a shipping target.
#if UINTPTR_MAX == UINT64_MAX
_Static_assert(sizeof(cosmos_completion_t) == 112, "completion layout");
_Static_assert(offsetof(cosmos_completion_t, http_status_code) == 16, "http_status_code offset");
_Static_assert(offsetof(cosmos_completion_t, is_from_wire) == 18, "is_from_wire offset");
_Static_assert(offsetof(cosmos_completion_t, message) == 24, "message offset");
_Static_assert(offsetof(cosmos_completion_t, diagnostics) == 80, "diagnostics offset");
_Static_assert(offsetof(cosmos_completion_t, backing) == 104, "backing offset");
#endif

// Test-only enqueue helper. Not part of the public ABI, forward-declared
// so the auto-discovered CMake target can link against it.
extern cosmos_status_code_t
__test_only_enqueue_ok_completion_with_all_value_kinds(cosmos_completion_queue_t *queue);
extern cosmos_status_code_t __test_only_enqueue_ordered_item_page_fixture(
    cosmos_completion_queue_t *queue, uint8_t binary_enabled,
    cosmos_bytes_t *out_expected_page, const uint8_t **out_source_page,
    uintptr_t *out_item_offset, uintptr_t *out_item_len);

// Small helper: build a runtime + queue, or return non-zero on failure so
// the caller can SKIP cleanly (mirrors the pattern in `submit_and_response.c`).
static int make_runtime_and_cq(cosmos_runtime_t **out_runtime,
                               cosmos_completion_queue_t **out_cq)
{
    *out_runtime = NULL;
    *out_cq = NULL;

    cosmos_runtime_options_t opts = cosmos_runtime_options_default();
    opts.user_agent_suffix = SV("abi-headers-c-tests");

    cosmos_runtime_t *runtime = NULL;
    cosmos_error_t *err = NULL;
    int32_t rc = cosmos_runtime_build(&opts, &runtime, &err);
    if (rc != COSMOS_STATUS_SUCCESS || runtime == NULL) {
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

    cosmos_status_code_t enq =
        __test_only_enqueue_ok_completion_with_all_value_kinds(cq);
    REQUIRE(enq == COSMOS_STATUS_SUCCESS,
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

static int test_completion_item_page_accessors_reject_missing_items(void)
{
    int result = TEST_PASS;
    cosmos_completion_t completion = {0};
    const uint8_t *page = (const uint8_t *)(uintptr_t)1;
    size_t page_len = 1, offset = 1, item_len = 1;

    ASSERT(cosmos_completion_item_count(NULL) == 0, "NULL has no items");
    ASSERT(cosmos_completion_item_count(&completion) == 0, "empty completion has no items");
    cosmos_status_code_t status = cosmos_completion_item_page(
        &completion, 0, &page, &page_len, &offset, &item_len);
    ASSERT(status != COSMOS_STATUS_SUCCESS, "missing item returns an error");
    ASSERT(page == NULL && page_len == 0 && offset == 0 && item_len == 0,
           "failed lookup clears every output");
    status = cosmos_completion_item_page(&completion, 0, NULL, &page_len, &offset, &item_len);
    ASSERT(status != COSMOS_STATUS_SUCCESS, "NULL output returns an error");

    return result;
}

static int check_ordered_item_pages(bool binary)
{
    int result = TEST_PASS;
    cosmos_runtime_t *runtime = NULL;
    cosmos_completion_queue_t *cq = NULL;
    cosmos_completion_t out = {0};
    cosmos_bytes_t expected_page = {0};
    const uint8_t *original_page = NULL, *page = NULL, *second_page = NULL;
    uintptr_t expected_offset = 0, expected_item_len = 0;
    uintptr_t page_len = 0, offset = 0, item_len = 0;
    uintptr_t second_page_len = 0, second_offset = 0, second_item_len = 0;
    size_t drained = 0;
    bool live_completion = false;

    if (make_runtime_and_cq(&runtime, &cq) != 0) {
        printf("    SKIP: could not build runtime/cq in this environment\n");
        return TEST_SKIP;
    }
    cosmos_status_code_t status = __test_only_enqueue_ordered_item_page_fixture(
        cq, (uint8_t)binary, &expected_page, &original_page,
        &expected_offset, &expected_item_len);
    REQUIRE(status == COSMOS_STATUS_SUCCESS, "ordered item fixture enqueued (rc=%d)", (int)status);

    drained = cosmos_completion_queue_wait(cq, &out, 1, 100);
    live_completion = drained == 1;
    REQUIRE(live_completion, "drained ordered item completion");
    REQUIRE(out.outcome == COSMOS_COMPLETION_OUTCOME_OK, "ordered item outcome is OK");
    REQUIRE(cosmos_completion_item_count(&out) == 2, "two ordered items addressable");

    status = cosmos_completion_item_page(&out, 0, &page, &page_len, &offset, &item_len);
    REQUIRE(status == COSMOS_STATUS_SUCCESS, "first page accessor succeeds");
    REQUIRE(page != NULL && expected_page.ptr != NULL, "source and snapshot have bytes");
    REQUIRE(page == original_page, "page pointer is the original driver source");
    REQUIRE(page_len == expected_page.len && page_len > 0, "page length matches independent snapshot");
    ASSERT(memcmp(page, expected_page.ptr, page_len) == 0, "page bytes match original source");
    REQUIRE(offset <= page_len && item_len <= page_len - offset, "first absolute range fits page");
    ASSERT(offset == expected_offset && item_len == expected_item_len,
           "first absolute offset and length match driver item range");
    if (binary) {
        ASSERT(page[0] == 0x80 && offset > 0, "binary page retains preamble before item");
        REQUIRE(out.body != NULL && out.body_len > 0, "legacy body is materialized");
        ASSERT(out.body != page && out.body[0] == 0x80,
               "legacy binary body is standalone, not the original page");
    } else {
        ASSERT(page[0] == '{' && offset == 0 && item_len == page_len,
               "text item is its own zero-offset page");
        ASSERT(out.body == page && out.body_len == page_len, "legacy text body shares first item");
    }

    status = cosmos_completion_item_page(
        &out, 1, &second_page, &second_page_len, &second_offset, &second_item_len);
    REQUIRE(status == COSMOS_STATUS_SUCCESS && second_page != NULL && second_page_len > 0,
            "second item is addressable");
    REQUIRE(second_offset <= second_page_len &&
                second_item_len <= second_page_len - second_offset,
            "second absolute range fits its page");
    ASSERT(second_page[0] == (binary ? 0x80 : '{'), "second page has expected encoding");
    ASSERT(binary ? second_offset > 0 : (second_offset == 0 && second_item_len == second_page_len),
           "second item has expected range shape");

    cosmos_completion_queue_free(cq);
    cq = NULL;
    cosmos_runtime_free(runtime);
    runtime = NULL;
    ASSERT(cosmos_completion_item_count(&out) == 2, "items outlive queue and runtime");
    const uint8_t *retained_page = NULL;
    uintptr_t retained_len = 0, retained_offset = 0, retained_item_len = 0;
    status = cosmos_completion_item_page(
        &out, 0, &retained_page, &retained_len, &retained_offset, &retained_item_len);
    REQUIRE(status == COSMOS_STATUS_SUCCESS && retained_page == page,
            "source page pointer survives until completion free");
    ASSERT(retained_len == page_len && retained_offset == offset && retained_item_len == item_len &&
               memcmp(retained_page, expected_page.ptr, page_len) == 0,
           "source bytes and range survive queue/runtime free");

    cosmos_completion_queue_free_completions(&out, 1);
    live_completion = false;
    ASSERT(cosmos_completion_item_count(&out) == 0, "freed completion has no items");
    page = (const uint8_t *)(uintptr_t)1;
    page_len = offset = item_len = 1;
    status = cosmos_completion_item_page(&out, 0, &page, &page_len, &offset, &item_len);
    ASSERT(status != COSMOS_STATUS_SUCCESS && page == NULL &&
               page_len == 0 && offset == 0 && item_len == 0,
           "freed completion rejects access and clears outputs");

cleanup:
    if (live_completion) {
        cosmos_completion_queue_free_completions(&out, 1);
    }
    if (expected_page.ptr != NULL) {
        cosmos_bytes_free(expected_page);
    }
    if (cq != NULL) {
        cosmos_completion_queue_free(cq);
    }
    if (runtime != NULL) {
        cosmos_runtime_free(runtime);
    }
    return result;
}

static int test_completion_item_page_binary(void)
{
    return check_ordered_item_pages(true);
}

static int test_completion_item_page_text(void)
{
    return check_ordered_item_pages(false);
}

TEST_SUITE_BEGIN("completion_headers_abi")
    TEST_REGISTER(completion_headers_dispatch_by_kind)
    TEST_REGISTER(completion_item_page_accessors_reject_missing_items)
    TEST_REGISTER(completion_item_page_binary)
    TEST_REGISTER(completion_item_page_text)
TEST_SUITE_END("completion_headers_abi")

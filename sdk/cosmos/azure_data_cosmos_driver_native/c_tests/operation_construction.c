// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

// Operation request + options construction (flat C-ABI struct surface).
//
// The builder/factory entry points (`cosmos_operation_options_builder_*`,
// `cosmos_operation_*`) were removed in favor of two submit functions that
// consume a flat `cosmos_operation_request_t` plus a flat
// `cosmos_operation_options_t` (seeded by `cosmos_operation_options_default`).
// This harness covers the network-free contracts of that surface:
//
//   1. `cosmos_operation_options_default` returns an all-unset value.
//   2. A host-populated `cosmos_operation_request_t` is ABI-compatible and
//      its submit pre-flight rejects a NULL driver with INVALID_ARGUMENT.
//   3. `out_pre_error` is optional (NULL is accepted).

#include "test_common.h"

// ─────────────────────────────────────────────────────────────────────
// Section 1 — cosmos_operation_options_default sentinels
// ─────────────────────────────────────────────────────────────────────

static int test_options_default_is_all_unset(void)
{
    int result = TEST_PASS;
    cosmos_operation_options_t opts = cosmos_operation_options_default();

    ASSERT(opts.read_consistency_strategy == 0,
           "read_consistency unset (=%d)", opts.read_consistency_strategy);
    ASSERT(opts.content_response_on_write == 0,
           "content_response unset (=%d)", opts.content_response_on_write);
    ASSERT(opts.session_capturing_disabled == 0,
           "session_capturing unset (=%d)", opts.session_capturing_disabled);
    ASSERT(opts.max_failover_retry_count < 0,
           "max_failover unset (=%d)", opts.max_failover_retry_count);
    ASSERT(opts.max_session_retry_count < 0,
           "max_session unset (=%d)", opts.max_session_retry_count);
    ASSERT(opts.end_to_end_timeout_ms < 0,
           "e2e timeout unset (=%lld)", (long long)opts.end_to_end_timeout_ms);
    ASSERT(opts.endpoint_unavailability_ttl_ms < 0,
           "endpoint ttl unset (=%lld)", (long long)opts.endpoint_unavailability_ttl_ms);
    ASSERT(opts.throughput_control_group == NULL, "throughput_control_group unset");
    ASSERT(opts.excluded_regions == NULL, "excluded_regions unset");
    ASSERT(opts.excluded_regions_len == 0, "excluded_regions_len 0");
    ASSERT(opts.custom_headers == NULL, "custom_headers unset");
    ASSERT(opts.custom_headers_len == 0, "custom_headers_len 0");
    return result;
}

// ─────────────────────────────────────────────────────────────────────
// Section 2 — request population + submit pre-flight (NULL driver)
// ─────────────────────────────────────────────────────────────────────

static int test_singleton_submit_rejects_null_driver(void)
{
    int result = TEST_PASS;
    cosmos_operation_options_t opts = cosmos_operation_options_default();
    opts.end_to_end_timeout_ms = 5000;

    cosmos_operation_request_t req = {0};
    req.kind = COSMOS_OPERATION_KIND_READ_ITEM;
    req.item_id = "id-1";
    req.precondition_kind = COSMOS_PRECONDITION_KIND_NONE;
    req.max_item_count = -1; // unset
    req.options = &opts;

    cosmos_error_code_t err = COSMOS_ERROR_CODE_SUCCESS;
    cosmos_operation_handle_t *h =
        cosmos_submit_singleton_operation(NULL, &req, NULL, 0, &err);
    ASSERT(h == NULL, "singleton submit returns NULL on NULL driver");
    ASSERT(err == COSMOS_ERROR_CODE_INVALID_ARGUMENT,
           "sets INVALID_ARGUMENT (err=%d)", err);
    return result;
}

static int test_feed_submit_rejects_null_driver(void)
{
    int result = TEST_PASS;
    cosmos_operation_request_t req = {0};
    req.kind = COSMOS_OPERATION_KIND_QUERY_ITEMS;
    req.max_item_count = -1; // unset
    req.options = NULL;      // NULL options = inherit driver/runtime defaults

    cosmos_error_code_t err = COSMOS_ERROR_CODE_SUCCESS;
    cosmos_operation_handle_t *h =
        cosmos_submit_operation(NULL, &req, NULL, 0, &err);
    ASSERT(h == NULL, "feed submit returns NULL on NULL driver");
    ASSERT(err == COSMOS_ERROR_CODE_INVALID_ARGUMENT,
           "sets INVALID_ARGUMENT (err=%d)", err);
    return result;
}

static int test_submit_null_out_error_is_safe(void)
{
    int result = TEST_PASS;
    cosmos_operation_request_t req = {0};
    req.kind = COSMOS_OPERATION_KIND_READ_ITEM;
    req.max_item_count = -1;

    // out_pre_error may be NULL — must not crash.
    cosmos_operation_handle_t *h =
        cosmos_submit_singleton_operation(NULL, &req, NULL, 0, NULL);
    ASSERT(h == NULL, "submit returns NULL with NULL out_pre_error");
    return result;
}

TEST_SUITE_BEGIN("Operation request + options construction")
TEST_REGISTER(options_default_is_all_unset)
TEST_REGISTER(singleton_submit_rejects_null_driver)
TEST_REGISTER(feed_submit_rejects_null_driver)
TEST_REGISTER(submit_null_out_error_is_safe)
TEST_SUITE_END("Operation request + options construction")

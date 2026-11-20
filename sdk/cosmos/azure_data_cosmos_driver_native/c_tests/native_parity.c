// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

#include "test_common.h"
#include <stddef.h>

extern cosmos_status_code_t cosmos_test_cursor_fixture(
    uint32_t scripted, cosmos_runtime_t **runtime, cosmos_driver_t **driver);

_Static_assert(offsetof(cosmos_feed_options_t, struct_size_bytes) == 0, "feed size prefix");
_Static_assert(offsetof(cosmos_feed_options_t, abi_version) == 4, "feed version prefix");
_Static_assert(offsetof(cosmos_runtime_options_v2_t, abi_version) == 4, "runtime version prefix");
_Static_assert(offsetof(cosmos_driver_options_config_v3_t, abi_version) == 4, "client version prefix");
_Static_assert(offsetof(cosmos_management_options_t, struct_size_bytes) == 0, "management size prefix");
_Static_assert(offsetof(cosmos_management_options_t, abi_version) == 4, "management version prefix");

static int body_contains(const cosmos_completion_t *completion, const char *text) {
    size_t len = strlen(text);
    for (size_t i = 0; i + len <= completion->body_len; ++i) {
        if (!memcmp(completion->body + i, text, len)) return 1;
    }
    return 0;
}

static int test_typed_management_headers(void) {
    int result = TEST_PASS;
    cosmos_runtime_t *runtime = NULL;
    cosmos_driver_t *driver = NULL;
    cosmos_account_ref_t *account = NULL;
    cosmos_completion_queue_t *queue = NULL;
    cosmos_operation_options_snapshot_t *snapshot = NULL;
    cosmos_operation_handle_t *op = NULL;
    cosmos_completion_t completion = {0};
    int has_completion = 0;
    cosmos_status_code_t status = 0;
    REQUIRE(!cosmos_test_cursor_fixture(1, &runtime, &driver), "create management wire fixture");
    REQUIRE(!cosmos_account_ref_with_master_key(SV("https://eastus.emulator.local"),
        SV("ZW11bGF0b3Ita2V5"), &account, NULL), "create account reference");
    queue = cosmos_completion_queue_create(runtime, NULL);
    REQUIRE(queue, "create management completion queue");
    cosmos_header_kv_t runtime_headers[] = {
        {SV("x-test-inherited"), SV("runtime")},
        {SV("x-ms-offer-throughput"), SV("111")}
    };
    cosmos_header_kv_t client_headers[] = {
        {SV("x-test-inherited"), SV("client")},
        {SV("x-ms-offer-throughput"), SV("222")}
    };
    cosmos_operation_options_t defaults = cosmos_operation_options_default();
    defaults.binary_encoding_enabled = 1;
    defaults.custom_headers = runtime_headers;
    defaults.custom_headers_len = 2;
    REQUIRE(!cosmos_runtime_set_operation_options(runtime, &defaults), "set inherited runtime map");
    cosmos_operation_options_t client = cosmos_operation_options_default();
    client.custom_headers = client_headers;
    client.custom_headers_len = 2;
    for (int use_client = 0; use_client < 2; ++use_client) {
        for (int mode = 0; mode < 6; ++mode) {
            int inherited_autoscale = mode == 2 || mode == 4;
            runtime_headers[1] = (cosmos_header_kv_t){
                inherited_autoscale ? SV("x-ms-cosmos-offer-autopilot-settings") : SV("x-ms-offer-throughput"),
                inherited_autoscale ? SV("{\"maxThroughput\":1111}") : SV("111")};
            client_headers[1] = (cosmos_header_kv_t){runtime_headers[1].name,
                inherited_autoscale ? SV("{\"maxThroughput\":2222}") : SV("222")};
            REQUIRE(!cosmos_runtime_set_operation_options(runtime, &defaults), "update inherited map");
            int64_t timeout = -2;
            REQUIRE(!cosmos_operation_options_snapshot_create(runtime, use_client ? &client : NULL,
                NULL, &snapshot, &timeout), "capture native inheritance");
            cosmos_cursor_request_t request;
            cosmos_cursor_request_init(&request);
            request.operation.kind = 1;
            request.operation.account = account;
            request.operation.options_snapshot = snapshot;
            request.operation.body = (const uint8_t *)"{\"id\":\"created\"}";
            request.operation.body_len = strlen((const char *)request.operation.body);
            cosmos_management_options_t management;
            cosmos_management_options_init(&management);
            if (mode == 1 || mode == 3 || mode == 4) {
                management.throughput_kind = 1;
                management.manual_throughput = 400;
            } else if (mode == 2 || mode == 5) {
                management.throughput_kind = 2;
                management.autoscale_max_throughput = 4000;
                management.has_auto_upgrade_policy = 1;
                management.has_throughput_policy = 1;
                management.increment_percent = 25;
            }
            if (mode == 3) cosmos_management_options_init(&management);
            status = 0;
            op = cosmos_submit_singleton_operation_v2(driver, &request.operation, &management,
                queue, 99, &status);
            if (mode >= 4) {
                ASSERT(!op && status != 0, "opposite-mode inherited headers reject before dispatch");
                cosmos_operation_options_snapshot_free(snapshot);
                snapshot = NULL;
                continue;
            }
            REQUIRE(op && !status, "submit typed or unset throughput");
            cosmos_management_options_init(&management);
            cosmos_operation_options_snapshot_free(snapshot);
            snapshot = NULL;
            REQUIRE(cosmos_completion_queue_wait(queue, &completion, 1, 10000) == 1,
                "receive actual wire response");
            has_completion = 1;
            REQUIRE(!completion.status && completion.user_data == 99, "management succeeded");
            ASSERT(body_contains(&completion, use_client ?
                "\"inherited\":\"client\"" : "\"inherited\":\"runtime\""),
                "native-resolved inherited headers survive");
            if (mode == 1) {
                ASSERT(body_contains(&completion, "\"throughput\":\"400\""),
                    "typed manual wins conflicting inherited header");
                ASSERT(body_contains(&completion, "\"autoscale\":null"), "no opposite autoscale header");
            } else if (mode == 2) {
                ASSERT(body_contains(&completion, "incrementPercent\\\":25"),
                    "complete autoscale upgrade policy reaches wire");
                ASSERT(body_contains(&completion, "maxThroughput\\\":4000"),
                    "typed autoscale wins conflicting inherited header");
                ASSERT(body_contains(&completion, "\"throughput\":null"), "no opposite manual header");
            } else {
                ASSERT(body_contains(&completion, use_client ?
                    "\"throughput\":\"222\"" : "\"throughput\":\"111\""),
                    "init/unset/reset preserves inherited throughput");
            }
            cosmos_completion_queue_free_completions(&completion, 1);
            has_completion = 0;
            cosmos_operation_handle_free(op);
            op = NULL;
        }
    }
    cosmos_management_options_t invalid;
    cosmos_management_options_init(&invalid);
    invalid.throughput_kind = 1;
    invalid.manual_throughput = 400;
    invalid.autoscale_max_throughput = 4000;
    ASSERT(!cosmos_submit_singleton_operation_v2(driver, NULL, &invalid, queue, 0, &status)
        && status != 0, "manual/autoscale conflict rejects before async submission");
cleanup:
    if (has_completion) cosmos_completion_queue_free_completions(&completion, 1);
    cosmos_operation_options_snapshot_free(snapshot);
    cosmos_operation_handle_free(op);
    cosmos_completion_queue_free(queue);
    cosmos_account_ref_free(account);
    cosmos_driver_free(driver);
    cosmos_runtime_free(runtime);
    return result;
}

static int test_range_ownership(void) {
    int result = TEST_PASS;
    cosmos_feed_range_t *range = NULL, *parsed = NULL, *full = NULL;
    char *text = NULL, *min = NULL, *max = NULL;
    uint8_t comparison = 0;
    REQUIRE(!cosmos_feed_range_from_epk(SV("10"), SV("80"), &range), "construct EPK range");
    REQUIRE(!cosmos_feed_range_bounds(range, &min, &max), "copy bounds");
    REQUIRE(!cosmos_feed_range_serialize(range, &text), "serialize range");
    cosmos_feed_range_free(range);
    range = NULL;
    ASSERT(!strcmp(min, "10") && !strcmp(max, "80"), "bounds outlive source handle");
    REQUIRE(!cosmos_feed_range_parse(string_view(text), &parsed), "parse owned text");
    cosmos_string_free(text);
    text = NULL;
    REQUIRE(!cosmos_feed_range_full(&full), "full range");
    REQUIRE(!cosmos_feed_range_is_subset_of(parsed, full, &comparison), "compare independent handles");
    ASSERT(comparison == 1, "parsed range remains a subset");
    REQUIRE(!cosmos_feed_range_is_logical_partition(parsed, &comparison), "inspect parsed semantics");
    ASSERT(comparison == 0, "parsed range has EPK semantics");
    ASSERT(cosmos_feed_range_from_epk(SV("80"), SV("10"), &range) != 0 && !range,
        "inverted range rejects without creating a handle");
cleanup:
    cosmos_string_free(text);
    cosmos_string_free(min);
    cosmos_string_free(max);
    cosmos_feed_range_free(range);
    cosmos_feed_range_free(parsed);
    cosmos_feed_range_free(full);
    return result;
}

static int test_discovery_and_additive_cursor(void) {
    int result = TEST_PASS;
    cosmos_runtime_t *runtime = NULL;
    cosmos_driver_t *driver = NULL;
    cosmos_container_ref_t *container = NULL;
    cosmos_completion_queue_t *queue = NULL, *cursor_queue = NULL;
    cosmos_feed_range_list_t *list = NULL;
    cosmos_feed_range_t *range = NULL;
    cosmos_operation_handle_t *op = NULL;
    cosmos_completion_t completion = {0};
    int has_completion = 0;
    cosmos_cursor_completion_t *opened = NULL;
    cosmos_cursor_t *cursor = NULL;
    cosmos_status_code_t status = 0;
    REQUIRE(!cosmos_test_cursor_fixture(1, &runtime, &driver), "create native fixture");
    REQUIRE(!cosmos_driver_resolve_container_blocking(runtime, driver, SV("testdb"),
        SV("testcoll"), &container, NULL), "resolve container");
    queue = cosmos_completion_queue_create(runtime, NULL);
    REQUIRE(queue, "create discovery queue");
    op = cosmos_container_read_feed_ranges_submit(driver, container, 1, queue, 42, &status);
    REQUIRE(op && !status, "submit forced discovery");
    REQUIRE(cosmos_completion_queue_wait(queue, &completion, 1, 10000) == 1, "receive discovery");
    has_completion = 1;
    REQUIRE(!completion.status && completion.user_data == 42, "status and correlation preserved");
    list = cosmos_completion_take_feed_ranges(&completion);
    ASSERT(!cosmos_completion_take_feed_ranges(&completion), "list detaches exactly once");
    cosmos_completion_queue_free_completions(&completion, 1);
    has_completion = 0;
    REQUIRE(cosmos_feed_range_list_len(list) == 2, "detached list outlives completion");
    REQUIRE(!cosmos_feed_range_list_get(list, 0, &range), "copy independent range handle");
    cosmos_feed_range_list_free(list);
    list = NULL;
    cosmos_operation_handle_free(op);
    op = NULL;
    cursor_queue = cosmos_cursor_queue_create(runtime, 2);
    cosmos_cursor_request_t request;
    cosmos_cursor_request_init(&request);
    cosmos_feed_options_t options;
    cosmos_feed_options_init(&options);
    options.has_max_buffered_query_window = 1;
    options.max_buffered_query_window = 6;
    options.max_item_count_kind = 1;
    request.operation.kind = 17;
    request.operation.container = container;
    request.operation.feed_range = range;
    const char *sql = "{\"query\":\"SELECT TOP 6 * FROM c ORDER BY c.rank\"}";
    request.operation.body = (const uint8_t *)sql;
    request.operation.body_len = strlen(sql);
    cosmos_operation_options_t legacy = cosmos_operation_options_default();
    legacy.query_plan_mode = 2;
    request.operation.options = &legacy;
    op = cosmos_cursor_open_submit_v2(driver, &request, &options, cursor_queue, 43, &status);
    REQUIRE(op && !status, "submit additive retained query");
    cosmos_feed_range_free(range);
    range = NULL;
    size_t count = 0;
    REQUIRE(!cosmos_cursor_queue_wait(cursor_queue, &opened, 1, 10000, &count) && count == 1,
        "receive retained query after input-handle free");
    REQUIRE(!opened->common.status && opened->result_kind == 1, "query opened");
    cursor = cosmos_cursor_completion_take_cursor(opened);
cleanup:
    cosmos_cursor_free(cursor);
    cosmos_cursor_completion_free(opened);
    if (has_completion) cosmos_completion_queue_free_completions(&completion, 1);
    cosmos_operation_handle_free(op);
    cosmos_feed_range_free(range);
    cosmos_feed_range_list_free(list);
    cosmos_completion_queue_free(cursor_queue);
    cosmos_completion_queue_free(queue);
    cosmos_container_ref_free(container);
    cosmos_driver_free(driver);
    cosmos_runtime_free(runtime);
    return result;
}

static int test_versioned_runtime_validation(void) {
    int result = TEST_PASS;
    cosmos_runtime_t *runtime = NULL;
    cosmos_runtime_options_v2_t options;
    cosmos_runtime_options_v2_init(&options);
    cosmos_connection_pool_options_t pool;
    cosmos_connection_pool_options_init(&pool);
    cosmos_operation_options_v2_t operation;
    cosmos_operation_options_v2_init(&operation);
    options.connection_pool = &pool;
    options.operation_options = &operation;
    operation.hedge_threshold.is_set = 1;
    operation.hedge_threshold.nanoseconds = 1;
    REQUIRE(!cosmos_runtime_build_v2(&options, &runtime, NULL), "build exact nanosecond defaults");
    cosmos_runtime_free(runtime);
    runtime = NULL;
    pool.proxy_allowed = 9;
    ASSERT(cosmos_runtime_build_v2(&options, &runtime, NULL) != 0 && !runtime,
        "malformed tri-state rejects without runtime");
    pool.proxy_allowed = 0;
    operation.hedge_threshold.nanoseconds = 1000000000;
    ASSERT(cosmos_runtime_build_v2(&options, &runtime, NULL) != 0 && !runtime,
        "unnormalized duration rejects");
cleanup:
    cosmos_runtime_free(runtime);
    return result;
}

TEST_SUITE_BEGIN("native_parity")
    TEST_REGISTER(typed_management_headers)
    TEST_REGISTER(range_ownership)
    TEST_REGISTER(discovery_and_additive_cursor)
    TEST_REGISTER(versioned_runtime_validation)
TEST_SUITE_END("native_parity")

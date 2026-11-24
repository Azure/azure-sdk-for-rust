// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

#include "test_common.h"
#include <stddef.h>

/* Only linked by the test-abi build; deliberately absent from the public header. */
extern cosmos_status_code_t cosmos_test_cursor_fixture(
    uint32_t scripted, cosmos_runtime_t **runtime, cosmos_driver_t **driver);

static int test_range_ownership(void) {
    int result = TEST_PASS;
    cosmos_feed_range_t *range = NULL, *parsed = NULL, *full = NULL;
    char *text = NULL;
    cosmos_feed_range_info_t info = {0};
    cosmos_feed_range_relation_t relation = {0};
    REQUIRE(!cosmos_feed_range_from_epk(SV("10"), SV("80"), &range), "construct EPK range");
    REQUIRE(!cosmos_feed_range_inspect(range, &info), "inspect range");
    REQUIRE(!cosmos_feed_range_serialize(range, &text), "serialize range");
    cosmos_feed_range_free(range);
    range = NULL;
    ASSERT(!strcmp(info.min_inclusive, "10") && !strcmp(info.max_exclusive, "80"),
        "bounds outlive source handle");
    ASSERT(info.is_logical_partition == 0, "EPK range has no logical identity");
    ASSERT(!strcmp(info.serialized, text), "inspection includes canonical serialization");
    REQUIRE(!cosmos_feed_range_parse(string_view(text), &parsed), "parse owned text");
    cosmos_string_free(text);
    text = NULL;
    REQUIRE(!cosmos_feed_range_full(&full), "full range");
    REQUIRE(!cosmos_feed_range_relate(parsed, full, &relation), "compare independent handles");
    ASSERT(relation.is_subset == 1 && relation.overlaps == 1, "combined relation returns both results");
    ASSERT(cosmos_feed_range_from_epk(SV("80"), SV("10"), &range) != 0 && !range,
        "inverted range rejects without creating a handle");
cleanup:
    cosmos_string_free(text);
    cosmos_string_free(info.min_inclusive);
    cosmos_string_free(info.max_exclusive);
    cosmos_string_free(info.serialized);
    cosmos_feed_range_free(range);
    cosmos_feed_range_free(parsed);
    cosmos_feed_range_free(full);
    return result;
}

static int test_discovery_and_scoped_cursor(void) {
    int result = TEST_PASS;
    cosmos_runtime_t *runtime = NULL;
    cosmos_driver_t *driver = NULL;
    cosmos_container_ref_t *container = NULL;
    cosmos_completion_queue_t *queue = NULL, *cursor_queue = NULL;
    cosmos_feed_range_list_t *list = NULL;
    cosmos_feed_range_t *range = NULL;
    char *min = NULL, *max = NULL, *serialized = NULL;
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
    REQUIRE(list && list->count == 2 && list->ranges, "detached flat list outlives completion");
    ASSERT(!strcmp(list->ranges[0].min_inclusive, "") &&
        !strcmp(list->ranges[0].max_exclusive, "80") &&
        !strcmp(list->ranges[1].min_inclusive, "80") &&
        !strcmp(list->ranges[1].max_exclusive, "FF"), "contiguous entries preserve exact bounds");
    ASSERT(!list->ranges[0].is_logical_partition && !list->ranges[1].is_logical_partition,
        "discovered ranges have EPK identity");
    min = malloc(strlen(list->ranges[0].min_inclusive) + 1);
    max = malloc(strlen(list->ranges[0].max_exclusive) + 1);
    serialized = malloc(strlen(list->ranges[0].serialized) + 1);
    REQUIRE(min && max && serialized, "allocate host-owned copies");
    strcpy(min, list->ranges[0].min_inclusive);
    strcpy(max, list->ranges[0].max_exclusive);
    strcpy(serialized, list->ranges[0].serialized);
    cosmos_feed_range_list_free(list);
    list = NULL;
    ASSERT(!strcmp(min, "") && !strcmp(max, "80"), "host strings outlive list");
    REQUIRE(!cosmos_feed_range_parse(string_view(serialized), &range), "construct scoped handle from host copy");
    cosmos_operation_handle_free(op);
    op = NULL;
    cursor_queue = cosmos_cursor_queue_create(runtime, 2);
    REQUIRE(cursor_queue, "create cursor queue");
    ASSERT(!cosmos_container_read_feed_ranges_submit(driver, container, 0, cursor_queue, 0, &status)
        && status != 0, "discovery rejects cursor-format queues");
    cosmos_cursor_request_t request;
    cosmos_cursor_request_init(&request);
    request.operation.kind = 17;
    request.operation.container = container;
    request.operation.feed_range = range;
    const char *sql = "{\"query\":\"SELECT TOP 6 * FROM c ORDER BY c.rank\"}";
    request.operation.body = (const uint8_t *)sql;
    request.operation.body_len = strlen(sql);
    cosmos_operation_options_t options = cosmos_operation_options_default();
    options.query_plan_mode = 2;
    request.operation.options = &options;
    op = cosmos_cursor_open_submit(driver, &request, cursor_queue, 43, &status);
    REQUIRE(op && !status, "submit scoped query");
    cosmos_feed_range_free(range);
    range = NULL;
    size_t count = 0;
    REQUIRE(!cosmos_cursor_queue_wait(cursor_queue, &opened, 1, 10000, &count) && count == 1,
        "receive scoped query after input-handle free");
    REQUIRE(!opened->common.status && opened->result_kind == 1, "query opened");
    cursor = cosmos_cursor_completion_take_cursor(opened);
cleanup:
    cosmos_cursor_free(cursor);
    cosmos_cursor_completion_free(opened);
    if (has_completion) cosmos_completion_queue_free_completions(&completion, 1);
    cosmos_operation_handle_free(op);
    cosmos_feed_range_free(range);
    cosmos_feed_range_list_free(list);
    free(min);
    free(max);
    free(serialized);
    cosmos_completion_queue_free(cursor_queue);
    cosmos_completion_queue_free(queue);
    cosmos_container_ref_free(container);
    cosmos_driver_free(driver);
    cosmos_runtime_free(runtime);
    return result;
}

TEST_SUITE_BEGIN("feed_ranges")
    TEST_REGISTER(range_ownership)
    TEST_REGISTER(discovery_and_scoped_cursor)
TEST_SUITE_END("feed_ranges")

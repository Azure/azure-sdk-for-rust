// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

#include "test_common.h"
#include <stddef.h>

/* Only linked by the test-abi build; deliberately absent from the public header. */
extern cosmos_status_code_t cosmos_test_cursor_fixture(
    uint32_t scripted, cosmos_runtime_t **runtime, cosmos_driver_t **driver);

_Static_assert(offsetof(cosmos_cursor_request_t, struct_size_bytes) == 0, "size prefix");
_Static_assert(offsetof(cosmos_cursor_request_t, abi_version) == 4, "version prefix");
_Static_assert(offsetof(cosmos_cursor_completion_t, abi_version) == 4, "result prefix");

typedef struct {
    cosmos_runtime_t *runtime;
    cosmos_driver_t *driver;
    cosmos_container_ref_t *container;
    cosmos_account_ref_t *account;
    cosmos_database_ref_t *database;
    cosmos_completion_queue_t *queue;
} fixture;

static void fixture_free(fixture *f) {
    cosmos_completion_queue_free(f->queue);
    cosmos_container_ref_free(f->container);
    cosmos_database_ref_free(f->database);
    cosmos_account_ref_free(f->account);
    cosmos_driver_free(f->driver);
    cosmos_runtime_free(f->runtime);
}

static int fixture_create(fixture *f, uint32_t scripted) {
    memset(f, 0, sizeof(*f));
    if (cosmos_test_cursor_fixture(scripted, &f->runtime, &f->driver)) return 0;
    if (cosmos_driver_resolve_container_blocking(f->runtime, f->driver,
            SV("testdb"), SV("testcoll"), &f->container, NULL)) return 0;
    if (cosmos_account_ref_with_master_key(SV("https://eastus.emulator.local"),
            SV("ZW11bGF0b3Ita2V5"), &f->account, NULL)) return 0;
    if (cosmos_database_ref_create(f->account, SV("testdb"), &f->database)) return 0;
    f->queue = cosmos_cursor_queue_create(f->runtime, 2);
    return f->queue != NULL;
}

static cosmos_cursor_completion_t *receive(fixture *f, cosmos_operation_handle_t *op) {
    cosmos_cursor_completion_t *completion = NULL;
    size_t count = 0;
    if (!op) return NULL;
    cosmos_status_code_t status = cosmos_cursor_queue_wait(f->queue, &completion, 1, 10000, &count);
    cosmos_operation_handle_free(op);
    if (status || count != 1) return NULL;
    return completion;
}

static cosmos_cursor_t *open_cursor(fixture *f, cosmos_cursor_request_t *request) {
    cosmos_status_code_t status = 0;
    cosmos_cursor_completion_t *completion = receive(f,
        cosmos_cursor_open_submit(f->driver, request, f->queue, 123, &status));
    if (!completion) return NULL;
    cosmos_cursor_t *cursor = NULL;
    if (!completion->common.status && completion->result_kind == 1 &&
        completion->common.user_data == 123) {
        cursor = cosmos_cursor_completion_take_cursor(completion);
    } else {
        fprintf(stderr, "Open error: %d %s\n", completion->common.status,
            completion->common.message ? completion->common.message : "");
    }
    cosmos_cursor_completion_free(completion);
    return cursor;
}

static char *text_copy(const uint8_t *bytes, size_t len) {
    char *text = malloc(len + 1);
    if (!text) abort();
    if (len) memcpy(text, bytes, len);
    text[len] = '\0';
    return text;
}

static size_t ranks_from_bytes(const uint8_t *bytes, size_t len, int *ranks, size_t count) {
    char *text = text_copy(bytes, len);
    const char *next = text;
    while ((next = strstr(next, "\"rank\":")) != NULL) {
        if (count >= 32) abort();
        ranks[count++] = atoi(next + 7);
        next += 7;
    }
    free(text);
    return count;
}

static int query_scenario(const char *query, int page_size, int distinct, int scripted) {
    int result = TEST_PASS;
    fixture f = {0};
    cosmos_cursor_t *cursor = NULL;
    cosmos_cursor_completion_t *page = NULL, *held = NULL, *checkpoint = NULL;
    char *saved = NULL;
    int ranks[32] = {0};
    size_t count = 0;
    int unsupported = 0, ended = 0;
    REQUIRE(fixture_create(&f, scripted), "populated two-range fixture");
    cosmos_cursor_request_t request;
    cosmos_cursor_request_init(&request);
    request.operation.kind = 17;
    request.operation.container = f.container;
    request.operation.max_item_count = page_size;
    cosmos_operation_options_t options = cosmos_operation_options_default();
    options.binary_encoding_enabled = 1;
    options.query_plan_mode = 2;
    request.operation.options = &options;
    char body[256];
    snprintf(body, sizeof(body), "{\"query\":\"%s\"}", query);
    request.operation.body = (const uint8_t *)body;
    request.operation.body_len = strlen(body);
    cursor = open_cursor(&f, &request);
    REQUIRE(cursor, "opened query without consuming page");
    for (int i = 0; i < 32; ++i) {
        page = receive(&f, cosmos_cursor_next_submit(cursor, 456, NULL));
        REQUIRE(page, "Next delivered");
        REQUIRE(!page->common.status, "successful page (%s)",
            page->common.message ? page->common.message : "");
        ASSERT(page->common.user_data == 456, "correlation preserved");
        if (page->result_kind == 4) { ended = 1; break; }
        REQUIRE(page->result_kind == 2, "explicit Page");
        if (page->body_kind == 2) {
            for (size_t j = 0; j < page->items_len; ++j) {
                if (distinct) {
                    char *value = text_copy(page->items[j].data, page->items[j].len);
                    REQUIRE(count < 32, "bounded test output");
                    ranks[count++] = atoi(value);
                    free(value);
                } else {
                    count = ranks_from_bytes(page->items[j].data, page->items[j].len, ranks, count);
                }
            }
        } else if (page->body_kind == 1) {
            count = ranks_from_bytes(page->common.body, page->common.body_len, ranks, count);
        }
        checkpoint = receive(&f, cosmos_cursor_checkpoint_submit(cursor, 0, NULL));
        REQUIRE(checkpoint, "checkpoint delivered");
        if (checkpoint->common.status) {
            unsigned sub = COSMOS_STATUS_SUB(checkpoint->common.status);
            ASSERT(sub == (scripted ? 20125u : 20124u), "original unsupported checkpoint status");
            unsupported = 1;
        }
        cosmos_cursor_completion_free(checkpoint); checkpoint = NULL;
        if (!held && page->body_kind == 2 && page->items_len) {
            held = page;
            saved = text_copy(held->items[0].data, held->items[0].len);
        } else cosmos_cursor_completion_free(page);
        page = NULL;
    }
    REQUIRE(ended, "finite feed explicitly exhausted");
    ASSERT(count == (distinct ? 3u : 6u), "all rows delivered: %zu", count);
    if (strstr(query, "ORDER BY")) {
        for (size_t i = 0; i < count; ++i) ASSERT(ranks[i] == (int)i, "global order %zu", i);
    } else {
        unsigned mask = 0;
        for (size_t i = 0; i < count; ++i) mask |= 1u << ranks[i];
        ASSERT(mask == (distinct ? 7u : 63u), "exact populated output");
    }
    if (distinct || scripted) ASSERT(unsupported, "unsupported checkpoint did not prevent draining");
    cosmos_cursor_completion_free(page); page = NULL;
    page = receive(&f, cosmos_cursor_next_submit(cursor, 0, NULL));
    REQUIRE(page && page->result_kind == 4, "repeat End");
    cosmos_cursor_free(cursor); cursor = NULL;
    if (held) ASSERT(memcmp(saved, held->items[0].data, held->items[0].len) == 0,
        "previous page survives prefetch and cursor free");
cleanup:
    free(saved);
    cosmos_cursor_completion_free(checkpoint);
    cosmos_cursor_completion_free(page);
    cosmos_cursor_completion_free(held);
    cosmos_cursor_free(cursor);
    fixture_free(&f);
    return result;
}

static int test_populated_queries(void) {
    for (int size = 1; size <= 2; ++size) {
        if (query_scenario("SELECT * FROM c", size, 0, 0)) return TEST_FAIL;
        if (query_scenario("SELECT * FROM c ORDER BY c.rank", size, 0, 0)) return TEST_FAIL;
        if (query_scenario("SELECT DISTINCT VALUE c.group FROM c", size, 1, 0)) return TEST_FAIL;
        if (query_scenario("SELECT TOP 6 * FROM c ORDER BY c.rank", size, 0, 1)) return TEST_FAIL;
    }
    return TEST_PASS;
}

static int test_read_feeds(void) {
    int result = TEST_PASS;
    fixture f = {0};
    cosmos_cursor_t *cursor = NULL;
    cosmos_cursor_completion_t *page = NULL, *checkpoint = NULL;
    REQUIRE(fixture_create(&f, 0), "fixture");
    const int kinds[] = {2, 10, 15};
    cosmos_partition_key_component_t pk = {0};
    pk.kind = COSMOS_PARTITION_KEY_COMPONENT_KIND_STRING;
    pk.value.string_value = (cosmos_string_view_t){(const uint8_t *)"pk-0", 4};
    for (size_t k = 0; k < 3; ++k) {
        cosmos_cursor_request_t request;
        cosmos_cursor_request_init(&request);
        request.operation.kind = kinds[k];
        request.operation.account = f.account;
        request.operation.database = f.database;
        request.operation.container = f.container;
        request.operation.max_item_count = 1;
        if (kinds[k] == 15) {
            request.operation.partition_key_components = &pk;
            request.operation.partition_key_len = 1;
        }
        cursor = open_cursor(&f, &request);
        REQUIRE(cursor, "read feed open kind %d", kinds[k]);
        int pages = 0, ended = 0;
        for (int i = 0; i < 32; ++i) {
            page = receive(&f, cosmos_cursor_next_submit(cursor, 0, NULL));
            REQUIRE(page && !page->common.status, "read feed page");
            if (page->result_kind == 4) { ended = 1; break; }
            REQUIRE(page->result_kind == 2, "read feed Page");
            pages++;
            cosmos_cursor_completion_free(page); page = NULL;
            checkpoint = receive(&f, cosmos_cursor_checkpoint_submit(cursor, 0, NULL));
            REQUIRE(checkpoint, "read feed checkpoint explicit result");
            ASSERT(COSMOS_STATUS_SUB(checkpoint->common.status) == 20117,
                "driver read-feed checkpoint unsupported, still pageable");
            cosmos_cursor_completion_free(checkpoint); checkpoint = NULL;
        }
        REQUIRE(ended && pages > 0, "read feed ends after pages");
        cosmos_cursor_completion_free(page); page = NULL;
        cosmos_cursor_free(cursor); cursor = NULL;
    }
cleanup:
    cosmos_cursor_completion_free(checkpoint);
    cosmos_cursor_completion_free(page);
    cosmos_cursor_free(cursor);
    fixture_free(&f);
    return result;
}

static int test_change_feeds(void) {
    int result = TEST_PASS;
    fixture f = {0};
    cosmos_cursor_t *cursor = NULL, *resumed = NULL;
    cosmos_cursor_completion_t *page = NULL, *checkpoint = NULL;
    REQUIRE(fixture_create(&f, 1), "scripted driver transport fixture");
    for (unsigned mode = 1; mode <= 2; ++mode) {
        cosmos_cursor_request_t request;
        cosmos_cursor_request_init(&request);
        request.change_feed_mode = mode;
        request.start_from = 2;
        request.operation.container = f.container;
        cursor = open_cursor(&f, &request);
        REQUIRE(cursor, "change feed open mode %u", mode);
        int idle = 0, populated = 0;
        for (int i = 0; i < 6; ++i) {
            page = receive(&f, cosmos_cursor_next_submit(cursor, 0, NULL));
            REQUIRE(page && !page->common.status && page->result_kind == 2, "change feed Page, never End");
            if (page->common.http_status_code == 304) idle++;
            else {
                REQUIRE(page->body_kind == 1, "whole raw change envelope");
                char *text = text_copy(page->common.body, page->common.body_len);
                ASSERT(strstr(text, "\"d0\"") && strstr(text, "\"d1\""), "both changes intact");
                if (mode == 2) ASSERT(strstr(text, "\"previous\"") && strstr(text, "\"metadata\""), "AVAD previous and metadata");
                free(text);
                populated++;
            }
            cosmos_cursor_completion_free(page); page = NULL;
        }
        ASSERT(idle && populated, "idle 304 and later changes");
        checkpoint = receive(&f, cosmos_cursor_checkpoint_submit(cursor, 0, NULL));
        REQUIRE(checkpoint && !checkpoint->common.status && checkpoint->checkpoint.len, "composite checkpoint");
        request.operation.continuation_token = checkpoint->checkpoint;
        request.start_from = 0;
        resumed = open_cursor(&f, &request);
        REQUIRE(resumed, "resume token owns position with unset start");
        cosmos_cursor_completion_free(checkpoint); checkpoint = NULL;
        page = receive(&f, cosmos_cursor_next_submit(resumed, 0, NULL));
        REQUIRE(page && page->result_kind == 2 && page->common.http_status_code == 304, "resume at idle position");
        cosmos_cursor_completion_free(page); page = NULL;
        cosmos_cursor_free(cursor); cursor = NULL;
        cosmos_cursor_free(resumed); resumed = NULL;
    }
cleanup:
    cosmos_cursor_completion_free(checkpoint);
    cosmos_cursor_completion_free(page);
    cosmos_cursor_free(cursor);
    cosmos_cursor_free(resumed);
    fixture_free(&f);
    return result;
}

TEST_SUITE_BEGIN("feed_cursor")
TEST_REGISTER(populated_queries)
TEST_REGISTER(read_feeds)
TEST_REGISTER(change_feeds)
TEST_SUITE_END("feed_cursor")

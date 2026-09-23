// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

// End-to-end linked-C coverage for native fault injection. The fixture uses
// the driver's mock transport, but rule construction, submit, retry,
// completion publication, and teardown all cross the public C ABI.

#include "test_common.h"

#include <inttypes.h>
#include <stdint.h>

extern cosmos_status_code_t __test_only_create_fault_injection_fixture(
    const cosmos_driver_options_t *options,
    cosmos_runtime_t **out_runtime,
    cosmos_driver_t **out_driver,
    cosmos_container_ref_t **out_container);

typedef struct {
    cosmos_runtime_t *runtime;
    cosmos_driver_t *driver;
    cosmos_container_ref_t *container;
    cosmos_completion_queue_t *queue;
} fault_fixture_t;

static void free_fixture(fault_fixture_t *fixture)
{
    cosmos_completion_queue_free(fixture->queue);
    cosmos_container_ref_free(fixture->container);
    cosmos_driver_free(fixture->driver);
    cosmos_runtime_free(fixture->runtime);
    memset(fixture, 0, sizeof(*fixture));
}

static int create_fixture(fault_fixture_t *fixture)
{
    int result = TEST_PASS;
    cosmos_account_ref_t *account = NULL;
    cosmos_driver_options_t *options = NULL;
    cosmos_error_t *error = NULL;

    uint8_t throttle_body[] = "{\"fault\":\"throttled\"}";
    cosmos_header_kv_t throttle_headers[] = {
        {.name = SV("x-ms-request-charge"), .value = SV("4.25")},
    };

    cosmos_fault_injection_rule_t rules[2] = {
        cosmos_fault_injection_rule_default(),
        cosmos_fault_injection_rule_default(),
    };
    cosmos_fault_injection_condition_t conditions[2] = {
        cosmos_fault_injection_condition_default(),
        cosmos_fault_injection_condition_default(),
    };
    cosmos_fault_injection_result_t results[2] = {
        cosmos_fault_injection_result_default(),
        cosmos_fault_injection_result_default(),
    };
    rules[0].id = SV("linked-c-throttle");
    conditions[0].operation_type =
        COSMOS_FAULT_INJECTION_OPERATION_TYPE_READ_ITEM;
    results[0].custom_status_code = 429;
    results[0].custom_sub_status = 3200;
    results[0].retry_after_ms = 0;
    results[0].custom_headers = throttle_headers;
    results[0].custom_headers_len = 1;
    results[0].body = throttle_body;
    results[0].body_len = sizeof(throttle_body) - 1;
    rules[0].condition = &conditions[0];
    rules[0].result = &results[0];
    rules[0].hit_limit = 100;

    rules[1].id = SV("linked-c-hit-limit");
    conditions[1].operation_type =
        COSMOS_FAULT_INJECTION_OPERATION_TYPE_DELETE_ITEM;
    results[1].custom_status_code = 418;
    rules[1].condition = &conditions[1];
    rules[1].result = &results[1];
    rules[1].hit_limit = 1;

    int32_t rc = cosmos_account_ref_with_master_key(
        SV("https://native-fault.invalid/"), SV("dGVzdA=="), &account, &error);
    REQUIRE(rc == COSMOS_STATUS_SUCCESS && account != NULL,
            "account constructed (rc=%d)", rc);

    cosmos_driver_options_config_v2_t config =
        cosmos_driver_options_config_v2_default();
    config.fault_injection_rules = rules;
    config.fault_injection_rules_len = 2;
    config.fault_injection_rule_stride = sizeof(rules[0]);
    rc = cosmos_driver_options_build_v2(account, &config, &options);
    REQUIRE(rc == COSMOS_STATUS_SUCCESS && options != NULL,
            "v2 options constructed (rc=%d)", rc);

    // Every nested pointer is borrowed only for build_v2. Destroy the source
    // bytes before driver creation to prove the options handle owns copies.
    memset(rules, 0xA5, sizeof(rules));
    memset(conditions, 0xA5, sizeof(conditions));
    memset(results, 0xA5, sizeof(results));
    memset(throttle_headers, 0xA5, sizeof(throttle_headers));
    memset(throttle_body, 0xA5, sizeof(throttle_body));

    rc = __test_only_create_fault_injection_fixture(
        options, &fixture->runtime, &fixture->driver, &fixture->container);
    REQUIRE(rc == COSMOS_STATUS_SUCCESS && fixture->runtime != NULL &&
                fixture->driver != NULL && fixture->container != NULL,
            "mock-backed driver fixture constructed (rc=%d)", rc);

    cosmos_driver_options_free(options);
    options = NULL;
    cosmos_account_ref_free(account);
    account = NULL;

    cosmos_completion_queue_options_t queue_options = {
        .capacity_hint = 0,
        .max_capacity = 0,
        .include_error_details = true,
    };
    fixture->queue =
        cosmos_completion_queue_create(fixture->runtime, &queue_options);
    REQUIRE(fixture->queue != NULL, "completion queue constructed");

cleanup:
    cosmos_error_free(error);
    cosmos_driver_options_free(options);
    cosmos_account_ref_free(account);
    if (result != TEST_PASS) {
        free_fixture(fixture);
    }
    return result;
}

static int find_i64_header(const cosmos_completion_t *completion,
                           cosmos_header_id_t id,
                           int64_t *value)
{
    for (size_t i = 0; i < completion->headers_len; i++) {
        const cosmos_response_header_t *header = &completion->headers[i];
        if (header->id == id && header->value.kind == COSMOS_VALUE_KIND_I64) {
            *value = header->value.payload.i64_value;
            return 1;
        }
    }
    return 0;
}

static int find_u64_header(const cosmos_completion_t *completion,
                           cosmos_header_id_t id,
                           uint64_t *value)
{
    for (size_t i = 0; i < completion->headers_len; i++) {
        const cosmos_response_header_t *header = &completion->headers[i];
        if (header->id == id && header->value.kind == COSMOS_VALUE_KIND_U64) {
            *value = header->value.payload.u64_value;
            return 1;
        }
    }
    return 0;
}

static int find_f64_header(const cosmos_completion_t *completion,
                           cosmos_header_id_t id,
                           double *value)
{
    for (size_t i = 0; i < completion->headers_len; i++) {
        const cosmos_response_header_t *header = &completion->headers[i];
        if (header->id == id && header->value.kind == COSMOS_VALUE_KIND_F64) {
            *value = header->value.payload.f64_value;
            return 1;
        }
    }
    return 0;
}

static int submit_item(fault_fixture_t *fixture,
                       int32_t kind,
                       cosmos_completion_t *completion)
{
    cosmos_partition_key_component_t pk = {
        .kind = COSMOS_PARTITION_KEY_COMPONENT_KIND_STRING,
        .value.string_value = SV("tenant"),
    };
    cosmos_operation_request_t request = {0};
    request.kind = kind;
    request.container = fixture->container;
    request.item_id = SV("item");
    request.partition_key_components = &pk;
    request.partition_key_len = 1;
    request.max_item_count = -1;

    cosmos_status_code_t pre_error = COSMOS_STATUS_SUCCESS;
    cosmos_operation_handle_t *operation = cosmos_submit_singleton_operation(
        fixture->driver, &request, fixture->queue, 73, &pre_error);
    if (operation == NULL || pre_error != COSMOS_STATUS_SUCCESS) {
        cosmos_operation_handle_free(operation);
        return 0;
    }
    size_t count =
        cosmos_completion_queue_wait(fixture->queue, completion, 1, 5000);
    cosmos_operation_handle_free(operation);
    return count == 1;
}

static int test_rules_flow_through_submit_and_completion(void)
{
    int result = TEST_PASS;
    fault_fixture_t fixture = {0};
    cosmos_completion_t completion = {0};
    int completion_owned = 0;

    REQUIRE(create_fixture(&fixture) == TEST_PASS, "fixture created");

    REQUIRE(submit_item(&fixture, COSMOS_OPERATION_KIND_READ_ITEM, &completion),
            "read completion received");
    completion_owned = 1;
    ASSERT(completion.outcome == COSMOS_COMPLETION_OUTCOME_ERROR,
           "read is injected as an error");
    ASSERT(completion.http_status_code == 429,
           "HTTP status is 429 (got %u)", (unsigned)completion.http_status_code);
    ASSERT(COSMOS_STATUS_HTTP(completion.status) == 429 &&
               COSMOS_STATUS_SUB(completion.status) == 3200,
           "packed status is 429/3200 (got %d)", completion.status);
    ASSERT(completion.body_len == strlen("{\"fault\":\"throttled\"}") &&
               memcmp(completion.body, "{\"fault\":\"throttled\"}",
                      completion.body_len) == 0,
           "injected body is published");

    int64_t sub_status = -1;
    uint64_t retry_after = UINT64_MAX;
    double request_charge = 0.0;
    ASSERT(find_i64_header(&completion, COSMOS_HEADER_ID_SUB_STATUS, &sub_status) &&
               sub_status == 3200,
           "typed substatus header is I64 3200 (got %" PRId64 ")", sub_status);
    ASSERT(find_u64_header(&completion, COSMOS_HEADER_ID_RETRY_AFTER_MS,
                           &retry_after) &&
               retry_after == 0,
           "typed retry-after header is U64 0 (got %" PRIu64 ")", retry_after);
    ASSERT(find_f64_header(&completion, COSMOS_HEADER_ID_REQUEST_CHARGE,
                           &request_charge) &&
               request_charge == 4.25,
           "typed request-charge header is F64 4.25 (got %f)", request_charge);

    cosmos_completion_queue_free_completions(&completion, 1);
    completion_owned = 0;
    memset(&completion, 0, sizeof(completion));

    REQUIRE(submit_item(&fixture, COSMOS_OPERATION_KIND_DELETE_ITEM, &completion),
            "first delete completion received");
    completion_owned = 1;
    ASSERT(completion.http_status_code == 418,
           "matching delete is injected once (got %u)",
           (unsigned)completion.http_status_code);
    cosmos_completion_queue_free_completions(&completion, 1);
    completion_owned = 0;
    memset(&completion, 0, sizeof(completion));

    REQUIRE(submit_item(&fixture, COSMOS_OPERATION_KIND_DELETE_ITEM, &completion),
            "second delete completion received");
    completion_owned = 1;
    ASSERT(completion.outcome == COSMOS_COMPLETION_OUTCOME_OK &&
               completion.http_status_code == 200,
           "hit limit lets the second delete reach transport (outcome=%d status=%u)",
           (int)completion.outcome, (unsigned)completion.http_status_code);

cleanup:
    if (completion_owned) {
        cosmos_completion_queue_free_completions(&completion, 1);
    }
    free_fixture(&fixture);
    return result;
}

TEST_SUITE_BEGIN("Native fault injection")
TEST_REGISTER(rules_flow_through_submit_and_completion)
TEST_SUITE_END("Native fault injection")

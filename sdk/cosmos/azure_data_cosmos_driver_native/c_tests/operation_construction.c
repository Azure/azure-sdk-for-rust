// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

// Phase 5 — operation options builder + operation factories/mutators.
//
// Covers the C surface of every Phase 5 entry point that does not
// touch the network:
//
//   1. Lifecycle & NULL-safety on every `_free`.
//   2. cosmos_operation_options_builder_*: enum setters, primitive
//      setters, duration setters, string setters, list setters,
//      custom-header set/clear, clear setters, and build.
//   3. cosmos_operation_*: each Phase 5 factory + every mutator.
//   4. cosmos_driver_options_builder_with_operation_options
//      (the Phase 3 deferral now wired).

#include "test_common.h"

// ─────────────────────────────────────────────────────────────────────
// Section 1 — lifecycle / NULL safety
// ─────────────────────────────────────────────────────────────────────

static int test_lifecycle_null_safe(void)
{
    int result = TEST_PASS;
    cosmos_operation_options_builder_free(NULL);
    cosmos_operation_options_free(NULL);
    cosmos_operation_free(NULL);
    ASSERT(1, "Phase 5 _free entry points NULL-safe");
    return result;
}

// ─────────────────────────────────────────────────────────────────────
// Section 2 — cosmos_operation_options_builder_*
// ─────────────────────────────────────────────────────────────────────

static int test_options_builder_happy_path(void)
{
    int result = TEST_PASS;
    cosmos_operation_options_builder_t *b = cosmos_operation_options_builder_new();
    REQUIRE(b != NULL, "options builder allocated");

    int32_t rc;
    rc = cosmos_operation_options_builder_with_read_consistency_strategy(
        b, COSMOS_READ_CONSISTENCY_EVENTUAL);
    ASSERT(rc == COSMOS_ERROR_CODE_SUCCESS, "read_consistency accepted (rc=%d)", rc);

    rc = cosmos_operation_options_builder_with_content_response_on_write(
        b, COSMOS_CONTENT_RESPONSE_ON_WRITE_ENABLED);
    ASSERT(rc == COSMOS_ERROR_CODE_SUCCESS, "content_response accepted (rc=%d)", rc);

    rc = cosmos_operation_options_builder_with_max_failover_retry_count(b, 5);
    ASSERT(rc == COSMOS_ERROR_CODE_SUCCESS, "max_failover_retry_count accepted (rc=%d)", rc);

    rc = cosmos_operation_options_builder_with_session_capturing_disabled(b, true);
    ASSERT(rc == COSMOS_ERROR_CODE_SUCCESS, "session_capturing_disabled accepted (rc=%d)", rc);

    rc = cosmos_operation_options_builder_with_end_to_end_timeout_ms(b, 5000);
    ASSERT(rc == COSMOS_ERROR_CODE_SUCCESS, "end_to_end_timeout_ms accepted (rc=%d)", rc);

    rc = cosmos_operation_options_builder_with_throughput_control_group(b, "default");
    ASSERT(rc == COSMOS_ERROR_CODE_SUCCESS, "throughput_control_group accepted (rc=%d)", rc);

    const char *regions[] = {"East US", "West US 3"};
    rc = cosmos_operation_options_builder_with_excluded_regions(b, regions, 2);
    ASSERT(rc == COSMOS_ERROR_CODE_SUCCESS, "excluded_regions accepted (rc=%d)", rc);

    cosmos_operation_options_t *opts = NULL;
    rc = cosmos_operation_options_builder_build(b, &opts);
    ASSERT(rc == COSMOS_ERROR_CODE_SUCCESS, "build returned SUCCESS (rc=%d)", rc);
    ASSERT(opts != NULL, "build produced a non-NULL handle");

    cosmos_operation_options_free(opts);
    return result;

cleanup:
    cosmos_operation_options_builder_free(b);
    return result;
}

static int test_options_builder_clear_setters(void)
{
    int result = TEST_PASS;
    cosmos_operation_options_builder_t *b = cosmos_operation_options_builder_new();

    /* Setting then clearing produces None (verified Rust-side; here we
     * just confirm the entry points exist and return SUCCESS). */
    cosmos_operation_options_builder_with_max_failover_retry_count(b, 7);
    int32_t rc = cosmos_operation_options_builder_clear_max_failover_retry_count(b);
    ASSERT(rc == COSMOS_ERROR_CODE_SUCCESS, "clear accepted (rc=%d)", rc);

    cosmos_operation_options_builder_with_read_consistency_strategy(
        b, COSMOS_READ_CONSISTENCY_SESSION);
    rc = cosmos_operation_options_builder_clear_read_consistency_strategy(b);
    ASSERT(rc == COSMOS_ERROR_CODE_SUCCESS, "clear read_consistency accepted (rc=%d)", rc);

    cosmos_operation_options_t *opts = NULL;
    cosmos_operation_options_builder_build(b, &opts);
    cosmos_operation_options_free(opts);
    return result;
}

static int test_options_builder_custom_headers(void)
{
    int result = TEST_PASS;
    cosmos_operation_options_builder_t *b = cosmos_operation_options_builder_new();

    int32_t rc = cosmos_operation_options_builder_set_custom_header(b, "x-app", "foo");
    ASSERT(rc == COSMOS_ERROR_CODE_SUCCESS, "set_custom_header(x-app) ok");

    rc = cosmos_operation_options_builder_set_custom_header(b, "x-other", "bar");
    ASSERT(rc == COSMOS_ERROR_CODE_SUCCESS, "set_custom_header(x-other) ok");

    /* Control character rejected. */
    rc = cosmos_operation_options_builder_set_custom_header(b, "x-app", "hi\x01world");
    ASSERT(rc == COSMOS_ERROR_CODE_INVALID_HEADER, "control char rejected (rc=%d)", rc);

    /* Empty name rejected. */
    rc = cosmos_operation_options_builder_set_custom_header(b, "", "v");
    ASSERT(rc == COSMOS_ERROR_CODE_INVALID_HEADER, "empty name rejected (rc=%d)", rc);

    /* Clear headers. */
    rc = cosmos_operation_options_builder_clear_custom_headers(b);
    ASSERT(rc == COSMOS_ERROR_CODE_SUCCESS, "clear_custom_headers ok");

    cosmos_operation_options_t *opts = NULL;
    cosmos_operation_options_builder_build(b, &opts);
    cosmos_operation_options_free(opts);
    return result;
}

static int test_options_builder_rejects_nulls(void)
{
    int result = TEST_PASS;

    /* NULL builder. */
    int32_t rc = cosmos_operation_options_builder_with_max_failover_retry_count(NULL, 1);
    ASSERT(rc == COSMOS_ERROR_CODE_INVALID_ARGUMENT, "NULL builder rejected (rc=%d)", rc);

    rc = cosmos_operation_options_builder_with_throughput_control_group(NULL, "g");
    ASSERT(rc == COSMOS_ERROR_CODE_INVALID_ARGUMENT, "NULL builder rejected (rc=%d)", rc);

    rc = cosmos_operation_options_builder_clear_max_failover_retry_count(NULL);
    ASSERT(rc == COSMOS_ERROR_CODE_INVALID_ARGUMENT, "NULL builder rejected (rc=%d)", rc);

    /* NULL string. */
    cosmos_operation_options_builder_t *b = cosmos_operation_options_builder_new();
    rc = cosmos_operation_options_builder_with_throughput_control_group(b, NULL);
    ASSERT(rc == COSMOS_ERROR_CODE_INVALID_ARGUMENT, "NULL string rejected (rc=%d)", rc);

    /* NULL regions ptr with non-zero len. */
    rc = cosmos_operation_options_builder_with_excluded_regions(b, NULL, 1);
    ASSERT(rc == COSMOS_ERROR_CODE_INVALID_ARGUMENT, "NULL regions ptr rejected (rc=%d)", rc);

    cosmos_operation_options_builder_free(b);
    return result;
}

// ─────────────────────────────────────────────────────────────────────
// Section 3 — cosmos_operation_* factories
// ─────────────────────────────────────────────────────────────────────

static cosmos_account_ref_t *make_account(void)
{
    cosmos_account_ref_t *account = NULL;
    cosmos_error_t *err = NULL;
    cosmos_account_ref_with_master_key(
        "https://myaccount.documents.azure.com:443/", "fake-key", &account, &err);
    return account;
}

static cosmos_database_ref_t *make_database(cosmos_account_ref_t *account)
{
    cosmos_database_ref_t *db = NULL;
    cosmos_database_ref_create(account, "mydb", &db);
    return db;
}

static int test_account_scope_factories(void)
{
    int result = TEST_PASS;
    cosmos_account_ref_t *account = make_account();
    REQUIRE(account != NULL, "account allocated");

    cosmos_operation_t *op = NULL;
    int32_t rc;

    rc = cosmos_operation_create_database(account, &op);
    ASSERT(rc == COSMOS_ERROR_CODE_SUCCESS, "create_database (rc=%d)", rc);
    cosmos_operation_free(op);
    op = NULL;

    rc = cosmos_operation_read_all_databases(account, &op);
    ASSERT(rc == COSMOS_ERROR_CODE_SUCCESS, "read_all_databases (rc=%d)", rc);
    cosmos_operation_free(op);
    op = NULL;

    rc = cosmos_operation_query_databases(account, &op);
    ASSERT(rc == COSMOS_ERROR_CODE_SUCCESS, "query_databases (rc=%d)", rc);
    cosmos_operation_free(op);
    op = NULL;

    rc = cosmos_operation_query_offers(account, &op);
    ASSERT(rc == COSMOS_ERROR_CODE_SUCCESS, "query_offers (rc=%d)", rc);
    cosmos_operation_free(op);

cleanup:
    cosmos_account_ref_free(account);
    return result;
}

static int test_database_scope_factories(void)
{
    int result = TEST_PASS;
    cosmos_account_ref_t *account = make_account();
    cosmos_database_ref_t *db = make_database(account);
    REQUIRE(db != NULL, "database allocated");

    cosmos_operation_t *op = NULL;
    int32_t rc;

    rc = cosmos_operation_read_database(db, &op);
    ASSERT(rc == COSMOS_ERROR_CODE_SUCCESS, "read_database (rc=%d)", rc);
    cosmos_operation_free(op);
    op = NULL;

    rc = cosmos_operation_delete_database(db, &op);
    ASSERT(rc == COSMOS_ERROR_CODE_SUCCESS, "delete_database (rc=%d)", rc);
    cosmos_operation_free(op);
    op = NULL;

    rc = cosmos_operation_create_container(db, &op);
    ASSERT(rc == COSMOS_ERROR_CODE_SUCCESS, "create_container (rc=%d)", rc);
    cosmos_operation_free(op);
    op = NULL;

    rc = cosmos_operation_read_all_containers(db, &op);
    ASSERT(rc == COSMOS_ERROR_CODE_SUCCESS, "read_all_containers (rc=%d)", rc);
    cosmos_operation_free(op);
    op = NULL;

    rc = cosmos_operation_query_containers(db, &op);
    ASSERT(rc == COSMOS_ERROR_CODE_SUCCESS, "query_containers (rc=%d)", rc);
    cosmos_operation_free(op);

cleanup:
    cosmos_database_ref_free(db);
    cosmos_account_ref_free(account);
    return result;
}

static int test_offer_factories(void)
{
    int result = TEST_PASS;
    cosmos_account_ref_t *account = make_account();
    cosmos_operation_t *op = NULL;
    int32_t rc;

    rc = cosmos_operation_read_offer(account, "offers/abc", &op);
    ASSERT(rc == COSMOS_ERROR_CODE_SUCCESS, "read_offer (rc=%d)", rc);
    cosmos_operation_free(op);
    op = NULL;

    rc = cosmos_operation_replace_offer(account, "offers/abc", &op);
    ASSERT(rc == COSMOS_ERROR_CODE_SUCCESS, "replace_offer (rc=%d)", rc);
    cosmos_operation_free(op);
    op = NULL;

    /* NULL resource_link rejected. */
    rc = cosmos_operation_read_offer(account, NULL, &op);
    ASSERT(rc == COSMOS_ERROR_CODE_INVALID_ARGUMENT, "NULL link rejected (rc=%d)", rc);

    cosmos_account_ref_free(account);
    return result;
}

static int test_factory_rejects_null_arguments(void)
{
    int result = TEST_PASS;
    cosmos_account_ref_t *account = make_account();
    cosmos_operation_t *op = NULL;
    int32_t rc;

    rc = cosmos_operation_create_database(NULL, &op);
    ASSERT(rc == COSMOS_ERROR_CODE_INVALID_ARGUMENT, "NULL account rejected (rc=%d)", rc);

    rc = cosmos_operation_create_database(account, NULL);
    ASSERT(rc == COSMOS_ERROR_CODE_INVALID_ARGUMENT, "NULL out_op rejected (rc=%d)", rc);

    cosmos_account_ref_free(account);
    return result;
}

// ─────────────────────────────────────────────────────────────────────
// Section 4 — cosmos_operation_with_* mutators
// ─────────────────────────────────────────────────────────────────────

static int test_mutator_body(void)
{
    int result = TEST_PASS;
    cosmos_account_ref_t *account = make_account();
    cosmos_operation_t *op = NULL;
    cosmos_operation_create_database(account, &op);

    const uint8_t payload[] = "{\"id\":\"db1\"}";
    int32_t rc = cosmos_operation_with_body(op, payload, sizeof(payload) - 1);
    ASSERT(rc == COSMOS_ERROR_CODE_SUCCESS, "with_body (rc=%d)", rc);

    /* Zero-length body. */
    rc = cosmos_operation_with_body(op, NULL, 0);
    ASSERT(rc == COSMOS_ERROR_CODE_SUCCESS, "zero-len body ok (rc=%d)", rc);

    /* NULL with non-zero len rejected. */
    rc = cosmos_operation_with_body(op, NULL, 1);
    ASSERT(rc == COSMOS_ERROR_CODE_INVALID_ARGUMENT, "NULL+nonzero rejected (rc=%d)", rc);

    cosmos_operation_free(op);
    cosmos_account_ref_free(account);
    return result;
}

static int test_mutator_session_and_activity(void)
{
    int result = TEST_PASS;
    cosmos_account_ref_t *account = make_account();
    cosmos_operation_t *op = NULL;
    cosmos_operation_create_database(account, &op);

    int32_t rc = cosmos_operation_with_session_token(op, "0:1#100");
    ASSERT(rc == COSMOS_ERROR_CODE_SUCCESS, "with_session_token (rc=%d)", rc);

    rc = cosmos_operation_with_activity_id(op, "11111111-1111-1111-1111-111111111111");
    ASSERT(rc == COSMOS_ERROR_CODE_SUCCESS, "with_activity_id (rc=%d)", rc);

    cosmos_operation_free(op);
    cosmos_account_ref_free(account);
    return result;
}

static int test_mutator_max_item_count(void)
{
    int result = TEST_PASS;
    cosmos_account_ref_t *account = make_account();
    cosmos_operation_t *op = NULL;
    cosmos_operation_create_database(account, &op);

    /* -1 → ServerDecides. */
    int32_t rc = cosmos_operation_with_max_item_count(op, -1);
    ASSERT(rc == COSMOS_ERROR_CODE_SUCCESS, "ServerDecides (rc=%d)", rc);

    /* Positive → Limit. */
    rc = cosmos_operation_with_max_item_count(op, 100);
    ASSERT(rc == COSMOS_ERROR_CODE_SUCCESS, "Limit (rc=%d)", rc);

    /* Zero rejected. */
    rc = cosmos_operation_with_max_item_count(op, 0);
    ASSERT(rc == COSMOS_ERROR_CODE_INVALID_OPTION_VALUE, "0 rejected (rc=%d)", rc);

    cosmos_operation_free(op);
    cosmos_account_ref_free(account);
    return result;
}

static int test_mutator_populate_metrics(void)
{
    int result = TEST_PASS;
    cosmos_account_ref_t *account = make_account();
    cosmos_operation_t *op = NULL;
    cosmos_operation_create_database(account, &op);

    int32_t rc = cosmos_operation_with_populate_index_metrics(op, true);
    ASSERT(rc == COSMOS_ERROR_CODE_SUCCESS, "populate_index_metrics (rc=%d)", rc);

    rc = cosmos_operation_with_populate_query_metrics(op, false);
    ASSERT(rc == COSMOS_ERROR_CODE_SUCCESS, "populate_query_metrics (rc=%d)", rc);

    cosmos_operation_free(op);
    cosmos_account_ref_free(account);
    return result;
}

static int test_mutator_precondition_double_set_rejected(void)
{
    int result = TEST_PASS;
    cosmos_account_ref_t *account = make_account();
    cosmos_operation_t *op = NULL;
    cosmos_operation_create_database(account, &op);

    int32_t rc = cosmos_operation_with_precondition_if_match(op, "\"etag-1\"");
    ASSERT(rc == COSMOS_ERROR_CODE_SUCCESS, "first if_match (rc=%d)", rc);

    /* Same kind second time. */
    rc = cosmos_operation_with_precondition_if_match(op, "\"etag-2\"");
    ASSERT(rc == COSMOS_ERROR_CODE_PRECONDITION_ALREADY_SET, "second rejected (rc=%d)", rc);

    /* Different kind. */
    rc = cosmos_operation_with_precondition_if_none_match(op, "\"etag-3\"");
    ASSERT(rc == COSMOS_ERROR_CODE_PRECONDITION_ALREADY_SET, "if_none_match rejected (rc=%d)", rc);

    cosmos_operation_free(op);
    cosmos_account_ref_free(account);
    return result;
}

static int test_mutator_rejects_null_operation(void)
{
    int result = TEST_PASS;
    int32_t rc;

    rc = cosmos_operation_with_body(NULL, NULL, 0);
    ASSERT(rc == COSMOS_ERROR_CODE_INVALID_ARGUMENT, "with_body(NULL) rejected (rc=%d)", rc);

    rc = cosmos_operation_with_session_token(NULL, "tok");
    ASSERT(rc == COSMOS_ERROR_CODE_INVALID_ARGUMENT, "with_session_token(NULL) rejected (rc=%d)", rc);

    rc = cosmos_operation_with_max_item_count(NULL, 10);
    ASSERT(rc == COSMOS_ERROR_CODE_INVALID_ARGUMENT, "with_max_item_count(NULL) rejected (rc=%d)", rc);

    rc = cosmos_operation_with_precondition_if_match(NULL, "\"x\"");
    ASSERT(rc == COSMOS_ERROR_CODE_INVALID_ARGUMENT, "with_precondition(NULL) rejected (rc=%d)", rc);

    return result;
}

// ─────────────────────────────────────────────────────────────────────
// Section 5 — Phase 3 deferral wired
// (cosmos_driver_options_builder_with_operation_options)
// ─────────────────────────────────────────────────────────────────────

static int test_driver_options_wires_operation_options(void)
{
    int result = TEST_PASS;
    cosmos_account_ref_t *account = make_account();
    REQUIRE(account != NULL, "account allocated");

    cosmos_driver_options_builder_t *db = cosmos_driver_options_builder_new(account);
    REQUIRE(db != NULL, "driver options builder allocated");

    cosmos_operation_options_builder_t *ob = cosmos_operation_options_builder_new();
    cosmos_operation_options_builder_with_max_failover_retry_count(ob, 7);
    cosmos_operation_options_t *opts = NULL;
    cosmos_operation_options_builder_build(ob, &opts);
    REQUIRE(opts != NULL, "operation options built");

    int32_t rc = cosmos_driver_options_builder_with_operation_options(db, opts);
    ASSERT(rc == COSMOS_ERROR_CODE_SUCCESS, "with_operation_options ok (rc=%d)", rc);

    /* NULL options rejected. */
    rc = cosmos_driver_options_builder_with_operation_options(db, NULL);
    ASSERT(rc == COSMOS_ERROR_CODE_INVALID_ARGUMENT, "NULL options rejected (rc=%d)", rc);

    cosmos_driver_options_t *dopts = NULL;
    cosmos_driver_options_builder_build(db, &dopts);
    cosmos_driver_options_free(dopts);
    cosmos_operation_options_free(opts);
    cosmos_account_ref_free(account);
    return result;

cleanup:
    cosmos_account_ref_free(account);
    return result;
}

TEST_SUITE_BEGIN("Phase 5 — Operation Options + Factories + Mutators")
TEST_REGISTER(lifecycle_null_safe)
TEST_REGISTER(options_builder_happy_path)
TEST_REGISTER(options_builder_clear_setters)
TEST_REGISTER(options_builder_custom_headers)
TEST_REGISTER(options_builder_rejects_nulls)
TEST_REGISTER(account_scope_factories)
TEST_REGISTER(database_scope_factories)
TEST_REGISTER(offer_factories)
TEST_REGISTER(factory_rejects_null_arguments)
TEST_REGISTER(mutator_body)
TEST_REGISTER(mutator_session_and_activity)
TEST_REGISTER(mutator_max_item_count)
TEST_REGISTER(mutator_populate_metrics)
TEST_REGISTER(mutator_precondition_double_set_rejected)
TEST_REGISTER(mutator_rejects_null_operation)
TEST_REGISTER(driver_options_wires_operation_options)
TEST_SUITE_END("Phase 5 — Operation Options + Factories + Mutators")

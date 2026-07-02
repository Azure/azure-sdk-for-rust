// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

// Account refs, database refs, driver options, and driver lifecycle.
//
// Covers the C surface of every entry point that does not
// touch the network:
//
//   1. Lifecycle & NULL-safety on every `_free`.
//   2. Validation paths on `cosmos_account_ref_with_master_key` (NULL
//      args + invalid endpoint URL).
//   3. `cosmos_database_ref_create` happy path + NULL-arg rejection.
//   4. `cosmos_driver_options_build` (flat) happy path + preferred-regions
//      round-trip + NULL-arg rejection.
//
// `cosmos_driver_get_or_create_blocking` is not exercised here because
// it touches the network. The emulator-backed scenario (stand up a driver,
// free it, recreate it, observe the cached instance, observe the
// OPTIONS_IGNORED_ON_CACHE_HIT advisory) is intentionally deferred to a
// CI-side integration test once the advisory itself lands. The Rust-side
// integration test `blocking_against_invalid_endpoint` (marked
// `#[ignore]`) exercises the failure path manually.

#include "test_common.h"

static int test_lifecycle_null_safe(void) {
    int result = TEST_PASS;
    cosmos_account_ref_free(NULL);
    cosmos_database_ref_free(NULL);
    cosmos_driver_options_free(NULL);
    cosmos_driver_free(NULL);
    ASSERT(1, "all _free entry points NULL-safe");
    return result;
}

static int test_account_ref_master_key_happy_path(void) {
    int result = TEST_PASS;
    cosmos_account_ref_t *account = NULL;
    cosmos_error_t *err = NULL;
    int32_t rc = cosmos_account_ref_with_master_key(
        "https://myaccount.documents.azure.com:443/",
        "fake-master-key",
        &account,
        &err);
    REQUIRE(rc == COSMOS_ERROR_CODE_SUCCESS,
            "with_master_key returned SUCCESS (rc=%d)", rc);
    REQUIRE(account != NULL, "out_account populated on success");
    ASSERT(err == NULL, "out_error untouched on success");

cleanup:
    cosmos_account_ref_free(account);
    return result;
}

static int test_account_ref_rejects_null_arguments(void) {
    int result = TEST_PASS;
    cosmos_account_ref_t *account = NULL;
    cosmos_error_t *err = NULL;

    int32_t rc = cosmos_account_ref_with_master_key(
        NULL, "k", &account, &err);
    ASSERT(rc == COSMOS_ERROR_CODE_INVALID_ARGUMENT,
           "NULL endpoint rejected (rc=%d)", rc);
    ASSERT(account == NULL, "out_account untouched on NULL endpoint");

    rc = cosmos_account_ref_with_master_key(
        "https://x.documents.azure.com:443/", NULL, &account, &err);
    ASSERT(rc == COSMOS_ERROR_CODE_INVALID_ARGUMENT,
           "NULL key rejected (rc=%d)", rc);

    rc = cosmos_account_ref_with_master_key(
        "https://x.documents.azure.com:443/", "k", NULL, &err);
    ASSERT(rc == COSMOS_ERROR_CODE_INVALID_ARGUMENT,
           "NULL out_account rejected (rc=%d)", rc);

    return result;
}

static int test_account_ref_rejects_invalid_endpoint(void) {
    int result = TEST_PASS;
    cosmos_account_ref_t *account = NULL;
    cosmos_error_t *err = NULL;
    int32_t rc = cosmos_account_ref_with_master_key(
        "not a url", "k", &account, &err);
    ASSERT(rc == COSMOS_ERROR_CODE_INVALID_ACCOUNT_REFERENCE,
           "invalid endpoint rejected (rc=%d)", rc);
    ASSERT(account == NULL, "no handle on failure");
    ASSERT(err != NULL, "rich error populated on parse failure");
    cosmos_error_free(err);
    return result;
}

static int test_database_ref_create_happy_path(void) {
    int result = TEST_PASS;
    cosmos_account_ref_t *account = NULL;
    cosmos_error_t *err = NULL;
    cosmos_account_ref_with_master_key(
        "https://x.documents.azure.com:443/", "k", &account, &err);
    REQUIRE(account != NULL, "account allocated");

    cosmos_database_ref_t *db = NULL;
    int32_t rc = cosmos_database_ref_create(account, "mydb", &db);
    ASSERT(rc == COSMOS_ERROR_CODE_SUCCESS,
           "database_ref_create returned SUCCESS (rc=%d)", rc);
    ASSERT(db != NULL, "database_ref produced a non-NULL handle");

    // Freeing the account does not invalidate the database (it cloned).
    cosmos_account_ref_free(account);
    account = NULL;
    cosmos_database_ref_free(db);
    return result;

cleanup:
    cosmos_database_ref_free(db);
    cosmos_account_ref_free(account);
    return result;
}

static int test_database_ref_rejects_null_arguments(void) {
    int result = TEST_PASS;
    cosmos_account_ref_t *account = NULL;
    cosmos_error_t *err = NULL;
    cosmos_account_ref_with_master_key(
        "https://x.documents.azure.com:443/", "k", &account, &err);
    REQUIRE(account != NULL, "account allocated");

    cosmos_database_ref_t *db = NULL;
    int32_t rc = cosmos_database_ref_create(NULL, "mydb", &db);
    ASSERT(rc == COSMOS_ERROR_CODE_INVALID_ARGUMENT,
           "NULL account rejected (rc=%d)", rc);
    rc = cosmos_database_ref_create(account, NULL, &db);
    ASSERT(rc == COSMOS_ERROR_CODE_INVALID_ARGUMENT,
           "NULL database_id rejected (rc=%d)", rc);
    rc = cosmos_database_ref_create(account, "mydb", NULL);
    ASSERT(rc == COSMOS_ERROR_CODE_INVALID_ARGUMENT,
           "NULL out_database rejected (rc=%d)", rc);

cleanup:
    cosmos_account_ref_free(account);
    return result;
}

static int test_driver_options_build_happy_path(void) {
    int result = TEST_PASS;
    cosmos_account_ref_t *account = NULL;
    cosmos_error_t *err = NULL;
    cosmos_account_ref_with_master_key(
        "https://x.documents.azure.com:443/", "k", &account, &err);
    REQUIRE(account != NULL, "account allocated");

    const char *regions[] = {"East US", "West US 3"};
    cosmos_driver_options_config_t cfg = cosmos_driver_options_config_default();
    cfg.preferred_regions = regions;
    cfg.preferred_regions_len = 2;

    cosmos_driver_options_t *opts = NULL;
    int32_t rc = cosmos_driver_options_build(account, &cfg, &opts);
    ASSERT(rc == COSMOS_ERROR_CODE_SUCCESS,
           "build returned SUCCESS (rc=%d)", rc);
    ASSERT(opts != NULL, "build produced a non-NULL handle");

    cosmos_driver_options_free(opts);
cleanup:
    cosmos_account_ref_free(account);
    return result;
}

static int test_driver_options_build_null_config(void) {
    int result = TEST_PASS;
    cosmos_account_ref_t *account = NULL;
    cosmos_error_t *err = NULL;
    cosmos_account_ref_with_master_key(
        "https://x.documents.azure.com:443/", "k", &account, &err);
    REQUIRE(account != NULL, "account allocated");

    // A NULL config means "no preferred regions, default operation options".
    cosmos_driver_options_t *opts = NULL;
    int32_t rc = cosmos_driver_options_build(account, NULL, &opts);
    ASSERT(rc == COSMOS_ERROR_CODE_SUCCESS, "build(NULL config) works (rc=%d)", rc);
    ASSERT(opts != NULL, "build produced a non-NULL handle");

    cosmos_driver_options_free(opts);
cleanup:
    cosmos_account_ref_free(account);
    return result;
}

static int test_driver_options_build_rejects_nulls(void) {
    int result = TEST_PASS;
    cosmos_account_ref_t *account = NULL;
    cosmos_error_t *err = NULL;
    cosmos_account_ref_with_master_key(
        "https://x.documents.azure.com:443/", "k", &account, &err);
    REQUIRE(account != NULL, "account allocated");

    cosmos_driver_options_config_t cfg = cosmos_driver_options_config_default();
    cosmos_driver_options_t *opts = NULL;

    // NULL account.
    int32_t rc = cosmos_driver_options_build(NULL, &cfg, &opts);
    ASSERT(rc == COSMOS_ERROR_CODE_INVALID_ARGUMENT,
           "NULL account rejected (rc=%d)", rc);
    ASSERT(opts == NULL, "out_options untouched on NULL account");

    // NULL out_options.
    rc = cosmos_driver_options_build(account, &cfg, NULL);
    ASSERT(rc == COSMOS_ERROR_CODE_INVALID_ARGUMENT,
           "NULL out_options rejected (rc=%d)", rc);

    // NULL entry within the regions array.
    const char *bad_regions[] = {NULL};
    cfg.preferred_regions = bad_regions;
    cfg.preferred_regions_len = 1;
    rc = cosmos_driver_options_build(account, &cfg, &opts);
    ASSERT(rc == COSMOS_ERROR_CODE_INVALID_ARGUMENT,
           "NULL region entry rejected (rc=%d)", rc);
    ASSERT(opts == NULL, "out_options untouched on NULL region entry");

cleanup:
    cosmos_account_ref_free(account);
    return result;
}

TEST_SUITE_BEGIN("Account / Database / Driver Options Lifecycle")
TEST_REGISTER(lifecycle_null_safe)
TEST_REGISTER(account_ref_master_key_happy_path)
TEST_REGISTER(account_ref_rejects_null_arguments)
TEST_REGISTER(account_ref_rejects_invalid_endpoint)
TEST_REGISTER(database_ref_create_happy_path)
TEST_REGISTER(database_ref_rejects_null_arguments)
TEST_REGISTER(driver_options_build_happy_path)
TEST_REGISTER(driver_options_build_null_config)
TEST_REGISTER(driver_options_build_rejects_nulls)
TEST_SUITE_END("Account / Database / Driver Options Lifecycle")

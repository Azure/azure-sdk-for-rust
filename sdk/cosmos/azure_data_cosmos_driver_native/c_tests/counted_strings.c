// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

#include "test_common.h"
#include <stddef.h>
#include <stdint.h>

#if UINTPTR_MAX == UINT64_MAX
_Static_assert(sizeof(cosmos_string_view_t) == 16, "counted view layout");
_Static_assert(sizeof(cosmos_header_kv_t) == 32, "header pair layout");
_Static_assert(sizeof(cosmos_runtime_options_t) == 64, "runtime layout");
_Static_assert(sizeof(cosmos_operation_options_t) == 96, "operation options layout");
_Static_assert(sizeof(cosmos_operation_request_t) == 224, "request layout");
_Static_assert(offsetof(cosmos_operation_request_t, item_id) == 32, "item view offset");
_Static_assert(offsetof(cosmos_operation_request_t, session_token) == 112, "session view offset");
_Static_assert(offsetof(cosmos_operation_request_t, patch_tracking_id) == 200, "tracking view offset");
#endif

static int test_counted_account_and_database(void) {
    int result = TEST_PASS;
    cosmos_account_ref_t *account = NULL;
    cosmos_database_ref_t *database = NULL;
    cosmos_error_t *error = NULL;
    int32_t rc = cosmos_account_ref_with_master_key(
        SV("https://localhost/\0suffix"), SV("key"), &account, &error);
    ASSERT(COSMOS_STATUS_SUB(rc) == COSMOS_SUB_STATUS_CLIENT_INVALID_ACCOUNT_ENDPOINT_URL,
           "endpoint must not shorten to its valid prefix");
    rc = cosmos_account_ref_with_master_key(
        SV("https://localhost/\0\xff"), SV("key"), &account, &error);
    ASSERT(COSMOS_STATUS_SUB(rc) == COSMOS_SUB_STATUS_CLIENT_FFI_INVALID_UTF8,
           "UTF-8 after NUL is validated");
    rc = cosmos_account_ref_with_master_key(
        SV("https://localhost/"), SV("key\0suffix"), &account, &error);
    ASSERT(COSMOS_STATUS_SUB(rc) == COSMOS_SUB_STATUS_CLIENT_FFI_INVALID_OPTION_VALUE,
           "master key NUL is rejected");
    rc = cosmos_account_ref_with_master_key(
        SV("https://localhost/"), SV("key"), &account, &error);
    REQUIRE(rc == COSMOS_STATUS_SUCCESS, "create account without network");
    rc = cosmos_database_ref_create(account, SV("doc\0suffix"), &database);
    ASSERT(COSMOS_STATUS_SUB(rc) == COSMOS_SUB_STATUS_CLIENT_FFI_INVALID_OPTION_VALUE,
           "resource identifier must not become doc");
    rc = cosmos_database_ref_create(account, SV("doc\0\xff"), &database);
    ASSERT(COSMOS_STATUS_SUB(rc) == COSMOS_SUB_STATUS_CLIENT_FFI_INVALID_UTF8,
           "invalid UTF-8 suffix rejected");
    cosmos_string_view_t invalid = {NULL, 1};
    rc = cosmos_database_ref_create(account, invalid, &database);
    ASSERT(COSMOS_STATUS_SUB(rc) == COSMOS_SUB_STATUS_CLIENT_FFI_NULL_ARGUMENT,
           "NULL/nonzero rejected");
    invalid = (cosmos_string_view_t){(const uint8_t *)"x", UINTPTR_MAX};
    rc = cosmos_database_ref_create(account, invalid, &database);
    ASSERT(COSMOS_STATUS_SUB(rc) == COSMOS_SUB_STATUS_CLIENT_FFI_NULL_ARGUMENT,
           "oversized input rejected before reading");
    char *utf8 = malloc(6);
    REQUIRE(utf8 != NULL, "allocate UTF-8 input");
    memcpy(utf8, "\xe6\xb0\xb4-db", 6);
    rc = cosmos_database_ref_create(account, (cosmos_string_view_t){(const uint8_t *)utf8, 6}, &database);
    free(utf8);
    ASSERT(rc == COSMOS_STATUS_SUCCESS && database != NULL, "nonterminated UTF-8 byte input copied");
cleanup:
    cosmos_database_ref_free(database);
    cosmos_account_ref_free(account);
    cosmos_error_free(error);
    return result;
}

static int test_counted_option_arrays(void) {
    int result = TEST_PASS;
    cosmos_account_ref_t *account = NULL;
    cosmos_driver_options_t *options = NULL;
    int32_t rc = cosmos_account_ref_with_master_key(
        SV("https://localhost/"), SV("key"), &account, NULL);
    REQUIRE(rc == COSMOS_STATUS_SUCCESS, "account created");
    cosmos_operation_options_t operation = cosmos_operation_options_default();
    cosmos_driver_options_config_t config = cosmos_driver_options_config_default();
    config.operation_options = &operation;
    cosmos_header_kv_t headers[] = {{SV("abc\0invalid"), SV("value")}};
    operation.custom_headers = headers;
    operation.custom_headers_len = 1;
    rc = cosmos_driver_options_build(account, &config, &options);
    ASSERT(COSMOS_STATUS_SUB(rc) == COSMOS_SUB_STATUS_CLIENT_FFI_INVALID_HEADER, "complete header name rejected");
    headers[0].name = SV("x-custom");
    headers[0].value = SV("abc\0invalid");
    rc = cosmos_driver_options_build(account, &config, &options);
    ASSERT(COSMOS_STATUS_SUB(rc) == COSMOS_SUB_STATUS_CLIENT_FFI_INVALID_HEADER, "complete header value rejected");
    headers[0].value = SV("abc\0\xff");
    rc = cosmos_driver_options_build(account, &config, &options);
    ASSERT(COSMOS_STATUS_SUB(rc) == COSMOS_SUB_STATUS_CLIENT_FFI_INVALID_UTF8, "header UTF-8 suffix rejected");
    headers[0].value = SV("");
    rc = cosmos_driver_options_build(account, &config, &options);
    ASSERT(rc == COSMOS_STATUS_SUCCESS, "explicit empty header value accepted");
    cosmos_driver_options_free(options);
    options = NULL;
    operation.custom_headers = NULL;
    rc = cosmos_driver_options_build(account, &config, &options);
    ASSERT(COSMOS_STATUS_SUB(rc) == COSMOS_SUB_STATUS_CLIENT_FFI_NULL_ARGUMENT, "NULL/nonzero header array rejected");
    operation.custom_headers_len = 0;
    cosmos_string_view_t regions[] = {SV("West US"), SV("East US\0invalid")};
    config.preferred_regions = regions;
    config.preferred_regions_len = 2;
    rc = cosmos_driver_options_build(account, &config, &options);
    ASSERT(COSMOS_STATUS_SUB(rc) == COSMOS_SUB_STATUS_CLIENT_FFI_INVALID_OPTION_VALUE, "preferred region NUL rejected");
    config.preferred_regions = NULL;
    config.preferred_regions_len = 0;
    operation.excluded_regions = regions;
    operation.excluded_regions_len = 2;
    rc = cosmos_driver_options_build(account, &config, &options);
    ASSERT(COSMOS_STATUS_SUB(rc) == COSMOS_SUB_STATUS_CLIENT_FFI_INVALID_OPTION_VALUE, "excluded region NUL rejected");
    regions[1] = SV("East US\0\xff");
    rc = cosmos_driver_options_build(account, &config, &options);
    ASSERT(COSMOS_STATUS_SUB(rc) == COSMOS_SUB_STATUS_CLIENT_FFI_INVALID_UTF8, "region UTF-8 suffix rejected");
cleanup:
    cosmos_driver_options_free(options);
    cosmos_account_ref_free(account);
    return result;
}

TEST_SUITE_BEGIN("Counted UTF-8")
TEST_REGISTER(counted_account_and_database)
TEST_REGISTER(counted_option_arrays)
TEST_SUITE_END("Counted UTF-8")

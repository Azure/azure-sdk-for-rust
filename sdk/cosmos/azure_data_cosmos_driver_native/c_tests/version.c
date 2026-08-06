// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

#include "test_common.h"

static int test_version_matches_header(void) {
    int result = TEST_PASS;
    const char *runtime_version = cosmos_version();
    const char *header_version = AZURECOSMOSDRIVER_H_VERSION;

    printf("    library version: %s\n", runtime_version ? runtime_version : "(null)");
    printf("    header  version: %s\n", header_version);

    REQUIRE(runtime_version != NULL, "cosmos_version() returned a non-null pointer");
    ASSERT(strcmp(runtime_version, header_version) == 0,
           "library version matches header version");

cleanup:
    return result;
}

static int test_string_free_handles_null(void) {
    int result = TEST_PASS;
    /* Documented behavior: passing NULL is a no-op (no crash, no UB). */
    cosmos_string_free(NULL);
    ASSERT(1, "cosmos_string_free(NULL) returned without crashing");
    return result;
}

static int test_bytes_free_handles_null(void) {
    int result = TEST_PASS;
    cosmos_bytes_free(NULL);
    ASSERT(1, "cosmos_bytes_free(NULL) returned without crashing");
    return result;
}

static int test_bytes_accessors_handle_null(void) {
    int result = TEST_PASS;
    const uint8_t *data = cosmos_bytes_data(NULL);
    size_t len = cosmos_bytes_len(NULL);
    ASSERT(data == NULL, "cosmos_bytes_data(NULL) returns NULL");
    ASSERT(len == 0, "cosmos_bytes_len(NULL) returns 0");
    return result;
}

TEST_SUITE_BEGIN("Version & Scaffolding")
TEST_REGISTER(version_matches_header)
TEST_REGISTER(string_free_handles_null)
TEST_REGISTER(bytes_free_handles_null)
TEST_REGISTER(bytes_accessors_handle_null)
TEST_SUITE_END("Version & Scaffolding")

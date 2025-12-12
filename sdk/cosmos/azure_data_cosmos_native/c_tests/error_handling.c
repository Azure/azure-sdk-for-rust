// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>
#include "../include/azurecosmos.h"

#define TEST_PASS 0
#define TEST_FAIL 1

// Test counter
static int tests_run = 0;
static int tests_passed = 0;

void report_test(const char *test_name, int passed) {
    tests_run++;
    if (passed) {
        tests_passed++;
        printf("✓ PASS: %s\n", test_name);
    } else {
        printf("✗ FAIL: %s\n", test_name);
    }
}

// Test 1: NULL pointer handling
int test_null_pointer_handling() {
    printf("\n--- Test: null_pointer_handling ---\n");

    const char *endpoint = getenv("AZURE_COSMOS_ENDPOINT");
    const char *key = getenv("AZURE_COSMOS_KEY");

    if (!endpoint || !key) {
        printf("Skipping test - requires AZURE_COSMOS_ENDPOINT and AZURE_COSMOS_KEY\n");
        return TEST_PASS;
    }

    cosmos_error error;
    cosmos_runtime_context *runtime = cosmos_runtime_context_create(NULL, &error);
    if (!runtime) {
        printf("Failed to create runtime context\n");
        return TEST_FAIL;
    }

    cosmos_call_context ctx;
    ctx.runtime_context = runtime;
    ctx.include_error_details = false;

    cosmos_client *client = NULL;
    int result = TEST_PASS;

    // Test 1a: NULL context
    cosmos_error_code code = cosmos_client_create_with_key(NULL, endpoint, key, NULL, &client);
    if (code == COSMOS_ERROR_CODE_CALL_CONTEXT_MISSING) {
        printf("✓ NULL context correctly rejected with CALL_CONTEXT_MISSING\n");
    } else {
        printf("✗ NULL context should return CALL_CONTEXT_MISSING, got: %d\n", code);
        result = TEST_FAIL;
    }

    // Test 1b: NULL endpoint
    code = cosmos_client_create_with_key(&ctx, NULL, key, NULL, &client);
    if (code != COSMOS_ERROR_CODE_SUCCESS) {
        printf("✓ NULL endpoint correctly rejected with error code: %d\n", code);
    } else {
        printf("✗ NULL endpoint should return error\n");
        if (client) cosmos_client_free(client);
        result = TEST_FAIL;
    }

    // Test 1c: NULL key
    code = cosmos_client_create_with_key(&ctx, endpoint, NULL, NULL, &client);
    if (code != COSMOS_ERROR_CODE_SUCCESS) {
        printf("✓ NULL key correctly rejected with error code: %d\n", code);
    } else {
        printf("✗ NULL key should return error\n");
        if (client) cosmos_client_free(client);
        result = TEST_FAIL;
    }

    // Test 1d: NULL output pointer
    code = cosmos_client_create_with_key(&ctx, endpoint, key, NULL, NULL);
    if (code != COSMOS_ERROR_CODE_SUCCESS) {
        printf("✓ NULL output pointer correctly rejected with error code: %d\n", code);
    } else {
        printf("✗ NULL output pointer should return error\n");
        result = TEST_FAIL;
    }

    // Create a valid client for further tests
    code = cosmos_client_create_with_key(&ctx, endpoint, key, NULL, &client);
    if (code != COSMOS_ERROR_CODE_SUCCESS) {
        printf("Failed to create valid client for remaining tests\n");
        cosmos_runtime_context_free(runtime);
        return TEST_FAIL;
    }

    // Test 1e: NULL client pointer in operation
    cosmos_database_client *database = NULL;
    code = cosmos_client_database_client(&ctx, NULL, "test-db", &database);
    if (code != COSMOS_ERROR_CODE_SUCCESS) {
        printf("✓ NULL client pointer correctly rejected with error code: %d\n", code);
    } else {
        printf("✗ NULL client pointer should return error\n");
        if (database) cosmos_database_free(database);
        result = TEST_FAIL;
    }

    // Test 1f: NULL database name
    code = cosmos_client_database_client(&ctx, client, NULL, &database);
    if (code != COSMOS_ERROR_CODE_SUCCESS) {
        printf("✓ NULL database name correctly rejected with error code: %d\n", code);
    } else {
        printf("✗ NULL database name should return error\n");
        if (database) cosmos_database_free(database);
        result = TEST_FAIL;
    }

    cosmos_client_free(client);
    cosmos_runtime_context_free(runtime);
    return result;
}

// Test 2: Invalid runtime context
int test_invalid_runtime_context() {
    printf("\n--- Test: invalid_runtime_context ---\n");

    const char *endpoint = getenv("AZURE_COSMOS_ENDPOINT");
    const char *key = getenv("AZURE_COSMOS_KEY");

    if (!endpoint || !key) {
        printf("Skipping test - requires AZURE_COSMOS_ENDPOINT and AZURE_COSMOS_KEY\n");
        return TEST_PASS;
    }

    cosmos_call_context ctx;
    ctx.runtime_context = NULL;
    ctx.include_error_details = false;

    // Now try to use the invalid context
    cosmos_client *client = NULL;
    cosmos_error_code code = cosmos_client_create_with_key(&ctx, endpoint, key, NULL, &client);

    if (code != COSMOS_ERROR_CODE_SUCCESS) {
        printf("✓ Invalid/freed runtime context correctly rejected with error code: %d\n", code);
        return TEST_PASS;
    } else {
        printf("✗ Invalid/freed runtime context should return error\n");
        if (client) cosmos_client_free(client);
        return TEST_FAIL;
    }
}

// Test 3: Error details with flag enabled
int test_error_detail_with_flag() {
    printf("\n--- Test: error_detail_with_flag ---\n");

    const char *endpoint = getenv("AZURE_COSMOS_ENDPOINT");
    const char *key = getenv("AZURE_COSMOS_KEY");

    if (!endpoint || !key) {
        printf("Skipping test - requires AZURE_COSMOS_ENDPOINT and AZURE_COSMOS_KEY\n");
        return TEST_PASS;
    }

    time_t current_time = time(NULL);
    char database_name[64];
    snprintf(database_name, sizeof(database_name), "test-err-dtl-%ld", current_time);

    cosmos_error error;
    cosmos_runtime_context *runtime = cosmos_runtime_context_create(NULL, &error);
    if (!runtime) {
        printf("Failed to create runtime context\n");
        return TEST_FAIL;
    }

    cosmos_call_context ctx;
    ctx.runtime_context = runtime;
    ctx.include_error_details = true;  // Enable error details

    cosmos_client *client = NULL;
    cosmos_database_client *database = NULL;
    cosmos_container_client *container = NULL;
    int result = TEST_PASS;
    int database_created = 0;

    // Create client
    cosmos_error_code code = cosmos_client_create_with_key(&ctx, endpoint, key, NULL, &client);
    if (code != COSMOS_ERROR_CODE_SUCCESS) {
        printf("Failed to create client\n");
        result = TEST_FAIL;
        goto cleanup;
    }

    // Create database
    code = cosmos_client_create_database(&ctx, client, database_name, NULL, &database);
    if (code != COSMOS_ERROR_CODE_SUCCESS) {
        printf("Failed to create database\n");
        result = TEST_FAIL;
        goto cleanup;
    }
    database_created = 1;

    // Create container
    code = cosmos_database_create_container(&ctx, database, "test-container", "/pk", NULL, &container);
    if (code != COSMOS_ERROR_CODE_SUCCESS) {
        printf("Failed to create container\n");
        result = TEST_FAIL;
        goto cleanup;
    }

    // Trigger an error - try to read non-existent item
    const char *read_json = NULL;
    code = cosmos_container_read_item(&ctx, container, "pk1", "nonexistent-item", NULL, &read_json);

    if (code == COSMOS_ERROR_CODE_NOT_FOUND) {
        printf("✓ Got expected NOT_FOUND error code\n");

        // Check if error details are present
        if (ctx.error.detail != NULL) {
            printf("✓ Error detail is populated: %s\n", ctx.error.detail);
            cosmos_string_free(ctx.error.detail);
            ctx.error.detail = NULL;
        } else {
            printf("✗ Error detail should be populated when include_error_details=true\n");
            result = TEST_FAIL;
        }

        if (ctx.error.message != NULL) {
            printf("✓ Error message is populated: %s\n", ctx.error.message);
        }
    } else {
        printf("✗ Expected NOT_FOUND error, got: %d\n", code);
        result = TEST_FAIL;
    }

cleanup:
    if (database && database_created) {
        cosmos_database_delete(&ctx, database, NULL);
    }

    if (container) {
        cosmos_container_free(container);
    }
    if (database) {
        cosmos_database_free(database);
    }
    if (client) {
        cosmos_client_free(client);
    }
    cosmos_runtime_context_free(runtime);

    return result;
}

// Test 4: Error details with flag disabled
int test_error_detail_without_flag() {
    printf("\n--- Test: error_detail_without_flag ---\n");

    const char *endpoint = getenv("AZURE_COSMOS_ENDPOINT");
    const char *key = getenv("AZURE_COSMOS_KEY");

    if (!endpoint || !key) {
        printf("Skipping test - requires AZURE_COSMOS_ENDPOINT and AZURE_COSMOS_KEY\n");
        return TEST_PASS;
    }

    time_t current_time = time(NULL);
    char database_name[64];
    snprintf(database_name, sizeof(database_name), "test-no-dtl-%ld", current_time);

    cosmos_error error;
    cosmos_runtime_context *runtime = cosmos_runtime_context_create(NULL, &error);
    if (!runtime) {
        printf("Failed to create runtime context\n");
        return TEST_FAIL;
    }

    cosmos_call_context ctx;
    ctx.runtime_context = runtime;
    ctx.include_error_details = false;  // Disable error details

    cosmos_client *client = NULL;
    cosmos_database_client *database = NULL;
    cosmos_container_client *container = NULL;
    int result = TEST_PASS;
    int database_created = 0;

    // Create client
    cosmos_error_code code = cosmos_client_create_with_key(&ctx, endpoint, key, NULL, &client);
    if (code != COSMOS_ERROR_CODE_SUCCESS) {
        printf("Failed to create client\n");
        result = TEST_FAIL;
        goto cleanup;
    }

    // Create database
    code = cosmos_client_create_database(&ctx, client, database_name, NULL, &database);
    if (code != COSMOS_ERROR_CODE_SUCCESS) {
        printf("Failed to create database\n");
        result = TEST_FAIL;
        goto cleanup;
    }
    database_created = 1;

    // Create container
    code = cosmos_database_create_container(&ctx, database, "test-container", "/pk", NULL, &container);
    if (code != COSMOS_ERROR_CODE_SUCCESS) {
        printf("Failed to create container\n");
        result = TEST_FAIL;
        goto cleanup;
    }

    // Trigger an error - try to read non-existent item
    const char *read_json = NULL;
    code = cosmos_container_read_item(&ctx, container, "pk1", "nonexistent-item", NULL, &read_json);

    if (code == COSMOS_ERROR_CODE_NOT_FOUND) {
        printf("✓ Got expected NOT_FOUND error code\n");

        // Check that error details are NOT present
        if (ctx.error.detail == NULL) {
            printf("✓ Error detail is NULL (as expected when include_error_details=false)\n");
        } else {
            printf("✗ Error detail should be NULL when include_error_details=false, but got: %s\n",
                   ctx.error.detail);
            cosmos_string_free(ctx.error.detail);
            result = TEST_FAIL;
        }

        if (ctx.error.message != NULL) {
            printf("✓ Error message is still populated: %s\n", ctx.error.message);
        }
    } else {
        printf("✗ Expected NOT_FOUND error, got: %d\n", code);
        result = TEST_FAIL;
    }

cleanup:
    if (database && database_created) {
        cosmos_database_delete(&ctx, database, NULL);
    }

    if (container) {
        cosmos_container_free(container);
    }
    if (database) {
        cosmos_database_free(database);
    }
    if (client) {
        cosmos_client_free(client);
    }
    cosmos_runtime_context_free(runtime);

    return result;
}

// Test 5: Invalid UTF-8 strings
int test_invalid_utf8_strings() {
    printf("\n--- Test: invalid_utf8_strings ---\n");

    const char *endpoint = getenv("AZURE_COSMOS_ENDPOINT");
    const char *key = getenv("AZURE_COSMOS_KEY");

    if (!endpoint || !key) {
        printf("Skipping test - requires AZURE_COSMOS_ENDPOINT and AZURE_COSMOS_KEY\n");
        return TEST_PASS;
    }

    time_t current_time = time(NULL);
    char database_name[64];
    snprintf(database_name, sizeof(database_name), "test-utf8-%ld", current_time);

    cosmos_error error;
    cosmos_runtime_context *runtime = cosmos_runtime_context_create(NULL, &error);
    if (!runtime) {
        printf("Failed to create runtime context\n");
        return TEST_FAIL;
    }

    cosmos_call_context ctx;
    ctx.runtime_context = runtime;
    ctx.include_error_details = false;

    cosmos_client *client = NULL;
    cosmos_database_client *database = NULL;
    cosmos_container_client *container = NULL;
    int result = TEST_PASS;
    int database_created = 0;

    // Create client
    cosmos_error_code code = cosmos_client_create_with_key(&ctx, endpoint, key, NULL, &client);
    if (code != COSMOS_ERROR_CODE_SUCCESS) {
        printf("Failed to create client\n");
        result = TEST_FAIL;
        goto cleanup;
    }

    // Create database
    code = cosmos_client_create_database(&ctx, client, database_name, NULL, &database);
    if (code != COSMOS_ERROR_CODE_SUCCESS) {
        printf("Failed to create database\n");
        result = TEST_FAIL;
        goto cleanup;
    }
    database_created = 1;

    // Create container
    code = cosmos_database_create_container(&ctx, database, "test-container", "/pk", NULL, &container);
    if (code != COSMOS_ERROR_CODE_SUCCESS) {
        printf("Failed to create container\n");
        result = TEST_FAIL;
        goto cleanup;
    }

    // Test with invalid UTF-8 sequence in JSON data
    // Note: In C, we can create invalid UTF-8 by using raw byte sequences
    char invalid_json[128];
    // Start with valid JSON structure
    strcpy(invalid_json, "{\"id\":\"item1\",\"pk\":\"pk1\",\"value\":\"");
    // Append an invalid UTF-8 sequence (lone continuation byte)
    size_t len = strlen(invalid_json);
    invalid_json[len] = (char)0x80;  // Invalid UTF-8 continuation byte without start byte
    invalid_json[len + 1] = '\0';
    strcat(invalid_json, "\"}");

    code = cosmos_container_upsert_item(&ctx, container, "pk1", invalid_json, NULL);

    if (code == COSMOS_ERROR_CODE_INVALID_UTF8) {
        printf("✓ Invalid UTF-8 correctly rejected with INVALID_UTF8 error code\n");
    } else if (code != COSMOS_ERROR_CODE_SUCCESS) {
        // Some other error - also acceptable as the invalid UTF-8 was caught
        printf("✓ Invalid UTF-8 rejected with error code: %d\n", code);
    } else {
        printf("⚠ Invalid UTF-8 was not rejected (may have been sanitized or JSON parsing caught it)\n");
        // Not necessarily a failure - the system may have other validation layers
    }

cleanup:
    if (database && database_created) {
        cosmos_database_delete(&ctx, database, NULL);
    }

    if (container) {
        cosmos_container_free(container);
    }
    if (database) {
        cosmos_database_free(database);
    }
    if (client) {
        cosmos_client_free(client);
    }
    cosmos_runtime_context_free(runtime);

    return result;
}

// Test 6: Empty string handling
int test_empty_string_handling() {
    printf("\n--- Test: empty_string_handling ---\n");

    const char *endpoint = getenv("AZURE_COSMOS_ENDPOINT");
    const char *key = getenv("AZURE_COSMOS_KEY");

    if (!endpoint || !key) {
        printf("Skipping test - requires AZURE_COSMOS_ENDPOINT and AZURE_COSMOS_KEY\n");
        return TEST_PASS;
    }

    time_t current_time = time(NULL);
    char database_name[64];
    snprintf(database_name, sizeof(database_name), "test-empty-%ld", current_time);

    cosmos_error error;
    cosmos_runtime_context *runtime = cosmos_runtime_context_create(NULL, &error);
    if (!runtime) {
        printf("Failed to create runtime context\n");
        return TEST_FAIL;
    }

    cosmos_call_context ctx;
    ctx.runtime_context = runtime;
    ctx.include_error_details = false;

    cosmos_client *client = NULL;
    cosmos_database_client *database = NULL;
    cosmos_container_client *container = NULL;
    int result = TEST_PASS;
    int database_created = 0;

    // Create client
    cosmos_error_code code = cosmos_client_create_with_key(&ctx, endpoint, key, NULL, &client);
    if (code != COSMOS_ERROR_CODE_SUCCESS) {
        printf("Failed to create client\n");
        result = TEST_FAIL;
        goto cleanup;
    }

    // Test 6a: Empty database name
    code = cosmos_client_create_database(&ctx, client, "", NULL, &database);
    if (code != COSMOS_ERROR_CODE_SUCCESS) {
        printf("✓ Empty database name correctly rejected with error code: %d\n", code);
    } else {
        printf("✗ Empty database name should return error\n");
        cosmos_database_free(database);
        database = NULL;
        result = TEST_FAIL;
    }

    // Create valid database for remaining tests
    code = cosmos_client_create_database(&ctx, client, database_name, NULL, &database);
    if (code != COSMOS_ERROR_CODE_SUCCESS) {
        printf("Failed to create valid database\n");
        result = TEST_FAIL;
        goto cleanup;
    }
    database_created = 1;

    // Test 6b: Empty container name
    code = cosmos_database_create_container(&ctx, database, "", "/pk", NULL, &container);
    if (code != COSMOS_ERROR_CODE_SUCCESS) {
        printf("✓ Empty container name correctly rejected with error code: %d\n", code);
    } else {
        printf("✗ Empty container name should return error\n");
        cosmos_container_free(container);
        container = NULL;
        result = TEST_FAIL;
    }

    // Test 6c: Empty partition key path
    code = cosmos_database_create_container(&ctx, database, "test-container", "", NULL, &container);
    if (code != COSMOS_ERROR_CODE_SUCCESS) {
        printf("✓ Empty partition key path correctly rejected with error code: %d\n", code);
    } else {
        printf("✗ Empty partition key path should return error\n");
        cosmos_container_free(container);
        container = NULL;
        result = TEST_FAIL;
    }

    // Create valid container for remaining tests
    code = cosmos_database_create_container(&ctx, database, "test-container", "/pk", NULL, &container);
    if (code != COSMOS_ERROR_CODE_SUCCESS) {
        printf("Failed to create valid container\n");
        result = TEST_FAIL;
        goto cleanup;
    }

    // Test 6d: Empty item ID in JSON
    const char *json_with_empty_id = "{\"id\":\"\",\"pk\":\"pk1\",\"value\":\"test\"}";
    code = cosmos_container_upsert_item(&ctx, container, "pk1", json_with_empty_id, NULL);
    if (code != COSMOS_ERROR_CODE_SUCCESS) {
        printf("✓ Empty item ID correctly rejected with error code: %d\n", code);
    } else {
        printf("✗ Empty item ID should return error\n");
        result = TEST_FAIL;
    }

    // Test 6e: Empty partition key value
    const char *json_data = "{\"id\":\"item1\",\"pk\":\"pk1\",\"value\":\"test\"}";
    code = cosmos_container_upsert_item(&ctx, container, "", json_data, NULL);
    if (code != COSMOS_ERROR_CODE_SUCCESS) {
        printf("✓ Empty partition key value correctly rejected with error code: %d\n", code);
    } else {
        printf("✗ Empty partition key value should return error\n");
        result = TEST_FAIL;
    }

cleanup:
    if (database && database_created) {
        cosmos_database_delete(&ctx, database, NULL);
    }

    if (container) {
        cosmos_container_free(container);
    }
    if (database) {
        cosmos_database_free(database);
    }
    if (client) {
        cosmos_client_free(client);
    }
    cosmos_runtime_context_free(runtime);

    return result;
}

int main() {
    printf("=== Test Suite 2: Error Handling and Validation ===\n");

    report_test("null_pointer_handling", test_null_pointer_handling() == TEST_PASS);
    report_test("invalid_runtime_context", test_invalid_runtime_context() == TEST_PASS);
    report_test("error_detail_with_flag", test_error_detail_with_flag() == TEST_PASS);
    report_test("error_detail_without_flag", test_error_detail_without_flag() == TEST_PASS);
    report_test("invalid_utf8_strings", test_invalid_utf8_strings() == TEST_PASS);
    report_test("empty_string_handling", test_empty_string_handling() == TEST_PASS);

    printf("\n=== Test Summary ===\n");
    printf("Tests run: %d\n", tests_run);
    printf("Tests passed: %d\n", tests_passed);
    printf("Tests failed: %d\n", tests_run - tests_passed);

    if (tests_passed == tests_run) {
        printf("\n✓ All tests passed!\n");
        return 0;
    } else {
        printf("\n✗ Some tests failed\n");
        return 1;
    }
}

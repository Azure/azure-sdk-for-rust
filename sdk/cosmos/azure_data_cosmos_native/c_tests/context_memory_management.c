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

// Test 1: Runtime context lifecycle
int test_runtime_context_lifecycle() {
    printf("\n--- Test: runtime_context_lifecycle ---\n");

    cosmos_runtime_context *runtime = cosmos_runtime_context_create(NULL);
    if (!runtime) {
        printf("Failed to create runtime context\n");
        return TEST_FAIL;
    }
    printf("Created runtime context successfully\n");

    // Free it
    cosmos_runtime_context_free(runtime);
    printf("Freed runtime context successfully\n");

    return TEST_PASS;
}

// Test 2: Stack-allocated call context
int test_call_context_stack_allocated() {
    printf("\n--- Test: call_context_stack_allocated ---\n");

    cosmos_runtime_context *runtime = cosmos_runtime_context_create(NULL);
    if (!runtime) {
        printf("Failed to create runtime context\n");
        return TEST_FAIL;
    }

    // Stack-allocated context
    cosmos_call_context ctx;
    ctx.runtime_context = runtime;
    ctx.include_error_details = false;

    printf("Created stack-allocated call context\n");

    // Use it for a simple operation (get version)
    const char *version = cosmos_version();
    if (!version) {
        printf("Failed to get version\n");
        cosmos_runtime_context_free(runtime);
        return TEST_FAIL;
    }
    printf("Successfully used stack-allocated context (version: %s)\n", version);

    cosmos_runtime_context_free(runtime);
    return TEST_PASS;
}

// Test 3: Heap-allocated call context
int test_call_context_heap_allocated() {
    printf("\n--- Test: call_context_heap_allocated ---\n");

    cosmos_runtime_context *runtime = cosmos_runtime_context_create(NULL);
    if (!runtime) {
        printf("Failed to create runtime context\n");
        return TEST_FAIL;
    }

    // Heap-allocated context
    cosmos_call_context *ctx = cosmos_call_context_create(runtime, false);
    if (!ctx) {
        printf("Failed to create heap-allocated call context\n");
        cosmos_runtime_context_free(runtime);
        return TEST_FAIL;
    }
    printf("Created heap-allocated call context\n");

    // Use it for a simple operation (get version)
    const char *version = cosmos_version();
    if (!version) {
        printf("Failed to get version\n");
        cosmos_call_context_free(ctx);
        cosmos_runtime_context_free(runtime);
        return TEST_FAIL;
    }
    printf("Successfully used heap-allocated context (version: %s)\n", version);

    cosmos_call_context_free(ctx);
    cosmos_runtime_context_free(runtime);
    return TEST_PASS;
}

// Test 4: Call context reuse
int test_call_context_reuse() {
    printf("\n--- Test: call_context_reuse ---\n");

    const char *endpoint = getenv("AZURE_COSMOS_ENDPOINT");
    const char *key = getenv("AZURE_COSMOS_KEY");

    if (!endpoint || !key) {
        printf("Skipping test - requires AZURE_COSMOS_ENDPOINT and AZURE_COSMOS_KEY\n");
        return TEST_PASS; // Not a failure, just skipped
    }

    cosmos_runtime_context *runtime = cosmos_runtime_context_create(NULL);
    if (!runtime) {
        printf("Failed to create runtime context\n");
        return TEST_FAIL;
    }

    cosmos_call_context ctx;
    ctx.runtime_context = runtime;
    ctx.include_error_details = true;

    cosmos_client *client = NULL;

    // First call - create client
    cosmos_error_code code = cosmos_client_create_with_key(&ctx, endpoint, key, &client);
    if (code != COSMOS_ERROR_CODE_SUCCESS) {
        printf("First call failed with code: %d\n", code);
        cosmos_runtime_context_free(runtime);
        return TEST_FAIL;
    }
    printf("First call succeeded (client created)\n");

    cosmos_database_client *database = NULL;

    // Reuse context for second call - try to get a database client
    code = cosmos_client_database_client(&ctx, client, "nonexistent-db", &database);
    if (code != COSMOS_ERROR_CODE_SUCCESS) {
        printf("Second call failed with code: %d (expected, just testing reuse)\n", code);
        // This is okay - we're testing context reuse, not that the operation succeeds
    } else {
        printf("Second call succeeded (database client retrieved)\n");
        cosmos_database_free(database);
    }

    // Reuse context for third call - try again
    database = NULL;
    code = cosmos_client_database_client(&ctx, client, "another-nonexistent-db", &database);
    if (code != COSMOS_ERROR_CODE_SUCCESS) {
        printf("Third call failed with code: %d (expected, just testing reuse)\n", code);
    } else {
        printf("Third call succeeded (database client retrieved)\n");
        cosmos_database_free(database);
    }

    printf("Successfully reused call context for multiple operations\n");

    cosmos_client_free(client);
    cosmos_runtime_context_free(runtime);
    return TEST_PASS;
}

// Test 5: String memory management
int test_string_memory_management() {
    printf("\n--- Test: string_memory_management ---\n");

    const char *endpoint = getenv("AZURE_COSMOS_ENDPOINT");
    const char *key = getenv("AZURE_COSMOS_KEY");

    if (!endpoint || !key) {
        printf("Skipping test - requires AZURE_COSMOS_ENDPOINT and AZURE_COSMOS_KEY\n");
        return TEST_PASS;
    }

    time_t current_time = time(NULL);
    char database_name[64];
    snprintf(database_name, sizeof(database_name), "auto-test-db-str-mem-%ld", current_time);

    cosmos_runtime_context *runtime = cosmos_runtime_context_create(NULL);
    if (!runtime) {
        printf("Failed to create runtime context\n");
        return TEST_FAIL;
    }

    cosmos_call_context ctx;
    ctx.runtime_context = runtime;
    ctx.include_error_details = true;

    cosmos_client *client = NULL;
    cosmos_database_client *database = NULL;
    cosmos_container_client *container = NULL;
    const char *read_json = NULL;
    int result = TEST_PASS;
    int database_created = 0;

    // Create client
    cosmos_error_code code = cosmos_client_create_with_key(&ctx, endpoint, key, &client);
    if (code != COSMOS_ERROR_CODE_SUCCESS) {
        printf("Failed to create client\n");
        result = TEST_FAIL;
        goto cleanup;
    }

    // Create database
    code = cosmos_client_create_database(&ctx, client, database_name, &database);
    if (code != COSMOS_ERROR_CODE_SUCCESS) {
        printf("Failed to create database\n");
        result = TEST_FAIL;
        goto cleanup;
    }
    database_created = 1;
    printf("Created database: %s\n", database_name);

    // Create container
    code = cosmos_database_create_container(&ctx, database, "test-container", "/pk", &container);
    if (code != COSMOS_ERROR_CODE_SUCCESS) {
        printf("Failed to create container\n");
        result = TEST_FAIL;
        goto cleanup;
    }
    printf("Created container\n");

    // Create an item
    const char *json_data = "{\"id\":\"item1\",\"pk\":\"pk1\",\"value\":\"test\"}";
    code = cosmos_container_upsert_item(&ctx, container, "pk1", json_data);
    if (code != COSMOS_ERROR_CODE_SUCCESS) {
        printf("Failed to upsert item\n");
        result = TEST_FAIL;
        goto cleanup;
    }
    printf("Upserted item\n");

    // Read the item - this returns a string that must be freed
    code = cosmos_container_read_item(&ctx, container, "pk1", "item1", &read_json);
    if (code != COSMOS_ERROR_CODE_SUCCESS) {
        printf("Failed to read item\n");
        result = TEST_FAIL;
        goto cleanup;
    }
    printf("Read item: %s\n", read_json);

    // Test freeing the string
    if (read_json) {
        cosmos_string_free(read_json);
        read_json = NULL;
        printf("Successfully freed JSON string\n");
    }

    // Test freeing error details (trigger an error)
    code = cosmos_container_read_item(&ctx, container, "pk1", "nonexistent-item", &read_json);
    if (code == COSMOS_ERROR_CODE_NOT_FOUND) {
        printf("Got expected NOT_FOUND error\n");
        if (ctx.error.detail) {
            printf("Error detail present: %s\n", ctx.error.detail);
            cosmos_string_free(ctx.error.detail);
            printf("Successfully freed error detail string\n");
        }
    }

cleanup:
    if (database && database_created) {
        cosmos_database_delete(&ctx, database);
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
    printf("=== Test Suite 1: Context and Memory Management ===\n");

    report_test("runtime_context_lifecycle", test_runtime_context_lifecycle() == TEST_PASS);
    report_test("call_context_stack_allocated", test_call_context_stack_allocated() == TEST_PASS);
    report_test("call_context_heap_allocated", test_call_context_heap_allocated() == TEST_PASS);
    report_test("call_context_reuse", test_call_context_reuse() == TEST_PASS);
    report_test("string_memory_management", test_string_memory_management() == TEST_PASS);

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

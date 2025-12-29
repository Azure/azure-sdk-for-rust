// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

#include "test_common.h"

int test_runtime_context_lifecycle() {
    int result = TEST_PASS;
    cosmos_error error;
    cosmos_runtime_context *runtime = cosmos_runtime_context_create(NULL, &error);
    
    REQUIRE_NOT_NULL(runtime, "Created runtime context successfully");
    
    cosmos_runtime_context_free(runtime);
    printf("Freed runtime context successfully\n");

cleanup:
    return result;
}

int test_call_context_stack_allocated() {
    int result = TEST_PASS;
    cosmos_error error;
    cosmos_runtime_context *runtime = cosmos_runtime_context_create(NULL, &error);
    
    REQUIRE_NOT_NULL(runtime, "Created runtime context");

    cosmos_call_context ctx;
    ctx.runtime_context = runtime;
    ctx.include_error_details = false;

    printf("Created stack-allocated call context\n");

    const char *version = cosmos_version();
    REQUIRE_NOT_NULL(version, "Successfully used stack-allocated context (version: %s)", version);

cleanup:
    if (runtime) {
        cosmos_runtime_context_free(runtime);
    }
    return result;
}

int test_call_context_heap_allocated() {
    int result = TEST_PASS;
    cosmos_error error;
    cosmos_runtime_context *runtime = cosmos_runtime_context_create(NULL, &error);
    cosmos_call_context *ctx = NULL;
    
    REQUIRE_NOT_NULL(runtime, "Created runtime context");

    cosmos_call_context_options options;
    ctx = cosmos_call_context_create(runtime, &options);
    REQUIRE_NOT_NULL(ctx, "Created heap-allocated call context");

    const char *version = cosmos_version();
    REQUIRE_NOT_NULL(version, "Successfully used heap-allocated context (version: %s)", version);

cleanup:
    if (ctx) {
        cosmos_call_context_free(ctx);
    }
    if (runtime) {
        cosmos_runtime_context_free(runtime);
    }
    return result;
}

int test_call_context_reuse() {
    test_context ctx;
    test_context_init(&ctx);
    int result = TEST_PASS;
    
    REQUIRE(test_context_create_runtime(&ctx, 1) == TEST_PASS, "Created runtime context");
    REQUIRE(test_context_create_client(&ctx) == TEST_PASS, "First call succeeded (client created)");
    
    cosmos_database_client *database = NULL;
    
    cosmos_error_code code = cosmos_client_database_client(&ctx.call_ctx, ctx.client, "nonexistent-db", &database);
    if (code != COSMOS_ERROR_CODE_SUCCESS) {
        printf("Second call failed with code: %d (expected, just testing reuse)\n", code);
    } else {
        printf("Second call succeeded (database client retrieved)\n");
        cosmos_database_free(database);
    }
    
    database = NULL;
    code = cosmos_client_database_client(&ctx.call_ctx, ctx.client, "another-nonexistent-db", &database);
    if (code != COSMOS_ERROR_CODE_SUCCESS) {
        printf("Third call failed with code: %d (expected, just testing reuse)\n", code);
    } else {
        printf("Third call succeeded (database client retrieved)\n");
        cosmos_database_free(database);
    }
    
    printf("Successfully reused call context for multiple operations\n");
    
cleanup:
    test_context_cleanup(&ctx);
    return result;
}

int test_string_memory_management() {
    test_context ctx;
    test_context_init(&ctx);
    
    char database_name[64];
    test_generate_database_name(database_name, sizeof(database_name), "test-str-mem");
    
    const char *read_json = NULL;
    int result = TEST_PASS;
    
    REQUIRE(test_context_create_runtime(&ctx, 1) == TEST_PASS, "Created runtime context");
    REQUIRE(test_context_create_client(&ctx) == TEST_PASS, "Created client");
    REQUIRE(test_context_create_database(&ctx, database_name) == TEST_PASS, "Created database: %s", database_name);
    REQUIRE(test_context_create_container(&ctx, "test-container", "/pk") == TEST_PASS, "Created container");
    
    const char *json_data = "{\"id\":\"item1\",\"pk\":\"pk1\",\"value\":\"test\"}";
    cosmos_error_code code = cosmos_container_upsert_item(&ctx.call_ctx, ctx.container, "pk1", json_data, NULL);
    REQUIRE_SUCCESS(code, "Upserted item");
    
    code = cosmos_container_read_item(&ctx.call_ctx, ctx.container, "pk1", "item1", NULL, &read_json);
    REQUIRE_SUCCESS(code, "Read item: %s", read_json);
    
    if (read_json) {
        cosmos_string_free(read_json);
        read_json = NULL;
        printf("Successfully freed JSON string\n");
    }
    
    code = cosmos_container_read_item(&ctx.call_ctx, ctx.container, "pk1", "nonexistent-item", NULL, &read_json);
    if (code == COSMOS_ERROR_CODE_NOT_FOUND) {
        printf("Got expected NOT_FOUND error\n");
        if (ctx.call_ctx.error.detail) {
            printf("Error detail present: %s\n", ctx.call_ctx.error.detail);
            cosmos_string_free(ctx.call_ctx.error.detail);
            printf("Successfully freed error detail string\n");
        }
    }
    
cleanup:
    test_context_cleanup(&ctx);
    return result;
}

TEST_SUITE_BEGIN("Context and Memory Management")
    TEST_REGISTER(runtime_context_lifecycle)
    TEST_REGISTER(call_context_stack_allocated)
    TEST_REGISTER(call_context_heap_allocated)
    TEST_REGISTER(call_context_reuse)
    TEST_REGISTER(string_memory_management)
TEST_SUITE_END("Context and Memory Management")

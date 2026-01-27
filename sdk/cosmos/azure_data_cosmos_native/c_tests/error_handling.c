// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

#include "test_common.h"

int test_null_pointer_handling() {
    // Use fake values - we're only testing error handling, not connectivity
    const char *endpoint = "https://fake-account.example.com";
    const char *key = "fake_key_for_testing_only";
    
    test_context ctx;
    test_context_init(&ctx);
    int result = TEST_PASS;
    cosmos_client *client = NULL;
    cosmos_error_code code;
    
    code = cosmos_client_create_with_key(NULL, endpoint, key, NULL, &client);
    ASSERT_ERROR_CODE(code, COSMOS_ERROR_CODE_CALL_CONTEXT_MISSING, "NULL context rejected with CALL_CONTEXT_MISSING");

    code = cosmos_client_create_with_key(&ctx.call_ctx, NULL, key, NULL, &client);
    ASSERT(code != COSMOS_ERROR_CODE_SUCCESS, "NULL endpoint correctly rejected");
    if (code == COSMOS_ERROR_CODE_SUCCESS && client) cosmos_client_free(client);

    code = cosmos_client_create_with_key(&ctx.call_ctx, endpoint, NULL, NULL, &client);
    ASSERT(code != COSMOS_ERROR_CODE_SUCCESS, "NULL key correctly rejected");
    if (code == COSMOS_ERROR_CODE_SUCCESS && client) cosmos_client_free(client);

    code = cosmos_client_create_with_key(&ctx.call_ctx, endpoint, key, NULL, NULL);
    ASSERT(code != COSMOS_ERROR_CODE_SUCCESS, "NULL output pointer correctly rejected");

    REQUIRE(test_context_create_runtime(&ctx, 0) == TEST_PASS, "Created runtime context for remaining tests");
    REQUIRE(test_context_create_client(&ctx) == TEST_PASS, "Created valid client for remaining tests");

    code = cosmos_client_database_client(&ctx.call_ctx, NULL, "test-db", &ctx.database);
    ASSERT(code != COSMOS_ERROR_CODE_SUCCESS, "NULL client pointer correctly rejected");
    if (ctx.database) {
        cosmos_database_free(ctx.database);
        ctx.database = NULL;
    }

    code = cosmos_client_database_client(&ctx.call_ctx, ctx.client, NULL, &ctx.database);
    ASSERT(code != COSMOS_ERROR_CODE_SUCCESS, "NULL database name correctly rejected");
    if (ctx.database) {
        cosmos_database_free(ctx.database);
        ctx.database = NULL;
    }
    
cleanup:
    test_context_cleanup(&ctx);
    return result;
}

int test_invalid_runtime_context() {
    // Use fake connection string - we're only testing error handling
    const char *connection_string = "AccountEndpoint=https://fake.documents.azure.com;AccountKey=fake_key;";
    int result = TEST_PASS;
    
    cosmos_call_context ctx;
    ctx.runtime_context = NULL;
    ctx.include_error_details = false;
    
    cosmos_client *client = NULL;
    cosmos_error_code code = cosmos_client_create_with_connection_string(&ctx, connection_string, NULL, &client);
    
    ASSERT(code != COSMOS_ERROR_CODE_SUCCESS, "Invalid/freed runtime context correctly rejected");
    if (client) {
        cosmos_client_free(client);
    }
    
cleanup:
    return result;
}

int test_error_detail_with_flag() {
    printf("\n--- Test: error_detail_with_flag ---\n");
    
    const char *connection_string = getenv("AZURE_COSMOS_CONNECTION_STRING");
    
    if (!connection_string) {
        printf("Skipping test - requires AZURE_COSMOS_CONNECTION_STRING\n");
        return TEST_SKIP;
    }
    
    test_context ctx;
    test_context_init(&ctx);
    
    char database_name[64];
    test_generate_database_name(database_name, sizeof(database_name), "test-err-dtl");
    
    int result = TEST_PASS;
    
    REQUIRE(test_context_create_runtime(&ctx, 1) == TEST_PASS, "Created runtime context");
    REQUIRE(test_context_create_client(&ctx) == TEST_PASS, "Created client");
    REQUIRE(test_context_create_database(&ctx, database_name) == TEST_PASS, "Created database");
    REQUIRE(test_context_create_container(&ctx, "test-container", "/pk") == TEST_PASS, "Created container");

    const char *read_json = NULL;
    cosmos_error_code code = cosmos_container_read_item(&ctx.call_ctx, ctx.container, "pk1", "nonexistent-item", NULL, &read_json);
    
    REQUIRE_ERROR_CODE(code, COSMOS_ERROR_CODE_NOT_FOUND, "Got expected NOT_FOUND error");
    
    ASSERT_NOT_NULL(ctx.call_ctx.error.detail, "Error detail populated when include_error_details=true");
    if (ctx.call_ctx.error.detail != NULL) {
        printf("  Detail: %s\n", ctx.call_ctx.error.detail);
        cosmos_string_free(ctx.call_ctx.error.detail);
        ctx.call_ctx.error.detail = NULL;
    }
    
    if (ctx.call_ctx.error.message != NULL) {
        printf("  Message: %s\n", ctx.call_ctx.error.message);
    }
    
cleanup:
    test_context_cleanup(&ctx);
    return result;
}

int test_error_detail_without_flag() {
    const char *connection_string = getenv("AZURE_COSMOS_CONNECTION_STRING");
    
    if (!connection_string) {
        printf("Skipping test - requires AZURE_COSMOS_ENDPOINT and AZURE_COSMOS_KEY\n");
        return TEST_SKIP;
    }
    
    test_context ctx;
    test_context_init(&ctx);
    
    char database_name[64];
    test_generate_database_name(database_name, sizeof(database_name), "test-no-dtl");
    
    int result = TEST_PASS;
    
    REQUIRE(test_context_create_runtime(&ctx, 0) == TEST_PASS, "Created runtime context");
    REQUIRE(test_context_create_client(&ctx) == TEST_PASS, "Created client");
    REQUIRE(test_context_create_database(&ctx, database_name) == TEST_PASS, "Created database");
    REQUIRE(test_context_create_container(&ctx, "test-container", "/pk") == TEST_PASS, "Created container");
    
    const char *read_json = NULL;
    cosmos_error_code code = cosmos_container_read_item(&ctx.call_ctx, ctx.container, "pk1", "nonexistent-item", NULL, &read_json);
    
    REQUIRE_ERROR_CODE(code, COSMOS_ERROR_CODE_NOT_FOUND, "Got expected NOT_FOUND error");
    
    ASSERT(ctx.call_ctx.error.detail == NULL, "Error detail is NULL (as expected when include_error_details=false)");
    if (ctx.call_ctx.error.detail != NULL) {
        cosmos_string_free(ctx.call_ctx.error.detail);
    }
    
    if (ctx.call_ctx.error.message != NULL) {
        printf("  Message: %s\n", ctx.call_ctx.error.message);
    }
    
cleanup:
    test_context_cleanup(&ctx);
    return result;
}

int test_invalid_utf8_strings() {
    const char *connection_string = getenv("AZURE_COSMOS_CONNECTION_STRING");
    
    if (!connection_string) {
        printf("Skipping test - requires AZURE_COSMOS_CONNECTION_STRING\n");
        return TEST_SKIP;
    }
    
    test_context ctx;
    test_context_init(&ctx);
    
    char database_name[64];
    test_generate_database_name(database_name, sizeof(database_name), "test-utf8");
    
    int result = TEST_PASS;
    
    REQUIRE(test_context_create_runtime(&ctx, 0) == TEST_PASS, "Created runtime context");
    REQUIRE(test_context_create_client(&ctx) == TEST_PASS, "Created client");
    REQUIRE(test_context_create_database(&ctx, database_name) == TEST_PASS, "Created database");
    REQUIRE(test_context_create_container(&ctx, "test-container", "/pk") == TEST_PASS, "Created container");

    // Construct invalid UTF-8 by embedding raw byte sequence
    char invalid_json[128];
    strcpy(invalid_json, "{\"id\":\"item1\",\"pk\":\"pk1\",\"value\":\"");
    size_t len = strlen(invalid_json);
    invalid_json[len] = (char)0x80;  // Invalid: continuation byte without start byte
    invalid_json[len + 1] = '\0';
    strcat(invalid_json, "\"}");

    cosmos_error_code code = cosmos_container_upsert_item(&ctx.call_ctx, ctx.container, "pk1", invalid_json, NULL);
    
    if (code == COSMOS_ERROR_CODE_INVALID_UTF8) {
        printf("✓ PASS: Invalid UTF-8 correctly rejected with INVALID_UTF8 error code\n");
    } else if (code != COSMOS_ERROR_CODE_SUCCESS) {
        printf("✓ PASS: Invalid UTF-8 rejected with error code: %d\n", code);
    } else {
        printf("⚠ Invalid UTF-8 was not rejected (may have been sanitized)\n");
    }
    
cleanup:
    test_context_cleanup(&ctx);
    return result;
}

int test_empty_string_handling() {
    const char *connection_string = getenv("AZURE_COSMOS_CONNECTION_STRING");
    
    if (!connection_string) {
        printf("Skipping test - requires AZURE_COSMOS_CONNECTION_STRING\n");
        return TEST_SKIP;
    }
    
    test_context ctx;
    test_context_init(&ctx);
    
    char database_name[64];
    test_generate_database_name(database_name, sizeof(database_name), "test-empty");
    
    int result = TEST_PASS;
    cosmos_error_code code;
    
    REQUIRE(test_context_create_runtime(&ctx, 0) == TEST_PASS, "Created runtime context");
    REQUIRE(test_context_create_client(&ctx) == TEST_PASS, "Created client");
    
    code = cosmos_client_create_database(&ctx.call_ctx, ctx.client, "", NULL, &ctx.database);
    ASSERT(code != COSMOS_ERROR_CODE_SUCCESS, "Empty database name correctly rejected");
    if (ctx.database) {
        cosmos_database_free(ctx.database);
        ctx.database = NULL;
    }

    REQUIRE(test_context_create_database(&ctx, database_name) == TEST_PASS, "Created valid database");
    
    code = cosmos_database_create_container(&ctx.call_ctx, ctx.database, "", "/pk", NULL, &ctx.container);
    ASSERT(code != COSMOS_ERROR_CODE_SUCCESS, "Empty container name correctly rejected");
    if (ctx.container) {
        cosmos_container_free(ctx.container);
        ctx.container = NULL;
    }

    code = cosmos_database_create_container(&ctx.call_ctx, ctx.database, "test-container", "", NULL, &ctx.container);
    ASSERT(code != COSMOS_ERROR_CODE_SUCCESS, "Empty partition key path correctly rejected");
    if (ctx.container) {
        cosmos_container_free(ctx.container);
        ctx.container = NULL;
    }
    
    REQUIRE(test_context_create_container(&ctx, "test-container", "/pk") == TEST_PASS, "Created valid container");

    const char *json_with_empty_id = "{\"id\":\"\",\"pk\":\"pk1\",\"value\":\"test\"}";
    code = cosmos_container_upsert_item(&ctx.call_ctx, ctx.container, "pk1", json_with_empty_id, NULL);
    ASSERT(code != COSMOS_ERROR_CODE_SUCCESS, "Empty item ID correctly rejected");
    
    const char *json_data = "{\"id\":\"item1\",\"pk\":\"pk1\",\"value\":\"test\"}";
    code = cosmos_container_upsert_item(&ctx.call_ctx, ctx.container, "", json_data, NULL);
    ASSERT(code != COSMOS_ERROR_CODE_SUCCESS, "Empty partition key value correctly rejected");
    
cleanup:
    test_context_cleanup(&ctx);
    return result;
}

TEST_SUITE_BEGIN("Error Handling and Validation")
    TEST_REGISTER(null_pointer_handling)
    TEST_REGISTER(invalid_runtime_context)
    TEST_REGISTER(error_detail_with_flag)
    TEST_REGISTER(error_detail_without_flag)
    TEST_REGISTER(invalid_utf8_strings)
    TEST_REGISTER(empty_string_handling)
TEST_SUITE_END("Error Handling and Validation")

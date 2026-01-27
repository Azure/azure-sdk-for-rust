// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

#include "test_common.h"

#define SENTINEL_VALUE "test-sentinel-12345"
#define ITEM_ID "test-item-id"
#define PARTITION_KEY_VALUE "test-partition"
#define PARTITION_KEY_PATH "/partitionKey"

int test_item_crud() {
    test_context ctx;
    test_context_init(&ctx);
    
    char database_name[64];
    test_generate_database_name(database_name, sizeof(database_name), "test-item-crud");
    
    const char *read_json = NULL;
    int result = TEST_PASS;
    
    printf("Database: %s\n", database_name);
    printf("Container: test-container\n");
    
    REQUIRE(test_context_create_runtime(&ctx, 1) == TEST_PASS, "Created runtime context");
    REQUIRE(test_context_create_client(&ctx) == TEST_PASS, "Created Cosmos client");
    REQUIRE(test_context_create_database(&ctx, database_name) == TEST_PASS, "Created database: %s", database_name);
    REQUIRE(test_context_create_container(&ctx, "test-container", PARTITION_KEY_PATH) == TEST_PASS, 
            "Created container with partition key: %s", PARTITION_KEY_PATH);
    
    char json_data[512];
    snprintf(json_data, sizeof(json_data),
        "{\"id\":\"%s\",\"partitionKey\":\"%s\",\"name\":\"Test Document\",\"sentinel\":\"%s\",\"description\":\"This is a test document for CRUD operations\"}",
        ITEM_ID, PARTITION_KEY_VALUE, SENTINEL_VALUE);
    
    printf("Upserting document: %s\n", json_data);
    
    cosmos_error_code code = cosmos_container_upsert_item(&ctx.call_ctx, ctx.container, PARTITION_KEY_VALUE, json_data, NULL);
    REQUIRE_SUCCESS(code, "Upserted item successfully");
    
    code = cosmos_container_read_item(&ctx.call_ctx, ctx.container, PARTITION_KEY_VALUE, ITEM_ID, NULL, &read_json);
    REQUIRE_SUCCESS(code, "Read item successfully");
    
    printf("Read back JSON: %s\n", read_json);
    
    REQUIRE(strstr(read_json, SENTINEL_VALUE) != NULL, "Sentinel value '%s' found in returned JSON", SENTINEL_VALUE);
    REQUIRE(strstr(read_json, ITEM_ID) != NULL, "Item ID '%s' found in returned JSON", ITEM_ID);
    
cleanup:
    if (read_json) {
        cosmos_string_free(read_json);
    }
    
    test_context_cleanup(&ctx);
    return result;
}

TEST_SUITE_BEGIN("Item CRUD")
    TEST_REGISTER(item_crud)
TEST_SUITE_END("Item CRUD")

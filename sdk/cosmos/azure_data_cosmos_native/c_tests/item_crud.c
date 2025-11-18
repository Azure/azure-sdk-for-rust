// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>
#include "../include/azurecosmos.h"

#define SENTINEL_VALUE "test-sentinel-12345"
#define ITEM_ID "test-item-id"
#define PARTITION_KEY_VALUE "test-partition"
#define PARTITION_KEY_PATH "/partitionKey"

void display_error(const cosmos_error *error) {
    printf("Error Code: %d\n", error->code);
    if (error->message) {
        printf("Error Message: %s\n", error->message);
    }
    if (error->detail) {
        printf("Error Details: %s\n", error->detail);
        cosmos_string_free(error->detail);
    }
}

int main() {
    cosmos_enable_tracing();

    // Get environment variables (only endpoint and key required)
    const char *endpoint = getenv("AZURE_COSMOS_ENDPOINT");
    const char *key = getenv("AZURE_COSMOS_KEY");

    if (!endpoint || !key) {
        printf("Error: Missing required environment variables.\n");
        printf("Required: AZURE_COSMOS_ENDPOINT, AZURE_COSMOS_KEY\n");
        return 1;
    }

    // Generate unique database and container names using timestamp
    time_t current_time = time(NULL);
    char database_name[64];
    snprintf(database_name, sizeof(database_name), "auto-test-db-%ld", current_time);

    printf("Running Cosmos DB item CRUD test...\n");
    printf("Endpoint: %s\n", endpoint);
    printf("Database: %s\n", database_name);
    printf("Container: test-container\n");

    cosmos_runtime_context *runtime = cosmos_runtime_context_create(NULL);
    if (!runtime) {
        printf("Failed to create runtime context\n");
        return 1;
    }
    cosmos_call_context ctx;
    ctx.runtime_context = runtime;
    ctx.include_error_details = true;

    cosmos_client *client = NULL;
    cosmos_database_client *database = NULL;
    cosmos_container_client *container = NULL;
    const char *read_json = NULL;
    int result = 0;
    int database_created = 0;
    int container_created = 0;

    // Create Cosmos client
    cosmos_error_code code = cosmos_client_create_with_key(&ctx, endpoint, key, &client);
    if (code != COSMOS_ERROR_CODE_SUCCESS) {
        display_error(&ctx.error);
        result = 1;
        goto cleanup;
    }
    printf("✓ Created Cosmos client\n");

    // Create database
    code = cosmos_client_create_database(&ctx, client, database_name, &database);
    if (code != COSMOS_ERROR_CODE_SUCCESS) {
        display_error(&ctx.error);
        result = 1;
        goto cleanup;
    }
    database_created = 1;
    printf("✓ Created database: %s\n", database_name);

    // Create container with partition key
    code = cosmos_database_create_container(&ctx, database, "test-container", PARTITION_KEY_PATH, &container);
    if (code != COSMOS_ERROR_CODE_SUCCESS) {
        display_error(&ctx.error);
        result = 1;
        goto cleanup;
    }
    container_created = 1;
    printf("✓ Created container: %s with partition key: %s\n", "test-container", PARTITION_KEY_PATH);

    // Construct JSON document with sentinel value
    char json_data[512];
    snprintf(json_data, sizeof(json_data),
        "{\"id\":\"%s\",\"partitionKey\":\"%s\",\"name\":\"Test Document\",\"sentinel\":\"%s\",\"description\":\"This is a test document for CRUD operations\"}",
        ITEM_ID, PARTITION_KEY_VALUE, SENTINEL_VALUE);

    printf("Upserting document: %s\n", json_data);

    // Upsert the item
    code = cosmos_container_upsert_item(&ctx, container, PARTITION_KEY_VALUE, json_data);
    if (code != COSMOS_ERROR_CODE_SUCCESS) {
        display_error(&ctx.error);
        result = 1;
        goto cleanup;
    }
    printf("✓ Upserted item successfully\n");

    // Read the item back
    code = cosmos_container_read_item(&ctx, container, PARTITION_KEY_VALUE, ITEM_ID, &read_json);
    if (code != COSMOS_ERROR_CODE_SUCCESS) {
        display_error(&ctx.error);
        result = 1;
        goto cleanup;
    }
    printf("✓ Read item successfully\n");

    printf("Read back JSON: %s\n", read_json);

    // Verify the sentinel value is present in the returned JSON
    if (strstr(read_json, SENTINEL_VALUE) == NULL) {
        printf("❌ FAIL: Sentinel value '%s' not found in returned JSON\n", SENTINEL_VALUE);
        result = 1;
        goto cleanup;
    }

    // Verify the item ID is present
    if (strstr(read_json, ITEM_ID) == NULL) {
        printf("❌ FAIL: Item ID '%s' not found in returned JSON\n", ITEM_ID);
        result = 1;
        goto cleanup;
    }

    printf("✓ All assertions passed!\n");
    printf("SUCCESS: Item CRUD test completed successfully.\n");

cleanup:
    // Clean up resources in reverse order, even on failure
    if (read_json) {
        cosmos_string_free(read_json);
    }

    // Delete database (this will also delete the container)
    if (database && database_created) {
        printf("Deleting database: %s\n", database_name);
        cosmos_error_code delete_code = cosmos_database_delete(&ctx, database);
        if (delete_code != COSMOS_ERROR_CODE_SUCCESS) {
            display_error(&ctx.error);
        } else {
            printf("✓ Deleted database successfully\n");
        }
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

    return result;
}

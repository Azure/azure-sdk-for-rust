// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "../include/azurecosmos.h"

#define SENTINEL_VALUE "test-sentinel-12345"
#define ITEM_ID "test-item-id"
#define PARTITION_KEY_VALUE "test-partition"

int main() {
    cosmos_enable_tracing();

    // Get environment variables
    const char *endpoint = getenv("AZURE_COSMOS_ENDPOINT");
    const char *key = getenv("AZURE_COSMOS_KEY");
    const char *database_name = getenv("AZURE_COSMOS_DATABASE");
    const char *container_name = getenv("AZURE_COSMOS_CONTAINER");

    if (!endpoint || !key || !database_name || !container_name) {
        printf("Error: Missing required environment variables.\n");
        printf("Required: AZURE_COSMOS_ENDPOINT, AZURE_COSMOS_KEY, AZURE_COSMOS_DATABASE, AZURE_COSMOS_CONTAINER\n");
        return 1;
    }

    printf("Running Cosmos DB item CRUD test...\n");
    printf("Endpoint: %s\n", endpoint);
    printf("Database: %s\n", database_name);
    printf("Container: %s\n", container_name);

    struct CosmosError error = {0};
    struct CosmosClient *client = NULL;
    struct DatabaseClient *database = NULL;
    struct ContainerClient *container = NULL;
    char *read_json = NULL;
    int result = 0;

    // Create Cosmos client
    CosmosErrorCode code = cosmos_client_create(endpoint, key, &client, &error);
    if (code != Success) {
        printf("Failed to create Cosmos client: %s (code: %d)\n", error.message, error.code);
        result = 1;
        goto cleanup;
    }
    printf("✓ Created Cosmos client\n");

    // Get database client
    code = cosmos_client_database_client(client, database_name, &database, &error);
    if (code != Success) {
        printf("Failed to get database client: %s (code: %d)\n", error.message, error.code);
        result = 1;
        goto cleanup;
    }
    printf("✓ Got database client\n");

    // Get container client
    code = cosmos_database_container_client(database, container_name, &container, &error);
    if (code != Success) {
        printf("Failed to get container client: %s (code: %d)\n", error.message, error.code);
        result = 1;
        goto cleanup;
    }
    printf("✓ Got container client\n");

    // Construct JSON document with sentinel value
    char json_data[512];
    snprintf(json_data, sizeof(json_data),
        "{\"id\":\"%s\",\"partitionKey\":\"%s\",\"name\":\"Test Document\",\"sentinel\":\"%s\",\"description\":\"This is a test document for CRUD operations\"}",
        ITEM_ID, PARTITION_KEY_VALUE, SENTINEL_VALUE);

    printf("Upserting document: %s\n", json_data);

    // Upsert the item
    code = cosmos_container_upsert_item(container, PARTITION_KEY_VALUE, json_data, &error);
    if (code != Success) {
        printf("Failed to upsert item: %s (code: %d)\n", error.message, error.code);
        result = 1;
        goto cleanup;
    }
    printf("✓ Upserted item successfully\n");

    // Read the item back
    code = cosmos_container_read_item(container, PARTITION_KEY_VALUE, ITEM_ID, &read_json, &error);
    if (code != Success) {
        printf("Failed to read item: %s (code: %d)\n", error.message, error.code);
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
//     // Free all allocated resources
//     if (read_json) {
//         cosmos_string_free(read_json);
//     }
//     if (container) {
//         cosmos_container_free(container);
//     }
//     if (database) {
//         cosmos_database_free(database);
//     }
//     if (client) {
//         cosmos_client_free(client);
//     }
//     if (error.message) {
//         cosmos_error_free(&error);
//     }

    return result;
}

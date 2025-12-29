// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

#ifndef TEST_COMMON_H
#define TEST_COMMON_H

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>
#include "../include/azurecosmos.h"

// Test result constants
#define TEST_PASS 0
#define TEST_FAIL 1
#define TEST_SKIP 2

// Test context containing all resources needed for tests
typedef struct {
    cosmos_runtime_context *runtime;
    cosmos_call_context call_ctx;
    cosmos_client *client;
    cosmos_database_client *database;
    cosmos_container_client *container;
    int database_created;
    int container_created;
} test_context;

// Global test counters
static int tests_run = 0;
static int tests_passed = 0;
static int tests_failed = 0;
static int tests_skipped = 0;

// Initialize a test context
static inline void test_context_init(test_context *ctx) {
    memset(ctx, 0, sizeof(test_context));
}

// Create runtime context and initialize call context
static inline int test_context_create_runtime(test_context *ctx, int include_error_details) {
    cosmos_error error;
    ctx->runtime = cosmos_runtime_context_create(NULL, &error);
    if (!ctx->runtime) {
        printf("Failed to create runtime context\n");
        return TEST_FAIL;
    }
    
    ctx->call_ctx.runtime_context = ctx->runtime;
    ctx->call_ctx.include_error_details = include_error_details;
    
    return TEST_PASS;
}

// Well-known emulator connection string
#define EMULATOR_CONNECTION_STRING "AccountEndpoint=https://localhost:8081;AccountKey=C2y6yDjf5/R+ob0N8A7Cgv30VRDJIWEHLM+4QDU5DE2nQ9nDuVTqobD4b8mGGyPMbIZnqyMsEcaGQy67XIw/Jw==;"

// Create cosmos client using connection string from environment
static inline int test_context_create_client(test_context *ctx) {
    const char *connection_string_env = getenv("AZURE_COSMOS_CONNECTION_STRING");
    
    if (!connection_string_env) {
        printf("Error: Missing required environment variable.\n");
        printf("Required: AZURE_COSMOS_CONNECTION_STRING (set to 'emulator' or a full connection string)\n");
        return TEST_FAIL;
    }
    
    // Handle special "emulator" value
    cosmos_client_options options = {0};
    const char *connection_string = connection_string_env;
    if (strcmp(connection_string_env, "emulator") == 0) {
        connection_string = EMULATOR_CONNECTION_STRING;

        // Disable certificate validation for emulator
        options.danger_allow_invalid_certificates = true;
    }
    
    cosmos_error_code code = cosmos_client_create_with_connection_string(&ctx->call_ctx, connection_string, &options, &ctx->client);
    if (code != COSMOS_ERROR_CODE_SUCCESS) {
        printf("Failed to create client from connection string\n");
        return TEST_FAIL;
    }
    
    return TEST_PASS;
}

// Create database with unique name
static inline int test_context_create_database(test_context *ctx, const char *database_name) {
    cosmos_error_code code = cosmos_client_create_database(&ctx->call_ctx, ctx->client, database_name, NULL, &ctx->database);
    if (code != COSMOS_ERROR_CODE_SUCCESS) {
        printf("Failed to create database: %s\n", database_name);
        return TEST_FAIL;
    }
    ctx->database_created = 1;
    return TEST_PASS;
}

// Create container
static inline int test_context_create_container(test_context *ctx, const char *container_name, const char *partition_key_path) {
    cosmos_error_code code = cosmos_database_create_container(&ctx->call_ctx, ctx->database, container_name, partition_key_path, NULL, &ctx->container);
    if (code != COSMOS_ERROR_CODE_SUCCESS) {
        printf("Failed to create container: %s\n", container_name);
        return TEST_FAIL;
    }
    ctx->container_created = 1;
    return TEST_PASS;
}

// Generate unique database name with prefix
static inline void test_generate_database_name(char *buffer, size_t size, const char *prefix) {
    // Cast to long long to avoid warnings on platforms in which time_t is 64 bits.
    // When time_t is 32 bits, this cast just extends it out to 64 bits without changing the value.
    long long current_time = (long long)time(NULL);
    snprintf(buffer, size, "%s-%lld", prefix, current_time);
}

// Cleanup test context - safe to call regardless of which resources were initialized
static inline void test_context_cleanup(test_context *ctx) {
    if (!ctx) {
        return;
    }
    
    // Delete database if it was created (this also deletes the container)
    if (ctx->database && ctx->database_created) {
        cosmos_database_delete(&ctx->call_ctx, ctx->database, NULL);
    }
    
    // Free resources in reverse order of creation
    if (ctx->container) {
        cosmos_container_free(ctx->container);
        ctx->container = NULL;
    }
    
    if (ctx->database) {
        cosmos_database_free(ctx->database);
        ctx->database = NULL;
    }
    
    if (ctx->client) {
        cosmos_client_free(ctx->client);
        ctx->client = NULL;
    }
    
    if (ctx->runtime) {
        cosmos_runtime_context_free(ctx->runtime);
        ctx->runtime = NULL;
    }
}

// Report individual test result
static inline void report_test(const char *test_name, int result) {
    tests_run++;
    if (result == TEST_PASS) {
        tests_passed++;
        printf("✓ PASS: %s\n", test_name);
    } else if (result == TEST_SKIP) {
        tests_skipped++;
        printf("⊘ SKIP: %s\n", test_name);
    } else {
        tests_failed++;
        printf("✗ FAIL: %s\n", test_name);
    }
}

// Print test suite summary
static inline int print_test_summary(const char *suite_name) {
    printf("\n=== Test Summary: %s ===\n", suite_name);
    printf("Tests run: %d\n", tests_run);
    printf("Tests passed: %d\n", tests_passed);
    printf("Tests failed: %d\n", tests_failed);
    printf("Tests skipped: %d\n", tests_skipped);
    
    if (tests_failed == 0 && tests_passed > 0) {
        printf("\n✓ All tests passed!\n");
        return 0;
    } else if (tests_run == 0) {
        printf("\n⊘ No tests were run\n");
        return 1;
    } else {
        printf("\n✗ Some tests failed\n");
        return 1;
    }
}

// ============================================================================
// Test Assertion Macros
// ============================================================================

// Assert that condition is true, fail test but continue
#define ASSERT(condition, message, ...) \
    do { \
        if (!(condition)) { \
            printf("✗ FAIL: " message "\n", ##__VA_ARGS__); \
            result = TEST_FAIL; \
        } else { \
            printf("✓ PASS: " message "\n", ##__VA_ARGS__); \
        } \
    } while(0)

// Assert that condition is true, fail test and goto cleanup
#define REQUIRE(condition, message, ...) \
    do { \
        if (!(condition)) { \
            printf("✗ FAIL: " message "\n", ##__VA_ARGS__); \
            result = TEST_FAIL; \
            goto cleanup; \
        } else { \
            printf("✓ PASS: " message "\n", ##__VA_ARGS__); \
        } \
    } while(0)

// Assert error code matches expected, continue on failure
#define ASSERT_ERROR_CODE(actual, expected, message, ...) \
    ASSERT((actual) == (expected), message " (expected: %d, got: %d)", ##__VA_ARGS__, (int)(expected), (int)(actual))

// Require error code matches expected, goto cleanup on failure
#define REQUIRE_ERROR_CODE(actual, expected, message, ...) \
    REQUIRE((actual) == (expected), message " (expected: %d, got: %d)", \
            ##__VA_ARGS__, (int)(expected), (int)(actual))

// Assert success (error code == COSMOS_ERROR_CODE_SUCCESS), continue on failure
#define ASSERT_SUCCESS(code, message, ...) \
    ASSERT_ERROR_CODE(code, COSMOS_ERROR_CODE_SUCCESS, message, ##__VA_ARGS__)

// Require success, goto cleanup on failure
#define REQUIRE_SUCCESS(code, message, ...) \
    REQUIRE_ERROR_CODE(code, COSMOS_ERROR_CODE_SUCCESS, message, ##__VA_ARGS__)

// Assert not null, continue on failure
#define ASSERT_NOT_NULL(ptr, message, ...) \
    ASSERT((ptr) != NULL, message, ##__VA_ARGS__)

// Require not null, goto cleanup on failure
#define REQUIRE_NOT_NULL(ptr, message, ...) \
    REQUIRE((ptr) != NULL, message, ##__VA_ARGS__)

// ============================================================================
// Test Framework Functions
// ============================================================================

// Test entry structure
typedef struct {
    const char* name;
    int (*func)();
} test_entry;

// Print all test names for discovery
static inline void test_framework_discover(const test_entry* tests) {
    for (int i = 0; tests[i].name != NULL; i++) {
        printf("%s\n", tests[i].name);
    }
}

// Run a single test by name
static inline int test_framework_run_single(const test_entry* tests, const char* test_name) {
    for (int i = 0; tests[i].name != NULL; i++) {
        if (strcmp(tests[i].name, test_name) == 0) {
            printf("\n--- Test: %s ---\n", test_name);
            int result = tests[i].func();
            return (result == TEST_PASS) ? 0 : 1;
        }
    }
    fprintf(stderr, "Unknown test: %s\n", test_name);
    return 1;
}

// Run all tests in the suite
static inline int test_framework_run_all(const test_entry* tests, const char* suite_name) {
    printf("\n=== Test Suite: %s ===\n", suite_name);
    tests_run = 0;
    tests_passed = 0;
    tests_failed = 0;
    tests_skipped = 0;
    
    for (int i = 0; tests[i].name != NULL; i++) {
        int result = tests[i].func();
        report_test(tests[i].name, result);
    }
    
    return print_test_summary(suite_name);
}

// Begin test suite registration
#define TEST_SUITE_BEGIN(suite_name) \
    static test_entry test_suite[] = {

// Register a test in the suite
#define TEST_REGISTER(name) \
    { #name, test_##name },

// End test suite registration and generate main function
#define TEST_SUITE_END(suite_name) \
    { NULL, NULL } \
    }; \
    \
    int main(int argc, char** argv) { \
        cosmos_enable_tracing(); \
        \
        if (argc == 2 && strcmp(argv[1], "--discover") == 0) { \
            test_framework_discover(test_suite); \
            return 0; \
        } \
        \
        if (argc == 2) { \
            return test_framework_run_single(test_suite, argv[1]); \
        } \
        \
        return test_framework_run_all(test_suite, suite_name); \
    }

#endif // TEST_COMMON_H

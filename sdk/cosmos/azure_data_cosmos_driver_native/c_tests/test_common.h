// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

// Minimal test harness for the `azurecosmosdriver` C ABI.
//
// Provides the suite registration / discovery machinery plus a couple of
// assertion macros shared by the per-surface test files.

#ifndef TEST_COMMON_H
#define TEST_COMMON_H

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "../include/azurecosmosdriver.h"

#define TEST_PASS 0
#define TEST_FAIL 1
#define TEST_SKIP 2

static int tests_run = 0;
static int tests_passed = 0;
static int tests_failed = 0;
static int tests_skipped = 0;

static inline void report_test(const char *test_name, int result) {
    tests_run++;
    if (result == TEST_PASS) {
        tests_passed++;
        printf("  PASS: %s\n", test_name);
    } else if (result == TEST_SKIP) {
        tests_skipped++;
        printf("  SKIP: %s\n", test_name);
    } else {
        tests_failed++;
        printf("  FAIL: %s\n", test_name);
    }
}

static inline int print_test_summary(const char *suite_name) {
    printf("\n=== Test Summary: %s ===\n", suite_name);
    printf("Tests run:     %d\n", tests_run);
    printf("Tests passed:  %d\n", tests_passed);
    printf("Tests failed:  %d\n", tests_failed);
    printf("Tests skipped: %d\n", tests_skipped);
    if (tests_failed == 0 && tests_passed > 0) {
        printf("\nAll tests passed.\n");
        return 0;
    } else if (tests_run == 0) {
        printf("\nNo tests were run.\n");
        return 1;
    } else {
        printf("\nSome tests failed.\n");
        return 1;
    }
}

#define ASSERT(condition, message, ...)                                                            \
    do {                                                                                           \
        if (!(condition)) {                                                                        \
            printf("    FAIL: " message "\n", ##__VA_ARGS__);                                      \
            result = TEST_FAIL;                                                                    \
        } else {                                                                                   \
            printf("    pass: " message "\n", ##__VA_ARGS__);                                      \
        }                                                                                          \
    } while (0)

#define REQUIRE(condition, message, ...)                                                           \
    do {                                                                                           \
        if (!(condition)) {                                                                        \
            printf("    FAIL: " message "\n", ##__VA_ARGS__);                                      \
            result = TEST_FAIL;                                                                    \
            goto cleanup;                                                                          \
        } else {                                                                                   \
            printf("    pass: " message "\n", ##__VA_ARGS__);                                      \
        }                                                                                          \
    } while (0)

typedef struct {
    const char *name;
    int (*func)(void);
} test_entry;

static inline void test_framework_discover(const test_entry *tests) {
    for (int i = 0; tests[i].name != NULL; i++) {
        printf("%s\n", tests[i].name);
    }
}

static inline int test_framework_run_single(const test_entry *tests, const char *test_name) {
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

static inline int test_framework_run_all(const test_entry *tests, const char *suite_name) {
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

#define TEST_SUITE_BEGIN(suite_name) static test_entry test_suite[] = {

#define TEST_REGISTER(name) {#name, test_##name},

#define TEST_SUITE_END(suite_name)                                                                 \
    {NULL, NULL}                                                                                   \
    }                                                                                              \
    ;                                                                                              \
                                                                                                   \
    int main(int argc, char **argv) {                                                              \
        if (argc == 2 && strcmp(argv[1], "--discover") == 0) {                                     \
            test_framework_discover(test_suite);                                                   \
            return 0;                                                                              \
        }                                                                                          \
        if (argc == 2) {                                                                           \
            return test_framework_run_single(test_suite, argv[1]);                                 \
        }                                                                                          \
        return test_framework_run_all(test_suite, suite_name);                                     \
    }

#endif /* TEST_COMMON_H */

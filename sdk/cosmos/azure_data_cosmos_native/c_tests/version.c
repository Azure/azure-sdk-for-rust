// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

#include "test_common.h"

int test_version_match() {
    int result = TEST_PASS;
    const char *version = cosmos_version();
    const char *header_version = COSMOSCLIENT_H_VERSION;
    
    printf("Cosmos Client Version: %s\n", version);
    printf("Header Version: %s\n", header_version);
    
    ASSERT(!strcmp(version, header_version), "Version strings match");
    
cleanup:
    return result;
}

TEST_SUITE_BEGIN("Version")
    TEST_REGISTER(version_match)
TEST_SUITE_END("Version")

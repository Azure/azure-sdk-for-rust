// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

#include <stdio.h>
#include <string.h>
#include "../include/azurecosmos.h"

int main() {
    const char *version = cosmos_version();
    const char *header_version = COSMOSCLIENT_H_VERSION;
    printf("Cosmos Client Version: %s\n", version);
    printf("Header Version: %s\n", header_version);
    if (!strcmp(version, header_version)) {
        printf("Version match successful.\n");
        return 0;
    } else {
        printf("Version mismatch: %s != %s\n", version, header_version);
        return 1;
    }
}

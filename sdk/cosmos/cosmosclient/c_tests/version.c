#include <stdio.h>
#include <string.h>
#include "../include/cosmosclient.h"

int main() {
    const char *version = cosmosclient_version();
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

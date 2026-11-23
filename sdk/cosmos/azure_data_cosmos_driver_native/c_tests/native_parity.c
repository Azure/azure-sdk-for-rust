// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

#include "test_common.h"
#include <stddef.h>

_Static_assert(offsetof(cosmos_runtime_options_v2_t, abi_version) == 4, "runtime version prefix");
_Static_assert(offsetof(cosmos_driver_options_config_v3_t, abi_version) == 4, "client version prefix");

static int test_versioned_runtime_validation(void) {
    int result = TEST_PASS;
    cosmos_runtime_t *runtime = NULL;
    cosmos_runtime_options_v2_t options;
    cosmos_runtime_options_v2_init(&options);
    options.hedge_threshold.is_set = 1;
    options.hedge_threshold.nanoseconds = 1;
    REQUIRE(!cosmos_runtime_build_v2(&options, &runtime, NULL), "build exact nanosecond defaults");
    cosmos_runtime_free(runtime);
    runtime = NULL;
    options.proxy_allowed = 9;
    ASSERT(cosmos_runtime_build_v2(&options, &runtime, NULL) != 0 && !runtime,
        "malformed tri-state rejects without runtime");
    options.proxy_allowed = 0;
    options.hedge_threshold.nanoseconds = 1000000000;
    ASSERT(cosmos_runtime_build_v2(&options, &runtime, NULL) != 0 && !runtime,
        "unnormalized duration rejects");
cleanup:
    cosmos_runtime_free(runtime);
    return result;
}

static int test_flat_client_validation(void) {
    int result = TEST_PASS;
    cosmos_account_ref_t *account = NULL;
    cosmos_driver_options_t *built = NULL;
    REQUIRE(!cosmos_account_ref_with_master_key(SV("https://primary.invalid"),
        SV("ZW11bGF0b3Ita2V5"), &account, NULL), "create client account");
    cosmos_driver_options_config_v3_t options;
    cosmos_driver_options_config_v3_init(&options);
    cosmos_string_view_t regions[] = {SV("East US")};
    options.preferred_regions = regions;
    options.preferred_regions_len = 1;
    options.resource_name_validation_enabled = 1;
    options.topology_cache_mode = 2;
    options.hedge_threshold.is_set = 1;
    options.hedge_threshold.nanoseconds = 1;
    REQUIRE(!cosmos_driver_options_build_v3(account, &options, &built),
        "one initializer configures flat client regions, topology, and exact defaults");
    cosmos_driver_options_free(built);
    built = NULL;
    cosmos_driver_options_config_v3_init(&options);
    options.read_failure_threshold.value = 2;
    ASSERT(cosmos_driver_options_build_v3(account, &options, &built) != 0 && !built,
        "unset inline field rejects nonzero payload");
    cosmos_driver_options_config_v3_init(&options);
    options.abi_version = 99;
    ASSERT(cosmos_driver_options_build_v3(account, &options, &built) != 0 && !built,
        "unknown client version rejects");
cleanup:
    cosmos_driver_options_free(built);
    cosmos_account_ref_free(account);
    return result;
}

TEST_SUITE_BEGIN("native_parity")
    TEST_REGISTER(versioned_runtime_validation)
    TEST_REGISTER(flat_client_validation)
TEST_SUITE_END("native_parity")

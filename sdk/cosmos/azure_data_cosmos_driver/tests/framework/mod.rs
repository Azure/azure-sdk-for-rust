// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Test framework for azure_data_cosmos_driver emulator tests.

mod env;
mod test_client;

pub use azure_data_cosmos_test::resourcemanager;

pub use env::{get_test_mode, is_azure_pipelines, CosmosTestMode};
pub use resourcemanager::{
    resource_manager_from_env, CosmosResourceManager, ResourceManager, ResourceThroughput,
};
pub use test_client::{
    is_regional_readiness_error, probe_driver_data_plane_ready, resolve_driver_container_ready,
    resolve_test_env, DriverTestClient, DriverTestRunContext,
};

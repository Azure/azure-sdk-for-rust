// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Test framework for azure_data_cosmos_driver emulator tests.

mod env;
mod test_client;

pub use env::{get_test_mode, is_azure_pipelines, CosmosTestMode};
pub use test_client::{resolve_test_env, DriverTestClient};

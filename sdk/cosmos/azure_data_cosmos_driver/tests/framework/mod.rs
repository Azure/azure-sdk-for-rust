// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Test framework for azure_data_cosmos_driver emulator tests.

pub(crate) mod env;
mod test_client;

pub use test_client::DriverTestClient;

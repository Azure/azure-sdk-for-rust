// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Provides a framework for integration tests for the Azure Cosmos DB service.
//!
//! The framework allows tests to easily run against real Cosmos DB instances, the local emulator, or a mock server using test-proxy.

mod test_account;

pub use test_account::TestAccount;

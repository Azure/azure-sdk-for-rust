// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Keyvault Keys performance tests.
//!
//! This test measures the performance of getting a secret from Azure Key Vault.
//! It sets up a secret in the Key Vault during the setup phase and then repeatedly retrieves it
//! during the run phase. The test can be configured with the vault URL via command line arguments
//! to target different Key Vault instances.
//!
//! To run the test, use the following command line arguments:
//!
//! cargo bench --package azure_security_keyvault_keys --bench perf -- --duration 10 --parallel 20 get_key -u https://<my_vault>.vault.azure.net/
//!

mod create_key;
mod get_key;

use azure_core::error::ResultExt;
use azure_core_test::perf::{CreatePerfTestReturn, PerfRunner, PerfTestFactory};
use clap::{Args, Subcommand};

/// Environment variable for the Azure Key Vault URL
pub const ENV_NAME: &str = "AZURE_KEYVAULT_URL";

#[tokio::main]
async fn main() -> azure_core::Result<()> {
    match PerfRunner::<KeysTest>::new(env!("CARGO_MANIFEST_DIR"), file!()) {
        Ok(runner) => runner.run().await,
        Err(e) => e.print().with_context(
            azure_core_test::ErrorKind::Other,
            "Failed to print parser error",
        ),
    }
}

#[derive(Subcommand, Clone, Debug)]
enum KeysTest {
    CreateKey(VaultArgs),
    GetKey(VaultArgs),
}

impl PerfTestFactory for KeysTest {
    fn name(&self) -> &'static str {
        match self {
            KeysTest::CreateKey(_) => "create_key",
            KeysTest::GetKey(_) => "get_key",
        }
    }

    fn create_test(&self) -> CreatePerfTestReturn {
        match self {
            KeysTest::CreateKey(options) => create_key::CreateKey::new(options.clone()),
            KeysTest::GetKey(options) => get_key::GetKey::new(options.clone()),
        }
    }
}

#[derive(Args, Clone, Debug)]
pub struct VaultArgs {
    // The URL of the Key Vault to use in the test
    #[arg(short = 'u', long)]
    vault_url: Option<String>,
}

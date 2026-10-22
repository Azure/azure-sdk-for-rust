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
//! cargo bench --package azure_security_keyvault_keys --bench perf -- --duration 10 --parallel 20 get-key -u https://<my_vault>.vault.azure.net/
//!

mod create_key;
mod get_key;

use azure_core_test::perf::{CreatePerfTestReturn, PerfRunner, PerfTestFactory};
use clap::{Args, Subcommand};

/// Environment variable for the Azure Key Vault URL
pub const ENV_NAME: &str = "AZURE_KEYVAULT_URL";

#[tokio::main]
async fn main() -> azure_core::Result<()> {
    match PerfRunner::<Tests>::new(env!("CARGO_MANIFEST_DIR"), file!()) {
        Ok(runner) => runner.run().await,
        Err(error) => error.exit(),
    }
}

#[derive(Subcommand, Debug, Clone)]
enum Tests {
    /// Create a key in Key Vault.
    CreateKey(VaultArgs),
    /// Get a key from Key Vault.
    GetKey(VaultArgs),
}

impl PerfTestFactory for Tests {
    fn name(&self) -> &'static str {
        match self {
            Self::CreateKey(_) => "create-key",
            Self::GetKey(_) => "get-key",
        }
    }

    fn create_test(&self) -> CreatePerfTestReturn {
        match self {
            Self::CreateKey(args) => create_key::CreateKey::new(args.clone()),
            Self::GetKey(args) => get_key::GetKey::new(args.clone()),
        }
    }
}

#[derive(Args, Debug, Clone)]
pub struct VaultArgs {
    /// The URL of the Key Vault to use in the test.
    #[arg(short = 'u', long)]
    vault_url: Option<String>,
}

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

use azure_core_test::perf::{PerfRunner, PerfTestMetadata, PerfTestOption};
use futures::FutureExt;

/// Environment variable for the Azure Key Vault URL
pub const ENV_NAME: &str = "AZURE_KEYVAULT_URL";

#[tokio::main]
async fn main() -> azure_core::Result<()> {
    let options = vec![PerfTestOption {
        name: "vault_url",
        short_activator: Some('u'),
        long_activator: "vault-url",
        display_message: "The URL of the Key Vault to use in the test",
        expected_args_len: 1,
        ..Default::default()
    }];
    let runner = PerfRunner::new(
        env!("CARGO_MANIFEST_DIR"),
        file!(),
        vec![
            PerfTestMetadata {
                name: "create-key",
                description: "Create a key in Key Vault",
                options: options.clone(),
                create_test: |runner| {
                    async move {
                        create_key::CreateKey::new(VaultArgs {
                            vault_url: runner.try_get_test_arg("vault_url")?,
                        })
                        .await
                    }
                    .boxed_local()
                },
            },
            PerfTestMetadata {
                name: "get-key",
                description: "Get a key from Key Vault",
                options,
                create_test: |runner| {
                    async move {
                        get_key::GetKey::new(VaultArgs {
                            vault_url: runner.try_get_test_arg("vault_url")?,
                        })
                        .await
                    }
                    .boxed_local()
                },
            },
        ],
    )?;
    runner.run().await
}

pub struct VaultArgs {
    vault_url: Option<String>,
}

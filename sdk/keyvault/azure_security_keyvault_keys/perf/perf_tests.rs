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

use azure_core_test::perf::PerfRunner;

/// Environment variable for the Azure Key Vault URL
pub const ENV_NAME: &str = "AZURE_KEYVAULT_URL";

#[tokio::main]
async fn main() -> azure_core::Result<()> {
    let runner = PerfRunner::new(
        env!("CARGO_MANIFEST_DIR"),
        file!(),
        vec![
            create_key::CreateKey::test_metadata(),
            get_key::GetKey::test_metadata(),
        ],
    )?;
    runner.run().await
}

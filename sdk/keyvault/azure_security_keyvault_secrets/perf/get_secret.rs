// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Keyvault Secrets performance tests.
//!
//! This test measures the performance of getting a secret from Azure Key Vault.
//! It sets up a secret in the Key Vault during the setup phase and then repeatedly retrieves it
//! during the run phase. The test can be configured with the vault URL via command line arguments
//! to target different Key Vault instances.
//!
//! To run the test, use the following command line arguments:
//!
//! cargo test --package azure_security_keyvault_secrets --test performance_tests -- --duration 10 --parallel 20 get_secret -u https://<my_vault>.vault.azure.net/
//!

use std::sync::{Arc, OnceLock};

use azure_core::Result;
use azure_core_test::{
    perf::{CreatePerfTestReturn, PerfRunner, PerfTest, PerfTestMetadata, PerfTestOption},
    TestContext,
};
use azure_security_keyvault_secrets::{models::SetSecretParameters, SecretClient};
use rand::{distr::Alphanumeric, Rng};
struct GetSecrets {
    vault_url: String,
    random_key_name: OnceLock<String>,
    client: OnceLock<SecretClient>,
}

impl GetSecrets {
    fn test_metadata() -> PerfTestMetadata {
        PerfTestMetadata {
            name: "get_secret",
            description: "Get a secret from Key Vault",
            options: vec![PerfTestOption {
                name: "vault_url",
                display_message: "The URL of the Key Vault to use in the test",
                mandatory: true,
                short_activator: 'u',
                long_activator: "vault-url",
                expected_args_len: 1,
                ..Default::default()
            }],
            create_test: Self::create_new_test,
        }
    }

    fn create_new_test(runner: &PerfRunner) -> CreatePerfTestReturn {
        async fn create_secret_client(runner: PerfRunner) -> Result<Box<dyn PerfTest>> {
            let vault_url_ref: Option<&String> = runner.try_get_test_arg("vault_url")?;
            let vault_url = vault_url_ref
                .expect("vault_url argument is mandatory")
                .clone();
            Ok(Box::new(GetSecrets {
                vault_url,
                random_key_name: OnceLock::new(),
                client: OnceLock::new(),
            }) as Box<dyn PerfTest>)
        }

        Box::pin(create_secret_client(runner.clone()))
    }

    fn create_random_key_name() -> String {
        let random_suffix: String = rand::rng()
            .sample_iter(&Alphanumeric)
            .take(8)
            .map(char::from)
            .collect();
        format!("perf-{}", random_suffix)
    }

    fn get_random_key_name(&self) -> &String {
        self.random_key_name
            .get_or_init(Self::create_random_key_name)
    }
}

#[cfg_attr(target_arch="wasm32", async_trait::async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait::async_trait)]
impl PerfTest for GetSecrets {
    async fn setup(&self, _context: Arc<TestContext>) -> azure_core::Result<()> {
        let credential = azure_identity::DeveloperToolsCredential::new(None)?;
        let client = SecretClient::new(self.vault_url.as_str(), credential.clone(), None)?;
        self.client.get_or_init(|| client);

        self.client
            .get()
            .unwrap()
            .set_secret(
                self.get_random_key_name(),
                SetSecretParameters {
                    value: Some("secret_value".into()),
                    ..Default::default()
                }
                .try_into()?,
                None,
            )
            .await?;
        Ok(())
    }
    async fn cleanup(&self, _context: Arc<TestContext>) -> azure_core::Result<()> {
        Ok(())
    }
    async fn run(&self, _context: Arc<TestContext>) -> Result<()> {
        let _secret = self
            .client
            .get()
            .unwrap()
            .get_secret(self.get_random_key_name(), None)
            .await?
            .into_body()?;
        Ok(())
    }
}

#[tokio::main]
async fn main() -> azure_core::Result<()> {
    let runner = PerfRunner::new(
        env!("CARGO_MANIFEST_DIR"),
        file!(),
        vec![GetSecrets::test_metadata()],
    )?;

    runner.run().await?;

    Ok(())
}

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
//! cargo bench --package azure_security_keyvault_secrets --bench perf -- --duration 10 --parallel 20 get_secret -u https://<my_vault>.vault.azure.net/
//!

use std::sync::{Arc, OnceLock};

use azure_core::{error::ResultExt, Result};
use azure_core_test::{
    perf::{CreatePerfTestReturn, PerfRunner, PerfTest, PerfTestFactory},
    Recording, TestContext,
};
use azure_security_keyvault_secrets::{
    models::SetSecretParameters, SecretClient, SecretClientOptions,
};
use clap::{Args, Subcommand};
use futures::FutureExt;

#[derive(Args, Clone, Debug)]
pub struct VaultArgs {
    // The URL of the Key Vault to use in the test
    #[arg(short = 'u', long)]
    vault_url: Option<String>,
}

struct GetSecrets {
    vault_url: Option<String>,
    random_key_name: OnceLock<String>,
    client: OnceLock<SecretClient>,
}

impl GetSecrets {
    pub fn new(args: VaultArgs) -> CreatePerfTestReturn {
        async move {
            Ok(Box::new(GetSecrets {
                vault_url: args.vault_url,
                random_key_name: OnceLock::new(),
                client: OnceLock::new(),
            }) as Box<dyn PerfTest>)
        }
        .boxed()
    }

    fn create_random_key_name(recording: &Recording) -> String {
        let random_suffix: String = recording.random_string::<8>(Some("perf-"));
        format!("perf-{}", random_suffix)
    }

    fn get_random_key_name(&self, recording: &Recording) -> &String {
        self.random_key_name
            .get_or_init(|| Self::create_random_key_name(recording))
    }
}

#[async_trait::async_trait]
impl PerfTest for GetSecrets {
    async fn setup(&self, context: Arc<TestContext>) -> azure_core::Result<()> {
        let recording = context.recording();
        let credential = recording.credential();

        let mut client_options = SecretClientOptions::default();
        recording.instrument_perf(&mut client_options.client_options)?;

        let vault_url = self
            .vault_url
            .clone()
            .unwrap_or_else(|| recording.var("AZURE_KEYVAULT_URL", None));

        let client = SecretClient::new(&vault_url, credential.clone(), Some(client_options))?;
        self.client.get_or_init(|| client);

        self.client
            .get()
            .unwrap()
            .set_secret(
                self.get_random_key_name(recording),
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
    async fn run(&self, context: Arc<TestContext>) -> Result<()> {
        let recording = context.recording();
        let _secret = self
            .client
            .get()
            .unwrap()
            .get_secret(self.get_random_key_name(recording), None)
            .await?
            .into_model()?;
        Ok(())
    }
}

#[tokio::main]
async fn main() -> azure_core::Result<()> {
    match PerfRunner::<SecretsTest>::new(env!("CARGO_MANIFEST_DIR"), file!()) {
        Ok(runner) => runner.run().await,
        Err(e) => e.print().with_context(
            azure_core_test::ErrorKind::Other,
            "Failed to print parser error",
        ),
    }
}

#[derive(Subcommand, Clone, Debug)]
enum SecretsTest {
    GetSecret(VaultArgs),
}

impl PerfTestFactory for SecretsTest {
    fn name(&self) -> &'static str {
        match self {
            SecretsTest::GetSecret(_) => "get_secret",
        }
    }

    fn create_test(&self) -> CreatePerfTestReturn {
        match self {
            SecretsTest::GetSecret(options) => GetSecrets::new(options.clone()),
        }
    }
}

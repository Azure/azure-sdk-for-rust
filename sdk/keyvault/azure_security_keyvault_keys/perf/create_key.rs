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
//! cargo test --package azure_security_keyvault_secrets --test perf -- --duration 10 --parallel 20 get_secret -u https://<my_vault>.vault.azure.net/
//!

use crate::ENV_NAME;
use azure_core::Result;
use azure_core_test::{
    perf::{CreatePerfTestReturn, PerfRunner, PerfTest, PerfTestMetadata, PerfTestOption},
    Recording, TestContext,
};
use azure_security_keyvault_keys::{
    models::{CreateKeyParameters, CurveName, KeyType},
    KeyClient, KeyClientOptions,
};
use futures::FutureExt;
use std::sync::{Arc, OnceLock};

pub struct CreateKey {
    vault_url: Option<String>,
    random_key_name: OnceLock<String>,
    client: OnceLock<KeyClient>,
    body: CreateKeyParameters,
}

impl CreateKey {
    pub(crate) fn test_metadata() -> PerfTestMetadata {
        PerfTestMetadata {
            name: "create_key",
            description: "Create a key in Key Vault",
            options: vec![PerfTestOption {
                name: "vault_url",
                display_message: "The URL of the Key Vault to use in the test",
                mandatory: false,
                short_activator: Some('u'),
                long_activator: "vault-url",
                expected_args_len: 1,
                ..Default::default()
            }],
            create_test: Self::create_new_test,
        }
    }

    fn create_new_test(runner: PerfRunner) -> CreatePerfTestReturn {
        async move {
            let vault_url_ref: Option<&String> = runner.try_get_test_arg("vault_url")?;
            let vault_url = vault_url_ref.cloned();
            Ok(Box::new(CreateKey {
                vault_url,
                random_key_name: OnceLock::new(),
                client: OnceLock::new(),
                body: CreateKeyParameters {
                    kty: Some(KeyType::Ec),
                    curve: Some(CurveName::P256),
                    ..Default::default()
                },
            }) as Box<dyn PerfTest>)
        }
        .boxed()
    }

    fn create_random_key_name(recording: &Recording) -> String {
        recording.random_string::<8>(Some("perf-"))
    }

    fn get_random_key_name(&self, recording: &Recording) -> &String {
        self.random_key_name
            .get_or_init(|| Self::create_random_key_name(recording))
    }
}

#[async_trait::async_trait]
impl PerfTest for CreateKey {
    async fn setup(&self, context: Arc<TestContext>) -> azure_core::Result<()> {
        let recording = context.recording();
        let credential = recording.credential();

        let mut client_options = KeyClientOptions::default();
        recording.instrument(&mut client_options.client_options);

        let vault_url = self
            .vault_url
            .clone()
            .unwrap_or_else(|| recording.var(ENV_NAME, None));

        let client = KeyClient::new(&vault_url, credential.clone(), Some(client_options))?;
        self.client.get_or_init(|| client);

        // Seed the initial key to ensure the benchmark runs in a clean state
        self.client
            .get()
            .unwrap()
            .create_key(
                self.get_random_key_name(recording),
                self.body.clone().try_into()?,
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
        let _key = self
            .client
            .get()
            .unwrap()
            .create_key(
                self.get_random_key_name(recording),
                self.body.clone().try_into()?,
                None,
            )
            .await?
            .into_body()?;
        Ok(())
    }
}

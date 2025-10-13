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
//! cargo test --package azure_security_keyvault_keys --test perf -- --duration 10 --parallel 20 get_key -u https://<my_vault>.vault.azure.net/
//!

mod create_key;
mod get_key;

use azure_core_test::perf::PerfRunner;

/// Environment variable for the Azure Key Vault URL
pub const ENV_NAME: &str = "AZURE_KEYVAULT_URL";

// fn key_2901_benchmark_default(c: &mut Criterion) {
//     // Check if the environment variable is set thus allowing the benchmarks to run
//     if std::env::var(ENV_NAME).is_err() {
//         println!("Skipping benchmarks. Set {} to run.", ENV_NAME);
//         return;
//     }
//     const KEY_NAME: &str = "key-2901";

//     //    let rt = tokio::runtime::Runtime::new().unwrap();
//     let rt = tokio::runtime::Builder::new_current_thread()
//         .enable_all()
//         .build()
//         .unwrap();

//     // Setup the KeyClient and the CreateKeyParameters
//     async fn setup_key_client() -> (KeyClient, CreateKeyParameters) {
//         let keyvault_url: String =
//             std::env::var(ENV_NAME).unwrap_or_else(|e| panic!("{} not set: {}", ENV_NAME, e));
//         let credential = credentials::from_env(None).unwrap();
//         let client: KeyClient = KeyClient::new(&keyvault_url, credential.clone(), None).unwrap();
//         let body = CreateKeyParameters {
//             kty: Some(KeyType::Rsa),
//             key_size: Some(2048),
//             ..Default::default()
//         };
//         (client, body)
//     }

//     let (client, body) = rt.block_on(async { setup_key_client().await });

//     // Create a key with name
//     async fn create_key(
//         name: &str,
//         client: &KeyClient,
//         body: CreateKeyParameters,
//     ) -> Result<Key, azure_core::Error> {
//         // Create a new key version
//         client
//             .create_key(name, body.try_into()?, None)
//             .await?
//             .into_body()
//     }

//     // prep key in order to run the benchmark in a clean state
//     let _ = rt.block_on(async { create_key(KEY_NAME, &client, body.clone()).await });

//     let dek = rand::random::<[u8; 32]>(); // Generate a random 32-byte data encryption key (DEK)

//     c.bench_function("key_2901_benchmark_default", |b| {
//         b.to_async(&rt).iter(|| async {
//             let parameters = KeyOperationParameters {
//                 algorithm: Some(EncryptionAlgorithm::RsaOaep256),
//                 value: Some(dek.to_vec()),
//                 ..Default::default()
//             };
//             let _wrap_key_response = client
//                 .wrap_key(KEY_NAME, parameters.try_into().unwrap(), None)
//                 .await
//                 .unwrap()
//                 .into_body()
//                 .unwrap();

//             black_box(());
//         });
//     });
// }

// fn key_2901_benchmark_slow(c: &mut Criterion) {
//     // Check if the environment variable is set thus allowing the benchmarks to run
//     if std::env::var(ENV_NAME).is_err() {
//         println!("Skipping benchmarks. Set {} to run.", ENV_NAME);
//         return;
//     }
//     const KEY_NAME: &str = "key-2901";

//     let rt = tokio::runtime::Builder::new_current_thread()
//         .enable_all()
//         .build()
//         .unwrap();

//     // Setup the KeyClient and the CreateKeyParameters
//     async fn setup_key_client() -> (KeyClient, CreateKeyParameters) {
//         let reqwest = Arc::new(
//             reqwest::Client::builder()
//                 .pool_max_idle_per_host(0)
//                 .build()
//                 .unwrap(),
//         );

//         let options = KeyClientOptions {
//             client_options: ClientOptions {
//                 transport: Some(Transport::new(reqwest)),
//                 ..Default::default()
//             },
//             ..Default::default()
//         };

//         let keyvault_url: String =
//             std::env::var(ENV_NAME).unwrap_or_else(|e| panic!("{} not set: {}", ENV_NAME, e));
//         let credential = credentials::from_env(None).unwrap();
//         let client: KeyClient =
//             KeyClient::new(&keyvault_url, credential.clone(), Some(options)).unwrap();
//         let body = CreateKeyParameters {
//             kty: Some(KeyType::Rsa),
//             key_size: Some(2048),
//             ..Default::default()
//         };
//         (client, body)
//     }

//     let (client, body) = rt.block_on(async { setup_key_client().await });

//     // Create a key with name
//     async fn create_key(
//         name: &str,
//         client: &KeyClient,
//         body: CreateKeyParameters,
//     ) -> Result<Key, azure_core::Error> {
//         // Create a new key version
//         client
//             .create_key(name, body.try_into()?, None)
//             .await?
//             .into_body()
//     }

//     // prep key in order to run the benchmark in a clean state
//     let _ = rt.block_on(async { create_key(KEY_NAME, &client, body.clone()).await });

//     let dek = rand::random::<[u8; 32]>(); // Generate a random 32-byte data encryption key (DEK)

//     c.bench_function("key_2901_benchmark_slow", |b| {
//         b.to_async(&rt).iter(|| async {
//             let parameters = KeyOperationParameters {
//                 algorithm: Some(EncryptionAlgorithm::RsaOaep256),
//                 value: Some(dek.to_vec()),
//                 ..Default::default()
//             };
//             let _wrap_key_response = client
//                 .wrap_key(KEY_NAME, parameters.try_into().unwrap(), None)
//                 .await
//                 .unwrap()
//                 .into_body()
//                 .unwrap();

//             black_box(());
//         });
//     });
// }

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

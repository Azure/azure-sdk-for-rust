use azure_core::http::{ClientOptions, Transport};
use azure_core_test::credentials;
use azure_security_keyvault_keys::{
    models::{
        CreateKeyParameters, CurveName, EncryptionAlgorithm, Key, KeyOperationParameters, KeyType,
    },
    KeyClient, KeyClientOptions,
};
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::sync::Arc;

/// Environment variable for the Azure Key Vault URL
const ENV_NAME: &str = "AZURE_KEYVAULT_URL";

fn key_operations_benchmark(c: &mut Criterion) {
    // Check if the environment variable is set thus allowing the benchmarks to run
    if std::env::var(ENV_NAME).is_err() {
        println!("Skipping benchmarks. Set {} to run.", ENV_NAME);
        return;
    }

    let rt = tokio::runtime::Runtime::new().unwrap();
    const KEY_NAME: &str = "test-key";

    // Setup the KeyClient and the CreateKeyParameters
    async fn setup_key_client() -> (KeyClient, CreateKeyParameters) {
        let keyvault_url: String =
            std::env::var(ENV_NAME).unwrap_or_else(|e| panic!("{} not set: {}", ENV_NAME, e));
        let credential = credentials::from_env(None).unwrap();
        let client: KeyClient = KeyClient::new(&keyvault_url, credential.clone(), None).unwrap();
        let body = CreateKeyParameters {
            kty: Some(KeyType::Ec),
            curve: Some(CurveName::P256),
            ..Default::default()
        };
        (client, body)
    }

    let (client, body) = rt.block_on(async { setup_key_client().await });

    // prep key in order to run the benchmark in a clean state
    let _ = rt.block_on(async { create_key(KEY_NAME, &client, body.clone()).await });

    // Create a key with name
    async fn create_key(
        name: &str,
        client: &KeyClient,
        body: CreateKeyParameters,
    ) -> Result<Key, azure_core::Error> {
        // Create a new key version
        client
            .create_key(name, body.try_into()?, None)
            .await?
            .into_body()
    }

    // Get a key by name
    async fn get_key(key_name: &str, client: &KeyClient) -> Result<Key, azure_core::Error> {
        // Get the key
        client.get_key(key_name, None).await?.into_body()
    }

    // Benchmark create key
    c.bench_function("create_key", |b| {
        b.to_async(&rt).iter(|| async {
            create_key(KEY_NAME, &client, body.clone())
                .await
                .unwrap_or_else(|e| panic!("Failed to create key {:?}", e));
            black_box(());
        });
    });

    // Benchmark create key
    c.bench_function("get_key", |b| {
        b.to_async(&rt).iter(|| async {
            get_key(KEY_NAME, &client)
                .await
                .unwrap_or_else(|e| panic!("Failed to get key {:?}", e));
            black_box(());
        });
    });
}

fn key_2901_benchmark_default(c: &mut Criterion) {
    // Check if the environment variable is set thus allowing the benchmarks to run
    if std::env::var(ENV_NAME).is_err() {
        println!("Skipping benchmarks. Set {} to run.", ENV_NAME);
        return;
    }
    const KEY_NAME: &str = "key-2901";

    //    let rt = tokio::runtime::Runtime::new().unwrap();
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();

    // Setup the KeyClient and the CreateKeyParameters
    async fn setup_key_client() -> (KeyClient, CreateKeyParameters) {
        let keyvault_url: String =
            std::env::var(ENV_NAME).unwrap_or_else(|e| panic!("{} not set: {}", ENV_NAME, e));
        let credential = credentials::from_env(None).unwrap();
        let client: KeyClient = KeyClient::new(&keyvault_url, credential.clone(), None).unwrap();
        let body = CreateKeyParameters {
            kty: Some(KeyType::Rsa),
            key_size: Some(2048),
            ..Default::default()
        };
        (client, body)
    }

    let (client, body) = rt.block_on(async { setup_key_client().await });

    // Create a key with name
    async fn create_key(
        name: &str,
        client: &KeyClient,
        body: CreateKeyParameters,
    ) -> Result<Key, azure_core::Error> {
        // Create a new key version
        client
            .create_key(name, body.try_into()?, None)
            .await?
            .into_body()
    }

    // prep key in order to run the benchmark in a clean state
    let _ = rt.block_on(async { create_key(KEY_NAME, &client, body.clone()).await });

    let dek = rand::random::<[u8; 32]>(); // Generate a random 32-byte data encryption key (DEK)

    c.bench_function("key_2901_benchmark_default", |b| {
        b.to_async(&rt).iter(|| async {
            let parameters = KeyOperationParameters {
                algorithm: Some(EncryptionAlgorithm::RsaOaep256),
                value: Some(dek.to_vec()),
                ..Default::default()
            };
            let _wrap_key_response = client
                .wrap_key(KEY_NAME, parameters.try_into().unwrap(), None)
                .await
                .unwrap()
                .into_body()
                .unwrap();

            black_box(());
        });
    });
}

fn key_2901_benchmark_slow(c: &mut Criterion) {
    // Check if the environment variable is set thus allowing the benchmarks to run
    if std::env::var(ENV_NAME).is_err() {
        println!("Skipping benchmarks. Set {} to run.", ENV_NAME);
        return;
    }
    const KEY_NAME: &str = "key-2901";

    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();

    // Setup the KeyClient and the CreateKeyParameters
    async fn setup_key_client() -> (KeyClient, CreateKeyParameters) {
        let reqwest = Arc::new(
            reqwest::Client::builder()
                .pool_max_idle_per_host(0)
                .build()
                .unwrap(),
        );

        let options = KeyClientOptions {
            client_options: ClientOptions {
                transport: Some(Transport::new(reqwest)),
                ..Default::default()
            },
            ..Default::default()
        };

        let keyvault_url: String =
            std::env::var(ENV_NAME).unwrap_or_else(|e| panic!("{} not set: {}", ENV_NAME, e));
        let credential = credentials::from_env(None).unwrap();
        let client: KeyClient =
            KeyClient::new(&keyvault_url, credential.clone(), Some(options)).unwrap();
        let body = CreateKeyParameters {
            kty: Some(KeyType::Rsa),
            key_size: Some(2048),
            ..Default::default()
        };
        (client, body)
    }

    let (client, body) = rt.block_on(async { setup_key_client().await });

    // Create a key with name
    async fn create_key(
        name: &str,
        client: &KeyClient,
        body: CreateKeyParameters,
    ) -> Result<Key, azure_core::Error> {
        // Create a new key version
        client
            .create_key(name, body.try_into()?, None)
            .await?
            .into_body()
    }

    // prep key in order to run the benchmark in a clean state
    let _ = rt.block_on(async { create_key(KEY_NAME, &client, body.clone()).await });

    let dek = rand::random::<[u8; 32]>(); // Generate a random 32-byte data encryption key (DEK)

    c.bench_function("key_2901_benchmark_slow", |b| {
        b.to_async(&rt).iter(|| async {
            let parameters = KeyOperationParameters {
                algorithm: Some(EncryptionAlgorithm::RsaOaep256),
                value: Some(dek.to_vec()),
                ..Default::default()
            };
            let _wrap_key_response = client
                .wrap_key(KEY_NAME, parameters.try_into().unwrap(), None)
                .await
                .unwrap()
                .into_body()
                .unwrap();

            black_box(());
        });
    });
}

// Main benchmark configuration
criterion_group! {
    name = benchmarks;
    config = Criterion::default()
        .sample_size(10)
        .warm_up_time(std::time::Duration::new(1, 0))
        .measurement_time(std::time::Duration::from_secs(30));
    targets = key_operations_benchmark, key_2901_benchmark_default, key_2901_benchmark_slow
}

criterion_main!(benchmarks);

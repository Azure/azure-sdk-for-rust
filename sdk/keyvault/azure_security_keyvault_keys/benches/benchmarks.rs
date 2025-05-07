use azure_core_test::credentials;
use azure_security_keyvault_keys::{
    models::{CreateKeyParameters, CurveName, Key, KeyType},
    KeyClient,
};
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn key_operations_benchmark(c: &mut Criterion) {
    const ENV_NAME: &str = "AZURE_KEYVAULT_URL";

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
            kty: Some(KeyType::EC),
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
            .await
    }

    // Get a key by name
    async fn get_key(key_name: &str, client: &KeyClient) -> Result<Key, azure_core::Error> {
        // Get the key
        client.get_key(key_name, "", None).await?.into_body().await
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

// Main benchmark configuration
criterion_group! {
    name = benchmarks;
    config = Criterion::default().sample_size(10).warm_up_time(std::time::Duration::new(1, 0));
    targets = key_operations_benchmark
}

criterion_main!(benchmarks);

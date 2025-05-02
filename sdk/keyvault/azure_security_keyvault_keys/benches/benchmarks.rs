use azure_identity::DefaultAzureCredential;
use azure_security_keyvault_keys::{
    models::{CreateKeyParameters, CurveName, Key, KeyType},
    KeyClient,
};
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn create_key_benchmark(c: &mut Criterion) {
    let rt = tokio::runtime::Runtime::new().unwrap();

    async fn setup_key_client() -> (KeyClient, CreateKeyParameters) {
        let keyvault_url: String = std::env::var("AZURE_KEYVAULT_URL")
            .unwrap_or_else(|e| panic!("AZURE_KEYVAULT_URL not set: {}", e));
        let credential = DefaultAzureCredential::new().unwrap();
        let client: KeyClient = KeyClient::new(&keyvault_url, credential.clone(), None).unwrap();
        let body = CreateKeyParameters {
            kty: Some(KeyType::EC),
            curve: Some(CurveName::P256),
            ..Default::default()
        };
        (client, body)
    }

    let (client, body) = rt.block_on(async { setup_key_client().await });

    async fn create_key(
        client: &KeyClient,
        body: CreateKeyParameters,
    ) -> Result<Key, azure_core::Error> {
        // Create the key
        let key: Key = client
            .create_key("key-name", body.try_into()?, None)
            .await?
            .into_body()
            .await?;
        Ok(key)
    }
    // Benchmark create key
    c.bench_function("create_key", |b| {
        b.to_async(&rt).iter(|| async {
            create_key(&client, body.clone())
                .await
                .unwrap_or_else(|e| panic!("Failed to create key {}", e));
            black_box(());
        });
    });
}
// Main benchmark configuration
criterion_group! {
    name = benchmarks;
    config = Criterion::default().sample_size(10).warm_up_time(std::time::Duration::new(1, 0));
    targets = create_key_benchmark
}

criterion_main!(benchmarks);

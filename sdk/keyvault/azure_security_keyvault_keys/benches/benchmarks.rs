use azure_core::http::{headers::Headers, HttpClient, Response, StatusCode};
use azure_core_test::http::MockHttpClient;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use futures::FutureExt;
use std::sync::Arc;

use azure_identity::DefaultAzureCredential;
use azure_security_keyvault_keys::{
    models::{CreateKeyParameters, CurveName, Key, KeyType},
    KeyClient,
};

fn create_key_benchmark(c: &mut Criterion) {
    let rt = tokio::runtime::Runtime::new().unwrap();

    // client to be used in the benchmark
    let mock_client = Arc::new(MockHttpClient::new(move |_| {
        async move { Ok(Response::from_bytes(StatusCode::Ok, Headers::new(), vec![])) }.boxed()
    })) as Arc<dyn HttpClient>;

    async fn create_key() -> Result<Key, azure_core::Error> {
        let keyvault_url: String = std::env::var("AZURE_KEYVAULT_URL")
            .unwrap_or_else(|e| panic!("AZURE_KEYVAULT_URL not set: {}", e));

        let credential = DefaultAzureCredential::new().unwrap();
        let client: KeyClient = KeyClient::new(&keyvault_url, credential.clone(), None).unwrap();
        // Create an EC key.
        let body = CreateKeyParameters {
            kty: Some(KeyType::EC),
            curve: Some(CurveName::P256),
            ..Default::default()
        };
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
            create_key()
                .await
                .unwrap_or_else(|_| panic!("Failed to create key"));
            black_box(());
        });
    });
}
// Main benchmark configuration
criterion_group! {
    name = benchmarks;
    config = Criterion::default();
    targets = create_key_benchmark
}

criterion_main!(benchmarks);

# azure_data_cosmos_benchmarks

- **Description**: Benchmarks for the Azure Cosmos DB Rust driver
- **Edition**: 2021
- **Rust version**: 1.88

## Features

- `default`

```rust
pub fn load_bench_config() -> BenchConfig;
pub async fn setup() -> (std::sync::Arc<azure_data_cosmos_driver::CosmosDriver>, azure_data_cosmos_driver::models::ItemReference);
pub async fn setup_live() -> (std::sync::Arc<azure_data_cosmos_driver::CosmosDriver>, azure_data_cosmos_driver::models::ItemReference);
#[derive(Debug)]
pub struct MockHttpClientFactory;
impl MockHttpClientFactory {
    fn new() -> Self;
}
impl Default for MockHttpClientFactory {
    fn default() -> Self;
}
impl HttpClientFactory for MockHttpClientFactory {
    fn build(&self, _connection_pool: &ConnectionPoolOptions, _config: HttpClientConfig) -> azure_data_cosmos_driver::error::Result<Arc<dyn TransportClient>>;
}
#[derive(Debug)]
pub struct MockTransportClient {
    pub latency: std::time::Duration,
}
impl MockTransportClient {
    fn new() -> Self;
    fn with_latency(latency: std::time::Duration) -> Self;
}
impl Default for MockTransportClient {
    fn default() -> Self;
}
impl TransportClient for MockTransportClient {
    #[allow(elided_named_lifetimes, clippy::async_yields_async, clippy::diverging_sub_expression, clippy::let_unit_value, clippy::needless_arbitrary_self_type, clippy::no_effect_underscore_binding, clippy::shadow_same, clippy::type_complexity, clippy::type_repetition_in_bounds, clippy::used_underscore_binding)]
    fn send(&self, request: &HttpRequest) -> ::core::pin::Pin<Box<dyn ::core::future::Future<Output = Result<HttpResponse, TransportError>> + ::core::marker::Send>>;
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BenchConfig {
    Mock,
    Live,
}
pub const ACCOUNT_PROPERTIES_PAYLOAD: &str = r#"{
    "_self": "",
    "id": "bench",
    "_rid": "bench.documents.azure.com",
    "media": "//media/",
    "addresses": "//addresses/",
    "_dbs": "//dbs/",
    "writableLocations": [
        { "name": "West US 2", "databaseAccountEndpoint": "https://bench-westus2.documents.azure.com:443/" }
    ],
    "readableLocations": [
        { "name": "West US 2", "databaseAccountEndpoint": "https://bench-westus2.documents.azure.com:443/" }
    ],
    "enableMultipleWriteLocations": false,
    "userReplicationPolicy": { "minReplicaSetSize": 3, "maxReplicasetSize": 4 },
    "userConsistencyPolicy": { "defaultConsistencyLevel": "Session" },
    "systemReplicationPolicy": { "minReplicaSetSize": 3, "maxReplicasetSize": 4 },
    "readPolicy": { "primaryReadCoefficient": 1, "secondaryReadCoefficient": 1 },
    "queryEngineConfiguration": "{}"
}"#;
pub const CONTAINER_PROPERTIES_PAYLOAD: &str = r#"{
    "id": "benchcontainer",
    "_rid": "benchcontainer==",
    "_self": "dbs/benchdb==/colls/benchcontainer==/",
    "_etag": "\"00000000-0000-0000-0000-000000000002\"",
    "partitionKey": { "paths": ["/pk"], "kind": "Hash", "version": 2 },
    "indexingPolicy": { "indexingMode": "consistent", "automatic": true },
    "_ts": 1,
    "_docs": "docs/",
    "_sprocs": "sprocs/",
    "_triggers": "triggers/",
    "_udfs": "udfs/",
    "_conflicts": "conflicts/"
}"#;
pub const DATABASE_PROPERTIES_PAYLOAD: &str = r#"{
    "id": "benchdb",
    "_rid": "benchdb==",
    "_self": "dbs/benchdb==/",
    "_etag": "\"00000000-0000-0000-0000-000000000001\"",
    "_colls": "colls/",
    "_users": "users/",
    "_ts": 1
}"#;
pub const ITEM_PAYLOAD: &str = r#"{
    "id": "item1",
    "pk": "pk1",
    "_rid": "benchitem==",
    "_self": "dbs/benchdb==/colls/benchcontainer==/docs/benchitem==/",
    "_etag": "\"00000000-0000-0000-0000-000000000003\"",
    "_attachments": "attachments/",
    "_ts": 1
}"#;
```

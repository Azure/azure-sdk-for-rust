# azure_data_cosmos_test

- **Edition**: 2021
- **Rust version**: 1.88

## Features

- `default`

```rust
#![crate_name = "azure_data_cosmos_test"]
#![crate_type = "lib"]
pub mod resourcemanager {
    pub fn resource_manager_from_env(credential: std::sync::Arc<dyn TokenCredential>) -> Result<ResourceManager>;
    pub struct AzCliResourceManager {
    }
    #[async_trait]
    impl CosmosResourceManager for AzCliResourceManager {
        async fn create_database(&self, database_name: &str) -> Result<()>;
        async fn create_or_update_container(&self, database_name: &str, container_name: &str, resource: &serde_json::Value, throughput: Option<ResourceThroughput>) -> Result<()>;
        async fn delete_container(&self, database_name: &str, container_name: &str) -> Result<()>;
        async fn delete_database(&self, database_name: &str) -> Result<()>;
        async fn merge_partitions(&self, database_name: &str, container_name: &str) -> Result<()>;
        async fn read_container_throughput(&self, database_name: &str, container_name: &str) -> Result<ResourceThroughput>;
        async fn read_database_throughput(&self, database_name: &str) -> Result<Option<ResourceThroughput>>;
        async fn replace_container_throughput(&self, database_name: &str, container_name: &str, throughput: ResourceThroughput) -> Result<ResourceThroughput>;
    }
    #[derive(Debug)]
    pub struct ResourceManagerError {
    }
    impl Display for ResourceManagerError {
        fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result;
    }
    impl Error for ResourceManagerError {
    }
    impl From<Error> for ResourceManagerError {
        fn from(error: serde_json::Error) -> Self;
    }
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub enum ResourceThroughput {
        Manual(u64),
        Autoscale { maximum: u64, current: Option<u64>, increment_percent: Option<u32> },
    }
    #[async_trait]
    pub trait CosmosResourceManager: Send + Sync {
        async fn create_database(&self, database_name: &str) -> Result<()>;
        async fn create_or_update_container(&self, database_name: &str, container_name: &str, resource: &serde_json::Value, throughput: Option<ResourceThroughput>) -> Result<()>;
        async fn delete_container(&self, database_name: &str, container_name: &str) -> Result<()>;
        async fn delete_database(&self, database_name: &str) -> Result<()>;
        async fn merge_partitions(&self, database_name: &str, container_name: &str) -> Result<()>;
        async fn read_container_throughput(&self, database_name: &str, container_name: &str) -> Result<ResourceThroughput>;
        async fn read_database_throughput(&self, database_name: &str) -> Result<Option<ResourceThroughput>>;
        async fn replace_container_throughput(&self, database_name: &str, container_name: &str, throughput: ResourceThroughput) -> Result<ResourceThroughput>;
    }
    pub type ResourceManager = std::sync::Arc<dyn CosmosResourceManager>;
    pub type Result<T> = std::result::Result<T, ResourceManagerError>;
}
```

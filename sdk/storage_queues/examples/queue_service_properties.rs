use azure_storage::prelude::*;
use azure_storage_queues::{prelude::*, CorsRule};

#[tokio::main]
async fn main() -> azure_core::Result<()> {
    tracing_subscriber::fmt().init();

    // First we retrieve the account name and access key from environment variables.
    let account =
        std::env::var("STORAGE_ACCOUNT").expect("Set env variable STORAGE_ACCOUNT first!");
    let access_key =
        std::env::var("STORAGE_ACCESS_KEY").expect("Set env variable STORAGE_ACCESS_KEY first!");

    let storage_credentials = StorageCredentials::access_key(account.clone(), access_key);
    let queue_service = QueueServiceClient::new(account, storage_credentials);
    let properties = queue_service.get_queue_service_properties().await?;
    println!("properties: {properties:#?}");

    let mut properties = properties.queue_service_properties;
    properties.cors.cors_rule = Some(vec![CorsRule {
        allowed_origins: "http://www.contoso.com,http://www.fabrikam.com".to_owned(),
        allowed_methods: "GET,PUT".to_owned(),
        allowed_headers: "x-ms-meta-abc,x-ms-meta-data*,x-ms-meta-target*,x-ms-meta-xyz".to_owned(),
        exposed_headers: String::new(),
        max_age_in_seconds: 50000,
    }]);

    queue_service
        .set_queue_service_properties(properties.clone())
        .await?;
    let properties = queue_service.get_queue_service_properties().await?;
    println!("properties: {properties:#?}");

    Ok(())
}

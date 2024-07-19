// cspell: words eventhub eventhubs
use azure_core::error::Result;
use azure_identity::{DefaultAzureCredential, TokenCredentialOptions};
use azure_messaging_eventhubs::producer::{ProducerClient, ProducerClientOptions};

use std::env;
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
    let _ = tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .finish();

    let host = env::var("EVENTHUBS_HOST").unwrap();
    let eventhub = env::var("EVENTHUB_NAME").unwrap();

    let credential = DefaultAzureCredential::create(TokenCredentialOptions::default()).unwrap();

    let client = ProducerClient::new(
        host,
        eventhub.clone(),
        credential,
        ProducerClientOptions::builder()
            .with_application_id("test_get_properties")
            .build(),
    )
    .unwrap();
    let result = client.open().await;
    info!("Open result: {:?}", result);
    if result.is_err() {
        println!("Error opening client: {:?}", result.err());
        return Ok(());
    }
    let properties = client.get_eventhub_properties().await.unwrap();
    println!("Eventhub Properties for: {eventhub} {:?}", properties);

    for partition in properties.partition_ids.iter() {
        let partition_properties = client.get_partition_properties(partition).await.unwrap();
        println!(
            "Partition Properties for: {partition} {:?}",
            partition_properties
        );
    }
    Ok(())
}

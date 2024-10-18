// Copyright (c) Microsoft Corporation. All Rights reserved
// Licensed under the MIT license.
// cspell: words

use azure_core::error::Result;
use azure_identity::DefaultAzureCredential;
use azure_messaging_eventhubs::producer::{ProducerClient, ProducerClientOptions};

use std::env;
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
    let _ = tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .finish();

    let host = env::var("EVENTHUBS_HOST").unwrap();
    let eventhub = env::var("EVENTHUB_NAME").unwrap();

    let credential = DefaultAzureCredential::new()?;

    let client = ProducerClient::new(
        host,
        eventhub.clone(),
        credential,
        ProducerClientOptions::builder()
            .with_application_id("test_get_properties")
            .build(),
    );
    let result = client.open().await;
    info!("Open result: {:?}", result);
    if result.is_err() {
        println!("Error opening client: {:?}", result.err());
        return Ok(());
    }
    let properties = client.get_eventhub_properties().await.unwrap();
    println!("Eventhub Properties for: {eventhub} {:?}", properties);
    Ok(())
}

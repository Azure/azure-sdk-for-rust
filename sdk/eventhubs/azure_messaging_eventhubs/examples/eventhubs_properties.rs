// Copyright (c) Microsoft Corporation. All Rights reserved
// Licensed under the MIT license.

use azure_core::error::Result;
use azure_identity::DefaultAzureCredential;
use azure_messaging_eventhubs::ProducerClient;

use std::env;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing subscriber from environment.
    tracing_subscriber::fmt().init();

    let host = env::var("EVENTHUBS_HOST").unwrap();
    let eventhub = env::var("EVENTHUB_NAME").unwrap();

    let credential = DefaultAzureCredential::new()?;

    let result = ProducerClient::builder(host, eventhub.clone(), credential.clone())
        .with_application_id("test_get_properties".to_string())
        .open()
        .await;
    if let Err(err) = result {
        println!("Error opening client: {:?}", err);
        return Ok(());
    }
    let client = result?;
    let properties = client.get_eventhub_properties().await.unwrap();
    println!("Eventhub Properties for: {eventhub} {:?}", properties);
    Ok(())
}

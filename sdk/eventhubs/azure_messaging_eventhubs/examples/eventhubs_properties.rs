// Copyright (c) Microsoft Corporation. All Rights reserved
// Licensed under the MIT license.

//! This sample shows retrieving the properties of an Event Hub.

use azure_identity::DeveloperToolsCredential;
use azure_messaging_eventhubs::ProducerClient;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Initialize tracing subscriber from environment.
    tracing_subscriber::fmt().init();

    let host = env::var("EVENTHUBS_HOST").unwrap();
    let eventhub = env::var("EVENTHUB_NAME").unwrap();

    let credential = DeveloperToolsCredential::new(None)?;

    let client = ProducerClient::builder()
        .with_application_id("test_get_properties".to_string())
        .open(host.as_str(), eventhub.as_str(), credential.clone())
        .await?;
    let properties = client.get_eventhub_properties().await?;
    println!("Eventhub Properties for: {eventhub} {properties:?}");
    Ok(())
}

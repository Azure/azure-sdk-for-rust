// Copyright (c) Microsoft Corporation. All Rights reserved
// Licensed under the MIT license.

//! This sample shows retrieving the properties of an Event Hub.

use azure_core::error::Result;
use azure_identity::DeveloperToolsCredential;
use azure_messaging_eventhubs::ProducerClient;
use std::env;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing subscriber from environment.
    tracing_subscriber::fmt().init();

    let host = env::var("EVENTHUBS_HOST").unwrap();
    let eventhub = env::var("EVENTHUB_NAME").unwrap();

    let credential = DeveloperToolsCredential::new(None)?;

    let result = ProducerClient::builder()
        .with_application_id("test_get_properties".to_string())
        .open(host.as_str(), eventhub.as_str(), credential.clone())
        .await;
    if let Err(err) = result {
        eprintln!("Error opening client: {err}");
        return Err(err);
    }
    let client = result?;
    let properties = client.get_eventhub_properties().await.unwrap();
    println!("Eventhub Properties for: {eventhub} {properties:?}");

    Ok(())
}

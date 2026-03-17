// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Queue service client example for Azure Queue Storage.
//!
//! This sample demonstrates service-level operations:
//! 1. Create a queue through [`QueueServiceClient`].
//! 2. Set and read service properties.
//! 3. List queues with metadata included.
//! 4. Query queue service statistics from the secondary endpoint when available.
//! 5. Delete the temporary queue.
//!
//! # Prerequisites
//!
//! - Set `AZURE_QUEUE_STORAGE_ACCOUNT_NAME` to your storage account name.
//! - Sign in with `az login` (or any other credential flow supported by [`DeveloperToolsCredential`]).
//! - For the statistics section, use a geo-redundant storage account with a readable secondary endpoint.
//!
//! # Usage
//!
//! ```bash
//! az login
//! export AZURE_QUEUE_STORAGE_ACCOUNT_NAME="<your-storage-account>"
//! cargo run --package azure_storage_queue --example queue_service_client
//! ```

use std::{collections::HashMap, env, sync::Arc};

use azure_core::credentials::TokenCredential;
use azure_identity::DeveloperToolsCredential;
use azure_storage_queue::{
    models::{
        CorsRule, ListQueuesIncludeType, QueueServiceClientListQueuesOptions,
        QueueServiceProperties,
    },
    QueueServiceClient,
};
use futures::StreamExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let account = env::var("AZURE_QUEUE_STORAGE_ACCOUNT_NAME")
        .expect("Set AZURE_QUEUE_STORAGE_ACCOUNT_NAME environment variable");

    let endpoint = format!("https://{}.queue.core.windows.net/", account);
    let queue_name = random_queue_name();

    let credential = DeveloperToolsCredential::new(None)?;
    let service_client = QueueServiceClient::new(&endpoint, Some(credential.clone()), None)?;
    let queue_client = service_client.queue_client(&queue_name)?;

    println!("Creating queue '{queue_name}'...");
    queue_client.create(None).await?;
    queue_client
        .set_metadata(
            &HashMap::from([("sample".to_string(), "service-client".to_string())]),
            None,
        )
        .await?;

    set_and_get_service_properties(&service_client).await?;
    list_queues(&service_client, &queue_name).await?;
    get_service_statistics(&account, credential).await?;

    queue_client.delete(None).await?;
    println!("Deleted queue '{queue_name}'");

    Ok(())
}

/// Sets a CORS rule on the service, then reads back the properties to confirm.
async fn set_and_get_service_properties(
    service_client: &QueueServiceClient,
) -> Result<(), Box<dyn std::error::Error>> {
    let properties = QueueServiceProperties {
        cors: Some(vec![CorsRule {
            allowed_origins: Some("https://example.com".to_string()),
            allowed_methods: Some("GET,POST".to_string()),
            max_age_in_seconds: Some(3600),
            exposed_headers: Some("x-ms-meta-data".to_string()),
            allowed_headers: Some("x-ms-meta-target".to_string()),
        }]),
        ..Default::default()
    };
    service_client
        .set_properties(properties.try_into()?, None)
        .await?;
    println!("Updated queue service properties");

    let retrieved = service_client.get_properties(None).await?.into_model()?;
    println!(
        "Service properties loaded. CORS rules configured: {}",
        retrieved.cors.as_ref().map(Vec::len).unwrap_or(0)
    );

    Ok(())
}

/// Lists queues matching a prefix, printing name and metadata for each.
async fn list_queues(
    service_client: &QueueServiceClient,
    prefix: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let options = QueueServiceClientListQueuesOptions {
        prefix: Some(prefix.to_string()),
        include: Some(vec![ListQueuesIncludeType::Metadata]),
        ..Default::default()
    };
    let mut pages = service_client.list_queues(Some(options))?.into_pages();
    println!("Listing queues with prefix '{prefix}'...");
    while let Some(page) = pages.next().await {
        let queue_list = page?.into_model()?;
        for queue in queue_list.queue_items {
            println!("  Queue: {}", queue.name.unwrap_or_default());
            for (key, value) in queue.metadata.unwrap_or_default() {
                println!("    {key}: {value}");
            }
        }
    }

    Ok(())
}

/// Queries geo-replication statistics from the secondary endpoint.
/// Prints a message and continues if the account has no readable secondary.
async fn get_service_statistics(
    account: &str,
    credential: Arc<dyn TokenCredential>,
) -> Result<(), Box<dyn std::error::Error>> {
    let secondary_endpoint = format!("https://{account}-secondary.queue.core.windows.net/");
    let secondary_client = QueueServiceClient::new(&secondary_endpoint, Some(credential), None)?;
    match secondary_client.get_statistics(None).await {
        Ok(response) => {
            let stats = response.into_model()?;
            if let Some(geo_replication) = stats.geo_replication {
                println!(
                    "Geo-replication status: {:?}, last sync time: {:?}",
                    geo_replication.status, geo_replication.last_sync_time
                );
            }
        }
        Err(err) => {
            eprintln!(
                "Skipping statistics example because the secondary endpoint is unavailable: {err}"
            );
        }
    }

    Ok(())
}

fn random_queue_name() -> String {
    use rand::RngExt;

    let mut rng = rand::rng();
    let random_suffix: u32 = rng.random_range(1000..9999);
    format!("sdk-test-queue-{random_suffix}")
}

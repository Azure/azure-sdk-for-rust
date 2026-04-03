// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use azure_identity::DeveloperToolsCredential;
use azure_resourcemanager_vmware::AVSClient;
use futures::TryStreamExt;
use std::env;

/// Lists all Azure VMware Solution private clouds in the current subscription.
///
/// # Usage
///
/// ```sh
/// # Set subscription ID via env var or pass as an argument
/// export AZURE_SUBSCRIPTION_ID="00000000-0000-0000-0000-000000000000"
/// cargo run -p azure_resourcemanager_vmware --example list_private_clouds
/// ```
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let subscription_id = env::args()
        .nth(1)
        .or_else(|| env::var("AZURE_SUBSCRIPTION_ID").ok())
        .expect("AZURE_SUBSCRIPTION_ID must be set or passed as an argument");

    let credential = DeveloperToolsCredential::new(None)?;
    let client = AVSClient::new(
        "https://management.azure.com",
        credential,
        subscription_id,
        None,
    )?;

    let private_clouds_client = client.get_avs_private_clouds_client();
    let mut pager = private_clouds_client.list_in_subscription(None)?;

    println!("{:<40} {:<15} {}", "NAME", "LOCATION", "ID");
    println!("{}", "-".repeat(100));

    while let Some(cloud) = pager.try_next().await? {
        println!(
            "{:<40} {:<15} {}",
            cloud.name.as_deref().unwrap_or("-"),
            cloud.location.as_deref().unwrap_or("-"),
            cloud.id.as_deref().unwrap_or("-"),
        );
    }

    Ok(())
}

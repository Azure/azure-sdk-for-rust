// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Sample demonstrating queue access policies (`set_access_policy` / `get_access_policy`).
//!
//! Access policies attach named, time-bounded permission sets to a queue. They can be
//! referenced by Shared Access Signature (SAS) tokens so that permissions can be
//! revoked or extended without regenerating the SAS token itself.
//!
//! This sample:
//! 1. Creates a queue.
//! 2. Sets an access policy named `"read-only"` that allows read (`r`) and process (`p`) access.
//! 3. Retrieves and prints the access policies on the queue.
//! 4. Clears all policies by sending an empty `SignedIdentifiers`.
//! 5. Deletes the queue.
//!
//! # Prerequisites
//!
//! - Set `AZURE_QUEUE_STORAGE_ACCOUNT_NAME` to your storage account name.
//! - Sign in with `az login` (or any credential supported by [`DeveloperToolsCredential`]).
//!
//! # Usage
//!
//! ```bash
//! az login
//! export AZURE_QUEUE_STORAGE_ACCOUNT_NAME="<your-storage-account>"
//! cargo run --package azure_storage_queue --example access_policy
//! ```

use azure_identity::DeveloperToolsCredential;
use azure_storage_queue::{
    models::{AccessPolicy, SignedIdentifier, SignedIdentifiers},
    QueueClient,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let account = std::env::var("AZURE_QUEUE_STORAGE_ACCOUNT_NAME")
        .expect("Set AZURE_QUEUE_STORAGE_ACCOUNT_NAME environment variable");

    let endpoint = format!("https://{}.queue.core.windows.net/", account);
    let queue_name = "access-policy-sample-queue";

    let credential = DeveloperToolsCredential::new(None)?;
    let queue_client = QueueClient::new(&endpoint, queue_name, Some(credential), None)?;

    // Create the queue.
    queue_client.create(None).await?;
    println!("Created queue '{queue_name}'");

    // --- Set access policies ---
    // Define a policy that grants read ('r') and process ('p') access.
    // start/expiry are optional; omitting them means the policy is active immediately
    // and never expires. Use a Shared Access Signature (SAS) to attach this policy
    // to a token with a specific expiry instead.
    let policy = AccessPolicy {
        start: None,
        expiry: None,
        permission: Some("rp".to_string()),
    };
    let identifiers = SignedIdentifiers {
        items: Some(vec![SignedIdentifier {
            id: Some("read-only".to_string()),
            access_policy: Some(policy),
        }]),
    };

    queue_client
        .set_access_policy(identifiers.try_into()?, None)
        .await?;
    println!("Set access policy 'read-only' on queue '{queue_name}'");

    // --- Get access policies ---
    let response = queue_client.get_access_policy(None).await?;
    let policies = response.into_model()?;
    for identifier in policies.items.unwrap_or_default() {
        let id = identifier.id.as_deref().unwrap_or("<unnamed>");
        if let Some(ap) = &identifier.access_policy {
            println!(
                "Policy '{}': permissions={}, start={:?}, expiry={:?}",
                id,
                ap.permission.as_deref().unwrap_or(""),
                ap.start,
                ap.expiry,
            );
        }
    }

    // --- Clear all policies ---
    let empty = SignedIdentifiers { items: None };
    queue_client
        .set_access_policy(empty.try_into()?, None)
        .await?;
    println!("Cleared all access policies on queue '{queue_name}'");

    // Delete the queue.
    queue_client.delete(None).await?;
    println!("Deleted queue '{queue_name}'");

    Ok(())
}

// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use azure_core::credentials::TokenCredential;
use azure_identity::DeveloperToolsCredential;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let subscription_id =
        std::env::var("AZURE_SUBSCRIPTION_ID").expect("AZURE_SUBSCRIPTION_ID required");

    let credential = DeveloperToolsCredential::new(None).map(Arc::new)?;

    // Enumerate the Azure storage accounts in the subscription using the REST API directly.
    // This is just an example: you would normally pass in an `Arc::new(credential)` to an Azure SDK client.
    let url = url::Url::parse(&format!("https://management.azure.com/subscriptions/{subscription_id}/providers/Microsoft.Storage/storageAccounts?api-version=2019-06-01"))?;

    let access_token = credential
        .get_token(&["https://management.azure.com/.default"], None)
        .await?;

    let response = reqwest::Client::new()
        .get(url)
        .header(
            "Authorization",
            format!("Bearer {}", access_token.token.secret()),
        )
        .send()
        .await?
        .text()
        .await?;

    println!("{response}");
    Ok(())
}

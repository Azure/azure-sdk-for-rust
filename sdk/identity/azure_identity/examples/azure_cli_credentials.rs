// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use azure_core::credentials::TokenCredential;
use azure_identity::AzureCliCredential;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let credential = AzureCliCredential::new(None)?;

    // Get a token for Azure Resource Management.
    // Azure SDK clients call `get_token` automatically, so you don't
    // normally need to call it directly. This is just to demonstrate
    // how to acquire a token using a specific credential.
    let access_token = credential
        .get_token(&["https://management.azure.com/.default"], None)
        .await?;
    println!("token expires on {}", access_token.expires_on);

    Ok(())
}

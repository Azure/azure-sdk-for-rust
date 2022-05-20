# Azure SDK for Rust - Azure Identity Crate

 Azure Identity crate for the unofficial Microsoft Azure SDK for Rust. This crate is part of a collection of crates: for more information please refer to [https://github.com/azure/azure-sdk-for-rust](https://github.com/azure/azure-sdk-for-rust).
This crate provides mechanisms for several ways to authenticate against Azure

Several implementations of `azure_core::auth::TokenCredential` trait are available:

- DefaultAzureCredential
- EnvironmentCredential
- ImdsManagedIdentityCredential
- AzureCliCredential
- AutoRefreshingTokenCredential

There are several [examples](https://github.com/Azure/azure-sdk-for-rust/tree/main/sdk/identity/examples) available. The [service examples](https://github.com/Azure/azure-sdk-for-rust/tree/main/services#examples) mostly use `AzureCliCredential`.

To authenticate using the client credential flow, you can do the following:

```rust
use azure_identity::client_credentials_flow;
use oauth2::{ClientId, ClientSecret};
use url::Url;

use std::env;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client_id =
        ClientId::new(env::var("CLIENT_ID").expect("Missing CLIENT_ID environment variable."));
    let client_secret = ClientSecret::new(
        env::var("CLIENT_SECRET").expect("Missing CLIENT_SECRET environment variable."),
    );
    let tenant_id = env::var("TENANT_ID").expect("Missing TENANT_ID environment variable.");
    let subscription_id =
        env::var("SUBSCRIPTION_ID").expect("Missing SUBSCRIPTION_ID environment variable.");

    let client = reqwest::Client::new();
    // This will give you the final token to use in authorization.
    let token = client_credentials_flow::perform(
        client,
        &client_id,
        &client_secret,
        &["https://management.azure.com/"],
        &tenant_id,
    )
    .await?;
    Ok(())
}
```

The supported authentication flows are:
* [Authorization code flow](https://docs.microsoft.com/azure/active-directory/develop/v2-oauth2-auth-code-flow).
* [Client credentials flow](https://docs.microsoft.com/azure/active-directory/develop/v2-oauth2-client-creds-grant-flow).
* [Device code flow](https://docs.microsoft.com/azure/active-directory/develop/v2-oauth2-device-code).

This crate also includes utilities for handling refresh tokens and accessing token credentials from many different sources.

A list of changes can be found in [CHANGELOG.md](https://github.com/Azure/azure-sdk-for-rust/blob/main/sdk/identity/CHANGELOG.md);

# azure_identity

Azure Identity crate for the unofficial Microsoft Azure SDK for Rust. This crate is part of a collection of crates: for more information please refer to <https://github.com/Azure/azure-sdk-for-rust>.

This crate provides several implementations of the [azure_core::auth::TokenCredential](https://docs.rs/azure_core/latest/azure_core/auth/trait.TokenCredential.html) trait.
It is recommended to start with `azure_identity::create_credential()?`, which will create an instance of `DefaultAzureCredential` by default. If you want to use a specific credential type, the `AZURE_CREDENTIAL_KIND` environment variable may be set to a value from `azure_credential_kinds`, such as `azurecli` or `virtualmachine`.

```rust,no_run
use azure_core::credentials::TokenCredential;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let subscription_id =
        std::env::var("AZURE_SUBSCRIPTION_ID").expect("AZURE_SUBSCRIPTION_ID required");

    let credential: Arc<dyn TokenCredential> = azure_identity::DefaultAzureCredential::new()?;

    // Let's enumerate the Azure storage accounts in the subscription using the REST API directly.
    // This is just an example. It is easier to use the Azure SDK for Rust crates.
    let url = url::Url::parse(&format!("https://management.azure.com/subscriptions/{subscription_id}/providers/Microsoft.Storage/storageAccounts?api-version=2019-06-01"))?;

    let access_token = credential
        .get_token(&["https://management.azure.com/.default"])
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
```

## Design

Each `TokenCredential` implementation provides a `new` constructor that returns an `azure_core::Result<Arc<Self>>`. The credential provider is contained within an `Arc` because these are designed to be reused by multiple clients for efficiency e.g.:

```rust
use azure_core::credentials::TokenCredential;
use azure_identity::DefaultAzureCredential;
# use azure_core::{ClientOptions, Result};
# use std::sync::Arc;
# struct StorageAccountClient;
# impl StorageAccountClient {
#     fn new(_endpoint: &str, _credential: Arc<dyn TokenCredential>, _options: Option<ClientOptions>) -> Result<Arc<Self>> {
#         Ok(Arc::new(StorageAccountClient))
#     }
# }
# struct SecretClient;
# impl SecretClient {
#     fn new(_endpoint: &str, _credential: Arc<dyn TokenCredential>, _options: Option<ClientOptions>) -> Result<Arc<Self>> {
#         Ok(Arc::new(SecretClient))
#     }
# }

let credential = DefaultAzureCredential::new().unwrap();
let storage_client = StorageAccountClient::new(
    "https://myaccount.blob.storage.azure.net",
    credential.clone(),
    None,
);
let secret_client = SecretClient::new("https://myvault.keyvault.azure.net",
    credential.clone(),
    None,
);
```

Credentials are cached in memory and refreshed as needed. Using the same credentials in multiple clients prevents authenticating and refreshing tokens numerous times for each client otherwise.

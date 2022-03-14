/*
Prints the name of pools using the data plane APIs

cargo run --package azure_svc_batch --example list_pools
*/

use azure_identity::token_credentials::AzureCliCredential;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let account_name = std::env::args().nth(1).expect("please specify batch account");
    let region = std::env::args().nth(2).expect("please specify region");

    let endpoint = format!("https://{}.{}.batch.azure.com", account_name, region);
    let scopes = &["https://batch.core.windows.net/"];
    let credential = Arc::new(AzureCliCredential {});
    let client = azure_svc_batch::ClientBuilder::new(credential)
        .endpoint(endpoint)
        .scopes(scopes)
        .build();

    let pools = client.pool().list().await?;

    for pool in pools.value {
        println!("id: {:?}", pool.id);
    }
    Ok(())
}

/*
Prints the name of pools using the data plane APIs

cargo run --package azure_svc_batch --example list_pools
*/

use azure_identity::token_credentials::AzureCliCredential;
use azure_svc_batch::operations::pool;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let account_name = std::env::args().nth(1).expect("please specify batch account");
    let region = std::env::args().nth(2).expect("please specify region");

    let base_path = format!("https://{}.{}.batch.azure.com", account_name, region);

    let http_client = azure_core::new_http_client();
    let token_credential = Box::new(AzureCliCredential {});
    let config = &azure_svc_batch::config(http_client, token_credential)
        .base_path(base_path)
        .token_credential_resource("https://batch.core.windows.net/")
        .build();

    let pools = pool::list(config, None, None, None, None, None, None, None, None).await?;

    for pool in pools.value {
        println!("id: {:?}", pool.id);
    }
    Ok(())
}

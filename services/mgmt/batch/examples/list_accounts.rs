/*
Lists Batch Accounts, similar to:
az batch account list --queyr [].id

cargo run --example list_accounts
*/

use azure_identity::token_credentials::AzureCliCredential;
use azure_mgmt_batch::operations::batch_account;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let http_client = azure_core::new_http_client();
    let token_credential = AzureCliCredential {};
    let subscription_id = &AzureCliCredential::get_subscription()?;
    let config = &azure_mgmt_batch::config(http_client, Box::new(token_credential)).build();

    let accounts = batch_account::list(config, subscription_id).await?;

    for account in accounts.value {
        println!("{}", account.resource.id.unwrap_or_default());
    }
    Ok(())
}

/*
Lists Batch Accounts, similar to:
az batch account list --queyr [].id

cargo run --example list_accounts
*/

use azure_identity::token_credentials::AzureCliCredential;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let credential = Arc::new(AzureCliCredential {});
    let subscription_id = &AzureCliCredential::get_subscription()?;
    let client = azure_mgmt_batch::ClientBuilder::new(credential).build();

    let accounts = client.batch_account().list(subscription_id).into_future().await?;

    for account in accounts.value {
        println!("{}", account.resource.id.unwrap_or_default());
    }
    Ok(())
}

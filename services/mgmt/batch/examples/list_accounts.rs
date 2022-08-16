/*
Lists Batch Accounts, similar to:
az batch account list --queyr [].id

cargo run --example list_accounts
*/

use azure_identity::AzureCliCredential;
use futures::stream::StreamExt;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let credential = Arc::new(AzureCliCredential::new());
    let subscription_id = AzureCliCredential::get_subscription()?;
    let client = azure_mgmt_batch::Client::builder(credential).build();

    let mut accounts = client.batch_account_client().list(subscription_id).into_stream();
    while let Some(accounts) = accounts.next().await {
        let accounts = accounts?;
        for account in accounts.value {
            println!("{}", account.resource.id.unwrap_or_default());
        }
    }

    Ok(())
}

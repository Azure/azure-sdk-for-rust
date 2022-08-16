/*
Lists the storage accounts, similar to:
az storage account list --query [].id

cargo run --package azure_mgmt_storage --example storage_account_list
*/

use azure_identity::AzureCliCredential;
use futures::stream::StreamExt;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let credential = Arc::new(AzureCliCredential::new());
    let subscription_id = AzureCliCredential::get_subscription()?;
    let client = azure_mgmt_storage::Client::builder(credential).build();

    let mut count = 0;
    let mut stream = client.storage_accounts_client().list(subscription_id).into_stream();
    while let Some(accounts) = stream.next().await {
        let accounts = accounts?;
        count += accounts.value.len();
        for account in &accounts.value {
            println!("{:?}", account.tracked_resource.resource.id);
        }
    }

    println!("# of storage accounts {}", count);
    Ok(())
}

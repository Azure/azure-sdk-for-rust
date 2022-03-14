/*
Lists the storage accounts, similar to:
az storage account list --query [].id

cargo run --package azure_mgmt_storage --example storage_account_list
*/

use azure_identity::token_credentials::AzureCliCredential;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let credential = Arc::new(AzureCliCredential {});
    let subscription_id = AzureCliCredential::get_subscription()?;
    let client = azure_mgmt_storage::ClientBuilder::new(credential).build();

    let accounts = client.storage_accounts().list(subscription_id).await?;
    println!("# of storage accounts {}", accounts.value.len());
    for account in &accounts.value {
        println!("{:?}", account.tracked_resource.resource.id);
    }
    Ok(())
}

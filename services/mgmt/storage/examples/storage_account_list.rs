/*
Lists the storage accounts, similar to:
az storage account list --query [].id

cargo run --example storage_account_list
*/

use azure_identity::token_credentials::AzureCliCredential;
use azure_mgmt_storage::operations::storage_accounts;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let http_client = azure_core::new_http_client();
    let token_credential = AzureCliCredential {};
    let subscription_id = &AzureCliCredential::get_subscription()?;
    let config = &azure_mgmt_storage::config(http_client, Box::new(token_credential)).build();

    let accounts = storage_accounts::list(config, subscription_id).await?;
    println!("# of storage accounts {}", accounts.value.len());
    for account in &accounts.value {
        println!("{:?}", account.tracked_resource.resource.id);
    }
    Ok(())
}

/*
Lists the storage accounts, similar to:
az storage account list --query [].id

cargo run --example storage_account_list
*/

use azure_identity::token_credentials::AzureCliCredential;
use azure_mgmt_storage::operations::storage_accounts;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[tokio::main]
async fn main() -> Result<()> {
    let http_client: std::sync::Arc<Box<dyn azure_core::HttpClient>> = std::sync::Arc::new(Box::new(reqwest::Client::new()));
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

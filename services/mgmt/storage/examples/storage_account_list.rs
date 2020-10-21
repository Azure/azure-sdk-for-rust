/*
Lists the storage accounts, similar to:
az storage account list --query [].id

export SUBSCRIPTION_ID=$(az account show --query id --output tsv)
export ACCESS_TOKEN=$(az account get-access-token --query accessToken --output tsv)
cargo run --example storage_account_list
*/

use azure_mgmt_storage::operations::storage_accounts;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[tokio::main]
async fn main() -> Result<()> {
    let subscription_id = &get_subscription_id()?;
    let access_token = &get_access_token()?;
    let config = &azure_mgmt_storage::OperationConfig::new(access_token);
    let accounts = storage_accounts::list(config, subscription_id).await?;
    println!("# of storage accounts {}", accounts.value.len());
    for account in &accounts.value {
        println!("{:?}", account.tracked_resource.resource.id);
    }
    Ok(())
}

fn get_subscription_id() -> Result<String> {
    Ok(std::env::var("SUBSCRIPTION_ID").map_err(|_| "SUBSCRIPTION_ID required")?)
}

fn get_access_token() -> Result<String> {
    Ok(std::env::var("ACCESS_TOKEN").map_err(|_| "ACCESS_TOKEN required")?)
}

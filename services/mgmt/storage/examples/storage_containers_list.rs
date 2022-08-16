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
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    let credential = Arc::new(AzureCliCredential::new());
    let subscription_id = AzureCliCredential::get_subscription()?;
    let client = azure_mgmt_storage::Client::builder(credential).build();

    let group = std::env::args().nth(1).expect("please specify resource group");
    let account = std::env::args().nth(2).expect("please specify account");

    let mut stream = client
        .blob_containers_client()
        .list(&group, &account, &subscription_id)
        .into_stream();
    let mut count = 0;
    while let Some(x) = stream.next().await {
        let x = x?;
        count += x.value.len();
        for x in x.value {
            println!("name: {}", x.azure_entity_resource.resource.name.unwrap_or_default());
        }
    }
    println!("saw {} entries", count);

    Ok(())
}

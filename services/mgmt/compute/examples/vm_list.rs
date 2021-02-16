/*
Lists the virtual , similar to:
az vm list --query [].id

cargo run --example vm_list
*/

use azure_identity::token_credentials::AzureCliCredential;
use azure_mgmt_compute::operations::virtual_machines;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[tokio::main]
async fn main() -> Result<()> {
    let http_client: std::sync::Arc<Box<dyn azure_core::HttpClient>> = std::sync::Arc::new(Box::new(reqwest::Client::new()));
    let token_credential = AzureCliCredential {};
    let subscription_id = &AzureCliCredential::get_subscription()?;
    let config = &azure_mgmt_compute::config(http_client, Box::new(token_credential)).build();

    let vms = virtual_machines::list_all(config, subscription_id, None).await?;
    println!("# of virtual machines {}", vms.value.len());
    for vm in &vms.value {
        println!("{:?}", &vm.resource.id);
    }
    Ok(())
}

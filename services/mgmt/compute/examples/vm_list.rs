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
    let token_credential = AzureCliCredential {};
    let subscription_id = &AzureCliCredential::get_subscription()?;
    let config = &azure_mgmt_compute::OperationConfig::new(Box::new(token_credential));
    let vms = virtual_machines::list_all(config, subscription_id, None).await?;
    println!("# of virtual machines {}", vms.value.len());
    for vm in &vms.value {
        println!("{:?}", &vm.resource.id);
    }
    Ok(())
}

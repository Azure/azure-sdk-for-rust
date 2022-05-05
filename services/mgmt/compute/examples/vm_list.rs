/*
Lists the virtual , similar to:
az vm list --query [].id

cargo run --package azure_mgmt_compute --example vm_list
*/

use azure_identity::token_credentials::AzureCliCredential;
use futures::stream::StreamExt;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let credential = Arc::new(AzureCliCredential {});
    let subscription_id = AzureCliCredential::get_subscription()?;
    let client = azure_mgmt_compute::ClientBuilder::new(credential).build();

    let mut count = 0;
    let mut vms = client.virtual_machines().list_all(subscription_id).into_stream();
    while let Some(vms) = vms.next().await {
        let vms = vms?;
        count += vms.value.len();
        for vm in vms.value {
            println!("{:?}", &vm.resource.id);
        }
    }
    println!("# of virtual machines {}", count);

    Ok(())
}

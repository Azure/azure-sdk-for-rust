/*
prints information about available pools.

$ cargo run --example list_pools --release -- myresourcegroup mybatchaccountname 2>/dev/null
name: "mypoolname"
provisioning state: Succeeded
vm_size: STANDARD_D2_V2
image reference: ImageReference { publisher: Some("canonical"), offer: Some("ubuntuserver"), sku: Some("18.04-lts"), version: Some("latest"), id: None }
*/

use azure_identity::AzureCliCredential;
use futures::stream::StreamExt;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let resource_group_name = std::env::args().nth(1).expect("please specify resource group");
    let account_name = std::env::args().nth(2).expect("please specify batch account");

    let credential = Arc::new(AzureCliCredential {});
    let subscription_id = AzureCliCredential::get_subscription()?;
    let client = azure_mgmt_batch::Client::builder(credential).build();

    let mut pools = client
        .pool_client()
        .list_by_batch_account(resource_group_name, account_name, subscription_id)
        .into_stream();

    while let Some(pools) = pools.next().await {
        let pools = pools?;
        for pool in pools.value {
            println!("name: {:?}", pool.proxy_resource.name.unwrap_or_default());
            if let Some(properties) = &pool.properties {
                if let Some(provisioning_state) = &properties.provisioning_state {
                    println!("provisioning state: {:?}", provisioning_state);
                }
                if let Some(vm_size) = &properties.vm_size {
                    println!("vm_size: {}", vm_size);
                }

                if let Some(image_reference) = properties
                    .deployment_configuration
                    .as_ref()
                    .map(|x| x.virtual_machine_configuration.as_ref().map(|x| &x.image_reference))
                    .flatten()
                {
                    println!("image reference: {:?}", image_reference);
                }
            }
        }
    }
    Ok(())
}

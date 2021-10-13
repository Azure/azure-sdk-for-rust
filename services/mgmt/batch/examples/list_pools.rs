/*
prints information about available pools.

$ cargo run --example list_pools --release -- myresourcegroup mybatchaccountname 2>/dev/null
name: "mypoolname"
provisioning state: Succeeded
vm_size: STANDARD_D2_V2
image reference: ImageReference { publisher: Some("canonical"), offer: Some("ubuntuserver"), sku: Some("18.04-lts"), version: Some("latest"), id: None }
*/

use azure_identity::token_credentials::AzureCliCredential;
use azure_mgmt_batch::operations::pool;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let resource_group_name = std::env::args().nth(1).expect("please specify resource group");
    let account_name = std::env::args().nth(2).expect("please specify batch account");

    let http_client = azure_core::new_http_client();
    let token_credential = AzureCliCredential {};
    let subscription_id = &AzureCliCredential::get_subscription()?;
    let config = &azure_mgmt_batch::config(http_client, Box::new(token_credential)).build();

    let pools = pool::list_by_batch_account(config, &resource_group_name, &account_name, None, None, None, subscription_id).await?;

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
    Ok(())
}

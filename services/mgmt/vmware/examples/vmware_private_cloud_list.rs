/*
Lists the private clouds, similar to:
az vmware private-cloud list --query [].id

az extension documentation:
https://docs.microsoft.com/cli/azure/ext/vmware/vmware/private-cloud?view=azure-cli-latest#ext_vmware_az_vmware_private_cloud_list
API documentation:
https://docs.microsoft.com/rest/api/vmware/privateclouds/list

cargo run --example vmware_private_cloud_list
*/

use azure_identity::token_credentials::AzureCliCredential;
use azure_mgmt_vmware::operations::private_clouds;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let http_client = azure_core::new_http_client();
    let token_credential = AzureCliCredential {};
    let subscription_id = &AzureCliCredential::get_subscription()?;
    let config = &azure_mgmt_vmware::config(http_client, Box::new(token_credential)).build();

    let clouds = private_clouds::list_in_subscription(config, subscription_id).await?;
    println!("# of private clouds {}", clouds.value.len());
    for cloud in &clouds.value {
        println!("{:?}", cloud.tracked_resource.resource.id);
    }
    Ok(())
}

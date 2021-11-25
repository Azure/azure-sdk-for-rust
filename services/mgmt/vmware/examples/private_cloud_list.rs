/*
Lists the private clouds, similar to:
az vmware private-cloud list --query [].id

az extension documentation:
https://docs.microsoft.com/cli/azure/ext/vmware/vmware/private-cloud?view=azure-cli-latest#ext_vmware_az_vmware_private_cloud_list
API documentation:
https://docs.microsoft.com/rest/api/vmware/privateclouds/list

cargo run --package azure_mgmt_vmware --example private_cloud_list
*/

use azure_identity::token_credentials::AzureCliCredential;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let credential = Arc::new(AzureCliCredential {});
    let client = azure_mgmt_vmware::ClientBuilder::new(credential).build();

    let name = "George"; // allow passing a reference for parameters
    let ops = client.operations().list(name).into_future().await?;
    println!("# of operations{}", ops.value.len());

    // let subscription_id = &AzureCliCredential::get_subscription()?;
    // let config = &azure_mgmt_vmware::config(http_client, Box::new(credential)).build();

    // let clouds = private_clouds::list_in_subscription(config, subscription_id).await?;
    // println!("# of private clouds {}", clouds.value.len());
    // for cloud in &clouds.value {
    //     println!("{:?}", cloud.tracked_resource.resource.id);
    // }
    Ok(())
}

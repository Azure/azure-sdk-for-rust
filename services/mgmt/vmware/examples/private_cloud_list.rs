/*
Lists the private clouds, similar to:
az vmware private-cloud list --query [].id

az extension documentation:
https://docs.microsoft.com/cli/azure/ext/vmware/vmware/private-cloud?view=azure-cli-latest#ext_vmware_az_vmware_private_cloud_list
API documentation:
https://docs.microsoft.com/rest/api/vmware/privateclouds/list

cargo run --package azure_mgmt_vmware --example private_cloud_list
*/

use azure_identity::AzureCliCredential;
use futures::stream::StreamExt;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let subscription_id = AzureCliCredential::get_subscription()?;
    let credential = Arc::new(AzureCliCredential::new());
    let client = azure_mgmt_vmware::Client::builder(credential).build();

    let mut count = 0;
    let mut clouds = client.private_clouds_client().list_in_subscription(subscription_id).into_stream();
    while let Some(clouds) = clouds.next().await {
        let clouds = clouds?;
        count += clouds.value.len();
        for cloud in clouds.value {
            println!("{:?}", cloud.tracked_resource.resource.id);
        }
    }
    println!("# of private clouds {}", count);
    Ok(())
}

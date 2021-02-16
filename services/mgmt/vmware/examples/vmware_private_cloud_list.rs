/*
Lists the private clouds, similar to:
az vmware private-cloud list --query [].id

az extension documentation:
https://docs.microsoft.com/en-us/cli/azure/ext/vmware/vmware/private-cloud?view=azure-cli-latest#ext_vmware_az_vmware_private_cloud_list
API documentation:
https://docs.microsoft.com/en-us/rest/api/vmware/privateclouds/list

cargo run --example vmware_private_cloud_list
*/

use azure_identity::token_credentials::AzureCliCredential;
use azure_mgmt_vmware::operations::private_clouds;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[tokio::main]
async fn main() -> Result<()> {
    let http_client: std::sync::Arc<Box<dyn azure_core::HttpClient>> = std::sync::Arc::new(Box::new(reqwest::Client::new()));
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

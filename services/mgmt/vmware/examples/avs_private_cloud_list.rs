/*
Lists the private clouds, similar to:
az vmware private-cloud list --query [].id

az extension documentation:
https://docs.microsoft.com/en-us/cli/azure/ext/vmware/vmware/private-cloud?view=azure-cli-latest#ext_vmware_az_vmware_private_cloud_list
API documentation:
https://docs.microsoft.com/en-us/rest/api/avs/privateclouds/list

export SUBSCRIPTION_ID=$(az account show --query id --output tsv)
export ACCESS_TOKEN=$(az account get-access-token --query accessToken --output tsv)
cargo run --example avs_private_cloud_list
*/

use azure_mgmt_avs::operations::private_clouds;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[tokio::main]
async fn main() -> Result<()> {
    let subscription_id = &get_subscription_id()?;
    let access_token = &get_access_token()?;
    let config = &azure_mgmt_avs::Configuration::new(access_token);
    let clouds = private_clouds::list_in_subscription(config, subscription_id).await?;
    println!("# of private clouds {}", clouds.value.len());
    for cloud in &clouds.value {
        println!("{:?}", cloud.tracked_resource.resource.id);
    }
    Ok(())
}

fn get_subscription_id() -> Result<String> {
    Ok(std::env::var("SUBSCRIPTION_ID").map_err(|_| "SUBSCRIPTION_ID required")?)
}

fn get_access_token() -> Result<String> {
    Ok(std::env::var("ACCESS_TOKEN").map_err(|_| "ACCESS_TOKEN required")?)
}

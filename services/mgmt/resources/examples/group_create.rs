/*
Create a resource group, similar to:
az group create --name $RESOURCE_GROUP_NAME --location $RESOURCE_GROUP_LOCATION

export SUBSCRIPTION_ID=$(az account show --query id --output tsv)
export ACCESS_TOKEN=$(az account get-access-token --query accessToken --output tsv)
export RESOURCE_GROUP_NAME=azuresdkforrust
export RESOURCE_GROUP_LOCATION=southcentralus
cargo run --example group_create
*/

use azure_mgmt_resources::{models::ResourceGroup, operations::resource_groups, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let access_token = &get_access_token()?;
    let subscription_id = &get_subscription_id()?;
    let resource_group_name = &get_resource_group_name()?;
    let resource_group_location = get_resource_group_location()?;
    let config = &azure_mgmt_resources::Configuration::new(access_token);

    let group = ResourceGroup {
        id: None,
        name: None,
        type_: None,
        properties: None,
        location: resource_group_location,
        managed_by: None,
        tags: None,
    };
    let group_created = resource_groups::create_or_update(config, resource_group_name, &group, subscription_id).await?;
    println!("group created: {:#?}", group_created);
    Ok(())
}

fn get_subscription_id() -> Result<String> {
    Ok(std::env::var("SUBSCRIPTION_ID").map_err(|_| "SUBSCRIPTION_ID required")?)
}

fn get_access_token() -> Result<String> {
    Ok(std::env::var("ACCESS_TOKEN").map_err(|_| "ACCESS_TOKEN required")?)
}

fn get_resource_group_name() -> Result<String> {
    Ok(std::env::var("RESOURCE_GROUP_NAME").map_err(|_| "RESOURCE_GROUP_NAME required")?)
}

fn get_resource_group_location() -> Result<String> {
    Ok(std::env::var("RESOURCE_GROUP_LOCATION").map_err(|_| "RESOURCE_GROUP_LOCATION required")?)
}

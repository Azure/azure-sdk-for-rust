/*
Create a resource group, similar to:
az group create --name $RESOURCE_GROUP_NAME --location $RESOURCE_GROUP_LOCATION

export RESOURCE_GROUP_NAME=azuresdkforrust
export RESOURCE_GROUP_LOCATION=southcentralus
cargo run --package azure_mgmt_resources --example group_create
*/

use azure_identity::AzureCliCredential;
use azure_mgmt_resources::models::ResourceGroup;
use std::env;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let credential = Arc::new(AzureCliCredential {});
    let subscription_id = &AzureCliCredential::get_subscription()?;
    let resource_group_name = &env::var("RESOURCE_GROUP_NAME").map_err(|_| "RESOURCE_GROUP_NAME required")?;
    let resource_group_location = env::var("RESOURCE_GROUP_LOCATION").map_err(|_| "RESOURCE_GROUP_LOCATION required")?;
    let client = azure_mgmt_resources::Client::builder(credential).build();

    let group = ResourceGroup {
        id: None,
        name: None,
        type_: None,
        properties: None,
        location: resource_group_location,
        managed_by: None,
        tags: None,
    };
    let group_created = client
        .resource_groups_client()
        .create_or_update(resource_group_name, group, subscription_id)
        .into_future()
        .await?;
    println!("group created: {:#?}", group_created);
    Ok(())
}

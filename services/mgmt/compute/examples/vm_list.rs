/*
Lists the virtual , similar to:
az vm list --query [].id

export SUBSCRIPTION_ID=$(az account show --query id --output tsv)
export ACCESS_TOKEN=$(az account get-access-token --query accessToken --output tsv)
cargo run --example vm_list
*/

use azure_mgmt_compute::operations::virtual_machines;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[tokio::main]
async fn main() -> Result<()> {
    let subscription_id = &get_subscription_id()?;
    let access_token = &get_access_token()?;
    let config = &azure_mgmt_compute::Configuration::new(access_token);
    let vms = virtual_machines::list_all(config, subscription_id, None).await?;
    println!("# of virtual machines {}", vms.value.len());
    for vm in &vms.value {
        println!("{:?}", &vm.resource.id);
    }
    Ok(())
}

fn get_subscription_id() -> Result<String> {
    Ok(std::env::var("SUBSCRIPTION_ID").map_err(|_| "SUBSCRIPTION_ID required")?)
}

fn get_access_token() -> Result<String> {
    Ok(std::env::var("ACCESS_TOKEN").map_err(|_| "ACCESS_TOKEN required")?)
}

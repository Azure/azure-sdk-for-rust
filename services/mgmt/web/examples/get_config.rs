// cargo run --package azure_mgmt_web --example get_config -- resource_group webapp

use azure_identity::AzureCliCredential;
use std::{error::Error, sync::Arc};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let resource_group_name = std::env::args().nth(1).expect("please specify resource group");
    let webapp_name = std::env::args().nth(2).expect("please specify webapp name");

    let credential = Arc::new(AzureCliCredential::new());
    let subscription_id = AzureCliCredential::get_subscription()?;
    let client = azure_mgmt_web::Client::builder(credential).build();

    let config = client.web_apps_client().get_configuration(resource_group_name, webapp_name, subscription_id).into_future().await?;
    println!("{:#?}", config);
    Ok(())
}

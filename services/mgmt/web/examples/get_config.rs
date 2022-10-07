// cargo run --package azure_mgmt_web --example get_config

use azure_identity::AzureCliCredential;
use azure_mgmt_web::ClientBuilder;
use serde_json::{to_string_pretty, Value};
use std::{error::Error, sync::Arc};

#[tokio::main]
async fn main()-> Result<(), Box<dyn Error>> {
    let resource_group = "cataggar-avs-cw";
    let webapp_name = "cataggar-avs-cw";
    let sub_id = "7f1fae41-7708-4fa4-89b3-f6552cad2fc1";

    let creds = Arc::new(AzureCliCredential::default());
    let client = ClientBuilder::new(creds).build();

    // let get = client.web_apps_client().get_configuration(resource_group, webapp_name, sub_id).into_future().await?;
    let get = client.web_apps_client().get_configuration(resource_group, webapp_name, sub_id).send().await?;
    let data = get.into_raw_response().into_body().collect().await?;
    // let data = to_string_pretty(&get.properties.unwrap())?;
    println!("{:?}", data);

    // let path = r#"C:\Users\cataggar\OneDrive - Microsoft\Documents\2022-10\2022-10-03 site config b.json"#;
    // let bytes = std::fs::read(path)?;
    // let body: azure_mgmt_web::models::SiteConfigResource = serde_json::from_slice(&bytes)?;

    Ok(())
}

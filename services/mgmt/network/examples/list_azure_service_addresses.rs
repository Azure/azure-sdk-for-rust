/*
prints information about network addresses used for each Azure Service

$ cargo run --release --example list_azure_service_addresses eastus 2>/dev/null | grep ServiceBus.EastUS2:
ServiceBus.EastUS2: ["13.68.110.36/32", "20.36.144.0/26", "20.62.63.0/25", "23.100.67.88/32", "40.70.146.64/29", "40.70.151.128/26", "52.147.163.79/32", "52.167.106.64/29", "52.167.109.128/26", "104.208.144.64/29", "2603:1030:40c:1::220/123", "2603:1030:40c:402::170/125", "2603:1030:40c:802::150/125", "2603:1030:40c:c02::150/125"]
$

*/

use azure_identity::AzureCliCredential;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let location = std::env::args().nth(1).expect("please specify region");
    let credential = Arc::new(AzureCliCredential::new());
    let subscription_id = AzureCliCredential::get_subscription()?;
    let client = azure_mgmt_network::Client::builder(credential).build();

    let response = client.service_tags_client().list(location, subscription_id).into_future().await?;
    for entry in response.values {
        if let Some(name) = entry.name {
            if let Some(properties) = entry.properties {
                println!("{}: {:?}", name, properties.address_prefixes);
            }
        }
    }

    Ok(())
}

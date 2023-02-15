/*
Assumes you already have a appconfiguration instance created.

Use the following command from the ./services folder.

cargo run --package azure_svc_appconfiguration --example get_key_values
*/

use azure_identity::AzureCliCredential;
use futures::stream::StreamExt;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let credential = Arc::new(AzureCliCredential::new());
    //https://azure-rust-sdk.azconfig.io
    // let azconfig_name = std::env::args().nth(1).expect("please specify an existing azconfig");
    // let endpoint = format!("https://{azconfig_name}.azconfig.io");
    let scopes = &["https://azconfig.io/"];
    let endpoint = "http://127.0.0.1:8080";
    let client = azure_svc_appconfiguration::Client::builder(credential)
        .endpoint(endpoint)
        .scopes(scopes)
        .build();

    let mut stream = client.get_key_values().into_stream();

    while let Some(rs) = stream.next().await {
        println!("{:?}", rs);
    }

    Ok(())
}

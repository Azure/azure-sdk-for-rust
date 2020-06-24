use azure_sdk_auth_aad::*;
use futures::stream::StreamExt;
use oauth2::{ClientId, ClientSecret};
use std::env;
use std::error::Error;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client_id =
        ClientId::new(env::var("CLIENT_ID").expect("Missing CLIENT_ID environment variable."));
    let tenant_id = env::var("TENANT_ID").expect("Missing TENANT_ID environment variable.");

    let storage_account_name = std::env::args()
        .nth(1)
        .expect("please specify the storage account name as first command line parameter");

    let client = Arc::new(reqwest::Client::new());
    let device_code_flow = begin_authorize_device_code_flow(
        client.clone(),
        &tenant_id,
        &client_id,
        &[&format!(
            "https://{}.blob.core.windows.net/.default",
            storage_account_name
        )],
    )
    .await?;

    println!("{}", device_code_flow.message());

    let mut stream = Box::pin(device_code_flow.stream());

    while let Some(resp) = stream.next().await {
        println!("{:?}", resp);
    }

    //// Let's enumerate the Azure SQL Databases instances
    //// in the subscription. Note: this way of calling the REST API
    //// will be different (and easier) using other Azure Rust SDK
    //// crates, this is just an example.
    //let url = Url::parse(&format!(
    //        "https://management.azure.com/subscriptions/{}/providers/Microsoft.Sql/servers?api-version=2015-05-01-preview",
    //        subscription_id
    //    ))?;

    //let resp = reqwest::Client::new()
    //    .get(url)
    //    .header(
    //        "Authorization",
    //        format!("Bearer {}", token.access_token().secret()),
    //    )
    //    .send()
    //    .await?
    //    .text()
    //    .await?;

    //println!("\n\nresp {:?}", resp);

    Ok(())
}

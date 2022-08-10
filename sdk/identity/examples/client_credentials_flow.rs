use azure_identity::client_credentials_flow;
use url::Url;

use std::env;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client_id = env::var("CLIENT_ID").expect("Missing CLIENT_ID environment variable.");
    let client_secret =
        env::var("CLIENT_SECRET").expect("Missing CLIENT_SECRET environment variable.");
    let tenant_id = env::var("TENANT_ID").expect("Missing TENANT_ID environment variable.");
    let subscription_id =
        env::var("SUBSCRIPTION_ID").expect("Missing SUBSCRIPTION_ID environment variable.");

    let http_client = azure_core::new_http_client();
    // This will give you the final token to use in authorization.
    let token = client_credentials_flow::perform(
        http_client.clone(),
        &client_id,
        &client_secret,
        &["https://management.azure.com/"],
        &tenant_id,
    )
    .await?;
    println!("Non interactive authorization == {:?}", token);

    // Let's enumerate the Azure SQL Databases instances
    // in the subscription. Note: this way of calling the REST API
    // will be different (and easier) using other Azure Rust SDK
    // crates, this is just an example.
    let url = Url::parse(&format!(
            "https://management.azure.com/subscriptions/{}/providers/Microsoft.Sql/servers?api-version=2015-05-01-preview",
            subscription_id
        ))?;

    let resp = reqwest::Client::new()
        .get(url)
        .header(
            "Authorization",
            format!("Bearer {}", token.access_token().secret()),
        )
        .send()
        .await?
        .text()
        .await?;

    println!("\n\nresp {:?}", resp);
    Ok(())
}

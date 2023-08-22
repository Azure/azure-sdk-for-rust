use azure_identity::{authority_hosts, federated_credentials_flow};
use url::Url;

use std::env;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client_id = env::var("CLIENT_ID").expect("Missing CLIENT_ID environment variable.");
    let token = env::var("FEDERATED_TOKEN").expect("Missing FEDERATED_TOKEN environment variable.");
    let tenant_id = env::var("TENANT_ID").expect("Missing TENANT_ID environment variable.");

    let vault_name = std::env::args()
        .nth(1)
        .expect("please specify the vault name as first command line parameter");

    let http_client = azure_core::new_http_client();
    // This will give you the final token to use in authorization.
    let token = federated_credentials_flow::perform(
        http_client.clone(),
        &client_id,
        &token,
        &["https://vault.azure.net/.default"],
        &tenant_id,
        authority_hosts::AZURE_PUBLIC_CLOUD,
    )
    .await
    .expect("federated_credentials_flow failed");
    println!("Non interactive authorization == {token:?}");

    let url = Url::parse(&format!(
        "https://{vault_name}.vault.azure.net/secrets?api-version=7.4"
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

    println!("\n\nresp {resp:?}");
    Ok(())
}

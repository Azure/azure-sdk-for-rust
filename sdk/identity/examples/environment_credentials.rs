use azure_core::auth::TokenCredential;
use azure_identity::*;
use std::{env::var, error::Error};
use url::Url;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let sub_id = var("AZURE_SUBSCRIPTION_ID")?;
    let creds = EnvironmentCredential::default();
    let res = creds.get_token("https://management.azure.com/").await?;
    eprintln!("Azure cli response == {res:?}");
    // Let's enumerate the Azure storage accounts
    // in the subscription. Note: this way of calling the REST API
    // will be different (and easier) using other Azure Rust SDK
    // crates, this is just an example.
    let url = Url::parse(&format!(
                 "https://management.azure.com/subscriptions/{sub_id}/providers/Microsoft.Storage/storageAccounts?api-version=2019-06-01"
             ))?;

    let resp = reqwest::Client::new()
        .get(url)
        .header("Authorization", format!("Bearer {}", res.token.secret()))
        .send()
        .await?
        .text()
        .await?;

    println!("{resp}");
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let subscription_id =
        std::env::var("AZURE_SUBSCRIPTION_ID").expect("AZURE_SUBSCRIPTION_ID required");

    let credential = azure_identity::new_credential();

    // Let's enumerate the Azure storage accounts in the subscription using the REST API directly.
    // This is just an example. It is easier to use the Azure SDK for Rust crates.
    let url = url::Url::parse(&format!("https://management.azure.com/subscriptions/{subscription_id}/providers/Microsoft.Storage/storageAccounts?api-version=2019-06-01"))?;

    let access_token = credential
        .get_token(&["https://management.azure.com/.default"])
        .await?;

    let response = reqwest::Client::new()
        .get(url)
        .header(
            "Authorization",
            format!("Bearer {}", access_token.token.secret()),
        )
        .send()
        .await?
        .text()
        .await?;

    println!("{response}");
    Ok(())
}

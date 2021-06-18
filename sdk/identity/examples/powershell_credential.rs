use azure_identity::token_credentials::*;
use std::error::Error;
use url::Url;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let credential = AzurePowerShellCredential::new(None);
    let res = credential
        .get_token("https://graph.microsoft.com/.default")
        .await?;

    println!("Azure PowerShell response == {:?}", res);

    // Let's look at some information from the Microsoft Graph as an example
    // The following request will return the authenticated user's profile.

    let resp = reqwest::Client::new()
        .get(Url::parse("https://graph.microsoft.com/v1.0/me")?)
        .header("Authorization", format!("Bearer {}", res.token.secret()))
        .send()
        .await?
        .text()
        .await?;

    println!("Microsoft Graph Response: {}", resp);

    Ok(())
}

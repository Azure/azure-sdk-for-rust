use azure_core::new_http_client;
use azure_identity::{authorization_code_flow, development::naive_redirect_server};
use oauth2::{ClientId, ClientSecret, TokenResponse};
use std::{env::var, error::Error};
use url::Url;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client_id =
        ClientId::new(var("CLIENT_ID").expect("Missing CLIENT_ID environment variable."));
    let client_secret = ClientSecret::new(
        var("CLIENT_SECRET").expect("Missing CLIENT_SECRET environment variable."),
    );
    let tenant_id = var("TENANT_ID").expect("Missing TENANT_ID environment variable.");
    let subscription_id =
        var("SUBSCRIPTION_ID").expect("Missing SUBSCRIPTION_ID environment variable.");

    // Create URL to browse for initial authorization
    let code_flow = authorization_code_flow::start(
        client_id,
        Some(client_secret),
        &tenant_id,
        Url::parse("http://localhost:3003/redirect").unwrap(),
        "https://management.azure.com/user_impersonation",
    );

    println!("c == {code_flow:?}");
    println!("\nbrowse this url:\n{}", code_flow.authorize_url);

    // Start a naive server to receive the redirect with the token. This naive server is blocking
    // so you should use something better.
    let code = naive_redirect_server(&code_flow, 3003).unwrap();

    println!("code received: {code:?}");

    // Exchange the token with one that can be used for authorization
    let token = code_flow.exchange(new_http_client(), code).await.unwrap();

    println!("token received: {token:?}");

    // Let's enumerate the Azure SQL Databases instances
    // in the subscription. Note: this way of calling the REST API
    // will be different (and easier) using other Azure Rust SDK
    // crates, this is just an example.
    let url = Url::parse(&format!(
            "https://management.azure.com/subscriptions/{subscription_id}/providers/Microsoft.Sql/servers?api-version=2015-05-01-preview"
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

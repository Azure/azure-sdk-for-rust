use azure_sdk_auth_aad::*;
use oauth2::{ClientId, ClientSecret};
use std::env;
use std::error::Error;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client_id =
        ClientId::new(env::var("CLIENT_ID").expect("Missing CLIENT_ID environment variable."));
    let client_secret = ClientSecret::new(
        env::var("CLIENT_SECRET").expect("Missing CLIENT_SECRET environment variable."),
    );
    let tenant_id = env::var("TENANT_ID").expect("Missing TENANT_ID environment variable.");

    let storage_account_name = std::env::args()
        .nth(1)
        .expect("please specify the storage account name as first command line parameter");
    let container_name = std::env::args()
        .nth(2)
        .expect("please specify the container name as second command line parameter");

    let client = Arc::new(reqwest::Client::new());

    let token = authorize_client_credentials_flow(
        client,
        &client_id,
        &client_secret,
        &format!(
            "https://{}.blob.core.windows.net/.default",
            storage_account_name
        ),
        &tenant_id,
    )
    .await?;

    println!("token received: {:?}", token);
    println!("token secret: {}", token.access_token().secret());

    let dt = chrono::Utc::now();
    let time = format!("{}", dt.format("%a, %d %h %Y %T GMT"));
    println!("x-ms-date ==> {}", time);

    let resp = reqwest::Client::new()
        .get(&format!(
            "https://{}.blob.core.windows.net/{}?restype=container&comp=list",
            storage_account_name, container_name
        ))
        .header(
            "Authorization",
            format!("Bearer {}", token.access_token().secret()),
        )
        .header("x-ms-version", "2019-07-07")
        .header("x-ms-date", time)
        .send()
        .await?
        .text()
        .await?;

    println!("\n\nresp {:?}", resp);

    Ok(())
}

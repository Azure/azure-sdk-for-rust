use azure_core::date;
use azure_identity::*;
use oauth2::{ClientId, ClientSecret, TokenResponse};
use std::env;
use std::error::Error;
use time::OffsetDateTime;
use url::Url;

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

    // Create URL to browse for initial authorization
    let c = authorization_code_flow::start(
        client_id,
        Some(client_secret),
        &tenant_id,
        Url::parse("http://localhost:3003/redirect").unwrap(),
        &format!(
            "https://{}.blob.core.windows.net/user_impersonation",
            storage_account_name
        ),
    );

    println!("c == {:?}", c);
    println!("\nbrowse this url:\n{}", c.authorize_url);

    // Start a naive redirect server to receive the redirect with the token.
    // This naive server is blocking so you should use something better.
    let code = development::naive_redirect_server(&c, 3003).unwrap();

    println!("code received: {:?}", code);

    // Exchange the token with one that can be used for authorization
    let token = c
        .exchange(azure_core::new_http_client(), code)
        .await
        .unwrap();

    println!("token received: {:?}", token);

    println!("token secret: {}", token.access_token().secret());

    let dt = OffsetDateTime::now_utc();
    let time = date::to_rfc1123(&dt);
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

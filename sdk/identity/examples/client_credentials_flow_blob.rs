use azure_core::date;
use azure_identity::client_credentials_flow;
use time::OffsetDateTime;

use std::env;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client_id = env::var("CLIENT_ID").expect("Missing CLIENT_ID environment variable.");
    let client_secret =
        env::var("CLIENT_SECRET").expect("Missing CLIENT_SECRET environment variable.");
    let tenant_id = env::var("TENANT_ID").expect("Missing TENANT_ID environment variable.");

    let storage_account_name = std::env::args()
        .nth(1)
        .expect("please specify the storage account name as first command line parameter");
    let container_name = std::env::args()
        .nth(2)
        .expect("please specify the container name as second command line parameter");

    let http_client = azure_core::new_http_client();

    let token = client_credentials_flow::perform(
        http_client.clone(),
        &client_id,
        &client_secret,
        &[&format!(
            "https://{}.blob.core.windows.net/.default",
            storage_account_name
        )],
        &tenant_id,
    )
    .await?;

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

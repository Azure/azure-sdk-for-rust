use azure_core::HttpClient;
use azure_cosmos::prelude::*;
use hyper_rustls::HttpsConnector;
use std::error::Error;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    // First we retrieve the account name and master key from environment variables.
    // We expect master keys (ie, not resource constrained)
    let master_key =
        std::env::var("COSMOS_MASTER_KEY").expect("Set env variable COSMOS_MASTER_KEY first!");
    let account = std::env::var("COSMOS_ACCOUNT").expect("Set env variable COSMOS_ACCOUNT first!");

    let database_name = std::env::args()
        .nth(1)
        .expect("Please provide the database name as first parameter");

    // use reqwest
    let authorization_token = AuthorizationToken::new_master(&master_key)?;

    let http_client: Box<dyn HttpClient> = Box::new(reqwest::Client::new());
    let http_client = Arc::new(http_client);

    let client = azure_cosmos::client_builder::new(&account, authorization_token)?
        .with_http_client(http_client)
        .build();
    let database_client = client.into_database_client(&database_name);

    let response = database_client.get_database().execute().await?;
    println!("from reqwest == {:?}", response);

    // use hyper
    let authorization_token = AuthorizationToken::new_master(&master_key)?;

    let http_client: Box<dyn HttpClient> =
        Box::new(hyper::Client::builder().build(HttpsConnector::new()));
    let http_client = Arc::new(http_client);

    let client = azure_cosmos::client_builder::new(&account, authorization_token)?
        .with_http_client(http_client)
        .build();
    let database_client = client.into_database_client(&database_name);

    let response = database_client.get_database().execute().await?;
    println!("from hyper == {:?}", response);

    Ok(())
}

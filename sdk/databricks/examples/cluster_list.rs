use azure_core::Body;
use azure_databricks::client::*;
use std::error::Error;
use http;
use bytes::Bytes;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>>{
    // to connect to databricks api, we need at least, the host and personal access token

    // get host_name from environment variables (includes the https://)
    let host_name: String = std::env::var("DATABRICKS_INSTANCE").expect("Set env variable DATABRICKS_INSTANCE first!");

    // get personal access token to databricks workspace
    let token: String = std::env::var("DATABRICKS_TOKEN").expect("Set env variable DATABRICKS_TOKEN first!");

    let creds = DatabricksCredentials::BearerToken(token);

    let client = DatabricksClient::new(creds, &host_name, None);

    let request = client.prepare_request(&http::Method::GET, Body::Bytes(Bytes::from("")));
    println!("{:#?}", request);

    Ok(())
}

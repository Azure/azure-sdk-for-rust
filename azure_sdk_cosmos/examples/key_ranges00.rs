use azure_sdk_cosmos::prelude::*;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let database = std::env::args()
        .nth(1)
        .expect("please specify database name as first command line parameter");
    let collection = std::env::args()
        .nth(2)
        .expect("please specify collection name as second command line parameter");

    let master_key =
        std::env::var("COSMOS_MASTER_KEY").expect("Set env variable COSMOS_MASTER_KEY first!");
    let account = std::env::var("COSMOS_ACCOUNT").expect("Set env variable COSMOS_ACCOUNT first!");

    let authorization_token = AuthorizationToken::new_master(&master_key)?;
    let client = ClientBuilder::new(account, authorization_token.clone())?;
    let client = client.with_database(&database);
    let client = client.with_collection(&collection);

    let resp = client.get_partition_key_ranges().execute().await?;
    println!("resp == {:#?}", resp);

    Ok(())
}

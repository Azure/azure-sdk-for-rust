use azure_data_cosmos::prelude::*;

#[tokio::main]
async fn main() -> azure_core::Result<()> {
    let database = std::env::args()
        .nth(1)
        .expect("please specify database name as first command line parameter");
    let collection = std::env::args()
        .nth(2)
        .expect("please specify collection name as second command line parameter");

    let master_key =
        std::env::var("COSMOS_MASTER_KEY").expect("Set env variable COSMOS_MASTER_KEY first!");
    let account = std::env::var("COSMOS_ACCOUNT").expect("Set env variable COSMOS_ACCOUNT first!");

    let authorization_token = AuthorizationToken::primary_from_base64(&master_key)?;

    let client = CosmosClient::new(
        account.clone(),
        authorization_token,
        CosmosOptions::default(),
    );

    let client = client
        .database_client(database)
        .collection_client(collection);

    let resp = client.get_partition_key_ranges().into_future().await?;
    println!("resp == {:#?}", resp);

    Ok(())
}

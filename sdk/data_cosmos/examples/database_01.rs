use azure_data_cosmos::prelude::*;
use clap::Parser;
use futures::stream::StreamExt;

#[derive(Debug, Parser)]
struct Args {
    /// Cosmos primary key name
    #[clap(env = "COSMOS_PRIMARY_KEY")]
    primary_key: String,
    /// The cosmos account your're using
    #[clap(env = "COSMOS_ACCOUNT")]
    account: String,
}

#[tokio::main]
async fn main() -> azure_core::Result<()> {
    // First we retrieve the account name and access key from environment variables.
    // We expect access keys (ie, not resource constrained)
    let args = Args::parse();
    let authorization_token = AuthorizationToken::primary_from_base64(&args.primary_key)?;

    let client = CosmosClient::new(args.account, authorization_token, CosmosOptions::default());

    let database = client.database_client("pollo");
    println!("database_name == {}", database.database_name());

    let collections = database
        .list_collections()
        .into_stream()
        .next()
        .await
        .unwrap()?;
    println!("collections == {:#?}", collections);

    let collection = database
        .collection_client("cnt")
        .get_collection()
        .into_future()
        .await?;
    println!("collection == {:#?}", collection);

    Ok(())
}

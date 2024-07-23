// REVIEW: Don't merge this file. It's just for adhoc testing purposes.

use azure_core::auth::Secret;
use azure_data_cosmos::{CosmosClient, CosmosClientMethods, DatabaseClientMethods};
use clap::Parser;

#[derive(Parser)]
pub struct Args {
    /// The CosmosDB endpoint to connect to.
    endpoint: String,

    /// The database to fetch information for.
    database: String,

    /// An authentication key to use when connecting to the Cosmos DB account. If omitted, the connection will use Entra ID.
    #[clap(long)]
    key: Option<String>,
}

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let client = if let Some(key) = args.key {
        CosmosClient::with_shared_key(&args.endpoint, key, None).unwrap()
    } else {
        let cred = azure_identity::create_default_credential().unwrap();
        CosmosClient::new(&args.endpoint, cred, None).unwrap()
    };

    let db_client = client.database(&args.database);
    let response = db_client.read(None).await?.read_body().await?;
    println!("{:?}", response);
    Ok(())
}

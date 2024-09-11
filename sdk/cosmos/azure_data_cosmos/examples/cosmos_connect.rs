use azure_data_cosmos::{clients::DatabaseClientMethods, CosmosClient, CosmosClientMethods};
use clap::Parser;

/// A simple example to show connecting to a Cosmos Account and retrieving the properties of a database.
#[derive(Parser)]
pub struct Args {
    /// The Cosmos endpoint to connect to.
    endpoint: String,

    /// The database to fetch information for.
    database: String,

    /// An authentication key to use when connecting to the Cosmos DB account. If omitted, the connection will use Entra ID.
    #[clap(long)]
    #[cfg(feature = "key_auth")]
    key: Option<String>,
}

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let client = create_client(&args);

    let db_client = client.database_client(&args.database);
    let response = db_client.read(None).await?.deserialize_body().await?;
    println!("{:?}", response);
    Ok(())
}

#[cfg(feature = "key_auth")]
fn create_client(args: &Args) -> CosmosClient {
    if let Some(key) = args.key.as_ref() {
        CosmosClient::with_key(&args.endpoint, key.clone(), None).unwrap()
    } else {
        let cred = azure_identity::create_default_credential().unwrap();
        CosmosClient::new(&args.endpoint, cred, None).unwrap()
    }
}

#[cfg(not(feature = "key_auth"))]
fn create_client(args: &Args) -> CosmosClient {
    let cred = azure_identity::create_default_credential().unwrap();
    CosmosClient::new(&args.endpoint, cred, None).unwrap()
}

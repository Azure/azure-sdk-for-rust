use azure_core::{RetryOptions, TransportOptions};
use clap::Parser;

use azure_data_cosmos::prelude::*;

#[derive(Debug, Parser)]
struct Args {
    /// Cosmos primary key name
    #[clap(env = "COSMOS_PRIMARY_KEY")]
    primary_key: String,
    /// The cosmos account your're using
    #[clap(env = "COSMOS_ACCOUNT")]
    account: String,
    /// The name of the database
    database_name: String,
}

#[tokio::main]
async fn main() -> azure_core::Result<()> {
    let args = Args::parse();
    let authorization_token = AuthorizationToken::primary_from_base64(&args.primary_key)?;

    let _database = CosmosClient::builder(args.account, authorization_token)
        .retry(RetryOptions::default())
        .transport(TransportOptions::default())
        .build()
        .database_client(args.database_name);

    // ... the database client is now ready to be used!

    Ok(())
}

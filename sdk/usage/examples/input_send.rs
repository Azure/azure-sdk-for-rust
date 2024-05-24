use azure_usage::prelude::*;
use clap::Parser;
use uuid::Uuid;

#[derive(Debug, Parser)]
struct Args {
    /// Cosmos primary key name
    #[clap(env = "USAGE_ACCOUNT_NAME")]
    account_name: String,
    /// The cosmos account your're using
    #[clap(env = "USAGE_ACCOUNT_LOCATION")]
    account_location: String,
}

#[tokio::main]
async fn main() -> azure_core::Result<()> {
    // Let's get the usage account name and location from env variables.
    let args = Args::parse();

    let client = UsageClient::new(args.account_name, args.account_location);

    Ok(())
}

use azure_usage::prelude::*;
use clap::Parser;

#[derive(Debug, Parser)]
struct Args {
    /// Cloud suffix
    #[clap(env = "USAGE_ACCOUNT_CLOUD")]
    cloud_suffix: String,
    /// Usage account name
    #[clap(env = "USAGE_ACCOUNT_NAME")]
    account_name: String,
    /// Usage account location
    #[clap(env = "USAGE_ACCOUNT_LOCATION")]
    account_location: String,
}

#[tokio::main]
async fn main() -> azure_core::Result<()> {
    // Let's get the usage account name and location from env variables.
    let args = Args::parse();

    let client = UsageClient::new(args.cloud_suffix, args.account_name, args.account_location);

    Ok(())
}

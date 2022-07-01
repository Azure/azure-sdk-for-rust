use azure_core::headers::{HeaderName, HeaderValue, Headers};
use azure_core::prelude::*;
use azure_core::CustomHeaders;
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
    // First we retrieve the account name and access key from environment variables.
    // We expect access keys (ie, not resource constrained)
    let args = Args::parse();
    let authorization_token = AuthorizationToken::primary_from_base64(&args.primary_key)?;

    let client = CosmosClient::new(args.account.clone(), authorization_token);
    let database = client.database_client(args.database_name.clone());

    let mut context = Context::new();

    // Next we create a CustomHeaders type and insert it into the context allowing us to insert custom headers.
    let custom_headers: CustomHeaders = {
        let mut custom_headers = std::collections::HashMap::<HeaderName, HeaderValue>::new();
        custom_headers.insert("MyCoolHeader".into(), "CORS maybe?".into());
        let hs: Headers = custom_headers.into();
        hs.into()
    };

    context.insert(custom_headers);

    let response = database
        .get_database()
        .context(context)
        .into_future()
        .await?;
    println!("response == {:?}", response);

    Ok(())
}

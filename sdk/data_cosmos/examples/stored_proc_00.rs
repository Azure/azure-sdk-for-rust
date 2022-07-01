/// This sample showcases execution of stored procedure
/// Create stored procedure called test_proc, like so:
/// function f(personToGreet) {
///     var context = getContext();
///     var response = context.getResponse();
///     response.setBody("Hello, " + personToGreet);
/// }
use azure_data_cosmos::prelude::*;
use clap::Parser;

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
    /// The name of the collection
    collection_name: String,
}

#[tokio::main]
async fn main() -> azure_core::Result<()> {
    let args = Args::parse();
    let authorization_token = AuthorizationToken::primary_from_base64(&args.primary_key)?;

    let client = CosmosClient::new(args.account, authorization_token);

    let ret = client
        .database_client(args.database_name)
        .collection_client(args.collection_name)
        .stored_procedure_client("test_proc")
        .execute_stored_procedure::<serde_json::Value>()
        .parameters(["Robert"])
        .into_future()
        .await?;

    println!("Response object:\n{:#?}", ret);
    println!("Response as JSON:\n{}", ret.payload);

    Ok(())
}

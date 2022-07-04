/// This sample showcases execution of stored procedure
/// Create stored procedure called test_proc, like so:
/// function f(personToGreet) {
///     var context = getContext();
///     var response = context.getResponse();
///     response.setBody("Hello, " + personToGreet);
/// }
use clap::Parser;

mod util;

#[derive(Debug, clap::Parser)]
struct Args {
    #[clap(flatten)]
    auth: util::Auth,
    /// The name of the database
    database_name: String,
    /// The name of the collection
    collection_name: String,
}

#[tokio::main]
async fn main() -> azure_core::Result<()> {
    let args = Args::parse();
    let client = args.auth.into_client()?;

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

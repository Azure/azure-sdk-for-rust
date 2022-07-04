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
    let client = args
        .auth
        .into_client()?
        .database_client(args.database_name)
        .collection_client(args.collection_name);

    let resp = client.get_partition_key_ranges().into_future().await?;
    println!("resp == {:#?}", resp);

    Ok(())
}

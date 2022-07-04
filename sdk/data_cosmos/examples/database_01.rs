use clap::Parser;
use futures::stream::StreamExt;

mod util;

#[tokio::main]
async fn main() -> azure_core::Result<()> {
    let client = util::Auth::parse().into_client()?;

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

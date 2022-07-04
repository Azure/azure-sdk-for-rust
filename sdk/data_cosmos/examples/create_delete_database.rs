use clap::Parser;
use futures::stream::StreamExt;

mod util;

#[derive(Debug, clap::Parser)]
struct Args {
    /// The name of the database
    database_name: String,
    #[clap(flatten)]
    auth: util::Auth,
}

#[tokio::main]
async fn main() -> azure_core::Result<()> {
    let args = Args::parse();
    let database_name = args.database_name;
    let client = args.auth.into_client()?;

    // The Cosmos' client exposes a lot of methods. This one lists the databases in the specified account.
    let mut list_databases_stream = client.list_databases().into_stream();
    while let Some(list_databases_response) = list_databases_stream.next().await {
        println!("list_databases_response = {:#?}", list_databases_response?);
    }
    drop(list_databases_stream);

    let db = client.create_database(&database_name).into_future().await?;
    println!("created database = {:#?}", db);

    // create collection!
    let database = client.database_client(database_name.clone());
    let create_collection_response = database
        .create_collection("panzadoro", "/id")
        .into_future()
        .await?;

    println!(
        "create_collection_response == {:#?}",
        create_collection_response
    );

    let db_collection = database.collection_client("panzadoro");

    let get_collection_response = db_collection.get_collection().into_future().await?;
    println!("get_collection_response == {:#?}", get_collection_response);

    let mut stream = database.list_collections().into_stream();
    while let Some(res) = stream.next().await {
        let res = res?;
        println!("res == {:#?}", res);
    }

    let delete_response = db_collection.delete_collection().into_future().await?;
    println!("collection deleted: {:#?}", delete_response);

    let resp = client
        .database_client(database_name)
        .delete_database()
        .into_future()
        .await?;
    println!("database deleted. resp == {:#?}", resp);

    Ok(())
}

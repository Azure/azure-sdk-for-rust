use clap::Parser;
use futures::stream::StreamExt;
mod util;

#[tokio::main]
async fn main() -> azure_core::Result<()> {
    let args = util::Auth::parse();
    let account = args.account().clone();
    let client = args.into_client()?;
    // The Cosmos' client exposes a lot of methods. This one lists the databases in the specified account.
    let databases = client
        .list_databases()
        .into_stream()
        .next()
        .await
        .unwrap()?;

    println!(
        "Account {} has {} database(s)",
        account,
        databases.databases.len()
    );

    // try get on the first database (if any)
    if let Some(db) = databases.databases.first() {
        println!("getting info of database {}", &db.id);
        let db = client
            .database_client(db.id.clone())
            .get_database()
            .into_future()
            .await?;
        println!("db {} found == {:?}", &db.database.id, &db);
    }

    // Each Cosmos' database contains one or more collections. We can enumerate them using the
    // list_collection method.

    for db in databases.databases {
        let database = client.database_client(db.id.clone());
        let collections = database
            .list_collections()
            .into_stream()
            .next()
            .await
            .unwrap()?;
        println!(
            "database {} has {} collection(s)",
            db.id,
            collections.collections.len()
        );

        for collection in collections.collections {
            println!("\tcollection {}", collection.id);

            let collection_response = database
                .collection_client(collection.id)
                .get_collection()
                .into_future()
                .await?;

            println!("\tcollection_response {:?}", collection_response);
        }
    }

    Ok(())
}

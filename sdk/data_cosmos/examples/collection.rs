#![feature(into_future)]
use azure_data_cosmos::prelude::*;
use futures::stream::StreamExt;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    // First we retrieve the account name and master key from environment variables.
    // We expect master keys (ie, not resource constrained)
    let master_key =
        std::env::var("COSMOS_MASTER_KEY").expect("Set env variable COSMOS_MASTER_KEY first!");
    let account = std::env::var("COSMOS_ACCOUNT").expect("Set env variable COSMOS_ACCOUNT first!");

    // This is how you construct an authorization token.
    // Remember to pick the correct token type.
    // Here we assume master.
    // Most methods return a ```Result<_, azure_data_cosmos::Error>```.
    // ```azure_data_cosmos::Error``` is an enum union of all the possible underlying
    // errors, plus Azure specific ones. For example if a REST call returns the
    // unexpected result (ie NotFound instead of Ok) we return an Err telling
    // you that.
    let authorization_token = AuthorizationToken::primary_from_base64(&master_key)?;

    // Once we have an authorization token you can create a client instance. You can change the
    // authorization token at later time if you need, for example, to escalate the privileges for a
    // single operation.
    // Here we are using reqwest but other clients are supported (check the documentation).
    let client = CosmosClient::new(
        account.clone(),
        authorization_token,
        CosmosOptions::default(),
    );

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
        let db = client.database_client(db.id.clone()).get_database().await?;
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
                .await?;

            println!("\tcollection_response {:?}", collection_response);
        }
    }

    Ok(())
}

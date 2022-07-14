use azure_data_cosmos::prelude::*;
use clap::Parser;
use futures::stream::StreamExt;

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

    // This is how you construct an authorization token.
    // Remember to pick the correct token type.
    // Here we assume master.
    // Most methods return a ```Result<_, azure_data_cosmos::Error>```.
    // ```azure_data_cosmos::Error``` is an enum union of all the possible underlying
    // errors, plus Azure specific ones. For example if a REST call returns the
    // unexpected result (ie NotFound instead of Ok) we return an Err telling
    // you that.
    let authorization_token =
        permission::AuthorizationToken::primary_from_base64(&args.primary_key)?;

    // Once we have an authorization token you can create a client instance. You can change the
    // authorization token at later time if you need, for example, to escalate the privileges for a
    // single operation.
    let client = CosmosClient::new(args.account, authorization_token, CosmosOptions::default());

    // The Cosmos' client exposes a lot of methods. This one lists the databases in the specified
    // account. Database do not implement Display but deref to &str so you can pass it to methods
    // both as struct or id.

    let mut list_databases_stream = client.list_databases().into_stream();
    while let Some(list_databases_response) = list_databases_stream.next().await {
        println!("list_databases_response = {:#?}", list_databases_response?);
    }
    drop(list_databases_stream);

    let db = client
        .create_database(&args.database_name)
        .into_future()
        .await?;
    println!("created database = {:#?}", db);

    // create collection!
    {
        let database = client.database_client(args.database_name.clone());
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
    }

    let resp = client
        .database_client(args.database_name)
        .delete_database()
        .into_future()
        .await?;
    println!("database deleted. resp == {:#?}", resp);

    Ok(())
}

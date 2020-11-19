use azure_cosmos::prelude::*;
use futures::stream::StreamExt;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // First we retrieve the account name and master key from environment variables.
    // We expect master keys (ie, not resource constrained)
    let master_key =
        std::env::var("COSMOS_MASTER_KEY").expect("Set env variable COSMOS_MASTER_KEY first!");
    let account = std::env::var("COSMOS_ACCOUNT").expect("Set env variable COSMOS_ACCOUNT first!");

    let database_name = std::env::args()
        .nth(1)
        .expect("please specify database name as first command line parameter");

    // This is how you construct an authorization token.
    // Remember to pick the correct token type.
    // Here we assume master.
    // Most methods return a ```Result<_, AzureError>```.
    // ```AzureError``` is an enum union of all the possible underlying
    // errors, plus Azure specific ones. For example if a REST call returns the
    // unexpected result (ie NotFound instead of Ok) we return an Err telling
    // you that.
    let authorization_token = AuthorizationToken::new_master(&master_key)?;

    // Once we have an authorization token you can create a client instance. You can change the
    // authorization token at later time if you need, for example, to escalate the privileges for a
    // single operation.
    let client = CosmosClient::new(account, authorization_token);

    // The Cosmos' client exposes a lot of methods. This one lists the databases in the specified
    // account. Database do not implement Display but deref to &str so you can pass it to methods
    // both as struct or id.

    let list_databases_response = client.list_databases().execute().await?;
    println!("list_databases_response = {:#?}", list_databases_response);

    let db = client
        .create_database()
        .with_database_name(&database_name)
        .execute()
        .await?;
    println!("created database = {:#?}", db);

    // create collection!
    {
        let db_client = client.clone().into_database_client(database_name.clone());

        let indexes = IncludedPathIndex {
            kind: KeyKind::Hash,
            data_type: DataType::String,
            precision: Some(3),
        };

        let ip = IncludedPath {
            path: "/*".to_owned(),
            indexes: Some(vec![indexes]),
        };

        let ip = IndexingPolicy {
            automatic: true,
            indexing_mode: IndexingMode::Consistent,
            included_paths: vec![ip],
            excluded_paths: vec![],
        };

        let create_collection_response = db_client
            .create_collection()
            .with_collection_name(&"panzadoro")
            .with_partition_key(&("/id".into()))
            .with_offer(Offer::Throughput(400))
            .with_indexing_policy(&ip)
            .execute()
            .await?;

        println!(
            "create_collection_response == {:#?}",
            create_collection_response
        );

        let db_collection = db_client.clone().into_collection_client("panzadoro");

        let get_collection_response = db_collection.get_collection().execute().await?;
        println!("get_collection_response == {:#?}", get_collection_response);

        let stream = db_client.list_collections();
        let mut stream = Box::pin(stream.stream());
        while let Some(res) = stream.next().await {
            let res = res?;
            println!("res == {:#?}", res);
        }

        let delete_response = db_collection.delete_collection().execute().await?;
        println!("collection deleted: {:#?}", delete_response);
    }

    let resp = client
        .into_database_client(database_name)
        .delete_database()
        .execute()
        .await?;
    println!("database deleted. resp == {:#?}", resp);

    Ok(())
}

use serde::{Deserialize, Serialize};
// Using the prelude module of the Cosmos crate makes easier to use the Rust Azure SDK for Cosmos.

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
    /// The name of the collection
    collection_name: String,
}

// This is the stuct we want to use in our sample.
// Make sure to have a collection with partition key "a_number" for this example to
// work (you can create with this SDK too, check the examples folder for that task).
#[derive(Serialize, Deserialize, Debug, Clone)]
struct MySampleStruct {
    id: String,
    a_string: String,
    a_number: u64,
    a_timestamp: i64,
}

// Here we mark "a_number" as partition key.
impl azure_data_cosmos::CosmosEntity for MySampleStruct {
    type Entity = u64;

    fn partition_key(&self) -> Self::Entity {
        self.a_number
    }
}

// This code will perform these tasks:
// 1. Create 10 documents in the collection.
// 2. Stream all the documents.
// 3. Query the documents.
// 4. Delete the documents returned by task 4.
// 5. Check the remaining documents.
#[tokio::main]
async fn main() -> azure_core::Result<()> {
    // Let's get Cosmos account and access key from env variables.
    // This helps automated testing.
    let args = Args::parse();
    // First, we create an authorization token. There are two types of tokens, master and resource
    // constrained. This SDK supports both.
    // Please check the Azure documentation for details or the examples folder
    // on how to create and use token-based permissions.
    let authorization_token = AuthorizationToken::primary_from_base64(&args.primary_key)?;

    // Next we will create a Cosmos client.
    let client = CosmosClient::new(
        args.account.clone(),
        authorization_token,
        CosmosOptions::default(),
    );

    // We know the database so we can obtain a database client.
    let database = client.database_client(args.database_name);
    // We know the collection so we can obtain a collection client.
    let collection = database.collection_client(args.collection_name);

    // TASK 1 - Insert 10 documents
    println!("Inserting 10 documents...");
    let mut session_token = None;
    for i in 0..10 {
        // define the document.
        let document_to_insert = MySampleStruct {
            id: format!("unique_id{}", i),
            a_string: "Something here".into(),
            a_number: i * 100, // this is the partition key
            a_timestamp: chrono::Utc::now().timestamp(),
        };

        // insert it and store the returned session token for later use!
        session_token = Some(
            collection
                .create_document(document_to_insert.clone())
                .is_upsert(true)
                .into_future()
                .await?
                .session_token, // get only the session token, if everything else was ok!
        );
    }
    // wow that was easy and fast, wasnt'it? :)
    println!("Done!");

    let session_token = ConsistencyLevel::Session(session_token.unwrap());

    // TASK 2
    {
        println!("\nStreaming documents");
        // we limit the number of documents to 3 for each batch as a demonstration. In practice
        // you will use a more sensible number (or accept the Azure default).
        let stream = collection
            .list_documents()
            .consistency_level(session_token.clone())
            .max_item_count(3);
        let mut stream = stream.into_stream::<MySampleStruct>();
        // TODO: As soon as the streaming functionality is stabilized
        // in Rust we can substitute this while let Some... into
        // for each (or whatever the Rust team picks).
        while let Some(res) = stream.next().await {
            let res = res?;
            println!("Received {} documents in one batch!", res.documents.len());
            res.documents.iter().for_each(|doc| println!("{:#?}", doc));
        }
    }

    // TASK 3
    println!("\nQuerying documents");
    let query_documents_response = collection
        .query_documents("SELECT * FROM A WHERE A.a_number < 600")
        .query_cross_partition(true) // this will perform a cross partition query! notice how simple it is!
        .consistency_level(session_token)
        .into_stream::<MySampleStruct>() // there are other ways to construct a query, this is the simplest.
        .next()
        .await
        .unwrap()?
        .into_documents() // queries can return Documents or Raw json (ie without etag, _rid, etc...). Since our query return docs we convert with this function.
        .unwrap(); // we know in advance that the conversion to Document will not fail since we SELECT'ed * FROM table

    println!(
        "Received {} documents!",
        query_documents_response.results.len()
    );

    query_documents_response
        .results
        .iter()
        .for_each(|document| {
            println!("number ==> {}", document.result.a_number);
        });

    // TASK 4
    let session_token = ConsistencyLevel::Session(query_documents_response.session_token);
    for ref document in query_documents_response.results {
        // From our query above we are sure to receive a Document.
        println!(
            "deleting id == {}, a_number == {}.",
            document.result.id, document.result.a_number
        );

        // to spice the delete a little we use optimistic concurreny
        collection
            .document_client(document.result.id.clone(), &document.result.a_number)?
            .delete_document()
            .consistency_level(session_token.clone())
            .if_match_condition(&document.document_attributes)
            .into_future()
            .await?;
    }

    // TASK 5
    // Now the list documents should return 4 documents!
    let list_documents_response = collection
        .list_documents()
        .consistency_level(session_token)
        .into_stream::<serde_json::Value>() // you can use this if you don't know/care about the return type!
        .next()
        .await
        .unwrap()?;
    assert_eq!(list_documents_response.documents.len(), 4);

    Ok(())
}

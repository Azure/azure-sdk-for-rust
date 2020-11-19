#[macro_use]
extern crate serde_derive;
// Using the prelude module of the Cosmos crate makes easier to use the Rust Azure SDK for Cosmos
// DB.
use azure_core::prelude::*;
use azure_cosmos::prelude::*;
use futures::stream::StreamExt;
use std::borrow::Cow;
use std::error::Error;

// This is the stuct we want to use in our sample.
// Make sure to have a collection with partition key "a_number" for this example to
// work (you can create with this SDK too, check the examples folder for that task).
#[derive(Serialize, Deserialize, Debug)]
struct MySampleStruct<'a> {
    id: Cow<'a, str>,
    a_string: Cow<'a, str>,
    a_number: u64,
    a_timestamp: i64,
}

// This code will perform these tasks:
// 1. Create 10 documents in the collection.
// 2. Stream all the documents.
// 3. Query the documents.
// 4. Delete the documents returned by task 4.
// 5. Check the remaining documents.
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Let's get Cosmos account and master key from env variables.
    // This helps automated testing.
    let master_key =
        std::env::var("COSMOS_MASTER_KEY").expect("Set env variable COSMOS_MASTER_KEY first!");
    let account = std::env::var("COSMOS_ACCOUNT").expect("Set env variable COSMOS_ACCOUNT first!");

    let database_name = std::env::args()
        .nth(1)
        .expect("please specify the database name as first command line parameter");
    let collection_name = std::env::args()
        .nth(2)
        .expect("please specify the collection name as first command line parameter");

    // First, we create an authorization token. There are two types of tokens, master and resource
    // constrained. This SDK supports both.
    // Please check the Azure documentation for details or the examples folder
    // on how to create and use token-based permissions.
    let authorization_token = AuthorizationToken::new_master(&master_key)?;

    // Next we will create a Cosmos client.
    let client = CosmosClient::new(account, authorization_token);
    // We know the database so we can obtain a database client.
    let database_client = client.into_database_client(database_name);
    // We know the collection so we can obtain a collection client.
    let collection_client = database_client.into_collection_client(collection_name);

    // TASK 1 - Insert 10 documents
    println!("Inserting 10 documents...");
    let mut session_token = None;
    for i in 0..10 {
        // define the document.
        let document_to_insert = Document::new(MySampleStruct {
            id: Cow::Owned(format!("unique_id{}", i)),
            a_string: Cow::Borrowed("Something here"),
            a_number: i * 100, // this is the partition key
            a_timestamp: chrono::Utc::now().timestamp(),
        });

        // insert it and store the returned session token for later use!
        session_token = Some(
            collection_client
                .create_document()
                .with_partition_keys(
                    PartitionKeys::new().push(&document_to_insert.document.a_number)?,
                )
                .with_is_upsert(true) // this option will overwrite a preexisting document (if any)
                .execute_with_document(&document_to_insert)
                .await?
                .session_token, // get only the session token, if everything else was ok!
        );
    }
    // wow that was easy and fast, wasnt'it? :)
    println!("Done!");

    let session_token = ConsistencyLevel::from(session_token.unwrap());

    // TASK 2
    {
        println!("\nStreaming documents");
        // we limit the number of documents to 3 for each batch as a demonstration. In practice
        // you will use a more sensible number (or accept the Azure default).
        let stream = collection_client
            .list_documents()
            .with_consistency_level(session_token.clone())
            .with_max_item_count(3);
        let mut stream = Box::pin(stream.stream::<MySampleStruct>());
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
    let query_documents_response = collection_client
        .query_documents()
        .with_query(&("SELECT * FROM A WHERE A.a_number < 600".into())) // there are other ways to construct a query, this is the simplest.
        .with_query_cross_partition(true) // this will perform a cross partition query! notice how simple it is!
        .with_consistency_level(session_token)
        .execute::<MySampleStruct>() // This will make sure the result is our custom struct!
        .await?
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
    let session_token = ConsistencyLevel::from(query_documents_response.session_token.clone());
    for ref document in query_documents_response.results {
        // From our query above we are sure to receive a Document.
        println!(
            "deleting id == {}, a_number == {}.",
            document.result.id, document.result.a_number
        );

        // to spice the delete a little we use optimistic concurreny
        collection_client
            .clone()
            .into_document_client(
                document.result.id.clone().into_owned(),
                document.result.a_number.into(),
            )
            .delete_document()
            .with_consistency_level(session_token.clone())
            .with_if_match_condition((&document.document_attributes).into())
            .execute()
            .await?;
    }

    // TASK 5
    // Now the list documents should return 4 documents!
    let list_documents_response = collection_client
        .list_documents()
        .with_consistency_level(session_token)
        .execute::<serde_json::Value>() // you can use this if you don't know/care about the return type!
        .await?;
    assert_eq!(list_documents_response.documents.len(), 4);

    Ok(())
}

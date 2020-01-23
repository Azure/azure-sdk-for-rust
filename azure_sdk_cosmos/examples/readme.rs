#[macro_use]
extern crate serde_derive;
// Using the prelude module of the Cosmos crate makes easier to use the Rust Azure SDK for Cosmos
// DB.
use azure_sdk_core::prelude::*;
use azure_sdk_cosmos::prelude::*;
use futures::stream::StreamExt;
use std::borrow::Cow;
use std::error::Error;

// This is the stuct we want to use in our sample.
// Make sure to have a collection with partition key "a_number" for this example to
// work (you can create with this SDK too, check the examples folder for that task).
#[derive(Serialize, Deserialize, Debug)]
struct MySampleStruct<'a> {
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
    let client = ClientBuilder::new(account, authorization_token.clone())?;
    // We know the database so we can obtain a database client.
    let database_client = client.with_database(&database_name);
    // We know the collection so we can obtain a collection client.
    let collection_client = database_client.with_collection(&collection_name);

    // TASK 1 - Insert 10 documents
    println!("Inserting 10 documents...");
    for i in 0..10 {
        // define the document.
        let document_to_insert = Document::new(
            format!("unique_id{}", i), // this is the primary key, AKA "/id".
            MySampleStruct {
                a_string: Cow::Borrowed("Something here"),
                a_number: i * 100, // this is the partition key
                a_timestamp: chrono::Utc::now().timestamp(),
            },
        );

        // insert it!
        collection_client
            .create_document()
            .with_document(&document_to_insert)
            .with_partition_keys(PartitionKeys::new().push(&document_to_insert.document.a_number)?)
            .with_is_upsert(true) // this option will overwrite a preexisting document (if any)
            .execute()
            .await?;
    }
    // wow that was easy and fast, wasnt'it? :)
    println!("Done!");

    // TASK 2
    println!("\nStreaming documents");
    // we limit the number of documents to 3 for each batch as a demonstration. In practice
    // you will use a more sensible number (or accept the Azure default).
    let stream = collection_client.list_documents().with_max_item_count(3);
    let mut stream = Box::pin(stream.stream::<MySampleStruct>());
    // TODO: As soon as the streaming functionality is stabilized
    // in Rust we can substitute this while let Some... into
    // for each (or whatever the Rust team picks).
    while let Some(res) = stream.next().await {
        let res = res?;
        println!("Received {} documents in one batch!", res.documents.len());
        res.documents.iter().for_each(|doc| println!("{:#?}", doc));
    }

    // TASK 3
    println!("\nQuerying documents");
    let query_documents_response = collection_client
        .query_documents()
        .with_query(&("SELECT * FROM A WHERE A.a_number < 600".into()))
        .with_query_cross_partition(true) // this will perform a cross partition query! notice how simple it is!
        .execute::<MySampleStruct>()
        .await?;

    println!(
        "Received {} documents!",
        query_documents_response.results.len()
    );

    query_documents_response
        .results
        .iter()
        .for_each(|document| println!("number ==> {}", document.result.a_number));

    // TASK 4
    for ref document in query_documents_response.results {
        println!(
            "deleting id == {}, a_number == {}.",
            document.document_attributes.id, document.result.a_number
        );

        // to spice the delete a little we use optimistic concurreny
        collection_client
            .with_document(&document.document_attributes.id)
            .delete_document()
            .with_partition_keys(PartitionKeys::new().push(&document.result.a_number)?)
            .with_if_match_condition((&document.document_attributes).into())
            .execute()
            .await?;
    }

    // TASK 5
    // Now the list documents should return 4 documents!
    let list_documents_response = collection_client
        .list_documents()
        .execute::<serde_json::Value>() // you can use this if you don't know/care about the return type!
        .await?;
    assert_eq!(list_documents_response.documents.len(), 4);

    Ok(())
}

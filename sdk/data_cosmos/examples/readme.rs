use serde::{Deserialize, Serialize};
// Using the prelude module of the Cosmos crate makes easier to use the Rust Azure SDK for Cosmos.

use azure_data_cosmos::prelude::*;
use clap::Parser;
use futures::stream::StreamExt;
use time::OffsetDateTime;

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
    string: String,
    number: u64,
    timestamp: i64,
}

// Here we mark `number` as the partition key.
impl azure_data_cosmos::CosmosEntity for MySampleStruct {
    type Entity = u64;

    fn partition_key(&self) -> Self::Entity {
        self.number
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
    // Let's get the Cosmos account name and primary key from env variables.
    let args = Args::parse();

    // First, we create an authorization token.
    // There are two types of tokens: primary and resource constrained. The SDK supports both, but
    // we'll use a primary token. Please check the Azure documentation for details or the examples folder
    // on how to create and use token-based permissions.
    let authorization_token = AuthorizationToken::primary_from_base64(&args.primary_key)?;

    // Next we will create a Cosmos client.
    let client = CosmosClient::new(args.account, authorization_token);

    // We know the database name so we can obtain a database client.
    let database = client.database_client(args.database_name);
    // We know the collection name so we can obtain a collection client.
    let collection = database.collection_client(args.collection_name);

    // Now, we'll insert 10 documents
    println!("Inserting 10 documents...");
    let mut session_token = None;
    for i in 0..10 {
        // Here we define the document.
        let document_to_insert = MySampleStruct {
            id: format!("unique_id{i}"),
            string: "Something here".into(),
            number: i * 100,
            timestamp: OffsetDateTime::now_utc().unix_timestamp(),
        };

        // Insert the document and store the returned session token for later use in the cosmos consistency level!
        session_token = Some(
            collection
                .create_document(document_to_insert)
                .is_upsert(true)
                .into_future()
                .await?
                .session_token, // get only the session token, if everything else was ok!
        );
    }
    println!("Done!");

    let session_token = ConsistencyLevel::Session(session_token.unwrap());

    println!("\nStreaming documents");
    // Next, we stream list the documents.
    // We limit the number of documents to 3 for each batch as a demonstration. In practice
    // you will use a more sensible number (or accept the Azure default).
    let mut stream = collection
        .list_documents()
        .consistency_level(session_token.clone())
        .max_item_count(3)
        .into_stream::<MySampleStruct>();
    while let Some(res) = stream.next().await {
        let res = res?;
        println!("Received {} documents in one batch!", res.documents.len());
        res.documents
            .iter()
            .for_each(|doc| println!("Document: {:#?}", doc));
    }

    println!("\nQuerying documents");
    let query_documents_response = collection
        .query_documents("SELECT * FROM A WHERE A.number < 600")
        .query_cross_partition(true) // this will perform a cross partition query!
        .consistency_level(session_token)
        .into_stream::<MySampleStruct>()
        .next()
        .await
        .unwrap()?;

    println!(
        "Received {} documents!",
        query_documents_response.results.len()
    );

    query_documents_response.documents().for_each(|document| {
        println!("number ==> {}", document.number);
    });

    let session_token = ConsistencyLevel::Session(query_documents_response.session_token);
    for (document, document_attributes) in query_documents_response.results {
        println!(
            "deleting id == {}, a_number == {}.",
            document.id, document.number
        );

        collection
            .document_client(document.id, &document.number)?
            .delete_document()
            .consistency_level(session_token.clone())
            .if_match_condition(&document_attributes.unwrap())
            .into_future()
            .await?;
    }

    // Now the list documents should return 4 documents!
    let list_documents_response = collection
        .list_documents()
        .consistency_level(session_token)
        .into_stream::<serde_json::Value>() // you can list json if you don't know/care about the return type!
        .next()
        .await
        .unwrap()?;
    assert_eq!(list_documents_response.documents.len(), 4);

    Ok(())
}

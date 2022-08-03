use clap::Parser;
use futures::stream::StreamExt;
use serde::{Deserialize, Serialize};
// Using the prelude module of the Cosmos crate makes easier to use the Rust Azure SDK for Cosmos DB.
use azure_core::prelude::*;

use azure_data_cosmos::prelude::*;
use time::OffsetDateTime;

#[derive(Debug, Parser)]
struct Args {
    /// Cosmos primary key name
    #[clap(env = "COSMOS_PRIMARY_KEY")]
    primary_key: String,
    /// The cosmos account your're using
    #[clap(env = "COSMOS_ACCOUNT")]
    account: String,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
struct MySampleStruct {
    id: String,
    a_string: String,
    a_number: u64,
    a_timestamp: i64,
}

impl azure_data_cosmos::CosmosEntity for MySampleStruct {
    type Entity = String;

    fn partition_key(&self) -> Self::Entity {
        self.id.clone()
    }
}

const DATABASE: &str = "azuresdktestdb";
const COLLECTION: &str = "azuresdktc";

// This code will perform these tasks:
// 1. Find an Azure Cosmos DB called *DATABASE*. If it does not exist, create it.
// 2. Find an Azure Cosmos collection called *COLLECTION* in *DATABASE*.
//      If it does not exist, create it.
// 3. Store an entry in collection *COLLECTION* of database *DATABASE*.
// 4. Delete everything.
#[tokio::main]
async fn main() -> azure_core::Result<()> {
    // Let's get Cosmos account and access key from env variables.
    // This helps automated testing.
    let args = Args::parse();

    // First, we create an authorization token. There are two types of tokens, master and resource
    // constrained. Please check the Azure documentation for details. You can change tokens
    // at will and it's a good practice to raise your privileges only when needed.
    let authorization_token = AuthorizationToken::primary_from_base64(&args.primary_key)?;

    // Next we will create a Cosmos client. You need an authorization_token but you can later
    // change it if needed.
    let client = CosmosClient::new(args.account, authorization_token);

    // list_databases will give us the databases available in our account. If there is
    // an error (for example, the given key is not valid) you will receive a
    // specific azure_data_cosmos::Error. In this example we will look for a specific database
    // so we chain a filter operation.
    let db = client
        .list_databases()
        .into_stream()
        .next()
        .await
        .unwrap()?
        .databases
        .into_iter()
        .find(|db| db.id == DATABASE);

    // If the requested database is not found we create it.
    let database = match db {
        Some(db) => db,
        None => {
            client
                .create_database(DATABASE)
                .into_future()
                .await?
                .database
        }
    };
    println!("database == {:?}", database);

    // Now we look for a specific collection. If is not already present
    // we will create it. The collection creation is more complex and
    // has many options (such as indexing and so on).
    let collection = {
        let collections = client
            .database_client(database.id.clone())
            .list_collections()
            .into_stream()
            .next()
            .await
            .unwrap()?;

        if let Some(collection) = collections
            .collections
            .into_iter()
            .find(|coll| coll.id == COLLECTION)
        {
            collection
        } else {
            client
                .clone()
                .database_client(database.id.clone())
                .create_collection(COLLECTION, "/id")
                .into_future()
                .await?
                .collection
        }
    };

    println!("collection = {:?}", collection);

    // Now that we have a database and a collection we can insert
    // data in them. Let's create a Document. The only constraint
    // is that we need an id and an arbitrary, Serializable type.
    let doc = MySampleStruct {
        id: "unique_id100".into(),
        a_string: "Something here".into(),
        a_number: 100,
        a_timestamp: OffsetDateTime::now_utc().unix_timestamp(),
    };

    // Now we store the struct in Azure Cosmos DB.
    // Notice how easy it is! :)
    // First we construct a "collection" specific client so we
    // do not need to specify it over and over.
    let collection = client
        .database_client(database.id.clone())
        .collection_client(collection.id);

    // The method create_document will return, upon success,
    // the document attributes.

    let create_document_response = collection
        .create_document(doc.clone())
        .into_future()
        .await?;
    println!(
        "create_document_response == {:#?}",
        create_document_response
    );

    // Now we list all the documents in our collection. It
    // should show we have 1 document.
    println!("Listing documents...");
    let list_documents_response = collection
        .list_documents()
        .into_stream::<MySampleStruct>()
        .next()
        .await
        .unwrap()?;
    println!(
        "list_documents_response contains {} documents",
        list_documents_response.documents.len()
    );

    // Now we get the same document by id.
    println!("getting document by id {}", &doc.id);
    let get_document_response = collection
        .clone()
        .document_client(doc.id.clone(), &doc.id)?
        .get_document::<MySampleStruct>()
        .into_future()
        .await?;
    println!("get_document_response == {:#?}", get_document_response);

    // The document can be no longer there so the result is
    // an Option<Document<T>>
    if let GetDocumentResponse::Found(document) = get_document_response {
        // Now, for the sake of experimentation, we will update (replace) the
        // document created. We do this only if the original document has not been
        // modified in the meantime. This is called optimistic concurrency.
        // In order to do so, we pass to this replace_document call
        // the etag received in the previous get_document. The etag is an opaque value that
        // changes every time the document is updated. If the passed etag is different in
        // CosmosDB it means something else updated the document before us!
        let replace_document_response = collection
            .clone()
            .document_client(doc.id.clone(), &doc.id)?
            .replace_document(doc)
            .if_match_condition(IfMatchCondition::Match(document.etag))
            .into_future()
            .await?;
        println!(
            "replace_document_response == {:#?}",
            replace_document_response
        );
    }

    // We will perform some cleanup. First we delete the collection...
    client
        .database_client(DATABASE.to_owned())
        .collection_client(COLLECTION.to_owned())
        .delete_collection()
        .into_future()
        .await?;
    println!("collection deleted");

    // And then we delete the database.
    client
        .database_client(database.id)
        .delete_database()
        .into_future()
        .await?;
    println!("database deleted");

    Ok(())
}

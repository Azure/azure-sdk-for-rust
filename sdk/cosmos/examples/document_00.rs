use serde::{Deserialize, Serialize};
// Using the prelude module of the Cosmos crate makes easier to use the Rust Azure SDK for Cosmos
// DB.
use azure_core::prelude::*;
use azure_cosmos::prelude::*;
use azure_cosmos::responses::GetDocumentResponse;
use std::borrow::Cow;
use std::error::Error;

#[derive(Clone, Serialize, Deserialize, Debug)]
struct MySampleStruct<'a> {
    id: Cow<'a, str>,
    a_string: Cow<'a, str>,
    a_number: u64,
    a_timestamp: i64,
}

impl<'a> azure_cosmos::CosmosEntity<'a> for MySampleStruct<'a> {
    type Entity = &'a str;

    fn partition_key(&'a self) -> Self::Entity {
        self.id.as_ref()
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
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    // Let's get Cosmos account and master key from env variables.
    // This helps automated testing.
    let master_key =
        std::env::var("COSMOS_MASTER_KEY").expect("Set env variable COSMOS_MASTER_KEY first!");
    let account = std::env::var("COSMOS_ACCOUNT").expect("Set env variable COSMOS_ACCOUNT first!");

    // First, we create an authorization token. There are two types of tokens, master and resource
    // constrained. Please check the Azure documentation for details. You can change tokens
    // at will and it's a good practice to raise your privileges only when needed.
    let authorization_token = AuthorizationToken::primary_from_base64(&master_key)?;

    // Next we will create a Cosmos client. You need an authorization_token but you can later
    // change it if needed.
    let client = CosmosClient::new(
        account.clone(),
        authorization_token.clone(),
        CosmosOptions::default(),
    );

    // list_databases will give us the databases available in our account. If there is
    // an error (for example, the given key is not valid) you will receive a
    // specific azure_cosmos::Error. In this example we will look for a specific database
    // so we chain a filter operation.
    let db = client
        .list_databases()
        .execute()
        .await?
        .databases
        .into_iter()
        .find(|db| db.id == DATABASE);

    // If the requested database is not found we create it.
    let database = match db {
        Some(db) => db,
        None => {
            client
                .create_database(Context::new(), DATABASE, CreateDatabaseOptions::new())
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
            .clone()
            .into_database_client(database.id.clone())
            .list_collections()
            .execute()
            .await?;

        if let Some(collection) = collections
            .collections
            .into_iter()
            .find(|coll| coll.id == COLLECTION)
        {
            collection
        } else {
            client
                .clone()
                .into_database_client(database.id.clone())
                .create_collection(
                    Context::new(),
                    COLLECTION,
                    CreateCollectionOptions::new("/id"),
                )
                .await?
                .collection
        }
    };

    println!("collection = {:?}", collection);

    // Now that we have a database and a collection we can insert
    // data in them. Let's create a Document. The only constraint
    // is that we need an id and an arbitrary, Serializable type.
    let doc = MySampleStruct {
        id: Cow::Owned("unique_id100".to_owned()),
        a_string: Cow::Borrowed("Something here"),
        a_number: 100,
        a_timestamp: chrono::Utc::now().timestamp(),
    };

    // Now we store the struct in Azure Cosmos DB.
    // Notice how easy it is! :)
    // First we construct a "collection" specific client so we
    // do not need to specify it over and over.
    let database_client = client.clone().into_database_client(database.id.clone());
    let collection_client = database_client.into_collection_client(collection.id);

    // The method create_document will return, upon success,
    // the document attributes.

    let create_document_response = collection_client
        .create_document(Context::new(), &doc, CreateDocumentOptions::new())
        .await?;
    println!(
        "create_document_response == {:#?}",
        create_document_response
    );

    // Now we list all the documents in our collection. It
    // should show we have 1 document.
    println!("Listing documents...");
    let list_documents_response = collection_client
        .list_documents()
        .execute::<MySampleStruct>()
        .await?;
    println!(
        "list_documents_response contains {} documents",
        list_documents_response.documents.len()
    );

    // Now we get the same document by id.
    println!("getting document by id {}", &doc.id);
    let get_document_response = collection_client
        .clone()
        .into_document_client(doc.id.clone(), &doc.id)?
        .get_document()
        .execute::<MySampleStruct>()
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
        let replace_document_response = collection_client
            .clone()
            .into_document_client(doc.id.clone(), &doc.id)?
            .replace_document()
            .if_match_condition(IfMatchCondition::Match(&document.etag))
            .execute(&doc)
            .await?;
        println!(
            "replace_document_response == {:#?}",
            replace_document_response
        );
    }

    // We will perform some cleanup. First we delete the collection...
    client
        .clone()
        .into_database_client(DATABASE.to_owned())
        .into_collection_client(COLLECTION.to_owned())
        .delete_collection()
        .execute()
        .await?;
    println!("collection deleted");

    // And then we delete the database.
    client
        .into_database_client(database.id)
        .delete_database()
        .execute()
        .await?;
    println!("database deleted");

    Ok(())
}

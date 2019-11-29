#[macro_use]
extern crate serde_derive;
// Using the prelude module of the Cosmos crate makes easier to use the Rust Azure SDK for Cosmos
// DB.
use azure_sdk_cosmos::prelude::*;
use std::error::Error;

#[derive(Serialize, Deserialize, Debug)]
struct MySampleStruct<'a> {
    id: &'a str,
    a_string: &'a str,
    a_number: u64,
    a_timestamp: i64,
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
async fn main() -> Result<(), Box<dyn Error>> {
    // Let's get Cosmos account and master key from env variables.
    // This helps automated testing.
    let master_key =
        std::env::var("COSMOS_MASTER_KEY").expect("Set env variable COSMOS_MASTER_KEY first!");
    let account = std::env::var("COSMOS_ACCOUNT").expect("Set env variable COSMOS_ACCOUNT first!");

    // First, we create an authorization token. There are two types of tokens, master and resource
    // constrained. Please check the Azure documentation for details. You can change tokens
    // at will and it's a good practice to raise your privileges only when needed.
    let authorization_token = AuthorizationToken::new(account, TokenType::Master, &master_key)?;

    // Next we will create a Cosmos client. You need an authorization_token but you can later
    // change it if needed.
    let client = ClientBuilder::new(authorization_token)?;

    // list_databases will give us the databases available in our account. If there is
    // an error (for example, the given key is not valid) you will receive a
    // specific AzureError. In this example we will look for a specific database
    // so we chain a filter operation.
    let db = client
        .list_databases()
        .await?
        .into_iter()
        .find(|db| db.id == DATABASE);

    // If the requested database is not found we create it.
    let database = match db {
        Some(db) => db,
        None => client.create_database(DATABASE).await?,
    };
    println!("database == {:?}", database);

    // Now we look for a specific collection. If is not already present
    // we will create it. The collection creation is more complex and
    // has many options (such as indexing and so on).
    let collection = {
        let collections = client.list_collections(&database.id).await?;

        if let Some(collection) = collections.into_iter().find(|coll| coll.id == COLLECTION) {
            collection
        } else {
            let indexes = IncludedPathIndex {
                kind: KeyKind::Hash,
                data_type: DataType::String,
                precision: Some(3),
            };

            let ip = IncludedPath {
                path: "/*".to_owned(),
                indexes: vec![indexes],
            };

            let ip = IndexingPolicy {
                automatic: true,
                indexing_mode: IndexingMode::Consistent,
                included_paths: vec![ip],
                excluded_paths: vec![],
            };

            // Notice here we specify the expected performance level.
            // Performance levels have price impact. Also, higher
            // performance levels force you to specify an indexing
            // strategy. Consult the documentation for more details.
            // you can also use the predefined performance levels. For example:
            // `Offer::S2`.
            client
                .create_collection_builder()
                .with_id(COLLECTION)
                .with_database_name(&database.id)
                .with_offer(Offer::Throughput(400))
                .with_indexing_policy(ip)
                .finalize()
                .await?
        }
    };

    println!("collection = {:?}", collection);

    // Now that we have a database and a collection we can insert
    // data in them. Let's create a struct. The only constraint
    // is that the struct should be Serializable.
    let doc = MySampleStruct {
        id: "unique_id1",
        a_string: "Something here",
        a_number: 100,
        a_timestamp: chrono::Utc::now().timestamp(),
    };

    // Now we store the struct in Azure Cosmos DB.
    // Notice how easy it is! :)
    // The method create_document will return, upon success,
    // the document attributes.
    let document_attributes = client
        .create_document(&database.id, &collection.id, &doc)
        .execute()
        .await?;
    println!("document_attributes == {:?}", document_attributes);

    // We will perform some cleanup. First we delete the collection...
    client.delete_collection(DATABASE, COLLECTION).await?;
    println!("collection deleted");

    // And then we delete the database.
    client.delete_database(DATABASE).await?;
    println!("database deleted");

    Ok(())
}

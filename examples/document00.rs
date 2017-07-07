extern crate azure_sdk_for_rust;

extern crate futures;
extern crate tokio_core;
extern crate tokio;
extern crate hyper;
extern crate hyper_tls;
extern crate chrono;

use std::error::Error;

use futures::future::*;
use tokio_core::reactor::Core;

use azure_sdk_for_rust::azure::cosmos::authorization_token::{AuthorizationToken, TokenType};
use azure_sdk_for_rust::azure::cosmos::client::Client;

#[macro_use]
extern crate serde_derive;
use azure_sdk_for_rust::azure::cosmos;

#[derive(Serialize, Deserialize, Debug)]
struct MySampleStruct<'a> {
    id: &'a str,
    a_string: &'a str,
    a_number: u64,
    a_timestamp: i64,
}


const DATABASE: &'static str = "azuresdktestdb";
const COLLECTION: &'static str = "azuresdktc";


fn main() {
    code().unwrap();
}

// This code will perform these tasks:
// 1. Find an Azure Cosmos DB called *DATABASE*. If it does not exist, create it.
// 2. Find an Azure Cosmos collection called *COLLECTION* in *DATABASE*. If it does not exist, create it.
// 3. Store an entry in collection *COLLECTION* of database *DATABASE*.
// 4. Delete everything.
//
// We will use multiple futures for this hoping to make the code clearer.
// There is no need to proceed this way in your code.
// You can go crazy with future combinators if you want to :)
fn code() -> Result<(), Box<Error>> {
    // Let's get Cosmos account and master key from env variables.
    // This helps automated testing.
    let master_key =
        std::env::var("COSMOS_MASTER_KEY").expect("Set env variable COSMOS_MASTER_KEY first!");
    let account = std::env::var("COSMOS_ACCOUNT").expect("Set env variable COSMOS_ACCOUNT first!");

    // First, we create an authorization token. There are two types of tokens, master and resource
    // constrained. Please check the Azure documentation for details. You can change tokens
    // at will and it's a good practice to raise your privileges only when needed.
    let authorization_token = AuthorizationToken::new(account, TokenType::Master, master_key)?;

    // We will create a tokio-core reactor which will drive our futures.
    let mut core = Core::new()?;

    // Next we will create a Cosmos client. You need an authorization_token but you can later
    // change it if needed. Notice the client will be tied to your reactor.
    let client = Client::new(&core.handle(), authorization_token)?;

    // list_databases will give us the databases available in our account. If there is
    // an error (for example, the given key is not valid) you will receive a
    // specific AzureError. In this example we will look for a specific database
    // so we chain a filter operation.
    let future = client.list_databases().and_then(|databases| {
        ok(databases.into_iter().find(|db| db.id == DATABASE))
    });

    // Now we run the future and check the answer. If the requested database
    // is not found we create it.
    let database = match core.run(future)? {
        Some(db) => db,
        None => core.run(client.create_database(DATABASE))?,
    };
    println!("database == {:?}", database);

    // Now we look for a specific collection. If is not already present
    // we will create it. The collection creation is more complex and
    // has many options (such as indexing and so on).
    let collection = {
        let collections = core.run(client.list_collections(&database))?;

        if let Some(collection) = collections.into_iter().find(|coll| coll.id == COLLECTION) {
            collection
        } else {
            let indexes = cosmos::collection::IncludedPathIndex {
                kind: cosmos::collection::KeyKind::Hash,
                data_type: cosmos::collection::DataType::String,
                precision: Some(3),
            };

            let ip = cosmos::collection::IncludedPath {
                path: "/*".to_owned(),
                indexes: vec![indexes],
            };


            let ip = cosmos::collection::IndexingPolicy {
                automatic: true,
                indexing_mode: cosmos::collection::IndexingMode::Consistent,
                included_paths: vec![ip],
                excluded_paths: vec![],
            };

            let coll = cosmos::collection::Collection::new(COLLECTION, ip);
            // Notice here we specify the expected performance level.
            // Performance levels have price impact. Also, higher
            // performance levels force you to specify an indexing
            // strategy. Consult the documentation for more details.
            core.run(client.create_collection(&database, 400, &coll))?
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
    let document_attributes = core.run(
        client.create_document_as_entity(&database, &collection, false, None, &doc),
    )?;
    println!("document_attributes == {:?}", document_attributes);

    // We will perform some cleanup. First we delete the collection...
    core.run(client.delete_collection(DATABASE, COLLECTION))?;
    println!("collection deleted");

    // And then we delete the database.
    core.run(client.delete_database(DATABASE))?;
    println!("database deleted");

    Ok(())
}

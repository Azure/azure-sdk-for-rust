use azure_core::Context;
use azure_cosmos::prelude::*;
use serde_json::Value;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    // First we retrieve the account name and master key from environment variables.
    // We expect master keys (ie, not resource constrained)
    let master_key =
        std::env::var("COSMOS_MASTER_KEY").expect("Set env variable COSMOS_MASTER_KEY first!");
    let account = std::env::var("COSMOS_ACCOUNT").expect("Set env variable COSMOS_ACCOUNT first!");

    let authorization_token = permission::AuthorizationToken::primary_from_base64(&master_key)?;

    let client = CosmosClient::new(account, authorization_token, CosmosOptions::default());

    let dbs = client.list_databases().execute().await?;

    for db in dbs.databases {
        println!("database == {:?}", db);
        let database = client.clone().into_database_client(db.name().to_owned());

        let collections = database.list_collections().execute().await?;
        for collection in collections.collections {
            println!("collection == {:?}", collection);
            let collection_client = database.clone().into_collection_client(collection.id);

            if collection_client.collection_name() == "democ" {
                println!("democ!");

                let data = r#"
                {
                    "id": "my_id",
                    "name": "John Tonno7",
                    "age": 43,
                    "phones": [
                        "+44 1234567",
                        "+44 2345678"
                    ]
                }"#;
                let document: Value = serde_json::from_str(data)?;

                let options = CreateDocumentOptions::new()
                    .is_upsert(true)
                    .partition_key(&43u32)
                    .unwrap();
                let resp = collection_client
                    .create_document(Context::new(), &document, options)
                    .await?;

                println!("resp == {:?}", resp);

                // call replace collection
                let mut indexing_policy_new = collection.indexing_policy.clone();
                indexing_policy_new
                    .excluded_paths
                    .push("/\"collo2\"/?".to_owned().into());

                println!("\nReplacing collection");
                let replace_collection_response = collection_client
                    .replace_collection()
                    .indexing_policy(&indexing_policy_new)
                    .execute("/age")
                    .await?;
                println!(
                    "replace_collection_response == {:#?}",
                    replace_collection_response
                );
            }

            let documents = collection_client
                .list_documents()
                .execute::<Value>()
                .await?;
            println!("\ndocuments as json == {:?}", documents);
        }
    }

    Ok(())
}

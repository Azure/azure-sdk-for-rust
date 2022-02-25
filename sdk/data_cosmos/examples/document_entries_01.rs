use azure_core::Context;
use azure_data_cosmos::prelude::*;
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Serialize, Deserialize, Clone, Debug)]
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

// This example expects you to have created a collection
// with partitionKey on "id".
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let database_name = std::env::args()
        .nth(1)
        .expect("please specify database name as first command line parameter");
    let collection_name = std::env::args()
        .nth(2)
        .expect("please specify collection name as second command line parameter");

    let master_key =
        std::env::var("COSMOS_MASTER_KEY").expect("Set env variable COSMOS_MASTER_KEY first!");
    let account = std::env::var("COSMOS_ACCOUNT").expect("Set env variable COSMOS_ACCOUNT first!");

    let authorization_token = AuthorizationToken::primary_from_base64(&master_key)?;

    let client = CosmosClient::new(account, authorization_token, CosmosOptions::default());
    let client = client.into_database_client(database_name);
    let client = client.into_collection_client(collection_name);

    let mut doc = MySampleStruct {
        id: format!("unique_id{}", 500),
        a_string: "Something here".into(),
        a_number: 600,
        a_timestamp: chrono::Utc::now().timestamp(),
    };

    // let's add an entity.
    let create_document_response = client
        .create_document(doc.clone())
        .is_upsert(true)
        .into_future()
        .await?;

    println!(
        "create_document_response == {:#?}\n\n\n",
        create_document_response
    );

    let get_document_response = client
        .clone()
        .into_document_client(doc.id.clone(), &doc.id)?
        .get_document::<serde_json::Value>(
            Context::new(),
            GetDocumentOptions::new().consistency_level(&create_document_response),
        )
        .await?;
    println!("get_document_response == {:#?}", get_document_response);

    let get_document_response = client
        .clone()
        .into_document_client("ciccia", &doc.id)?
        .get_document::<serde_json::Value>(
            Context::new(),
            GetDocumentOptions::new().consistency_level(&create_document_response),
        )
        .await?;
    println!(
        "get_document_response == {:#?}\n\n\n",
        get_document_response
    );

    let list_documents_response = client
        .list_documents()
        .consistency_level(&get_document_response)
        .execute::<serde_json::Value>()
        .await?;
    println!("list_documents_response == {:#?}", list_documents_response);

    let query_documents_response = client
        .query_documents()
        .consistency_level(&list_documents_response)
        .query_cross_partition(true)
        .execute::<serde_json::Value, _>("SELECT * FROM c WHERE c.a_number = 600")
        .await?;
    println!(
        "query_documents_response == {:#?}",
        query_documents_response
    );

    doc.a_number = 43;

    let replace_document_response = client
        .into_document_client(doc.id.clone(), &doc.id)?
        .replace_document(
            Context::new(),
            &doc,
            ReplaceDocumentOptions::new().consistency_level(&query_documents_response),
        )
        .await?;
    println!(
        "replace_document_response == {:#?}",
        replace_document_response
    );

    Ok(())
}

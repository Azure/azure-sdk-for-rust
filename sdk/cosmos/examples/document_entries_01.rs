use azure_core::HttpClient;
use azure_cosmos::prelude::*;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::error::Error;
use std::sync::Arc;

#[derive(Serialize, Deserialize, Clone, Debug)]
struct MySampleStruct<'a> {
    id: Cow<'a, str>,
    a_string: Cow<'a, str>,
    a_number: u64,
    a_timestamp: i64,
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

    let http_client: Arc<Box<dyn HttpClient>> = Arc::new(Box::new(reqwest::Client::new()));
    let client = CosmosClient::new(http_client, account, authorization_token);
    let client = client.into_database_client(database_name);
    let client = client.into_collection_client(collection_name);

    let mut doc = Document::new(MySampleStruct {
        id: Cow::Owned(format!("unique_id{}", 500)),
        a_string: Cow::Borrowed("Something here"),
        a_number: 600,
        a_timestamp: chrono::Utc::now().timestamp(),
    });

    let partition_keys = PartitionKeys::from([&doc.document.id]);
    // let's add an entity.
    let create_document_response = client
        .create_document()
        .partition_keys(partition_keys.clone())
        .is_upsert(true)
        .execute(&doc)
        .await?;

    println!(
        "create_document_response == {:#?}\n\n\n",
        create_document_response
    );

    let document_client = client
        .clone()
        .into_document_client(doc.document.id.clone().into_owned(), partition_keys.clone());

    let get_document_response = document_client
        .get_document()
        .consistency_level(&create_document_response)
        .execute::<serde_json::Value>()
        .await?;
    println!("get_document_response == {:#?}", get_document_response);

    let document_client = client
        .clone()
        .into_document_client("ciccia", partition_keys.clone());

    let get_document_response = document_client
        .get_document()
        .consistency_level(&get_document_response)
        .execute::<serde_json::Value>()
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
        .query(&("SELECT * FROM c WHERE c.a_number = 600".into()))
        .consistency_level(&list_documents_response)
        .query_cross_partition(true)
        .execute::<serde_json::Value>()
        .await?;
    println!(
        "query_documents_response == {:#?}",
        query_documents_response
    );

    doc.document.a_number = 43;

    let replace_document_response = client
        .replace_document()
        .consistency_level(&query_documents_response)
        .partition_keys(partition_keys)
        .execute(&doc.document.id, &doc)
        .await?;
    println!(
        "replace_document_response == {:#?}",
        replace_document_response
    );

    Ok(())
}

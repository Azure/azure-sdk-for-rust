use azure_cosmos::prelude::*;
use std::borrow::Cow;
use std::error::Error;
#[macro_use]
extern crate serde_derive;

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
async fn main() -> Result<(), Box<dyn Error>> {
    let database_name = std::env::args()
        .nth(1)
        .expect("please specify database name as first command line parameter");
    let collection_name = std::env::args()
        .nth(2)
        .expect("please specify collection name as second command line parameter");

    let master_key =
        std::env::var("COSMOS_MASTER_KEY").expect("Set env variable COSMOS_MASTER_KEY first!");
    let account = std::env::var("COSMOS_ACCOUNT").expect("Set env variable COSMOS_ACCOUNT first!");

    let authorization_token = AuthorizationToken::new_master(&master_key)?;

    let client = CosmosClient::new(account, authorization_token);
    let client = client.into_database_client(database_name);
    let client = client.into_collection_client(collection_name);

    let mut doc = Document::new(MySampleStruct {
        id: Cow::Owned(format!("unique_id{}", 500)),
        a_string: Cow::Borrowed("Something here"),
        a_number: 600,
        a_timestamp: chrono::Utc::now().timestamp(),
    });

    let mut partition_keys = PartitionKeys::new();
    partition_keys.push(&doc.document.id)?;

    // let's add an entity.
    let create_document_response = client
        .create_document()
        .with_partition_keys(&partition_keys)
        .with_is_upsert(true)
        .execute_with_document(&doc)
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
        .with_consistency_level((&create_document_response).into())
        .execute::<serde_json::Value>()
        .await?;
    println!("get_document_response == {:#?}", get_document_response);

    let document_client = client
        .clone()
        .into_document_client("ciccia", partition_keys.clone());

    let get_document_response = document_client
        .get_document()
        .with_consistency_level((&get_document_response).into())
        .execute::<serde_json::Value>()
        .await?;
    println!(
        "get_document_response == {:#?}\n\n\n",
        get_document_response
    );

    let list_documents_response = client
        .list_documents()
        .with_consistency_level((&get_document_response).into())
        .execute::<serde_json::Value>()
        .await?;
    println!("list_documents_response == {:#?}", list_documents_response);

    let query_documents_response = client
        .query_documents()
        .with_query(&("SELECT * FROM c WHERE c.a_number = 600".into()))
        .with_consistency_level((&list_documents_response).into())
        .with_query_cross_partition(true)
        .execute::<serde_json::Value>()
        .await?;
    println!(
        "query_documents_response == {:#?}",
        query_documents_response
    );

    doc.document.a_number = 43;

    let replace_document_response = client
        .replace_document()
        .with_consistency_level((&query_documents_response).into())
        .with_document_id(&doc.document.id)
        .with_partition_keys(&partition_keys)
        .execute_with_document(&doc)
        .await?;
    println!(
        "replace_document_response == {:#?}",
        replace_document_response
    );

    Ok(())
}

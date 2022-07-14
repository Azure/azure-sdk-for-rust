use azure_data_cosmos::prelude::*;
use clap::Parser;
use futures::StreamExt;
use serde::{Deserialize, Serialize};

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
async fn main() -> azure_core::Result<()> {
    let args = Args::parse();
    let authorization_token = AuthorizationToken::primary_from_base64(&args.primary_key)?;

    let client = CosmosClient::new(args.account, authorization_token)
        .database_client(args.database_name)
        .collection_client(args.collection_name);

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
        .document_client(doc.id.clone(), &doc.id)?
        .get_document::<serde_json::Value>()
        .consistency_level(&create_document_response)
        .into_future()
        .await?;
    println!("get_document_response == {:#?}", get_document_response);

    let get_document_response = client
        .document_client("ciccia", &doc.id)?
        .get_document::<serde_json::Value>()
        .consistency_level(&create_document_response)
        .into_future()
        .await?;
    println!(
        "get_document_response == {:#?}\n\n\n",
        get_document_response
    );

    let list_documents_response = client
        .list_documents()
        .consistency_level(&get_document_response)
        .into_stream::<serde_json::Value>()
        .next()
        .await
        .unwrap()?;
    println!("list_documents_response == {:#?}", list_documents_response);

    let query_documents_response = client
        .query_documents("SELECT * FROM c WHERE c.a_number = 600")
        .consistency_level(&list_documents_response)
        .query_cross_partition(true)
        .into_stream::<serde_json::Value>()
        .next()
        .await
        .unwrap()?;
    println!(
        "query_documents_response == {:#?}",
        query_documents_response
    );

    doc.a_number = 43;

    let replace_document_response = client
        .document_client(doc.id.clone(), &doc.id)?
        .replace_document(doc)
        .consistency_level(&query_documents_response)
        .into_future()
        .await?;
    println!(
        "replace_document_response == {:#?}",
        replace_document_response
    );

    Ok(())
}

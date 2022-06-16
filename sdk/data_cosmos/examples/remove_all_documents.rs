use azure_core::error::Result;
use azure_data_cosmos::prelude::*;
use futures::stream::StreamExt;
use serde_json::Value;

// This example expects you to have created a collection
// with partitionKey on "id".
#[tokio::main]
async fn main() -> Result<()> {
    let database_name = std::env::args()
        .nth(1)
        .expect("please specify the database name as first command line parameter");
    let collection_name = std::env::args()
        .nth(2)
        .expect("please specify the collection name as second command line parameter");
    let partition_key_name = std::env::args()
        .nth(3)
        .expect("please specify the partition key as third command line parameter");

    let master_key =
        std::env::var("COSMOS_MASTER_KEY").expect("Set env variable COSMOS_MASTER_KEY first!");
    let account = std::env::var("COSMOS_ACCOUNT").expect("Set env variable COSMOS_ACCOUNT first!");

    let authorization_token = AuthorizationToken::primary_from_base64(&master_key)?;

    // Next we will create a Cosmos client.
    let client = CosmosClient::new(
        account.clone(),
        authorization_token,
        CosmosOptions::default(),
    );

    let client = client.database_client(database_name);
    let client = client.collection_client(collection_name);

    let mut documents = Vec::new();

    let stream = client.list_documents();
    let mut stream = stream.into_stream::<serde_json::Value>();
    while let Some(res) = stream.next().await {
        for doc in res?.documents {
            documents.push(doc);
        }
    }

    for document in documents {
        // find id and partition key from document json
        let doc_as_obj = match document.document {
            Value::Object(map) => map,
            _ => panic!("expected one object"),
        };

        let id = match &doc_as_obj["id"] {
            Value::String(id) => id,
            _ => panic!("cannot find id field as string"),
        };
        let partition_key: String = match &doc_as_obj[&partition_key_name] {
            Value::String(id) => id.to_owned(),
            Value::Number(num) => {
                format!(
                    "{}",
                    num.as_i64().expect("only numbers up to i64 are supported")
                )
            }
            _ => panic!("cannot find supplied partition key as string"),
        };

        println!(
            "deleting id =={:#?}, partition key == {:#?}",
            id, partition_key
        );

        client
            .document_client(id.clone(), &partition_key)?
            .delete_document()
            .into_future()
            .await?;
    }

    Ok(())
}

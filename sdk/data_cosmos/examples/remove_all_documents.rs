use clap::Parser;
use futures::stream::StreamExt;
use serde_json::Value;

mod util;

#[derive(Debug, clap::Parser)]
struct Args {
    #[clap(flatten)]
    auth: util::Auth,
    /// The name of the database
    database_name: String,
    /// The name of the collection
    collection_name: String,
    /// The name of the partition key
    partition_key_name: String,
}

// This example expects you to have created a collection
// with partitionKey on "id".
#[tokio::main]
async fn main() -> azure_core::Result<()> {
    let args = Args::parse();
    let client = args.auth.into_client()?;
    let collection = client
        .database_client(args.database_name)
        .collection_client(args.collection_name);

    let mut documents = Vec::new();

    let stream = collection.list_documents();
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
        let partition_key: String = match &doc_as_obj[&args.partition_key_name] {
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

        collection
            .document_client(id.clone(), &partition_key)?
            .delete_document()
            .into_future()
            .await?;
    }

    Ok(())
}

use azure_core::modify_conditions::IfMatchCondition;
use azure_core::prelude::*;
use azure_cosmos::prelude::*;
use azure_cosmos::responses::GetDocumentResponse;
use futures::stream::StreamExt;
use std::borrow::Cow;
use std::error::Error;
#[macro_use]
extern crate serde_derive;

// Now we create a sample struct. The Cow trick
// allows us to use the same struct for serializing
// (without having to own the items if not needed) and
// for deserializing (where owning is required).
// We do not need to define the "id" field here, it will be
// specified in the Document struct below.
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
    let client = client.clone().into_database_client(database_name);
    let client = client.into_collection_client(collection_name);

    let mut response = None;
    for i in 0u64..5 {
        let doc = Document::new(MySampleStruct {
            id: Cow::Owned(format!("unique_id{}", i)),
            a_string: Cow::Borrowed("Something here"),
            a_number: i,
            a_timestamp: chrono::Utc::now().timestamp(),
        });

        // let's add an entity.
        response = Some(
            client
                .create_document()
                .with_partition_keys(PartitionKeys::new().push(&doc.document.id)?)
                .execute_with_document(&doc)
                .await?,
        );
    }

    println!("Created 5 documents.");

    // Let's get 3 entries at a time.
    let response = client
        .list_documents()
        .with_consistency_level(response.unwrap().into())
        .with_max_item_count(3)
        .execute::<MySampleStruct>()
        .await?;

    assert_eq!(response.documents.len(), 3);
    println!("response == {:#?}", response);

    // we inserted 5 documents and retrieved the first 3.
    // continuation_token must be present
    assert!(response.continuation_token.is_some());

    let session_token = (&response).into();
    let ct = response.continuation_token.clone().unwrap();
    println!("ct == {}", ct);

    let response = client
        .list_documents()
        .with_consistency_level(session_token)
        .with_continuation(&ct)
        .execute::<MySampleStruct>()
        .await?;

    assert_eq!(response.documents.len(), 2);
    println!("response == {:#?}", response);

    // we got the last 2 entries. Now continuation_token
    // must be absent
    assert!(response.continuation_token.is_none());

    // we can have Rust pass the continuation_token for
    // us if we call the stream function. Here we
    // ask for 3 items at the time but of course you don't have to do that, the
    // stream function will work regardless of the limits imposed.
    let session_token: ConsistencyLevel = (&response).into();
    {
        println!("\nStreaming documents");
        let stream = client
            .list_documents()
            .with_consistency_level(session_token.clone())
            .with_max_item_count(3);
        let mut stream = Box::pin(stream.stream::<MySampleStruct>());
        // TODO: As soon as the streaming functionality is completed
        // in Rust substitute this while let Some... into
        // for each (or whatever the Rust team picks).
        while let Some(res) = stream.next().await {
            let res = res?;
            println!("Received {} documents in one batch!", res.documents.len());
        }
    }

    println!("\n\nLooking for a specific item");
    let id = format!("unique_id{}", 3);
    let partition_keys: PartitionKeys = (&id).into();

    let response = client
        .clone()
        .into_document_client(id.clone(), partition_keys.clone())
        .get_document()
        .with_consistency_level(session_token)
        .execute::<MySampleStruct>()
        .await?;

    assert!(match response {
        GetDocumentResponse::Found(_) => true,
        _ => false,
    });
    println!("response == {:#?}", response);

    let mut doc = match response {
        GetDocumentResponse::Found(ref resp) => resp.clone(),
        GetDocumentResponse::NotFound(_) => panic!(),
    };
    doc.document.document.a_string = "Something else here".into();

    println!("\n\nReplacing document");
    let replace_document_response = client
        .replace_document()
        .with_partition_keys(&partition_keys)
        .with_document_id(&id)
        .with_consistency_level(ConsistencyLevel::from(&response))
        .with_if_match_condition(IfMatchCondition::Match(&doc.etag)) // use optimistic concurrency check
        .execute_with_document(&doc.document)
        .await?;

    println!(
        "replace_document_response == {:#?}",
        replace_document_response
    );

    // This id should not be found. We expect None as result and
    // has_been_found == false
    println!("\n\nLooking for non-existing item");
    let id = format!("unique_id{}", 100);
    let partition_keys: PartitionKeys = (&id).into();

    let response = client
        .clone()
        .into_document_client(id.clone(), partition_keys)
        .get_document()
        .with_consistency_level((&response).into())
        .execute::<MySampleStruct>()
        .await?;

    assert!(match response {
        GetDocumentResponse::NotFound(_) => true,
        _ => false,
    });
    println!("response == {:#?}", response);

    for i in 0u64..5 {
        let id = format!("unique_id{}", i);
        let partition_keys: PartitionKeys = (&id).into();
        client
            .clone()
            .into_document_client(id, partition_keys)
            .delete_document()
            .with_consistency_level((&response).into())
            .execute()
            .await?;
    }
    println!("Cleaned up");

    Ok(())
}

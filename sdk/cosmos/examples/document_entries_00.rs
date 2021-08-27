use azure_core::prelude::*;
use azure_cosmos::prelude::*;
use azure_cosmos::responses::GetDocumentResponse;
use futures::stream::StreamExt;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::error::Error;

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

impl<'a> azure_cosmos::CosmosEntity<'a> for MySampleStruct<'a> {
    type Entity = &'a str;

    fn partition_key(&'a self) -> Self::Entity {
        self.id.as_ref()
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

    let authorization_token = permission::AuthorizationToken::primary_from_base64(&master_key)?;

    let client = CosmosClient::new(account, authorization_token, CosmosOptions::default());
    let client = client.into_database_client(database_name);
    let client = client.into_collection_client(collection_name);

    let mut response = None;
    for i in 0u64..5 {
        let doc = MySampleStruct {
            id: Cow::Owned(format!("unique_id{}", i)),
            a_string: Cow::Borrowed("Something here"),
            a_number: i,
            a_timestamp: chrono::Utc::now().timestamp(),
        };

        // let's add an entity.
        response = Some(
            client
                .create_document(Context::new(), &doc, CreateDocumentOptions::new())
                .await?,
        );
    }

    println!("Created 5 documents.");

    // Let's get 3 entries at a time.
    let response = client
        .list_documents()
        .consistency_level(response.unwrap())
        .max_item_count(3i32)
        .execute::<MySampleStruct>()
        .await?;

    assert_eq!(response.documents.len(), 3);
    println!("response == {:#?}", response);

    // we inserted 5 documents and retrieved the first 3.
    // continuation_token must be present
    assert!(response.continuation_token.is_some());

    let session_token = &response;
    let ct = response.continuation_token.clone().unwrap();
    println!("ct == {}", ct);

    let response = client
        .list_documents()
        .consistency_level(session_token)
        .continuation(ct.as_str())
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
            .consistency_level(session_token.clone())
            .max_item_count(3);
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
    let partition_key = &id;

    let response = client
        .clone()
        .into_document_client(id.clone(), partition_key)?
        .get_document()
        .consistency_level(session_token)
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
        .clone()
        .into_document_client(id.clone(), &id)?
        .replace_document()
        .consistency_level(ConsistencyLevel::from(&response))
        .if_match_condition(IfMatchCondition::Match(&doc.etag)) // use optimistic concurrency check
        .execute(&doc.document)
        .await?;

    println!(
        "replace_document_response == {:#?}",
        replace_document_response
    );

    // This id should not be found. We expect None as result and
    // has_been_found == false
    println!("\n\nLooking for non-existing item");
    let id = format!("unique_id{}", 100);

    let response = client
        .clone()
        .into_document_client(id.clone(), &id)?
        .get_document()
        .consistency_level(&response)
        .execute::<MySampleStruct>()
        .await?;

    assert!(match response {
        GetDocumentResponse::NotFound(_) => true,
        _ => false,
    });
    println!("response == {:#?}", response);

    for i in 0u64..5 {
        let id = format!("unique_id{}", i);
        client
            .clone()
            .into_document_client(id.clone(), &id)?
            .delete_document()
            .consistency_level(&response)
            .execute()
            .await?;
    }
    println!("Cleaned up");

    Ok(())
}

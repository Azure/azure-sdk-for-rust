use azure_sdk_cosmos::prelude::*;
use std::error::Error;
#[macro_use]
extern crate serde_derive;

// Now we create the same struct twice. The second
// will be used by the 'get' methods. There must not
// be references: the struct must own all the data. This
// is required in order to satisfy DeserializeOwned.
#[derive(Serialize, Deserialize, Debug)]
struct MySampleStruct<'a> {
    id: &'a str,
    a_string: &'a str,
    a_number: u64,
    a_timestamp: i64,
}

// Shadow struct. See above. Of course
// you do not need both if your starting
// struct owns all its data.
#[derive(Serialize, Deserialize, Debug)]
struct MySampleStructOwned {
    id: String,
    a_string: String,
    a_number: u64,
    a_timestamp: i64,
}

// This example expects you to have created a collection
// with partitionKey on "id". This SDK works with
// unpartitioned collections too but this example,
// for simplicity sake, does not :)
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

    let authorization_token = AuthorizationToken::new(account, TokenType::Master, &master_key)?;

    let client = ClientBuilder::new(authorization_token)?;

    for i in 0u64..5 {
        let doc = MySampleStructOwned {
            id: format!("unique_id{}", i),
            a_string: "Something here".to_owned(),
            a_number: i,
            a_timestamp: chrono::Utc::now().timestamp(),
        };

        // let's add an entity.
        client
            .create_document(&database_name, &collection_name, &doc)
            .partition_key(doc.id)
            .execute()
            .await?;
    }

    println!("Created 5 documents.");

    // let's get 3 entries at a time
    let response = client
        .list_documents(&database_name, &collection_name)
        .max_item_count(3u64)
        .execute::<MySampleStructOwned>()
        .await?;

    assert_eq!(response.documents.len(), 3);
    println!("response == {:#?}", response);

    // we inserted 5 documents and retrieved the first 3.
    // continuation_token must be present
    assert_eq!(
        response.additional_headers.continuation_token.is_some(),
        true
    );

    let ct = response.additional_headers.continuation_token.unwrap();
    println!("ct == {}", ct);

    let response = client
        .list_documents(&database_name, &collection_name)
        .continuation_token(ct)
        .execute::<MySampleStructOwned>()
        .await?;

    assert_eq!(response.documents.len(), 2);
    println!("response == {:#?}", response);

    // we got the last 2 entries. Now continuation_token
    // must be absent
    assert_eq!(
        response.additional_headers.continuation_token.is_some(),
        false
    );

    println!("\n\nLooking for a specific item");
    let id = format!("unique_id{}", 3);

    let response = client
        .get_document(&database_name, &collection_name, &id)
        .partition_key(id.clone())
        .execute::<MySampleStructOwned>()
        .await?;

    assert_eq!(response.document.is_some(), true);
    println!("response == {:#?}", response);
    let mut doc = response.document.unwrap();
    doc.entity.a_string = "Something else here".into();

    let etag = doc.document_attributes.etag().to_owned();

    let _response = client
        .replace_document(&database_name, &collection_name, &doc)
        .partition_key(id)
        .if_match(etag) // use optimistic concurrency check
        .execute()
        .await?;

    // This id should not be found. We expect None as result
    println!("\n\nLooking for non-existing item");
    let id = format!("unique_id{}", 100);

    let response = client
        .get_document(&database_name, &collection_name, &id)
        .partition_key(id.clone())
        .execute::<MySampleStructOwned>()
        .await?;

    assert_eq!(response.document.is_some(), false);
    println!("response == {:#?}", response);

    for i in 0u64..5 {
        let id = format!("unique_id{}", i);
        client
            .delete_document(&database_name, &collection_name, &id)
            .partition_key(id.clone())
            .execute()
            .await?;
    }
    println!("Cleaned up");

    Ok(())
}

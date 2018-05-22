extern crate azure_sdk_for_rust;

extern crate chrono;
extern crate futures;
extern crate hyper;
extern crate hyper_tls;
extern crate tokio_core;

use std::error::Error;

use tokio_core::reactor::Core;

use azure_sdk_for_rust::cosmos::{AuthorizationToken, TokenType, Client, DocumentRequestExt};

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
fn main() {
    code().unwrap();
}

fn code() -> Result<(), Box<Error>> {
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

    let mut core = Core::new()?;

    let client = Client::new(&core.handle(), authorization_token)?;

    core.run(
        futures::future::join_all((0..5).map(|i| {
            let doc = MySampleStruct {
                id: &format!("unique_id{}", i),
                a_string: "Something here",
                a_number: i,
                a_timestamp: chrono::Utc::now().timestamp(),
            };

            // let's add an entity. we ignore the errors at this point and just
            // notify the user.
            client.create_document(&database_name, &collection_name, &doc)
                .unwrap()
                .partition_key(doc.id)
                .execute()
        }))
    ).unwrap();
    println!("Created 5 documents.");

    // let's get 3 entries at a time
    let response = core.run(
        client.list_documents(&database_name, &collection_name)
            .unwrap()
            .max_item_count(3)
            .execute::<MySampleStructOwned>()
    ).unwrap();

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

    let response = core.run(
        client.list_documents(&database_name, &collection_name)
            .unwrap()
            .continuation_token(ct)
            .execute::<MySampleStructOwned>()
    ).unwrap();

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

    let response = core.run(
        client.get_document(&database_name, &collection_name, &id)
            .unwrap()
            .partition_key(&id)
            .execute::<MySampleStructOwned>()
    ).unwrap();

    assert_eq!(response.document.is_some(), true);
    println!("response == {:#?}", response);
    let mut doc = response.document.unwrap();
    doc.entity.a_string = "Something else here".into();

    let _response = core.run(
        client.replace_document(&database_name, &collection_name, &doc)
            .unwrap()
            .partition_key(&id)
            .if_match(doc.document_attributes.etag) // use optimistic concurrency check
            .execute()
        ).unwrap();

    // This id should not be found. We expect None as result
    println!("\n\nLooking for non-existing item");
    let id = format!("unique_id{}", 100);

    let response = core.run(
        client.get_document(&database_name, &collection_name, &id)
            .unwrap()
            .partition_key(&id)
            .execute::<MySampleStructOwned>()
    ).unwrap();

    assert_eq!(response.document.is_some(), false);
    println!("response == {:#?}", response);

    core.run(
        futures::future::join_all((0..5).map(|i| {
            let id = format!("unique_id{}", i);
            client.delete_document(&database_name, &collection_name, &id)
                .unwrap()
                .partition_key(&id)
                .execute()
        }))
    ).unwrap();
    println!("Cleaned up");

    Ok(())
}

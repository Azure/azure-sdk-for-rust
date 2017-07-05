extern crate azure_sdk_for_rust;

extern crate futures;
extern crate tokio_core;
extern crate tokio;
extern crate hyper;
extern crate hyper_tls;
extern crate chrono;

use std::error::Error;

use tokio_core::reactor::Core;

use azure_sdk_for_rust::azure::cosmos::authorization_token::{AuthorizationToken, TokenType};
use azure_sdk_for_rust::azure::cosmos::client::Client;
use azure_sdk_for_rust::azure::cosmos::list_documents::LIST_DOCUMENTS_OPTIONS_DEFAULT;
use azure_sdk_for_rust::azure::cosmos::get_document::GET_DOCUMENT_OPTIONS_DEFAULT;

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

fn main() {
    code().unwrap();
}

fn code() -> Result<(), Box<Error>> {
    let database_name = std::env::args().nth(1).expect(
        "please specify database name as first command line parameter",
    );
    let collection_name = std::env::args().nth(2).expect(
        "please specify collection name as second command line parameter",
    );

    let master_key = std::env::var("COSMOS_MASTER_KEY")
        .expect("Set env variable COSMOS_MASTER_KEY first!");
    let account = std::env::var("COSMOS_ACCOUNT").expect("Set env variable COSMOS_ACCOUNT first!");

    let authorization_token = AuthorizationToken::new(account, TokenType::Master, master_key)?;

    let mut core = Core::new()?;

    let client = Client::new(&core.handle(), authorization_token)?;

    for i in 0..5 {
        let doc = MySampleStruct {
            id: &format!("unique_id{}", i),
            a_string: "Something here",
            a_number: i,
            a_timestamp: chrono::Utc::now().timestamp(),
        };

        println!("Adding one entity");
        core.run(
            client.create_document_as_entity(&database_name, &collection_name, false, None, &doc),
        );
    }

    // let's get 3 entries at a time
    let mut ldo = LIST_DOCUMENTS_OPTIONS_DEFAULT.clone();
    ldo.max_item_count = Some(3);

    let (entries, ldah) = core.run(
        client.list_documents::<_, MySampleStructOwned>(&database_name, &collection_name, &ldo),
    ).unwrap();

    assert_eq!(entries.documents.len(), 3);
    println!("entries == {:?}\nldah = {:?}", entries, ldah);

    // we inserted 5 documents and retrieved the first 3.
    // continuation_token must be present
    assert_eq!(ldah.continuation_token.is_some(), true);
    if let Some(ct) = ldah.continuation_token {
        println!("ct == {}", ct);

        let mut ldo = LIST_DOCUMENTS_OPTIONS_DEFAULT.clone();
        ldo.continuation_token = Some(&ct);

        let (entries, ldah) = core.run(
            client.list_documents::<_, MySampleStructOwned>(&database_name, &collection_name, &ldo),
        ).unwrap();

        assert_eq!(entries.documents.len(), 2);
        println!("entries == {:?}\nldah = {:?}", entries, ldah);

        // we got the last 2 entries. Now continuation_token
        // must be clear
        assert_eq!(ldah.continuation_token.is_some(), false);

        let gdo = GET_DOCUMENT_OPTIONS_DEFAULT.clone();
        let id = format!("unique_id{}", 3);

        let (entry, gdah) =
            core.run(
                client.get_document::<_, MySampleStructOwned>(
                    &database_name,
                    &collection_name,
                    &id,
                    &gdo,
                ),
            ).unwrap();

        assert_eq!(entry.is_some(), true);
        println!("entry == {:?}\ngdah == {:?}", entry, gdah);

        // This id should not be found. We expect None as result
        let id = format!("unique_id{}", 100);

        let (entry, gdah) =
            core.run(
                client.get_document::<_, MySampleStructOwned>(
                    &database_name,
                    &collection_name,
                    &id,
                    &gdo,
                ),
            ).unwrap();

        assert_eq!(entry.is_some(), false);
        println!("entry == {:?}\ngdah == {:?}", entry, gdah);


    }

    Ok(())
}

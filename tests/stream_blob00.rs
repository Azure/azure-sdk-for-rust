#![cfg(all(test, feature = "test_e2e"))]

extern crate azure_sdk_for_rust;
extern crate chrono;
extern crate futures;
extern crate hyper;
extern crate tokio_core;

use azure_sdk_for_rust::core::lease::{LeaseState, LeaseStatus};
use azure_sdk_for_rust::core::range::Range;
use azure_sdk_for_rust::storage::blob::{Blob, BlobType, PUT_OPTIONS_DEFAULT};
use azure_sdk_for_rust::storage::client::Client;
use azure_sdk_for_rust::storage::container::{
    Container, PublicAccess, LIST_CONTAINER_OPTIONS_DEFAULT,
};
use futures::future::ok;
use futures::prelude::*;
use hyper::mime::Mime;
use tokio_core::reactor::Core;

#[test]
fn create_blob_and_stream_back() {
    code().unwrap();
}

fn code() -> Result<(), Box<std::error::Error>> {
    let container_name = "azuresdkforrust";
    let file_name = "azure_sdk_for_rust_stream_test.txt";

    // First we retrieve the account name and master key from environment variables.
    let account =
        std::env::var("STORAGE_ACCOUNT").expect("Set env variable STORAGE_ACCOUNT first!");
    let master_key =
        std::env::var("STORAGE_MASTER_KEY").expect("Set env variable STORAGE_MASTER_KEY first!");

    let mut reactor = Core::new()?;
    let client = Client::new(&reactor.handle(), &account, &master_key)?;

    if reactor
        .run(Container::list(&client, &LIST_CONTAINER_OPTIONS_DEFAULT))?
        .iter()
        .find(|x| x.name == container_name)
        .is_none()
    {
        reactor.run(Container::create(
            &client,
            container_name,
            PublicAccess::Blob,
        ))?;
    }

    let string = "0123456789ABCDEF0123456789ABCDEF0123456789ABCDEF0123456789ABCDEF0123456789ABCDEF0123456789ABCDEF0123456789ABCDEF0123456789ABCDEF0123456789ABCDEF0123456789ABCDEF0123456789ABCDEF0123456789ABCDEF0123456789ABCDEF0123456789ABCDEF0123456789ABCDEF0123456789ABCDEF";

    let new_blob = Blob {
        name: file_name.to_owned(),
        container_name: container_name.to_owned(),
        snapshot_time: None,
        last_modified: chrono::Utc::now(),
        etag: "".to_owned(),
        content_length: string.len() as u64,
        content_type: Some("text/plain".parse::<Mime>().unwrap()),
        content_encoding: None,
        content_language: None,
        content_md5: None,
        cache_control: None,
        x_ms_blob_sequence_number: None,
        blob_type: BlobType::BlockBlob,
        lease_status: LeaseStatus::Unlocked,
        lease_state: LeaseState::Available,
        lease_duration: None,
        copy_id: None,
        copy_status: None,
        copy_source: None,
        copy_progress: None,
        copy_completion: None,
        copy_status_description: None,
    };

    let fut = new_blob
        .put(&client, &PUT_OPTIONS_DEFAULT, Some(string.as_ref()))
        .map(|_| {
            println!("{}/{} blob created!", container_name, file_name);
        });
    reactor.run(fut)?;

    // this is how you stream data from azure blob. Notice that you have
    // to specify the range requested. Also make sure to specify how big
    // a chunk is going to be. Bigger chunks are of course more efficient as the
    // http overhead will be less but it also means you will have to wait for more
    // time before receiving anything. In this example we use an awkward value
    // just to make the test worthwile.
    let stream = Blob::stream(
        &client,
        &container_name,
        file_name,
        None,
        &Range::new(0, string.len() as u64),
        None,
        13,
    );

    let result = std::rc::Rc::new(std::cell::RefCell::new(Vec::new()));

    {
        let mut res_closure = result.borrow_mut();
        let fut = stream.for_each(move |mut value| {
            println!("received {:?} bytes", value.len());
            res_closure.append(&mut value);

            ok(())
        });

        reactor.run(fut)?;
    }

    let returned_string = {
        let mut rlock = result.borrow_mut();
        String::from_utf8(rlock.to_vec())?
    };

    // You can of course conctenate all the
    // pieces as shown below.
    // It generally does not make sense as you
    // will lose the ability to process the data as it
    // comes in.
    //
    //let fut = stream.concat2().map(|res| {
    //    println!("all blocks received");
    //    res
    //});
    //
    //let result = reactor.run(fut)?;
    //let returned_string = String::from_utf8(result)?;

    println!("{}", returned_string);

    assert!(
        string == returned_string,
        "string = {}, returned_string = {}",
        string,
        returned_string
    );

    let future = Blob::delete(&client, &container_name, file_name, None).map(|_| {
        println!("{}/{} blob deleted!", container_name, file_name);
    });

    reactor.run(future)?;

    Ok(())
}

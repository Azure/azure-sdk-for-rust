#![cfg(all(test,feature = "test_e2e"))]

extern crate azure_sdk_for_rust;
extern crate chrono;
extern crate env_logger;
#[macro_use]
extern crate log;
extern crate mime;
extern crate serde;

mod util;

use std::io::Cursor;
use std::ops::Deref;

use azure_sdk_for_rust::azure::core::lease::{LeaseState, LeaseStatus};
use azure_sdk_for_rust::azure::storage::blob::{Blob, BlobType, PUT_OPTIONS_DEFAULT};
use azure_sdk_for_rust::azure::storage::client::Client;
use azure_sdk_for_rust::azure::storage::container::{Container, PublicAccess,
                                                    LIST_CONTAINER_OPTIONS_DEFAULT};
use chrono::UTC;
use mime::Mime;
use util::get_from_env;

#[test]
fn create_and_delete_container() {
    let name: &'static str = "azuresdkrustetoets";

    let client = create_client();
    Container::create(&client, name, PublicAccess::Container).unwrap();

    let mut lco = LIST_CONTAINER_OPTIONS_DEFAULT.clone();
    lco.prefix = Some(name.to_owned());

    let list = Container::list(&client, &lco).unwrap();
    let cont_list: Vec<&Container> = list.deref()
        .into_iter()
        .filter(|e| e.name == name)
        .collect();

    if cont_list.len() != 1 {
        panic!("More than 1 container returned with the same name!");
    }

    let mut cont = cont_list[0].clone();

    cont.delete(&client).unwrap();
}

#[test]
fn list_containers() {
    let client = create_client();

    trace!("running list_containers");
    let mut lco = LIST_CONTAINER_OPTIONS_DEFAULT.clone();
    lco.max_results = 2;

    loop {
        let ret = Container::list(&client, &lco).unwrap();

        trace!("ret {:?}\n\n", ret);
        if !ret.is_complete() {
            lco.next_marker = Some(ret.next_marker().unwrap().to_owned());
        } else {
            break;
        }
    }
}

#[test]
fn put_blob() {
    let client = &create_client();

    let blob_name: &'static str = "m1";
    let container_name: &'static str = "rust-upload-test";
    let value = "abcdef";
    let mut data = Cursor::new(value);
    let len = value.len() as u64;

    if Container::list(client, &LIST_CONTAINER_OPTIONS_DEFAULT)
           .unwrap()
           .iter()
           .find(|x| x.name == container_name)
           .is_none() {
        Container::create(client, container_name, PublicAccess::Blob).unwrap();
    }

    let new_blob = Blob {
        name: blob_name.to_owned(),
        container_name: container_name.to_owned(),
        snapshot_time: None,
        last_modified: UTC::now(),
        etag: "".to_owned(),
        content_length: len,
        content_type: "application/octet-stream".parse::<Mime>().unwrap(),
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

    new_blob
        .put(client, &PUT_OPTIONS_DEFAULT, Some((&mut data, len)))
        .unwrap();

    trace!("created {:?}", new_blob);
}

fn create_client() -> Client {
    let azure_storage_account = get_from_env("AZURE_STORAGE_ACCOUNT");
    let azure_storage_key = get_from_env("AZURE_STORAGE_KEY");
    Client::new(&azure_storage_account, &azure_storage_key, false)
}

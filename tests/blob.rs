#![cfg(all(test, feature = "test_e2e"))]
extern crate azure_sdk_for_rust;

extern crate futures;
extern crate hyper;
extern crate hyper_tls;
extern crate tokio;
extern crate tokio_core;


extern crate chrono;
extern crate env_logger;
#[macro_use]
extern crate log;
extern crate serde;

use std::ops::Deref;

use tokio_core::reactor::Core;

use azure_sdk_for_rust::core::{
    errors::AzureError,
    lease::{LeaseState, LeaseStatus}
};
use azure_sdk_for_rust::storage::{
    blob::{Blob, BlobType, PUT_OPTIONS_DEFAULT},
    client::Client,
    container::{Container, PublicAccess, LIST_CONTAINER_OPTIONS_DEFAULT}
};
use chrono::Utc;
use hyper::mime::Mime;

#[test]
fn create_and_delete_container() {
    let name: &'static str = "azuresdkrustetoets";

    let (client, mut core) = initialize().unwrap();
    core.run(Container::create(&client, name, PublicAccess::Container))
        .unwrap();

    let mut lco = LIST_CONTAINER_OPTIONS_DEFAULT.clone();
    lco.prefix = Some(name.to_owned());

    let list = core.run(Container::list(&client, &lco)).unwrap();
    let cont_list: Vec<&Container> = list.deref()
        .into_iter()
        .filter(|e| e.name == name)
        .collect();

    if cont_list.len() != 1 {
        panic!("More than 1 container returned with the same name!");
    }

    let mut cont = cont_list[0].clone();

    core.run(cont.delete(&client)).unwrap();
}

#[test]
fn list_containers() {
    let (client, mut core) = initialize().unwrap();

    trace!("running list_containers");
    let mut lco = LIST_CONTAINER_OPTIONS_DEFAULT.clone();
    lco.max_results = 2;

    loop {
        let ret = core.run(Container::list(&client, &lco)).unwrap();

        trace!("ret {:?}\n\n", ret);
        if !ret.is_complete() {
            lco.next_marker = Some(ret.token().unwrap().to_owned());
        } else {
            break;
        }
    }
}

#[test]
fn put_blob() {
    let (client, mut core) = initialize().unwrap();

    let blob_name: &'static str = "m1";
    let container_name: &'static str = "rust-upload-test";
    let value = "abcdef";

    if core.run(Container::list(&client, &LIST_CONTAINER_OPTIONS_DEFAULT))
        .unwrap()
        .iter()
        .find(|x| x.name == container_name)
        .is_none()
    {
        core.run(Container::create(
            &client,
            container_name,
            PublicAccess::Blob,
        )).unwrap();
    }

    let new_blob = Blob {
        name: blob_name.to_owned(),
        container_name: container_name.to_owned(),
        snapshot_time: None,
        last_modified: Utc::now(),
        etag: "".to_owned(),
        content_length: value.as_bytes().len() as u64,
        content_type: Some("application/octet-stream".parse::<Mime>().unwrap()),
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

    core.run(new_blob.put(
        &client,
        &PUT_OPTIONS_DEFAULT,
        Some(&value.as_bytes()),
    )).unwrap();

    trace!("created {:?}", new_blob);
}

fn initialize() -> Result<(Client, Core), AzureError> {
    let account =
        std::env::var("STORAGE_ACCOUNT").expect("Set env variable STORAGE_ACCOUNT first!");
    let master_key =
        std::env::var("STORAGE_MASTER_KEY").expect("Set env variable STORAGE_MASTER_KEY first!");
    let core = Core::new()?;

    Ok((Client::new(&core.handle(), &account, &master_key)?, core))
}

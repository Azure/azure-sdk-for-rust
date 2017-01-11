// #![feature(plugin)]
// #![plugin(clippy)]

#[macro_use]
extern crate hyper;
extern crate chrono;
extern crate url;
extern crate crypto;
extern crate rustc_serialize as serialize;
extern crate xml;
#[macro_use]
extern crate mime;
extern crate time;

#[macro_use]
extern crate log;
extern crate env_logger;

extern crate uuid;

use azure::core::lease::{LeaseState, LeaseStatus, LeaseAction};
use azure::storage::client::Client;
use azure::storage::blob::{Blob, BlobType, LIST_BLOB_OPTIONS_DEFAULT, PUT_OPTIONS_DEFAULT,
                           PUT_BLOCK_OPTIONS_DEFAULT, PUT_PAGE_OPTIONS_DEFAULT,
                           LEASE_BLOB_OPTIONS_DEFAULT};
use azure::storage::container::{Container, PublicAccess, LIST_CONTAINER_OPTIONS_DEFAULT};
use azure::core::ba512_range::BA512Range;

use std::fs;
use time::Duration;

// use azure::storage::container::PublicAccess;

#[macro_use]
pub mod azure;

// use chrono::datetime::DateTime;
use chrono::UTC;

use mime::Mime;

use azure::storage::table::*;

fn get_from_env(varname: &str) -> String {
    match std::env::var(varname) {
        Ok(val) => val,
        Err(_) => {
            panic!("Please set {} env variable first!", varname);
        }
    }
}

fn create_storage_client() -> Client {
    let azure_storage_account = get_from_env("AZURE_STORAGE_ACCOUNT");
    let azure_storage_key = get_from_env("AZURE_STORAGE_KEY");
    Client::new(&azure_storage_account, &azure_storage_key, true)
}

fn main() {
    let client = create_storage_client();
    list_containers(&client);
    match list_tables(&client) {
        Err(e) => {  println!("{:?}",e) ; panic!("1") }
        _ => {}
    };
}

#[allow(dead_code)]
#[allow(unused_variables)]
fn main_old() {
    env_logger::init().unwrap();

    let azure_storage_account = match std::env::var("AZURE_STORAGE_ACCOUNT") {
        Ok(val) => val,
        Err(_) => {
            panic!("Please set AZURE_STORAGE_ACCOUNT env variable first!");
        }
    };

    let azure_storage_key = match std::env::var("AZURE_STORAGE_KEY") {
        Ok(val) => val,
        Err(_) => {
            panic!("Please set AZURE_STORAGE_KEY env variable first!");
        }
    };

    let client = Client::new(&azure_storage_account, &azure_storage_key, true);

    let policy_name = match std::env::var("AZURE_POLICY_NAME") {
        Ok(val) => val,
        Err(_) => {
            panic!("Please set AZURE_POLICY_NAME env variable first!");
        }
    };

    let policy_key = match std::env::var("AZURE_POLICY_KEY") {
        Ok(val) => val,
        Err(_) => {
            panic!("Please set AZURE_POLICY_KEY env variable first!");
        }
    };

    let sb_namespace = match std::env::var("AZURE_SERVICE_BUS_NAMESPACE") {
        Ok(val) => val,
        Err(_) => {
            panic!("Please set AZURE_SERVICE_BUS_NAMESPACE env variable first!");
        }
    };

    let ev_name = match std::env::var("AZURE_EVENT_HUB_NAME") {
        Ok(val) => val,
        Err(_) => {
            panic!("Please set AZURE_EVENT_HUB_NAME env variable first!");
        }
    };

    let mut eh_client = azure::service_bus::event_hub::Client::new(&sb_namespace,
                                                                   &ev_name,
                                                                   &policy_name,
                                                                   &policy_key);
    // "todeleh",
    // "write_policy",
    // "9GIzBhQhMKg/patjrI2XS6gSGn6ju2+N40CQEYmowJ8=");

    // client.create_container("balocco3", PublicAccess::Blob).unwrap();
    // // println!("{:?}", new);
    //

    info!("Beginning tests");

    for i in 0..20 {
        info!("Sending message {}", i);
        send_event(&mut eh_client);
    }

    // lease_blob(&client);

    // put_block_blob(&client);
    //
    // put_page_blob(&client);

    // {
    //     let vhds = ret.iter_mut().find(|x| x.name == "canotto").unwrap();
    //
    //     let blobs = vhds.list_blobs(&client, true, true, true, true).unwrap();
    //
    //     println!("len == {:?}", blobs.len());
    //
    //     for blob in &blobs {
    //         println!("{}, {} KB ({:?})",
    //                  blob.name,
    //                  (blob.content_length / 1024),
    //                  blob.lease_state)
    //     }
    //
    //     let (blob, mut stream) = vhds.get_blob(&client, "DataCollector01.csv", None, None, None)
    //                                  .unwrap();
    //     println!("blob == {:?}", blob);
    //
    //     let mut buffer = String::new();
    //     stream.read_to_string(&mut buffer).unwrap();
    //
    //     // println!("buffer == {:?}", buffer);
    // }


    // for i in 0..2 {
    //     use std::fs::metadata;
    //     use std::fs::File;
    //
    //     let file_name: &'static str = "C:\\temp\\prova.txt";
    //     let container_name: &'static str = "rust";
    //
    //     {
    //         let containers = Container::list(&client).unwrap();
    //
    //         let cont = containers.iter().find(|x| x.name == container_name);
    //         if let None = cont {
    //             Container::create(&client, container_name, PublicAccess::Blob).unwrap();
    //         }
    //     }
    //
    //     let metadata = metadata(file_name).unwrap();
    //     let mut file = File::open(file_name).unwrap();
    //
    //     let new_blob = Blob {
    //         name: format!("go_rust{}.txt", i),
    //         snapshot_time: None,
    //         last_modified: UTC::now(),
    //         etag: "".to_owned(),
    //         content_length: metadata.len(),
    //         content_type: "application/octet-stream".parse::<Mime>().unwrap(),
    //         content_encoding: None,
    //         content_language: None,
    //         content_md5: None,
    //         cache_control: None,
    //         x_ms_blob_sequence_number: None,
    //         blob_type: BlobType::BlockBlob,
    //         lease_status: LeaseStatus::Unlocked,
    //         lease_state: LeaseState::Available,
    //         lease_duration: None,
    //         copy_id: None,
    //         copy_status: None,
    //         copy_source: None,
    //         copy_progress: None,
    //         copy_completion: None,
    //         copy_status_description: None,
    //     };
    //
    //     new_blob.put(&client,
    //                  container_name,
    //                  None,
    //                  Some((&mut file, metadata.len())))
    //             .unwrap();
    //
    //     println!("{} created", new_blob.name);
    // }




    // bal2.delete(&client).unwrap();
    // println!("{:?} deleted!", bal2);

    // let ret = client.delete_container("balocco2").unwrap();
    // println!("{:?}", ret);
    // inc_a!("main");
}

#[allow(dead_code)]
fn send_event(cli: &mut azure::service_bus::event_hub::Client) {
    debug!("running send_event");
    let file_name = "C:\\temp\\samplein.json";

    let metadata = fs::metadata(file_name).unwrap();
    let mut file_handle = fs::File::open(file_name).unwrap();

    cli.send_event((&mut file_handle, metadata.len()), Duration::hours(1))
       .unwrap();
}

#[allow(dead_code)]
fn lease_blob(client: &Client) {
    println!("running lease_blob");

    let ret = Container::list(client, &LIST_CONTAINER_OPTIONS_DEFAULT).unwrap();
    let vhds = ret.iter().find(|x| x.name == "rust").unwrap();
    let blobs = Blob::list(&client, &vhds.name, &LIST_BLOB_OPTIONS_DEFAULT).unwrap();
    let blob = blobs.iter().find(|ref x| x.name == "go_rust12.txt").unwrap();

    println!("blob == {:?}", blob);

    let mut lbo = LEASE_BLOB_OPTIONS_DEFAULT.clone();
    lbo.lease_duration = Some(30);
    let ret = blob.lease(client, LeaseAction::Acquire, &lbo).unwrap();
    println!("ret == {:?}", ret);

}

#[allow(dead_code)]
fn list_blobs(client: &Client) {
    println!("running list_blobs");

    let mut lbo2 = LIST_BLOB_OPTIONS_DEFAULT.clone();
    lbo2.max_results = 15;

    loop {
        let uc = Blob::list(&client, "rust", &lbo2).unwrap();

        println!("uc {:?}\n\n", uc);

        if !uc.is_complete() {
            lbo2.next_marker = Some(uc.next_marker().unwrap().to_owned());
        } else {
            break;
        }
    }
}

#[allow(dead_code)]
fn list_containers(client: &Client) {
    println!("running list_containers");

    let mut lco = LIST_CONTAINER_OPTIONS_DEFAULT.clone();
    lco.max_results = 2;
    loop {
        let ret = Container::list(&client, &lco).unwrap();

        println!("ret {:?}\n\n", ret);

        if !ret.is_complete() {
            lco.next_marker = Some(ret.next_marker().unwrap().to_owned());
        } else {
            break;
        }
    }
}

#[allow(dead_code)]
fn put_block_blob(client: &Client) {
    use std::fs::metadata;
    use std::fs::File;

    println!("\nrunning put_block_blob");

    let blob_name: &'static str = "Win64OpenSSL-1_0_2e.exe";
    let file_name: &'static str = "C:\\temp\\Win64OpenSSL-1_0_2e.exe";
    let container_name: &'static str = "rust";
    let metadata = metadata(file_name).unwrap();
    let mut file = File::open(file_name).unwrap();

    let content_length = metadata.len();

    {
        let containers = Container::list(&client, &LIST_CONTAINER_OPTIONS_DEFAULT).unwrap();

        let cont = containers.iter().find(|x| x.name == container_name);
        if let None = cont {
            Container::create(client, container_name, PublicAccess::Blob).unwrap();
        }
    }

    let new_blob = Blob {
        name: blob_name.to_owned(),
        container_name: container_name.to_owned(),
        snapshot_time: None,
        last_modified: UTC::now(),
        etag: "".to_owned(),
        content_length: content_length,
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

    new_blob.put_block(&client,
                       "block_name",
                       &PUT_BLOCK_OPTIONS_DEFAULT,
                       (&mut file, 1024 * 1024))
            .unwrap();

    println!("created {:?}", new_blob);
}

#[allow(dead_code)]
fn put_page_blob(client: &Client) {
    use std::fs::metadata;
    use std::fs::File;

    println!("\nrunning put_page_blob");

    let blob_name: &'static str = "MindDB_Log.ldf";
    let file_name: &'static str = "C:\\temp\\MindDB_Log.ldf";
    let container_name: &'static str = "rust";
    let metadata = metadata(file_name).unwrap();
    let mut file = File::open(file_name).unwrap();

    {
        let containers = Container::list(&client, &LIST_CONTAINER_OPTIONS_DEFAULT).unwrap();

        let cont = containers.iter().find(|x| x.name == container_name);
        if let None = cont {
            Container::create(client, container_name, PublicAccess::Blob).unwrap();
        }
    }

    // align to 512 bytes
    let content_length = metadata.len() % 512 + metadata.len();

    let new_blob = Blob {
        name: blob_name.to_owned(),
        container_name: container_name.to_owned(),
        snapshot_time: None,
        last_modified: UTC::now(),
        etag: "".to_owned(),
        content_length: content_length,
        content_type: "application/octet-stream".parse::<Mime>().unwrap(),
        content_encoding: None,
        content_language: None,
        content_md5: None,
        cache_control: None,
        x_ms_blob_sequence_number: None,
        blob_type: BlobType::PageBlob,
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

    new_blob.put(&client, &PUT_OPTIONS_DEFAULT, None)
            .unwrap();

    let range = BA512Range::new(0, 1024 * 1024 - 1).unwrap();

    new_blob.put_page(client,
                      &range, // 1MB
                      &PUT_PAGE_OPTIONS_DEFAULT,
                      (&mut file, range.size()))
            .unwrap();

    println!("created {:?}", new_blob);
}

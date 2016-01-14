#[macro_use]
extern crate hyper;
extern crate chrono;
extern crate url;
extern crate crypto;
extern crate rustc_serialize as serialize;
extern crate xml;
#[macro_use]
extern crate mime;

#[macro_use]
extern crate log;
extern crate env_logger;


use azure::storage::{LeaseState, LeaseStatus};
use azure::storage::client::Client;
use azure::storage::blob::{Blob, BlobType, ListBlobOptions, LIST_BLOB_OPTIONS_DEFAULT,
                           PUT_OPTIONS_DEFAULT, PUT_BLOCK_OPTIONS_DEFAULT,
                           PUT_PAGE_OPTIONS_DEFAULT};
use azure::storage::container::{Container, PublicAccess, LIST_CONTAINER_OPTIONS_DEFAULT};
use azure::core::ba512_range::BA512Range;

// use azure::storage::container::PublicAccess;

#[macro_use]
pub mod azure;

// use chrono::datetime::DateTime;
use chrono::UTC;

use mime::Mime;

fn main() {
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

    // client.create_container("balocco3", PublicAccess::Blob).unwrap();
    // // println!("{:?}", new);
    //

    info!("Beginning tests");

    put_block_blob(&client);

    put_page_blob(&client);

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
                      (&mut file, range.len()))
            .unwrap();

    println!("created {:?}", new_blob);
}

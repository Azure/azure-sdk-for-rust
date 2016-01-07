#[macro_use]
extern crate hyper;
extern crate chrono;
extern crate url;
extern crate crypto;
extern crate rustc_serialize as serialize;
extern crate xml;
#[macro_use]
extern crate mime;


use azure::storage::{LeaseState, LeaseStatus};
use azure::storage::client::Client;
use azure::storage::blob::{Blob, BlobType};
use azure::storage::container::{Container, PublicAccess};

// use azure::storage::container::PublicAccess;

#[macro_use]
pub mod azure;

// use chrono::datetime::DateTime;
use chrono::UTC;

use mime::Mime;

fn main() {
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
    let ret = Container::list(&client).unwrap();
    println!("{:?}", ret);

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



    {
        use std::fs::metadata;
        use std::fs::File;

        let file_name: &'static str = "C:\\temp\\list.txt";
        let container_name: &'static str = "rust";

        {
            let containers = Container::list(&client).unwrap();

            let cont = containers.iter().find(|x| x.name == container_name);
            if let None = cont {
                Container::create(&client, container_name, PublicAccess::Blob).unwrap();
            }
        }

        let metadata = metadata(file_name).unwrap();
        let mut file = File::open(file_name).unwrap();

        let new_blob = Blob {
            name: "from_rust.txt".to_owned(),
            snapshot_time: None,
            last_modified: UTC::now(),
            etag: "".to_owned(),
            content_length: 1024 * 1024 * 4, // 4MB
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

        new_blob.put(&client,
                     container_name,
                     None,
                     Some((&mut file, metadata.len())))
                .unwrap();
    }


    // bal2.delete(&client).unwrap();
    // println!("{:?} deleted!", bal2);

    // let ret = client.delete_container("balocco2").unwrap();
    // println!("{:?}", ret);
    // inc_a!("main");
}

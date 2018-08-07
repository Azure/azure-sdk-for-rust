extern crate azure_sdk_for_rust;
extern crate chrono;
extern crate env_logger;
extern crate futures;
extern crate hyper;
extern crate hyper_tls;
extern crate tokio_core;

use azure_sdk_for_rust::{
    core::lease::{LeaseState, LeaseStatus},
    prelude::*,
    storage::blob::{get_block_list, put_block_list, Blob, BlobBlockType, BlobType, BlockList, BlockListType, PUT_BLOCK_OPTIONS_DEFAULT},
};
use futures::future::*;
use std::collections::HashMap;
use std::error::Error;
use tokio_core::reactor::Core;

fn main() {
    env_logger::init();
    code().unwrap();
}

// We run a separate method to use the elegant quotation mark operator.
// A series of unwrap(), unwrap() would have achieved the same result.
fn code() -> Result<(), Box<Error>> {
    // First we retrieve the account name and master key from environment variables.
    let account = std::env::var("STORAGE_ACCOUNT").expect("Set env variable STORAGE_ACCOUNT first!");
    let master_key = std::env::var("STORAGE_MASTER_KEY").expect("Set env variable STORAGE_MASTER_KEY first!");

    let container_name = std::env::args()
        .nth(1)
        .expect("please specify container name as first command line parameter");

    let mut core = Core::new()?;

    let client = Client::new(&account, &master_key)?;

    let name = "asdkrustputblock.txt";

    let contents1 = "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA";
    let contents2 = "BBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBB";
    let contents3 = "CCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCC";

    let new_blob = Blob {
        name: name.to_owned(),
        container_name: container_name.to_owned(),
        snapshot_time: None,
        last_modified: None,
        etag: None,
        content_length: 0,
        // here we pass text/plain as content_type. This means your browser will
        // try to show you the file if you click on it in the Azure portal.
        // Make sure to send a text file :)
        content_type: Some("text/plain".to_owned()),
        content_encoding: None,
        content_language: None,
        content_md5: None,
        cache_control: None,
        x_ms_blob_sequence_number: None,
        blob_type: BlobType::BlockBlob,
        lease_status: Some(LeaseStatus::Unlocked),
        lease_state: LeaseState::Available,
        lease_duration: None,
        copy_id: None,
        copy_status: None,
        copy_source: None,
        copy_progress: None,
        copy_completion_time: None,
        copy_status_description: None,
        access_tier: String::from(""),
        access_tier_change_time: None,
        access_tier_inferred: None,
        content_disposition: None,
        creation_time: chrono::Utc::now(),
        deleted_time: None,
        incremental_copy: None,
        metadata: HashMap::new(),
        remaining_retention_days: None,
        server_encrypted: false,
    };

    let mut block_list = BlockList::default();

    let future = new_blob
        .put_block(&client, "block1", &PUT_BLOCK_OPTIONS_DEFAULT, &contents1.as_bytes())
        .map(|encoded_block_id| {
            println!("block1 blob for blob {} created", name);
            block_list.blocks.push(BlobBlockType::Uncommitted(encoded_block_id));
            block_list
        }).and_then(|mut block_list| {
            new_blob
                .put_block(&client, "block2", &PUT_BLOCK_OPTIONS_DEFAULT, &contents2.as_bytes())
                .map(|encoded_block_id| {
                    println!("block2 blob for blob {} created", name);
                    block_list.blocks.push(BlobBlockType::Uncommitted(encoded_block_id));
                    block_list
                })
        }).and_then(|mut block_list| {
            new_blob
                .put_block(&client, "block3", &PUT_BLOCK_OPTIONS_DEFAULT, &contents3.as_bytes())
                .map(|encoded_block_id| {
                    println!("block3 blob for blob {} created", name);
                    block_list.blocks.push(BlobBlockType::Uncommitted(encoded_block_id));
                    block_list
                })
        }).map(|block_list| {
            println!("{:?}", block_list);
            block_list
        });

    let block_list = core.run(future)?;
    println!("computed block list == {:?}", block_list);

    let future = get_block_list(
        &client,
        &(&container_name as &str, name),
        &BlockListType::All,
        None,
        None,
        None,
        None,
    );

    let received_block_list = core.run(future)?;
    println!("current block list: {:?}", received_block_list);

    // now we can finalize the blob with put_block_list
    let future = put_block_list(
        &client,
        &(&container_name as &str, name),
        None,
        None,
        &received_block_list.block_list.into(),
    ).map(|_| {
        println!("blob finalized!");
    });
    core.run(future)?;

    let future = Blob::delete(&client, &container_name, &name, None).map(|_| println!("Blob deleted!"));

    core.run(future)?;

    Ok(())
}

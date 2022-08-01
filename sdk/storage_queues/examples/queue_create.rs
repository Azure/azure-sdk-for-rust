#[macro_use]
extern crate log;
use azure_core::{date, prelude::*};

use azure_storage::core::prelude::*;
use azure_storage_queues::prelude::*;
use time::OffsetDateTime;

#[tokio::main]
async fn main() -> azure_core::Result<()> {
    // First we retrieve the account name and access key from environment variables.
    let account =
        std::env::var("STORAGE_ACCOUNT").expect("Set env variable STORAGE_ACCOUNT first!");
    let access_key =
        std::env::var("STORAGE_ACCESS_KEY").expect("Set env variable STORAGE_ACCESS_KEY first!");

    let queue_name = std::env::args()
        .nth(1)
        .expect("Please pass the queue name as first parameter");

    let storage_account = StorageClient::new_access_key(&account, &access_key);

    let queue = storage_account.queue_client(queue_name);

    trace!("creating queue");

    // this step is optional but here we show
    // how to add metadata to a new queue.
    let mut metadata = Metadata::new();
    metadata
        .as_mut()
        .insert("source".into(), "Azure SDK for Rust".into());
    metadata.as_mut().insert(
        "created".into(),
        format!("{:?}", OffsetDateTime::now_utc()).into(),
    );

    let response = queue
        .create()
        .metadata(metadata.clone())
        .into_future()
        .await?;
    println!("response == {:#?}", response);

    // let's add some more metadata
    metadata.insert("version".to_owned(), "TBD".to_owned());
    metadata.insert(
        "updated".to_owned(),
        format!("{:?}", OffsetDateTime::now_utc()),
    );

    println!("metadata == {:#?}", metadata);

    let response = queue.set_metadata(metadata).into_future().await?;
    println!("response == {:#?}", response);

    // let's get back the metadata
    let response = queue.get_metadata().into_future().await?;
    println!("response == {:#?}", response);

    // use two queue stored access policies
    let policies = vec![
        QueueStoredAccessPolicy::new(
            "first_sap_read_process",
            OffsetDateTime::now_utc() - date::duration_from_hours(1),
            OffsetDateTime::now_utc() + date::duration_from_days(1),
        )
        .enable_read()
        .enable_process(),
        QueueStoredAccessPolicy::new(
            "sap_admin",
            OffsetDateTime::now_utc() - date::duration_from_hours(1),
            OffsetDateTime::now_utc() + date::duration_from_hours(5),
        )
        .enable_all(),
    ];

    let response = queue.set_acl(policies).into_future().await?;
    println!("response == {:#?}", response);

    // get the queue ACL
    let response = queue.get_acl().into_future().await?;
    println!("response == {:#?}", response);

    // now let's delete it
    let response = queue.delete().into_future().await?;
    println!("response == {:#?}", response);

    Ok(())
}

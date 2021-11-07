#[macro_use]
extern crate log;
use azure_core::prelude::*;
use azure_messaging_queues::prelude::*;
use azure_storage::core::prelude::*;
use chrono::{Duration, Utc};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    // First we retrieve the account name and master key from environment variables.
    let account =
        std::env::var("STORAGE_ACCOUNT").expect("Set env variable STORAGE_ACCOUNT first!");
    let master_key =
        std::env::var("STORAGE_MASTER_KEY").expect("Set env variable STORAGE_MASTER_KEY first!");

    let queue_name = std::env::args()
        .nth(1)
        .expect("Please pass the queue name as first parameter");

    let http_client = new_http_client();

    let storage_account_client =
        StorageAccountClient::new_access_key(http_client.clone(), &account, &master_key);
    let queue = storage_account_client.as_queue_client(queue_name);

    trace!("creating queue");

    // this step is optional but here we show
    // how to add metadata to a new queue.
    let mut metadata = Metadata::new();
    metadata
        .as_mut()
        .insert("source".into(), "Azure SDK for Rust".into());
    metadata
        .as_mut()
        .insert("created".into(), format!("{:?}", Utc::now()).into());

    let response = queue.create().metadata(&metadata).execute().await?;
    println!("response == {:#?}", response);

    // let's add some more metadata
    metadata.insert("version".to_owned(), "TBD".to_owned());
    metadata.insert("updated".to_owned(), format!("{:?}", Utc::now()));

    println!("metadata == {:#?}", metadata);

    let response = queue.set_metadata().execute(&metadata).await?;
    println!("response == {:#?}", response);

    // let's get back the metadata
    let response = queue.get_metadata().execute().await?;
    println!("response == {:#?}", response);

    // create two queue stored access policies
    let mut queue_stored_acess_policies = Vec::new();
    queue_stored_acess_policies.push(
        QueueStoredAccessPolicy::new(
            "first_sap_read_process",
            Utc::now() - Duration::hours(1),
            Utc::now() + Duration::days(1),
        )
        .enable_read()
        .enable_process(),
    );
    queue_stored_acess_policies.push(
        QueueStoredAccessPolicy::new(
            "sap_admin",
            Utc::now() - chrono::Duration::hours(1),
            Utc::now() + chrono::Duration::hours(5),
        )
        .enable_all(),
    );

    let response = queue
        .set_acl()
        .execute(&queue_stored_acess_policies)
        .await?;
    println!("response == {:#?}", response);

    // get the queue ACL
    let response = queue.get_acl().execute().await?;
    println!("response == {:#?}", response);

    // now let's delete it
    let response = queue
        .delete()
        .client_request_id("myclientid")
        .execute()
        .await?;
    println!("response == {:#?}", response);

    Ok(())
}

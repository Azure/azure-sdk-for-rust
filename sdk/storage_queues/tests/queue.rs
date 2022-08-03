#![cfg(all(test, feature = "test_e2e"))]
use azure_core::{date, prelude::*};
use azure_storage::core::prelude::*;
use azure_storage_queues::prelude::*;
use futures::StreamExt;
use std::time::Duration;
use time::OffsetDateTime;
use uuid::Uuid;

#[tokio::test]
async fn queue_create_put_and_get() -> azure_core::Result<()> {
    let account =
        std::env::var("STORAGE_ACCOUNT").expect("Set env variable STORAGE_ACCOUNT first!");
    let access_key =
        std::env::var("STORAGE_ACCESS_KEY").expect("Set env variable STORAGE_ACCESS_KEY first!");

    let queue_name = format!("sdk-{}", Uuid::new_v4());

    let storage_client = StorageClient::new_access_key(&account, &access_key);

    let queue_service_client = storage_client.queue_service_client();

    println!("creating queue {}", queue_name);

    let queue = storage_client.queue_client(queue_name);

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

    // create two queue stored access policies
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

    let mut stream = queue_service_client.list_queues().into_stream();
    while let Some(entry) = stream.next().await {
        let entry = entry?;
        println!("entry == {:#?}", entry);
    }

    for i in 0u32..5 {
        println!("putting message {}", i);

        let response = queue
            .put_message(format!("Azure SDK for Rust {}", OffsetDateTime::now_utc()))
            .into_future()
            .await?;

        println!("response == {:#?}", response);
    }

    let get_messages_response = queue
        .get_messages()
        .number_of_messages(2)
        .visibility_timeout(Duration::from_secs(10))
        .into_future()
        .await?;
    println!("get_messages_response == {:#?}", get_messages_response);

    for message_to_update in get_messages_response.messages.into_iter() {
        let pop_receipt = queue.pop_receipt_client(message_to_update);

        let response = pop_receipt
            .update(
                format!("new body at {}", OffsetDateTime::now_utc()),
                Duration::from_secs(4),
            )
            .into_future()
            .await?;
        println!("response == {:#?}", response);
    }

    let get_response = queue
        .get_messages()
        .number_of_messages(2)
        .visibility_timeout(Duration::from_secs(5))
        .into_future()
        .await?;

    println!("get_response == {:#?}", get_response);

    for message_to_delete in get_response.messages {
        println!("deleting message {:?}", message_to_delete);

        let delete_response = queue
            .pop_receipt_client(message_to_delete)
            .delete()
            .into_future()
            .await?;

        println!("delete_response == {:#?}", delete_response);
    }

    // now let's delete the queue
    let response = queue.delete().into_future().await?;
    println!("response == {:#?}", response);

    Ok(())
}

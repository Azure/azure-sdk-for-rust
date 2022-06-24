#![cfg(all(test, feature = "test_e2e"))]
use azure_core::prelude::*;
use azure_storage::core::prelude::*;
use azure_storage_queues::prelude::*;
use chrono::Utc;
use std::time::Duration;

#[tokio::test]
async fn queue_create_put_and_get() -> azure_core::Result<()> {
    let account =
        std::env::var("STORAGE_ACCOUNT").expect("Set env variable STORAGE_ACCOUNT first!");
    let access_key =
        std::env::var("STORAGE_ACCESS_KEY").expect("Set env variable STORAGE_ACCESS_KEY first!");

    let queue_name = "rustazuree2e";
    let http_client = azure_core::new_http_client();

    let storage_account_client =
        StorageAccountClient::new_access_key(http_client.clone(), &account, &access_key);
    let queue = storage_account_client
        .storage_client()
        .queue_client(queue_name);

    println!("creating queue {}", queue_name);

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
            Utc::now() - chrono::Duration::hours(1),
            Utc::now() + chrono::Duration::days(1),
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

    for i in 0u32..5 {
        println!("putting message {}", i);

        let response = queue
            .put_message()
            .client_request_id("optional correlation token")
            .execute(format!("Azure SDK for Rust rocks! {}", chrono::Utc::now()))
            .await?;

        println!("response == {:#?}", response);
    }

    let get_messages_response = queue
        .get_messages()
        .number_of_messages(2)
        .visibility_timeout(Duration::from_secs(10))
        .execute()
        .await?;
    println!("get_messages_response == {:#?}", get_messages_response);

    for message_to_update in get_messages_response.messages.into_iter() {
        let pop_receipt = queue.pop_receipt_client(message_to_update);

        let response = pop_receipt
            .update(Duration::from_secs(4))
            .execute(format!("new body at {}", chrono::Utc::now()))
            .await?;
        println!("response == {:#?}", response);
    }

    let get_response = queue
        .get_messages()
        .number_of_messages(2)
        .visibility_timeout(Duration::from_secs(5))
        .execute()
        .await?;

    println!("get_response == {:#?}", get_response);

    for message_to_delete in get_response.messages {
        println!("deleting message {:?}", message_to_delete);

        let delete_response = queue
            .pop_receipt_client(message_to_delete)
            .delete()
            .execute()
            .await?;

        println!("delete_response == {:#?}", delete_response);
    }

    // now let's delete the queue
    let response = queue
        .delete()
        .client_request_id("myclientid")
        .execute()
        .await?;
    println!("response == {:#?}", response);

    Ok(())
}

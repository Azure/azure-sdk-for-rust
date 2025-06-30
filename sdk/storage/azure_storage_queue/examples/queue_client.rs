use std::collections::HashMap;

mod helpers;
use helpers::endpoint::get_endpoint;
use helpers::logs::log_operation_result;
use helpers::random_queue_name::get_random_queue_name;

use azure_identity::DefaultAzureCredential;
use azure_storage_queue::{
    clients::QueueClient,
    models::{
        ListOfSignedIdentifier, QueueClientDequeueOptions, QueueClientGetMetadataResultHeaders,
        QueueClientPeekOptions, QueueClientSetMetadataOptions, QueueClientUpdateOptions,
        QueueMessage,
    },
};

async fn send_and_delete_message(
    queue_client: &QueueClient,
    message: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let result = queue_client.enqueue_message(message, None).await;

    if let Ok(response) = result {
        let message = response.into_body().await?;

        if let Some(message) = message {
            if let (Some(message_id), Some(pop_receipt)) = (message.message_id, message.pop_receipt)
            {
                println!(
                    "Deleting message with ID: {} and pop receipt: {}",
                    message_id, pop_receipt
                );
                let delete_result = queue_client
                    .delete_message(&message_id, &pop_receipt, None)
                    .await;
                log_operation_result(&delete_result, "delete_message");
            }
        }
    }

    Ok(())
}

async fn send_and_update_message(
    queue_client: &QueueClient,
    message: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let result = queue_client.enqueue_message(message, None).await;

    if let Ok(response) = result {
        let message = response.into_body().await?;

        if let Some(message) = message {
            if let (Some(message_id), Some(pop_receipt)) = (message.message_id, message.pop_receipt)
            {
                println!(
                    "Updating message with ID: {} and pop receipt: {}",
                    message_id, pop_receipt
                );
                let queue_message = QueueMessage {
                    message_text: Some("Updated message text from Rust".to_string()),
                };
                let update_option = QueueClientUpdateOptions {
                    // Serialize the message text as bytes for the update
                    queue_message: Some(queue_message.try_into()?),
                    ..Default::default()
                };
                let update_result = queue_client
                    .update_message(&message_id.clone(), &pop_receipt, 50, Some(update_option))
                    .await;
                log_operation_result(&update_result, "update_message");
            }
        }
    }

    Ok(())
}

async fn set_and_get_access_policies(
    queue_client: &QueueClient,
) -> Result<(), Box<dyn std::error::Error>> {
    //     .checked_add(std::time::Duration::from_secs(3600)) // 1 hour from now
    //     .ok_or("Failed to calculate expiry time")?;
    // let acl = ListOfSignedIdentifier {
    //     items: Some(vec![SignedIdentifier {
    //         id: Some("policy1".to_string()),
    //         access_policy: Some(AccessPolicy {
    //             start: Some(OffsetDateTime::now_utc()),
    //             expiry: Some(expiry_time.into()),
    //             permission: Some("raup".to_string()),
    //         }),
    //     }]),
    // };

    //     let acl_xml = quick_xml::se::to_string(&acl);
    //     println!("Access Policy XML: {}", acl_xml?);

    //     let acl_xml = "<SignedIdentifiers>
    //   <SignedIdentifier>
    //     <AccessPolicy>
    //       <Expiry>2025-06-27T15:02:39.351158345Z</Expiry>
    //       <Permission>raup</Permission>
    //       <Start>2025-06-26T14:02:39.351160525Z</Start>
    //     </AccessPolicy>
    //     <Id>MTIzNDU2Nzg5MDEyMzQ1Njc4OTAxMjM0NTY3ODkwMTI=</Id>
    //   </SignedIdentifier>
    // </SignedIdentifiers>";

    //     let result = queue_client
    //         .set_access_policy(acl_xml.try_into()?, None)
    //         .await;
    // TODO: Fix set and get access policies
    let acl = ListOfSignedIdentifier {
        ..Default::default()
    };
    let result = queue_client.set_access_policy(acl.try_into()?, None).await;
    log_operation_result(&result, "set_access_policy");

    let result = queue_client.get_access_policy(None).await;
    log_operation_result(&result, "get_access_policy");
    let properties = result.unwrap().into_body().await?;
    if let Some(policies) = properties.items {
        for policy in policies {
            println!(
                "Access Policy - Id: {}, Start: {:?}, Expiry: {:?}, Permissions: {}",
                &policy.id.unwrap_or_default(),
                policy.access_policy.clone().unwrap().start.unwrap(),
                policy.access_policy.clone().unwrap().expiry.unwrap(),
                policy.access_policy.clone().unwrap().permission.unwrap()
            );
        }
    } else {
        println!("No access policies found.");
    }

    Ok(())
}

async fn set_and_get_metadata(
    queue_client: &QueueClient,
) -> Result<(), Box<dyn std::error::Error>> {
    let metadata_options = Some(QueueClientSetMetadataOptions {
        metadata: Some(HashMap::from([
            ("key1".to_string(), "value1".to_string()),
            ("key2".to_string(), "value2".to_string()),
        ])),
        ..Default::default()
    });
    let result = queue_client.set_metadata(metadata_options).await;
    log_operation_result(&result, "set_metadata");

    let result = queue_client.get_metadata(None).await;
    log_operation_result(&result, "get_metadata");

    let metadata = result.unwrap().metadata().unwrap_or_default();
    for (key, value) in metadata {
        println!("Metadata - {}: {}", key, value);
    }

    Ok(())
}

async fn peek_and_receive_messages(
    queue_client: &QueueClient,
) -> Result<(), Box<dyn std::error::Error>> {
    _ = queue_client
        .enqueue_message("Message 1 from Rust Queue SDK", None)
        .await;
    _ = queue_client
        .enqueue_message("Message 2 from Rust Queue SDK", None)
        .await;

    let options = QueueClientPeekOptions {
        number_of_messages: Some(5),
        ..Default::default()
    };

    let result = queue_client.peek_messages(Some(options)).await;
    log_operation_result(&result, "peek_messages");

    if let Ok(response) = result {
        let messages = response.into_body().await?;
        if let Some(messages) = messages.items {
            for msg in messages {
                println!(
                    "Successfully peeked message ({}): {}",
                    msg.message_id.unwrap(),
                    msg.message_text.unwrap_or_default()
                );
            }
        }
    }

    let options = QueueClientDequeueOptions {
        number_of_messages: Some(5),
        ..Default::default()
    };

    let result = queue_client.dequeue_messages(Some(options)).await;
    log_operation_result(&result, "dequeue_messages");

    if let Ok(response) = result {
        let messages = response.into_body().await?;
        if let Some(messages) = messages.items {
            for msg in messages {
                println!(
                    "Successfully received message ({}): {}",
                    msg.message_id.unwrap(),
                    msg.message_text.unwrap_or_default()
                );
            }
        }
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let credential = DefaultAzureCredential::new()?;

    // Retrieve the storage account endpoint from environment variable.
    let endpoint = get_endpoint();

    let queue_name = get_random_queue_name();
    let queue_client = QueueClient::new(&endpoint, &queue_name, credential.clone(), None)?;

    // Create and manage queue
    let result = queue_client.create(None).await;
    log_operation_result(&result, "create");

    let result = queue_client.exists().await;
    log_operation_result(&result, "check_exists");

    let result = queue_client.create_if_not_exists(None).await;
    log_operation_result(&result, "create_if_not_exists");

    // Set and get queue metadata
    set_and_get_metadata(&queue_client).await?;

    let result = queue_client.enqueue_message("Example Message", None).await;
    log_operation_result(&result, "enqueue_message");

    send_and_update_message(
        &queue_client,
        "Example message created from Rust, ready for update",
    )
    .await?;

    // Clear messages
    let result = queue_client.clear(None).await;
    log_operation_result(&result, "clear");

    // Send and process messages
    send_and_delete_message(
        &queue_client,
        "Example message created from Rust, ready for deletion",
    )
    .await?;

    // Receive messages
    peek_and_receive_messages(&queue_client).await?;

    // Set and get access policies
    set_and_get_access_policies(&queue_client).await?;

    // Cleanup
    let result = queue_client.delete(None).await;
    log_operation_result(&result, "delete");

    let non_existing_queue_client =
        QueueClient::new(&endpoint, "non-existent-queue", credential.clone(), None)?;
    let result = non_existing_queue_client.exists().await;
    log_operation_result(&result, "check_non_existent");

    let result = non_existing_queue_client.delete_if_exists(None).await;
    log_operation_result(&result, "delete_if_exists");

    Ok(())
}

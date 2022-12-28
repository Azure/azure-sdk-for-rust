//! This mod contains long running tests that are not run by default.
//!
//! This test may be used to test sender recovery after a network interruption.

use azure_messaging_servicebus::{ServiceBusReceivedMessage, ServiceBusReceiver, ServiceBusSender};

mod common;

async fn send_one_message_per_minute(
    mut sender: ServiceBusSender,
    total: usize,
) -> Result<(), anyhow::Error> {
    for i in 0..total {
        let message = format!("message {}", i);
        sender.send_message(message).await?;
        println!("sent message {}", i);
        tokio::time::sleep(std::time::Duration::from_secs(60)).await;
    }

    // Remember to dispose the sender
    sender.dispose().await?;
    Ok(())
}

async fn receive_and_complete_once_per_minute(
    mut receiver: ServiceBusReceiver,
    total: usize,
    max_wait_time: Option<std::time::Duration>,
) -> Result<Vec<ServiceBusReceivedMessage>, anyhow::Error> {
    let mut total_received = 0;
    let mut received = Vec::new();
    while total_received < total {
        let received_batch = receiver
            .receive_messages_with_max_wait_time(1, max_wait_time)
            .await?;

        // Complete all messages
        for message in &received_batch {
            receiver.complete_message(message).await?;
            // receiver.abandon_message(message, None).await?;
        }

        total_received += received_batch.len();
        received.extend(received_batch);
        tokio::time::sleep(std::time::Duration::from_secs(60)).await;
    }

    // Remember to dispose the receiver
    receiver.dispose().await?;
    Ok(received)
}

#[tokio::test]
#[ignore]
async fn send_to_queue_every_minute_for_two_hour() {
    // Run this test with:
    //
    // ```sh
    // cargo test --test long_tests -- --ignored --exact --nocapture
    // ```

    common::setup_dotenv();

    let connection_string = std::env::var("SERVICE_BUS_CONNECTION_STRING").unwrap();
    let queue_name = std::env::var("SERVICE_BUS_QUEUE").unwrap();

    let mut client = azure_messaging_servicebus::client::ServiceBusClient::new(
        &connection_string,
        Default::default(),
    )
    .await
    .unwrap();

    let sender = client
        .create_sender(&queue_name, Default::default())
        .await
        .unwrap();
    let receiver = client
        .create_receiver_for_queue(&queue_name, Default::default())
        .await
        .unwrap();

    let total = 120;

    let sender_handle = tokio::spawn(send_one_message_per_minute(sender, total));
    let receiver_handle = tokio::spawn(receive_and_complete_once_per_minute(
        receiver,
        total,
        Some(std::time::Duration::from_secs(30)),
    ));

    let result = tokio::time::timeout(std::time::Duration::from_secs(60 * 60 * 2), async {
        // Sender task should finish first
        let sender_result = sender_handle.await.unwrap();
        let receiver_result = receiver_handle.await.unwrap();
        match (sender_result, receiver_result) {
            (Ok(_), Ok(received)) => Ok(received),
            (Err(e), _) => Err(e),
            (_, Err(e)) => Err(e),
        }
    })
    .await
    .unwrap();
    let total_received = result.unwrap().len();
    assert_eq!(total_received, total);

    client.dispose().await.unwrap();
}

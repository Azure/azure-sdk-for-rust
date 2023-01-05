//! This mod contains long running tests that are not run by default.
//!
//! This test may be used to test sender recovery after a network interruption.

#[macro_use]
mod macros;

cfg_not_wasm32! {
    use azure_messaging_servicebus::{ServiceBusReceivedMessage, ServiceBusReceiver, ServiceBusSender};

    mod common;

    async fn send_one_message_per_minute(
        mut sender: ServiceBusSender,
        total: usize,
    ) -> Result<(), anyhow::Error> {
        for i in 0..total {
            let message = format!("message {}", i);
            println!("sending message {}", i);
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
    ) -> Result<Vec<ServiceBusReceivedMessage>, anyhow::Error> {
        let mut total_received = 0;
        let mut received = Vec::new();
        while total_received < total {
            let message = receiver.receive_message().await?;
            receiver.complete_message(&message).await?;

            println!(
                "received message {}",
                std::str::from_utf8(message.body().unwrap()).unwrap()
            );

            total_received += 1;
            received.push(message);
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
        // cargo test --test long_tests -- --ignored send_to_queue_every_minute_for_two_hour --exact --nocapture
        // ```

        use azure_messaging_servicebus::{ServiceBusClient};

        common::setup_dotenv();

        let connection_string = std::env::var("SERVICE_BUS_CONNECTION_STRING").unwrap();
        let queue_name = std::env::var("SERVICE_BUS_QUEUE").unwrap();

        let mut client = ServiceBusClient::new(
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
        let receiver_handle = tokio::spawn(receive_and_complete_once_per_minute(receiver, total));

        // Wait roughly 50% longer than the total time in case of network interruptions
        let duration = std::time::Duration::from_secs(60 * total as u64 * 3 / 2);
        let result = tokio::time::timeout(duration, async {
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

        // remove duplicates
        let messages = result.unwrap();
        let messages = messages
            .into_iter()
            .map(|m| m.body().unwrap().to_vec())
            .collect::<std::collections::HashSet<_>>();

        let total_received = messages.len();
        assert_eq!(total_received, total);

        client.dispose().await.unwrap();
    }
}

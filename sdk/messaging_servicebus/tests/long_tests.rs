//! This mod contains long running tests that are not run by default.

use serial_test::serial;

mod common;

#[tokio::test]
#[ignore]
#[serial]
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

    let mut sender = client
        .create_sender(&queue_name, Default::default())
        .await
        .unwrap();

    let total = 120;
    for i in 0..total {
        let message = format!("message {}", i);
        sender.send_message(message).await.unwrap();
        println!("sent message {}", i);
        tokio::time::sleep(std::time::Duration::from_secs(60)).await;
    }

    let receive_all = || async move {
        let mut total_received = 0;
        while total_received < total {
            let received = common::create_client_and_receive_messages_from_queue(
                &connection_string,
                Default::default(),
                &queue_name,
                Default::default(),
                total,
                None,
            )
            .await
            .unwrap();

            total_received += received.len() as u32;
        }
        total_received
    };

    let result = tokio::time::timeout(std::time::Duration::from_secs(60 * 10), receive_all()).await;
    assert_eq!(result, Ok(total));

    sender.dispose().await.unwrap();
    client.dispose().await.unwrap();
}

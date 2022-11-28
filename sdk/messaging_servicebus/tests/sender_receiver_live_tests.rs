mod common;
use azure_messaging_servicebus::{
    client::{
        service_bus_client::ServiceBusClient, service_bus_client_options::ServiceBusClientOptions,
    },
    ServiceBusMessage,
};
use common::setup_dotenv;

#[tokio::test]
async fn drain_queue() {
    setup_dotenv();
    let connection_string = std::env::var("SERVICE_BUS_CONNECTION_STRING").unwrap();
    let queue_name = std::env::var("SERVICE_BUS_QUEUE").unwrap();
    let max_messages = 100;

    common::drain_queue(
        connection_string,
        Default::default(),
        queue_name,
        Default::default(),
        max_messages,
    )
    .await;
}

#[tokio::test]
async fn send_and_receive_one_message_on_queue() {
    setup_dotenv();
    let connection_string = std::env::var("SERVICE_BUS_CONNECTION_STRING").unwrap();
    let queue_name = std::env::var("SERVICE_BUS_QUEUE").unwrap();

    let message = ServiceBusMessage::new("test message");
    let messages = std::iter::once(message);
    let total = messages.len();

    common::create_client_and_send_messages_separately_to_queue(
        connection_string.clone(),
        Default::default(),
        queue_name.clone(),
        Default::default(),
        messages,
    )
    .await
    .unwrap();

    let received = common::create_client_and_receive_messages_from_queue(
        connection_string,
        Default::default(),
        queue_name,
        Default::default(),
        total as u32,
        None,
    )
    .await
    .unwrap();

    assert_eq!(received.len(), total);
    let received_message_body = received[0].body().unwrap();
    assert_eq!(received_message_body, b"test message");
}

#[tokio::test]
async fn send_one_message_and_try_receiver_more_than_one_on_queue() {
    setup_dotenv();
    let connection_string = std::env::var("SERVICE_BUS_CONNECTION_STRING").unwrap();
    let queue_name = std::env::var("SERVICE_BUS_QUEUE").unwrap();

    let message = ServiceBusMessage::new("test message");
    let messages = std::iter::once(message);
    let total = messages.len();

    common::create_client_and_send_messages_separately_to_queue(
        connection_string.clone(),
        Default::default(),
        queue_name.clone(),
        Default::default(),
        messages,
    )
    .await
    .unwrap();

    let mut receiver_client_options = ServiceBusClientOptions::default();
    receiver_client_options.retry_options = common::zero_retry_options();

    let received = common::create_client_and_receive_messages_from_queue(
        connection_string,
        receiver_client_options,
        queue_name,
        Default::default(),
        total as u32 + 1,
        None,
    )
    .await
    .unwrap();

    // Please note that if the test queue's messsage lock duration is shorter than the max retry
    // delay, the same message may be received more than once.
    assert_eq!(received.len(), total);
    let received_message_body = received[0].body().unwrap();
    assert_eq!(received_message_body, b"test message");
}

#[tokio::test]
async fn send_and_receive_multiple_messages_separately() {
    setup_dotenv();
    let connection_string = std::env::var("SERVICE_BUS_CONNECTION_STRING").unwrap();
    let queue_name = std::env::var("SERVICE_BUS_QUEUE").unwrap();

    let expected = ["test message 1", "test message 2", "test message 3"];
    let messages = vec![
        ServiceBusMessage::new(expected[0]),
        ServiceBusMessage::new(expected[1]),
        ServiceBusMessage::new(expected[2]),
    ];
    let total = messages.len();

    common::create_client_and_send_messages_separately_to_queue(
        connection_string.clone(),
        Default::default(),
        queue_name.clone(),
        Default::default(),
        messages.into_iter(),
    )
    .await
    .unwrap();

    let received = common::create_client_and_receive_messages_from_queue(
        connection_string,
        Default::default(),
        queue_name,
        Default::default(),
        total as u32,
        None,
    )
    .await
    .unwrap();

    assert_eq!(received.len(), total);
    for i in 0..total {
        let received_message_body = received[i].body().unwrap();
        assert_eq!(received_message_body, expected[i].as_bytes());
    }
}

#[tokio::test]
async fn send_messagebatch_and_try_receive_messages_of_the_same_amount() {
    setup_dotenv();
    let connection_string = std::env::var("SERVICE_BUS_CONNECTION_STRING").unwrap();
    let queue_name = std::env::var("SERVICE_BUS_QUEUE").unwrap();

    let expected = ["test message 1", "test message 2", "test message 3"];

    let mut client = ServiceBusClient::new_with_options(&connection_string, Default::default())
        .await
        .unwrap();
    let mut sender = client
        .create_sender(queue_name.clone(), Default::default())
        .await
        .unwrap();
    let mut message_batch = sender.create_message_batch(Default::default()).unwrap();

    let total = expected.len();
    for message in expected {
        message_batch.try_add_message(message).unwrap();
    }
    sender.send_message_batch(message_batch).await.unwrap();

    let received = common::create_client_and_receive_messages_from_queue(
        connection_string,
        Default::default(),
        queue_name,
        Default::default(),
        total as u32,
        None,
    )
    .await
    .unwrap();

    assert_eq!(received.len(), total);
    for i in 0..total {
        let received_message_body = received[i].body().unwrap();
        assert_eq!(received_message_body, expected[i].as_bytes());
    }
}

#[tokio::test]
async fn send_messagebatch_and_try_receive_more_than_sent() {
    setup_dotenv();
    let connection_string = std::env::var("SERVICE_BUS_CONNECTION_STRING").unwrap();
    let queue_name = std::env::var("SERVICE_BUS_QUEUE").unwrap();

    let expected = ["test message 1", "test message 2", "test message 3"];

    let mut client = ServiceBusClient::new_with_options(&connection_string, Default::default())
        .await
        .unwrap();
    let mut sender = client
        .create_sender(queue_name.clone(), Default::default())
        .await
        .unwrap();
    let mut message_batch = sender.create_message_batch(Default::default()).unwrap();

    let total = expected.len();
    for message in expected {
        message_batch.try_add_message(message).unwrap();
    }
    sender.send_message_batch(message_batch).await.unwrap();

    let mut receiving_client_options = ServiceBusClientOptions::default();
    receiving_client_options.retry_options = common::zero_retry_options();

    let received = common::create_client_and_receive_messages_from_queue(
        connection_string,
        receiving_client_options,
        queue_name,
        Default::default(),
        total as u32 + 1,
        None,
    )
    .await
    .unwrap();

    // Please note that if the test queue's messsage lock duration is shorter than the max retry
    // delay, the same message may be received more than once.
    assert_eq!(received.len(), total);
    for i in 0..total {
        let received_message_body = received[i].body().unwrap();
        assert_eq!(received_message_body, expected[i].as_bytes());
    }
}

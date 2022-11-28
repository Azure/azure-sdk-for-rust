mod common;
use azure_messaging_servicebus::{
    client::{
        service_bus_client::ServiceBusClient, service_bus_client_options::ServiceBusClientOptions,
    },
    ServiceBusMessage, ServiceBusReceiverOptions, primitives::sub_queue::SubQueue,
};
use common::setup_dotenv;

#[tokio::test]
async fn drain_queue() {
    setup_dotenv();
    let connection_string = std::env::var("SERVICE_BUS_CONNECTION_STRING").unwrap();
    let queue_name = std::env::var("SERVICE_BUS_QUEUE").unwrap();
    let max_messages = 100;

    let mut client_options = ServiceBusClientOptions::default();
    client_options.retry_options = common::zero_retry_options();
    common::drain_queue(
        connection_string,
        client_options,
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
async fn send_and_receive_multiple_messages_separately_with_prefetch() {
    setup_dotenv();
    let connection_string = std::env::var("SERVICE_BUS_CONNECTION_STRING").unwrap();
    let queue_name = std::env::var("SERVICE_BUS_QUEUE").unwrap();

    let expected = ["test message 1", "test message 2", "test message 3"];
    let messages = vec![
        ServiceBusMessage::new(expected[0]),
        ServiceBusMessage::new(expected[1]),
        ServiceBusMessage::new(expected[2]),
    ];
    let max_messages = messages.len() as u32;

    common::create_client_and_send_messages_separately_to_queue(
        connection_string.clone(),
        Default::default(),
        queue_name.clone(),
        Default::default(),
        messages.into_iter(),
    )
    .await
    .unwrap();

    let mut receiver_options = ServiceBusReceiverOptions::default();
    receiver_options.prefetch_count = max_messages;
    common::create_client_and_receive_messages_from_queue(
        connection_string,
        Default::default(),
        queue_name,
        receiver_options,
        max_messages,
        None,
    )
    .await
    .unwrap();
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

#[tokio::test]
async fn send_and_receive_sessionful_messages() {
    setup_dotenv();
    let connection_string = std::env::var("SERVICE_BUS_CONNECTION_STRING").unwrap();
    let queue_name = std::env::var("SERVICE_BUS_SESSION_QUEUE").unwrap();

    let expected_for_session_id_1 = ["test message 1", "test message 2", "test message 3"];
    let expected_for_session_id_2 = ["test message 4", "test message 5", "test message 6"];
    let session_id_1 = "test_session_1";
    let session_id_2 = "test_session_2";

    let connection_string_clone = connection_string.clone();
    let queue_name_clone = queue_name.clone();
    let handle_1 = tokio::spawn(async move {
        common::create_client_and_receive_sessionful_messages_from_queue(
            connection_string_clone,
            Default::default(),
            queue_name_clone,
            Default::default(),
            session_id_1.to_string(),
            expected_for_session_id_1.len() as u32,
            None,
        )
        .await
    });

    let connection_string_clone = connection_string.clone();
    let queue_name_clone = queue_name.clone();
    let handle_2 = tokio::spawn(async move {
        common::create_client_and_receive_sessionful_messages_from_queue(
            connection_string_clone,
            Default::default(),
            queue_name_clone,
            Default::default(),
            session_id_2.to_string(),
            expected_for_session_id_2.len() as u32,
            None,
        )
        .await
    });

    // Send 2nd session id first
    let messages = expected_for_session_id_2.iter().map(|message| {
        let mut message = ServiceBusMessage::new(message.as_bytes());
        message.set_session_id(session_id_2).unwrap();
        message
    });
    common::create_client_and_send_messages_separately_to_queue(
        connection_string.clone(),
        Default::default(),
        queue_name.clone(),
        Default::default(),
        messages,
    )
    .await
    .unwrap();

    // Send 1st session id last
    let messages = expected_for_session_id_1.iter().map(|message| {
        let mut message = ServiceBusMessage::new(*message);
        message.set_session_id(session_id_1).unwrap(); // length must not exceed max length
        message
    });
    common::create_client_and_send_messages_separately_to_queue(
        connection_string,
        Default::default(),
        queue_name,
        Default::default(),
        messages,
    )
    .await
    .unwrap();

    let received_from_session_1 = handle_1.await.unwrap().unwrap();
    let received_from_session_2 = handle_2.await.unwrap().unwrap();

    assert_eq!(
        received_from_session_1.len(),
        expected_for_session_id_1.len()
    );
    for i in 0..expected_for_session_id_1.len() {
        let received_message_body = received_from_session_1[i].body().unwrap();
        assert_eq!(
            received_message_body,
            expected_for_session_id_1[i].as_bytes()
        );
    }

    assert_eq!(
        received_from_session_2.len(),
        expected_for_session_id_2.len()
    );
    for i in 0..expected_for_session_id_2.len() {
        let received_message_body = received_from_session_2[i].body().unwrap();
        assert_eq!(
            received_message_body,
            expected_for_session_id_2[i].as_bytes()
        );
    }
}

#[tokio::test]
async fn send_and_abandon_messages_then_receive_messages() {
    setup_dotenv();
    let connection_string = std::env::var("SERVICE_BUS_CONNECTION_STRING").unwrap();
    let queue_name = std::env::var("SERVICE_BUS_QUEUE").unwrap();

    let expected = ["test message 1", "test message 2", "test message 3"];
    let messages = expected
        .iter()
        .map(|message| ServiceBusMessage::new(*message));

    common::create_client_and_send_messages_separately_to_queue(
        connection_string.clone(),
        Default::default(),
        queue_name.clone(),
        Default::default(),
        messages,
    )
    .await
    .unwrap();

    common::create_client_and_abandon_messages_from_queue(
        connection_string.clone(),
        Default::default(),
        queue_name.clone(),
        Default::default(),
        expected.len() as u32,
        None,
    )
    .await
    .unwrap();

    let received = common::create_client_and_receive_messages_from_queue(
        connection_string,
        Default::default(),
        queue_name,
        Default::default(),
        expected.len() as u32,
        None,
    )
    .await
    .unwrap();

    assert_eq!(received.len(), expected.len());
    for i in 0..expected.len() {
        let received_message_body = received[i].body().unwrap();
        assert_eq!(received_message_body, expected[i].as_bytes());
    }
}

#[tokio::test]
async fn send_and_deadletter_then_receive_from_deadletter_queue() {
    setup_dotenv();
    let connection_string = std::env::var("SERVICE_BUS_CONNECTION_STRING").unwrap();
    let queue_name = std::env::var("SERVICE_BUS_QUEUE").unwrap();

    let expected = ["test message 1", "test message 2", "test message 3"];
    let messages = expected
        .iter()
        .map(|message| ServiceBusMessage::new(*message));

    common::create_client_and_send_messages_separately_to_queue(
        connection_string.clone(),
        Default::default(),
        queue_name.clone(),
        Default::default(),
        messages,
    )
    .await
    .unwrap();

    common::create_client_and_deadletter_messages_from_queue(
        connection_string.clone(),
        Default::default(),
        queue_name.clone(),
        Default::default(),
        expected.len() as u32,
        None,
    )
    .await
    .unwrap();

    let mut receiver_options = ServiceBusReceiverOptions::default();
    receiver_options.sub_queue = SubQueue::DeadLetter;
    let received = common::create_client_and_receive_messages_from_queue(
        connection_string,
        Default::default(),
        queue_name,
        receiver_options,
        expected.len() as u32,
        None,
    )
    .await
    .unwrap();

    assert_eq!(received.len(), expected.len());
    for i in 0..expected.len() {
        let received_message_body = received[i].body().unwrap();
        assert_eq!(received_message_body, expected[i].as_bytes());
    }
}

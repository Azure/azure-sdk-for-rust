use std::env;
use std::time::Duration as StdDuration;
use time::Duration as TimeSpan;

use azure_messaging_servicebus::{
    client::service_bus_client::ServiceBusClient,
    primitives::{service_bus_peeked_message::ServiceBusPeekedMessage, sub_queue::SubQueue},
    receiver::service_bus_session_receiver::ServiceBusSessionReceiverOptions,
    ServiceBusMessage, ServiceBusReceiverOptions, ServiceBusSenderOptions,
};

mod common;

use common::setup_dotenv;

async fn client_send_single_message(options: ServiceBusSenderOptions) {
    setup_dotenv();
    let connection_string = env::var("SERVICE_BUS_CONNECTION_STRING").unwrap();
    let queue = env::var("SERVICE_BUS_QUEUE").unwrap();

    let mut client = ServiceBusClient::new(connection_string, Default::default()).await.unwrap();
    let mut sender = client.create_sender(queue, options).await.unwrap();
    sender.send_message("hello world").await.unwrap();

    sender.dispose().await.unwrap();
    client.dispose().await.unwrap();
}

async fn client_send_multiple_messages(total: u32, options: ServiceBusSenderOptions) {
    setup_dotenv();
    let connection_string = env::var("SERVICE_BUS_CONNECTION_STRING").unwrap();
    let queue = env::var("SERVICE_BUS_QUEUE").unwrap();

    let mut client = ServiceBusClient::new(connection_string, Default::default()).await.unwrap();
    let mut sender = client.create_sender(queue, options).await.unwrap();

    let messages = (0..total)
        .map(|i| format!("message {}", i).into_bytes())
        .map(ServiceBusMessage::from)
        .collect::<Vec<ServiceBusMessage>>();

    sender.send_messages(messages).await.unwrap();

    sender.dispose().await.unwrap();
    client.dispose().await.unwrap();
}

async fn client_send_message_batch(total: u32, options: ServiceBusSenderOptions) {
    setup_dotenv();
    let connection_string = env::var("SERVICE_BUS_CONNECTION_STRING").unwrap();
    let queue = env::var("SERVICE_BUS_QUEUE").unwrap();

    let mut client = ServiceBusClient::new(connection_string, Default::default()).await.unwrap();
    let mut sender = client.create_sender(queue, options).await.unwrap();

    let mut batch = sender
        .create_message_batch(Default::default())
        .unwrap();
    (0..total)
        .map(|i| format!("message {}", i).into_bytes())
        .for_each(|m| {
            batch.try_add_message(ServiceBusMessage::from(m)).unwrap();
        });
    sender.send_message_batch(batch).await.unwrap();

    sender.dispose().await.unwrap();
    client.dispose().await.unwrap();
}

async fn client_receive_messages(total: u32, options: ServiceBusReceiverOptions) {
    setup_dotenv();
    let connection_string = env::var("SERVICE_BUS_CONNECTION_STRING").unwrap();
    let queue = env::var("SERVICE_BUS_QUEUE").unwrap();

    let mut client = ServiceBusClient::new(connection_string, Default::default()).await.unwrap();
    let mut receiver = client
        .create_receiver_for_queue(queue, options)
        .await
        .unwrap();
    let messages = receiver.receive_messages(total, None).await.unwrap();

    assert_eq!(messages.len(), total as usize);

    for message in messages {
        receiver.complete_message(&message).await.unwrap();
    }

    receiver.dispose().await.unwrap();
    client.dispose().await.unwrap();
}

async fn client_recv_from_session(
    total: u32,
    options: ServiceBusSessionReceiverOptions,
    session_id: String,
) {
    setup_dotenv();
    let connection_string = env::var("SERVICE_BUS_CONNECTION_STRING").unwrap();
    let session_enabled_queue = env::var("SERVICE_BUS_SESSION_QUEUE").unwrap();

    let mut client = ServiceBusClient::new(connection_string, Default::default()).await.unwrap();
    let mut receiver = client
        .accept_next_session_for_queue(session_enabled_queue, session_id, options)
        .await
        .unwrap();
    let messages = receiver.receive_messages(total, None).await.unwrap();

    assert_eq!(messages.len(), total as usize);

    for message in messages {
        receiver.complete_message(&message).await.unwrap();
    }

    receiver.dispose().await.unwrap();
    client.dispose().await.unwrap();
}

async fn client_send_to_session(total: u32, options: ServiceBusSenderOptions, session_id: String) {
    setup_dotenv();
    let connection_string = env::var("SERVICE_BUS_CONNECTION_STRING").unwrap();
    let session_enabled_queue = env::var("SERVICE_BUS_SESSION_QUEUE").unwrap();

    let mut client = ServiceBusClient::new(connection_string, Default::default()).await.unwrap();
    let mut sender = client
        .create_sender(session_enabled_queue, options)
        .await
        .unwrap();
    let messages: Vec<ServiceBusMessage> = (0..total)
        .map(|i| {
            let mut message = ServiceBusMessage::from(format!("message {}", i));
            message.set_session_id(session_id.clone());
            message
        })
        .collect();

    sender.send_messages(messages).await.unwrap();

    sender.dispose().await.unwrap();
    client.dispose().await.unwrap();
}

async fn abandon_one_message() {
    setup_dotenv();
    let connection_string = env::var("SERVICE_BUS_CONNECTION_STRING").unwrap();
    let queue = env::var("SERVICE_BUS_QUEUE").unwrap();

    let mut client = ServiceBusClient::new(connection_string, Default::default()).await.unwrap();
    let mut receiver = client
        .create_receiver_for_queue(queue, Default::default())
        .await
        .unwrap();

    let message = receiver.receive_message().await.unwrap();

    if let Some(message) = message {
        receiver.abandon_message(&message, None).await.unwrap();
    }

    receiver.dispose().await.unwrap();
    client.dispose().await.unwrap();
}

async fn dead_letter_one_message() {
    setup_dotenv();
    let connection_string = env::var("SERVICE_BUS_CONNECTION_STRING").unwrap();
    let queue = env::var("SERVICE_BUS_QUEUE").unwrap();

    let mut client = ServiceBusClient::new(connection_string, Default::default()).await.unwrap();
    let mut receiver = client
        .create_receiver_for_queue(queue, Default::default())
        .await
        .unwrap();

    let message = receiver.receive_message().await.unwrap();

    if let Some(message) = message {
        receiver
            .dead_letter_message(&message, None, None, None)
            .await
            .unwrap();
    }

    receiver.dispose().await.unwrap();
    client.dispose().await.unwrap();
}

async fn recv_from_dead_letter_queue() -> usize {
    setup_dotenv();
    let connection_string = env::var("SERVICE_BUS_CONNECTION_STRING").unwrap();
    let queue = env::var("SERVICE_BUS_QUEUE").unwrap();

    let mut client = ServiceBusClient::new(connection_string, Default::default()).await.unwrap();
    let options = ServiceBusReceiverOptions {
        sub_queue: SubQueue::DeadLetter,
        ..Default::default()
    };
    let mut receiver = client
        .create_receiver_for_queue(queue, options)
        .await
        .unwrap();

    let message = receiver.receive_message().await.unwrap();

    let count = if let Some(message) = message {
        receiver.complete_message(&message).await.unwrap();
        1
    } else {
        0
    };

    receiver.dispose().await.unwrap();
    client.dispose().await.unwrap();

    count
}

async fn client_schedule_single_message_via_service_bus_sender(delay: TimeSpan) -> i64 {
    use time::OffsetDateTime;

    setup_dotenv();

    let connection_string = env::var("SERVICE_BUS_CONNECTION_STRING").unwrap();
    let queue = env::var("SERVICE_BUS_QUEUE").unwrap();

    let mut client = ServiceBusClient::new(connection_string, Default::default()).await.unwrap();
    let mut sender = client
        .create_sender(queue, Default::default())
        .await
        .unwrap();

    let message = ServiceBusMessage::from("hello world");
    let enqueue_time = OffsetDateTime::now_utc() + delay;
    let seq = sender
        .schedule_message(message, enqueue_time)
        .await
        .unwrap();

    sender.dispose().await.unwrap();
    client.dispose().await.unwrap();

    seq
}

async fn client_cancel_single_scheduled_message(seq: i64) {
    setup_dotenv();

    let connection_string = env::var("SERVICE_BUS_CONNECTION_STRING").unwrap();
    let queue = env::var("SERVICE_BUS_QUEUE").unwrap();

    let mut client = ServiceBusClient::new(connection_string, Default::default()).await.unwrap();
    let mut sender = client
        .create_sender(queue, Default::default())
        .await
        .unwrap();

    sender.cancel_scheduled_message(seq).await.unwrap();

    sender.dispose().await.unwrap();
    client.dispose().await.unwrap();
}

async fn peek_one_message() -> Option<ServiceBusPeekedMessage> {
    setup_dotenv();

    let connection_string = env::var("SERVICE_BUS_CONNECTION_STRING").unwrap();
    let queue = env::var("SERVICE_BUS_QUEUE").unwrap();

    let mut client = ServiceBusClient::new(connection_string, Default::default()).await.unwrap();
    let mut receiver = client
        .create_receiver_for_queue(queue, Default::default())
        .await
        .unwrap();

    let message = receiver.peek_message(None).await.unwrap();

    receiver.dispose().await.unwrap();
    client.dispose().await.unwrap();

    message
}

async fn defer_one_message() -> i64 {
    setup_dotenv();

    let connection_string = env::var("SERVICE_BUS_CONNECTION_STRING").unwrap();
    let queue = env::var("SERVICE_BUS_QUEUE").unwrap();

    let mut client = ServiceBusClient::new(connection_string, Default::default()).await.unwrap();
    let mut receiver = client
        .create_receiver_for_queue(queue, Default::default())
        .await
        .unwrap();

    let message = receiver.receive_message().await.unwrap().unwrap();
    let seq = message.sequence_number();
    receiver.defer_message(&message, None).await.unwrap();

    receiver.dispose().await.unwrap();
    client.dispose().await.unwrap();

    seq
}

async fn receive_one_deferred_message(seq: i64) {
    setup_dotenv();

    let connection_string = env::var("SERVICE_BUS_CONNECTION_STRING").unwrap();
    let queue = env::var("SERVICE_BUS_QUEUE").unwrap();

    let mut client = ServiceBusClient::new(connection_string, Default::default()).await.unwrap();
    let mut receiver = client
        .create_receiver_for_queue(queue, Default::default())
        .await
        .unwrap();

    let message = receiver
        .receive_deferred_message(seq)
        .await
        .unwrap()
        .unwrap();
    receiver.complete_message(&message).await.unwrap();

    receiver.dispose().await.unwrap();
    client.dispose().await.unwrap();
}

async fn receive_then_renew_lock() {
    setup_dotenv();

    let connection_string = env::var("SERVICE_BUS_CONNECTION_STRING").unwrap();
    let queue = env::var("SERVICE_BUS_QUEUE").unwrap();

    let mut client = ServiceBusClient::new(connection_string, Default::default()).await.unwrap();
    let mut receiver = client
        .create_receiver_for_queue(queue, Default::default())
        .await
        .unwrap();

    let mut message = receiver.receive_message().await.unwrap().unwrap();

    println!("Received message: {:?}", message);

    receiver.renew_message_lock(&mut message).await.unwrap();

    receiver.complete_message(&message).await.unwrap();

    receiver.dispose().await.unwrap();
    client.dispose().await.unwrap();
}

#[test]
fn hello_world() {
    setup_dotenv();
    assert_eq!(env::var("HELLO").unwrap(), "hello");
}

#[tokio::test]
async fn client_can_connect_with_connection_string() {
    setup_dotenv();
    let connection_string = env::var("SERVICE_BUS_CONNECTION_STRING").unwrap();
    let mut client = ServiceBusClient::new(connection_string, Default::default()).await.unwrap();

    client.dispose().await.unwrap();
}

#[tokio::test]
async fn client_can_create_sender() {
    setup_dotenv();
    let connection_string = env::var("SERVICE_BUS_CONNECTION_STRING").unwrap();
    let queue = env::var("SERVICE_BUS_QUEUE").unwrap();

    let mut client = ServiceBusClient::new(connection_string, Default::default()).await.unwrap();
    let sender = client
        .create_sender(queue, Default::default())
        .await
        .unwrap();

    sender.dispose().await.unwrap();
    client.dispose().await.unwrap();
}

#[tokio::test]
async fn client_can_create_receiver() {
    setup_dotenv();
    let connection_string = env::var("SERVICE_BUS_CONNECTION_STRING").unwrap();
    let queue = env::var("SERVICE_BUS_QUEUE").unwrap();

    let mut client = ServiceBusClient::new(connection_string, Default::default()).await.unwrap();
    let receiver = client
        .create_receiver_for_queue(queue, Default::default())
        .await
        .unwrap();

    receiver.dispose().await.unwrap();
    client.dispose().await.unwrap();
}

#[tokio::test]
async fn client_send_and_receive_single_message_with_default_options() {
    client_send_single_message(Default::default()).await;
    client_receive_messages(1, Default::default()).await;
}

#[tokio::test]
async fn client_send_and_receive_multiple_messages_with_default_options() {
    let total = 3;
    client_send_multiple_messages(total, Default::default()).await;
    client_receive_messages(total, Default::default()).await;
}

#[tokio::test]
async fn client_send_message_batch_and_receive_messages_with_default_options() {
    let total = 3;
    client_send_message_batch(total, Default::default()).await;
    client_receive_messages(total - 1, Default::default()).await;
    client_receive_messages(1, Default::default()).await;
}

#[tokio::test]
async fn client_can_create_session_receiver() {
    client_recv_from_session(0, Default::default(), "session_id".to_string()).await;
}

#[tokio::test]
async fn test_send_and_receive_sessionful_messages() {
    let total = 3;
    let session_id = "session_id".to_string();

    client_send_to_session(total, Default::default(), session_id.clone()).await;
    client_recv_from_session(total, Default::default(), session_id).await;
}

#[tokio::test]
async fn client_schedule_message_via_service_bus_message() {
    use time::OffsetDateTime;

    setup_dotenv();

    let mut message = ServiceBusMessage::from("hello world");
    let enqueue_time = OffsetDateTime::now_utc() + TimeSpan::seconds(20);
    message.set_scheduled_enqueue_time(enqueue_time);

    let connection_string = env::var("SERVICE_BUS_CONNECTION_STRING").unwrap();
    let queue = env::var("SERVICE_BUS_QUEUE").unwrap();

    let mut client = ServiceBusClient::new(connection_string, Default::default()).await.unwrap();
    let mut sender = client
        .create_sender(queue, Default::default())
        .await
        .unwrap();

    sender.send_message(message).await.unwrap();

    sender.dispose().await.unwrap();
    client.dispose().await.unwrap();

    tokio::time::sleep(std::time::Duration::from_secs(30)).await;
    client_receive_messages(1, Default::default()).await;
}

#[tokio::test]
async fn client_schedule_and_cancel_single_message() {
    // TODO: remove sleep?
    let seq = client_schedule_single_message_via_service_bus_sender(TimeSpan::minutes(1)).await;
    tokio::time::sleep(StdDuration::from_secs(30)).await;
    client_cancel_single_scheduled_message(seq).await;
}

#[tokio::test]
#[should_panic]
async fn client_cancel_non_existent_schedule_message() {
    let seq = 1; // The queue should be well surpassed this number by now
    client_cancel_single_scheduled_message(seq).await;
}

#[tokio::test]
async fn test_abancon_message() {
    client_send_single_message(Default::default()).await;
    abandon_one_message().await;
    client_receive_messages(1, Default::default()).await;
}

#[tokio::test]
async fn test_dead_letter_message() {
    client_send_single_message(Default::default()).await;
    dead_letter_one_message().await;
    let count = recv_from_dead_letter_queue().await;
    assert_eq!(count, 1);
}

#[tokio::test]
async fn test_send_and_peek_one_message() {
    client_send_single_message(Default::default()).await;
    let peeked = peek_one_message().await;
    assert!(peeked.is_some());
}

#[tokio::test]
async fn test_send_and_defer_one_message() {
    client_send_single_message(Default::default()).await;
    let _seq = defer_one_message().await;
}

#[tokio::test]
async fn test_receive_deferred_message() {
    client_send_single_message(Default::default()).await;
    let seq = defer_one_message().await;
    println!("seq: {}", seq);
    receive_one_deferred_message(seq).await;
}

#[tokio::test]
async fn test_renew_message_lock() {
    client_send_single_message(Default::default()).await;
    receive_then_renew_lock().await;
}

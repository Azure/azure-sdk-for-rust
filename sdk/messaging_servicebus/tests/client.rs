use std::env;
use std::time::Duration as StdDuration;
use time::Duration as TimeSpan;

use azure_messaging_servicebus::{
    client::service_bus_client::ServiceBusClient,
    receiver::service_bus_session_receiver::ServiceBusSessionReceiverOptions, ServiceBusMessage,
    ServiceBusReceiverOptions, ServiceBusSenderOptions,
};

fn setup_dotenv() {
    dotenv::from_filename("./sdk/messaging_servicebus/tests/.env").ok();
}

async fn client_send_single_message(options: ServiceBusSenderOptions) {
    setup_dotenv();
    let connection_string = env::var("SERVICE_BUS_CONNECTION_STRING").unwrap();
    let queue = env::var("SERVICE_BUS_QUEUE").unwrap();

    let mut client = ServiceBusClient::new(connection_string).await.unwrap();
    let mut sender = client.create_sender(queue, options).await.unwrap();
    sender.send_message("hello world").await.unwrap();

    sender.dispose().await.unwrap();
    client.dispose().await.unwrap();
}

async fn client_send_multiple_messages(total: u32, options: ServiceBusSenderOptions) {
    setup_dotenv();
    let connection_string = env::var("SERVICE_BUS_CONNECTION_STRING").unwrap();
    let queue = env::var("SERVICE_BUS_QUEUE").unwrap();

    let mut client = ServiceBusClient::new(connection_string).await.unwrap();
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

    let mut client = ServiceBusClient::new(connection_string).await.unwrap();
    let mut sender = client.create_sender(queue, options).await.unwrap();

    let mut batch = sender
        .create_message_batch(Default::default())
        .await
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

    let mut client = ServiceBusClient::new(connection_string).await.unwrap();
    let mut receiver = client.create_receiver(queue, options).await.unwrap();
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

    let mut client = ServiceBusClient::new(connection_string).await.unwrap();
    let mut receiver = client
        .accept_session(session_enabled_queue, session_id, options)
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

    let mut client = ServiceBusClient::new(connection_string).await.unwrap();
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

#[test]
fn hello_world() {
    setup_dotenv();
    assert_eq!(env::var("HELLO").unwrap(), "hello");
}

#[tokio::test]
async fn client_can_connect_with_connection_string() {
    setup_dotenv();
    let connection_string = env::var("SERVICE_BUS_CONNECTION_STRING").unwrap();
    let mut client = ServiceBusClient::new(connection_string).await.unwrap();

    client.dispose().await.unwrap();
}

#[tokio::test]
async fn client_can_create_sender() {
    setup_dotenv();
    let connection_string = env::var("SERVICE_BUS_CONNECTION_STRING").unwrap();
    let queue = env::var("SERVICE_BUS_QUEUE").unwrap();

    let mut client = ServiceBusClient::new(connection_string).await.unwrap();
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

    let mut client = ServiceBusClient::new(connection_string).await.unwrap();
    let receiver = client
        .create_receiver(queue, Default::default())
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
async fn client_send_and_receive_single_sessionful_message() {
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
    let enqueue_time = OffsetDateTime::now_utc() + TimeSpan::minutes(2);
    message.set_scheduled_enqueue_time(enqueue_time);

    let connection_string = env::var("SERVICE_BUS_CONNECTION_STRING").unwrap();
    let queue = env::var("SERVICE_BUS_QUEUE").unwrap();

    let mut client = ServiceBusClient::new(connection_string).await.unwrap();
    let mut sender = client
        .create_sender(queue, Default::default())
        .await
        .unwrap();

    sender.send_message(message).await.unwrap();

    sender.dispose().await.unwrap();
    client.dispose().await.unwrap();
}

async fn client_schedule_single_message_via_service_bus_sender(delay: TimeSpan) -> i64 {
    use time::OffsetDateTime;

    setup_dotenv();

    let connection_string = env::var("SERVICE_BUS_CONNECTION_STRING").unwrap();
    let queue = env::var("SERVICE_BUS_QUEUE").unwrap();

    let mut client = ServiceBusClient::new(connection_string).await.unwrap();
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

    let mut client = ServiceBusClient::new(connection_string).await.unwrap();
    let mut sender = client
        .create_sender(queue, Default::default())
        .await
        .unwrap();

    sender.cancel_scheduled_message(seq).await.unwrap();

    sender.dispose().await.unwrap();
    client.dispose().await.unwrap();
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

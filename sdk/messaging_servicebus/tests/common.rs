use std::time::Duration as StdDuration;

use azure_messaging_servicebus::{
    client::{
        service_bus_client::ServiceBusClient, service_bus_client_options::ServiceBusClientOptions,
    },
    core::{TransportMessageBatch, TransportSender},
    primitives::{
        service_bus_peeked_message::ServiceBusPeekedMessage,
        service_bus_received_message::ServiceBusReceivedMessage,
        service_bus_retry_options::ServiceBusRetryOptions,
    },
    receiver::service_bus_session_receiver::ServiceBusSessionReceiverOptions,
    ServiceBusMessage, ServiceBusReceiverOptions, ServiceBusSender, ServiceBusSenderOptions,
};
use time::OffsetDateTime;

#[allow(dead_code)]
pub fn zero_retry_options() -> ServiceBusRetryOptions {
    ServiceBusRetryOptions {
        max_retries: 0,
        mode: Default::default(),
        delay: ServiceBusRetryOptions::DEFAULT_DELAY,
        max_delay: StdDuration::from_secs(10),
        try_timeout: StdDuration::from_secs(10),
    }
}

#[allow(dead_code)]
pub fn setup_dotenv() {
    dotenv::from_filename("./sdk/messaging_servicebus/tests/.env").ok();
}

#[allow(dead_code)]
pub async fn drain_queue(
    connection_string: String,
    client_options: ServiceBusClientOptions,
    queue_name: String,
    receiver_options: ServiceBusReceiverOptions,
    max_messages: u32,
) {
    let mut client = ServiceBusClient::new_with_options(connection_string, client_options)
        .await
        .unwrap();
    let mut receiver = client
        .create_receiver_for_queue(queue_name, receiver_options)
        .await
        .unwrap();
    let messages = receiver.receive_messages(max_messages, None).await.unwrap();

    for message in messages {
        receiver.complete_message(&message).await.unwrap();
    }

    receiver.dispose().await.unwrap();
    client.dispose().await.unwrap();
}

#[allow(dead_code)]
pub async fn create_client_and_send_messages_separately_to_queue(
    connection_string: String,
    client_options: ServiceBusClientOptions,
    queue_name: String,
    sender_options: ServiceBusSenderOptions,
    messages: impl Iterator<Item = impl Into<ServiceBusMessage>>,
) -> Result<(), anyhow::Error> {
    let mut client = ServiceBusClient::new_with_options(connection_string, client_options).await?;
    let mut sender = client.create_sender(queue_name, sender_options).await?;

    send_messages_separately(&mut sender, messages).await?;

    sender.dispose().await?;
    client.dispose().await?;
    Ok(())
}

#[allow(dead_code)]
pub async fn send_messages_separately<S>(
    sender: &mut ServiceBusSender<S>,
    messages: impl Iterator<Item = impl Into<ServiceBusMessage>>,
) -> Result<(), anyhow::Error>
where
    S: TransportSender + Send + Sync,
    <S as TransportSender>::MessageBatch: TransportMessageBatch + Send + Sync,
    <S as TransportSender>::SendError: Send + Sync + 'static,
{
    for message in messages {
        sender.send_message(message).await?;
    }
    Ok(())
}

#[allow(dead_code)]
pub async fn create_client_and_receive_messages_from_queue(
    connection_string: String,
    client_options: ServiceBusClientOptions,
    queue_name: String,
    receiver_options: ServiceBusReceiverOptions,
    max_messages: u32,
    max_wait_time: Option<StdDuration>,
) -> Result<Vec<ServiceBusReceivedMessage>, anyhow::Error> {
    let mut client = ServiceBusClient::new_with_options(connection_string, client_options).await?;
    let mut receiver = client
        .create_receiver_for_queue(queue_name, receiver_options)
        .await?;

    let messages = receiver
        .receive_messages(max_messages, max_wait_time)
        .await?;

    for message in &messages {
        receiver.complete_message(message).await?;
    }

    receiver.dispose().await?;
    client.dispose().await?;
    Ok(messages)
}

#[allow(dead_code)]
pub async fn create_client_and_receive_sessionful_messages_from_queue(
    connection_string: String,
    client_options: ServiceBusClientOptions,
    queue_name: String,
    receiver_options: ServiceBusSessionReceiverOptions,
    session_id: String,
    max_messages: u32,
    max_wait_time: Option<StdDuration>,
) -> Result<Vec<ServiceBusReceivedMessage>, anyhow::Error> {
    let mut client = ServiceBusClient::new_with_options(connection_string, client_options).await?;
    let mut receiver = client
        .accept_next_session_for_queue(queue_name, session_id, receiver_options)
        .await?;

    let messages = receiver
        .receive_messages(max_messages, max_wait_time)
        .await?;

    for message in &messages {
        receiver.complete_message(message).await?;
    }

    receiver.dispose().await?;
    client.dispose().await?;
    Ok(messages)
}

#[allow(dead_code)]
pub async fn create_client_and_abandon_messages_from_queue(
    connection_string: String,
    client_options: ServiceBusClientOptions,
    queue_name: String,
    receiver_options: ServiceBusReceiverOptions,
    max_messages: u32,
    max_wait_time: Option<StdDuration>,
) -> Result<Vec<ServiceBusReceivedMessage>, anyhow::Error> {
    let mut client = ServiceBusClient::new_with_options(connection_string, client_options).await?;
    let mut receiver = client
        .create_receiver_for_queue(queue_name, receiver_options)
        .await?;

    let messages = receiver
        .receive_messages(max_messages, max_wait_time)
        .await?;

    for message in &messages {
        receiver.abandon_message(message, None).await?;
    }

    receiver.dispose().await?;
    client.dispose().await?;
    Ok(messages)
}

#[allow(dead_code)]
pub async fn create_client_and_deadletter_messages_from_queue(
    connection_string: String,
    client_options: ServiceBusClientOptions,
    queue_name: String,
    receiver_options: ServiceBusReceiverOptions,
    max_messages: u32,
    max_wait_time: Option<StdDuration>,
) -> Result<Vec<ServiceBusReceivedMessage>, anyhow::Error> {
    let mut client = ServiceBusClient::new_with_options(connection_string, client_options).await?;
    let mut receiver = client
        .create_receiver_for_queue(queue_name, receiver_options)
        .await?;

    let messages = receiver
        .receive_messages(max_messages, max_wait_time)
        .await?;

    for message in &messages {
        receiver
            .dead_letter_message(message, None, None, None)
            .await?;
    }

    receiver.dispose().await?;
    client.dispose().await?;
    Ok(messages)
}

#[allow(dead_code)]
pub async fn create_client_and_schedule_messages(
    connection_string: String,
    client_options: ServiceBusClientOptions,
    queue_name: String,
    sender_options: ServiceBusSenderOptions,
    messages: impl Iterator<Item = impl Into<ServiceBusMessage>> + ExactSizeIterator + Send,
    enqueue_time: OffsetDateTime,
) -> Result<Vec<i64>, anyhow::Error> {
    let mut client = ServiceBusClient::new_with_options(connection_string, client_options).await?;
    let mut sender = client.create_sender(queue_name, sender_options).await?;

    let sequence_numbers = sender.schedule_messages(messages, enqueue_time).await?;

    sender.dispose().await?;
    client.dispose().await?;
    Ok(sequence_numbers)
}

#[allow(dead_code)]
pub async fn create_client_and_peek_messages(
    connection_string: String,
    client_options: ServiceBusClientOptions,
    queue_name: String,
    receiver_options: ServiceBusReceiverOptions,
    max_messages: u32,
) -> Result<Vec<ServiceBusPeekedMessage>, anyhow::Error> {
    let mut client = ServiceBusClient::new_with_options(connection_string, client_options).await?;
    let mut receiver = client
        .create_receiver_for_queue(queue_name, receiver_options)
        .await?;

    let messages = receiver.peek_messages(max_messages, None).await?;

    receiver.dispose().await?;
    client.dispose().await?;
    Ok(messages)
}

#[allow(dead_code)]
pub async fn create_client_and_defer_messages(
    connection_string: String,
    client_options: ServiceBusClientOptions,
    queue_name: String,
    receiver_options: ServiceBusReceiverOptions,
    max_messages: u32,
    max_wait_time: Option<StdDuration>,
) -> Result<Vec<i64>, anyhow::Error> {
    let mut client = ServiceBusClient::new_with_options(connection_string, client_options).await?;
    let mut receiver = client
        .create_receiver_for_queue(queue_name, receiver_options)
        .await?;

    let messages = receiver
        .receive_messages(max_messages, max_wait_time)
        .await?;

    for message in &messages {
        receiver.defer_message(message, None).await?;
    }

    receiver.dispose().await?;
    client.dispose().await?;
    Ok(messages.into_iter().map(|m| m.sequence_number()).collect())
}

#[allow(dead_code)]
pub async fn create_client_and_receive_deferred_messages(
    connection_string: String,
    client_options: ServiceBusClientOptions,
    queue_name: String,
    receiver_options: ServiceBusReceiverOptions,
    sequence_numbers: Vec<i64>,
) -> Result<Vec<ServiceBusReceivedMessage>, anyhow::Error> {
    let mut client = ServiceBusClient::new_with_options(connection_string, client_options).await?;
    let mut receiver = client
        .create_receiver_for_queue(queue_name, receiver_options)
        .await?;

    let messages = receiver.receive_deferred_messages(sequence_numbers).await?;

    for message in &messages {
        receiver.complete_message(message).await?;
    }

    receiver.dispose().await?;
    client.dispose().await?;
    Ok(messages)
}
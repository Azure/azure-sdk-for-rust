use std::time::Duration as StdDuration;

use azure_messaging_servicebus::{
    ServiceBusClient, ServiceBusClientOptions, ServiceBusMessage, ServiceBusPeekedMessage,
    ServiceBusReceivedMessage, ServiceBusReceiverOptions, ServiceBusRetryOptions, ServiceBusSender,
    ServiceBusSenderOptions, ServiceBusSessionReceiverOptions,
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
    let _ = dotenv::from_filename("./sdk/messaging_servicebus/.env");
}

// #[allow(dead_code)]
// pub async fn drain_queue(
//     connection_string: &str,
//     client_options: ServiceBusClientOptions,
//     queue_name: &str,
//     receiver_options: ServiceBusReceiverOptions,
//     max_messages: u32,
// ) {
//     let mut client = ServiceBusClient::new(connection_string, client_options)
//         .await
//         .unwrap();
//     let mut receiver = client
//         .create_receiver_for_queue(queue_name, receiver_options)
//         .await
//         .unwrap();
//     let messages = receiver
//         .receive_messages_with_max_wait_time(max_messages, std::time::Duration::from_secs(10))
//         .await
//         .unwrap();

//     for message in messages {
//         receiver.complete_message(&message).await.unwrap();
//     }

//     receiver.dispose().await.unwrap();
//     client.dispose().await.unwrap();
// }

#[allow(dead_code)]
pub async fn create_client_and_send_messages_separately_to_queue_or_topic(
    connection_string: &str,
    client_options: ServiceBusClientOptions,
    queue_or_topic_name: &str,
    sender_options: ServiceBusSenderOptions,
    messages: impl Iterator<Item = impl Into<ServiceBusMessage>>,
) -> Result<(), anyhow::Error> {
    let mut client = ServiceBusClient::new(connection_string, client_options).await?;
    let mut sender = client
        .create_sender(queue_or_topic_name, sender_options)
        .await?;

    send_messages_separately(&mut sender, messages).await?;

    sender.dispose().await?;
    client.dispose().await?;
    Ok(())
}

#[allow(dead_code)]
pub async fn send_messages_separately(
    sender: &mut ServiceBusSender,
    messages: impl Iterator<Item = impl Into<ServiceBusMessage>>,
) -> Result<(), anyhow::Error> {
    for message in messages {
        sender.send_message(message).await?;
    }
    Ok(())
}

#[allow(dead_code)]
pub async fn create_client_and_receive_messages_from_queue(
    connection_string: &str,
    client_options: ServiceBusClientOptions,
    queue_name: &str,
    receiver_options: ServiceBusReceiverOptions,
    max_messages: u32,
    max_wait_time: Option<StdDuration>,
) -> Result<Vec<ServiceBusReceivedMessage>, anyhow::Error> {
    let mut client = ServiceBusClient::new(connection_string, client_options).await?;
    let mut receiver = client
        .create_receiver_for_queue(queue_name, receiver_options)
        .await?;

    let messages = receiver
        .receive_messages_with_max_wait_time(max_messages, max_wait_time)
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
    connection_string: &str,
    client_options: ServiceBusClientOptions,
    queue_name: &str,
    receiver_options: ServiceBusSessionReceiverOptions,
    session_id: Option<String>,
    max_messages: u32,
    max_wait_time: Option<StdDuration>,
) -> Result<Vec<ServiceBusReceivedMessage>, anyhow::Error> {
    let mut client = ServiceBusClient::new(connection_string, client_options).await?;
    let mut receiver = match session_id {
        Some(session_id) => {
            client
                .accept_session_for_queue(queue_name, session_id, receiver_options)
                .await?
        }
        None => {
            client
                .accept_next_session_for_queue(queue_name, receiver_options)
                .await?
        }
    };

    let messages = receiver
        .receive_messages_with_max_wait_time(max_messages, max_wait_time)
        .await?;

    for message in &messages {
        receiver.complete_message(message).await?;
    }

    receiver.dispose().await?;
    client.dispose().await?;
    Ok(messages)
}

#[allow(dead_code)]
pub async fn create_client_and_receive_messages_from_subscription(
    connection_string: &str,
    client_options: ServiceBusClientOptions,
    topic_name: &str,
    subscription_name: &str,
    receiver_options: ServiceBusReceiverOptions,
    max_messages: u32,
    max_wait_time: Option<StdDuration>,
) -> Result<Vec<ServiceBusReceivedMessage>, anyhow::Error> {
    let mut client = ServiceBusClient::new(connection_string, client_options).await?;
    let mut receiver = client
        .create_receiver_for_subscription(topic_name, subscription_name, receiver_options)
        .await?;

    let messages = receiver
        .receive_messages_with_max_wait_time(max_messages, max_wait_time)
        .await?;

    for message in &messages {
        receiver.complete_message(message).await?;
    }

    receiver.dispose().await?;
    client.dispose().await?;
    Ok(messages)
}

#[allow(dead_code)]
pub async fn create_client_and_receive_sessionful_messages_from_subscription(
    connection_string: &str,
    client_options: ServiceBusClientOptions,
    topic_name: &str,
    subscription_name: &str,
    receiver_options: ServiceBusSessionReceiverOptions,
    session_id: String,
    max_messages: u32,
    max_wait_time: Option<StdDuration>,
) -> Result<Vec<ServiceBusReceivedMessage>, anyhow::Error> {
    let mut client = ServiceBusClient::new(connection_string, client_options).await?;
    let mut receiver = client
        .accept_session_for_subscription(
            topic_name,
            subscription_name,
            session_id,
            receiver_options,
        )
        .await?;

    let messages = receiver
        .receive_messages_with_max_wait_time(max_messages, max_wait_time)
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
    connection_string: &str,
    client_options: ServiceBusClientOptions,
    queue_name: &str,
    receiver_options: ServiceBusReceiverOptions,
    max_messages: u32,
    max_wait_time: Option<StdDuration>,
) -> Result<Vec<ServiceBusReceivedMessage>, anyhow::Error> {
    let mut client = ServiceBusClient::new(connection_string, client_options).await?;
    let mut receiver = client
        .create_receiver_for_queue(queue_name, receiver_options)
        .await?;

    let messages = receiver
        .receive_messages_with_max_wait_time(max_messages, max_wait_time)
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
    connection_string: &str,
    client_options: ServiceBusClientOptions,
    queue_name: &str,
    receiver_options: ServiceBusReceiverOptions,
    max_messages: u32,
    max_wait_time: Option<StdDuration>,
) -> Result<Vec<ServiceBusReceivedMessage>, anyhow::Error> {
    let mut client = ServiceBusClient::new(connection_string, client_options).await?;
    let mut receiver = client
        .create_receiver_for_queue(queue_name, receiver_options)
        .await?;

    let messages = receiver
        .receive_messages_with_max_wait_time(max_messages, max_wait_time)
        .await?;

    for message in &messages {
        receiver
            .dead_letter_message(message, Default::default())
            .await?;
    }

    receiver.dispose().await?;
    client.dispose().await?;
    Ok(messages)
}

#[allow(dead_code)]
pub async fn create_client_and_schedule_messages(
    connection_string: &str,
    client_options: ServiceBusClientOptions,
    queue_name: &str,
    sender_options: ServiceBusSenderOptions,
    messages: impl Iterator<Item = impl Into<ServiceBusMessage>> + ExactSizeIterator + Send,
    enqueue_time: OffsetDateTime,
) -> Result<Vec<i64>, anyhow::Error> {
    let mut client = ServiceBusClient::new(connection_string, client_options).await?;
    let mut sender = client.create_sender(queue_name, sender_options).await?;

    let sequence_numbers = sender.schedule_messages(messages, enqueue_time).await?;

    sender.dispose().await?;
    client.dispose().await?;
    Ok(sequence_numbers)
}

#[allow(dead_code)]
pub async fn create_client_and_peek_messages(
    connection_string: &str,
    client_options: ServiceBusClientOptions,
    queue_name: &str,
    receiver_options: ServiceBusReceiverOptions,
    max_messages: u32,
) -> Result<Vec<ServiceBusPeekedMessage>, anyhow::Error> {
    let mut client = ServiceBusClient::new(connection_string, client_options).await?;
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
    connection_string: &str,
    client_options: ServiceBusClientOptions,
    queue_name: &str,
    receiver_options: ServiceBusReceiverOptions,
    max_messages: u32,
    max_wait_time: Option<StdDuration>,
) -> Result<Vec<i64>, anyhow::Error> {
    let mut client = ServiceBusClient::new(connection_string, client_options).await?;
    let mut receiver = client
        .create_receiver_for_queue(queue_name, receiver_options)
        .await?;

    let messages = receiver
        .receive_messages_with_max_wait_time(max_messages, max_wait_time)
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
    connection_string: &str,
    client_options: ServiceBusClientOptions,
    queue_name: &str,
    receiver_options: ServiceBusReceiverOptions,
    sequence_numbers: Vec<i64>,
) -> Result<Vec<ServiceBusReceivedMessage>, anyhow::Error> {
    let mut client = ServiceBusClient::new(connection_string, client_options).await?;
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

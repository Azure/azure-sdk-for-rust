use azure_messaging_servicebus::ServiceBusMessage;

mod common;
use common::setup_dotenv;

// #[tokio::test]
// async fn send_and_receive_one_message() {
//     setup_dotenv();

//     let connection_string = std::env::var("SERVICE_BUS_CONNECTION_STRING").unwrap();
//     let topic_name = std::env::var("SERVICE_BUS_TOPIC").unwrap();
//     let subscription_name = std::env::var("SERVICE_BUS_SUBSCRIPTION").unwrap();

//     let message = ServiceBusMessage::new("test message");
//     let messages = std::iter::once(message);
//     let total = messages.len() as u32;

//     common::create_client_and_send_messages_separately_to_queue_or_topic(
//         connection_string.clone(),
//         Default::default(),
//         topic_name.clone(),
//         sender_options,
//         messages,
//     )
// }

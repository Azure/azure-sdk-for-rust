use azure_messaging_servicebus::{ServiceBusMessage, ServiceBusReceiverOptions};
use serial_test::serial;

mod common;
use common::setup_dotenv;

// #[tokio::test]
// #[serial]
// async fn drain_subscription() {
//     use azure_messaging_servicebus::prelude::*;

//     setup_dotenv();

//     let connection_string = std::env::var("SERVICE_BUS_CONNECTION_STRING").unwrap();
//     let topic_name = std::env::var("SERVICE_BUS_SESSION_TOPIC").unwrap();
//     let subscription_name = std::env::var("SERVICE_BUS_SESSION_SUBSCRIPTION").unwrap();
//     let max_messages = 4;

//     let client_options = ServiceBusClientOptions {
//         retry_options: common::zero_retry_options(),
//         ..Default::default()
//     };
//     let mut client = ServiceBusClient::new(connection_string, client_options)
//         .await
//         .unwrap();
//     let options = ServiceBusReceiverOptions {
//         sub_queue: SubQueue::DeadLetter,
//         ..Default::default()
//     };
//     let mut receiver = client
//         .create_receiver_for_subscription(topic_name, subscription_name, options)
//         .await
//         .unwrap();
//     let received = receiver
//         .receive_messages_with_max_wait_time(max_messages, std::time::Duration::from_secs(10))
//         .await
//         .unwrap();
//     for message in &received {
//         println!("Received message: {}", message);
//         receiver.complete_message(message).await.unwrap();
//     }

//     receiver.dispose().await.unwrap();
//     client.dispose().await.unwrap();
// }

#[tokio::test]
#[serial]
async fn send_and_receive_one_message() {
    setup_dotenv();

    let connection_string = std::env::var("SERVICE_BUS_CONNECTION_STRING").unwrap();
    let topic_name = std::env::var("SERVICE_BUS_TOPIC").unwrap();
    let subscription_name = std::env::var("SERVICE_BUS_SUBSCRIPTION").unwrap();

    let message_body = b"test message";
    let message = ServiceBusMessage::new(&message_body[..]);
    let messages = std::iter::once(message);
    let max_messages = messages.len() as u32;

    common::create_client_and_send_messages_separately_to_queue_or_topic(
        &connection_string,
        Default::default(),
        &topic_name,
        Default::default(),
        messages,
    )
    .await
    .unwrap();

    let received = common::create_client_and_receive_messages_from_subscription(
        &connection_string,
        Default::default(),
        &topic_name,
        &subscription_name,
        Default::default(),
        max_messages,
        None,
    )
    .await
    .unwrap();

    assert_eq!(received.len(), max_messages as usize);
    let received_message_body = received[0].body().unwrap();
    assert_eq!(received_message_body, message_body);
}

#[tokio::test]
#[serial]
async fn send_and_receive_multiple_messages_separately() {
    setup_dotenv();
    let connection_string = std::env::var("SERVICE_BUS_CONNECTION_STRING").unwrap();
    let topic_name = std::env::var("SERVICE_BUS_TOPIC").unwrap();
    let subscription_name = std::env::var("SERVICE_BUS_SUBSCRIPTION").unwrap();

    let expected = ["test message 1", "test message 2", "test message 3"];
    let messages = vec![
        ServiceBusMessage::new(expected[0]),
        ServiceBusMessage::new(expected[1]),
        ServiceBusMessage::new(expected[2]),
    ];
    let total = messages.len();

    common::create_client_and_send_messages_separately_to_queue_or_topic(
        &connection_string,
        Default::default(),
        &topic_name,
        Default::default(),
        messages.into_iter(),
    )
    .await
    .unwrap();

    let received = common::create_client_and_receive_messages_from_subscription(
        &connection_string,
        Default::default(),
        &topic_name,
        &subscription_name,
        Default::default(),
        total as u32,
        None,
    )
    .await
    .unwrap();

    assert_eq!(received.len(), total);
    for (i, message) in received.iter().enumerate() {
        let received_message_body = message.body().unwrap();
        assert_eq!(received_message_body, expected[i].as_bytes());
    }
}

#[tokio::test]
#[serial]
async fn send_and_receive_multiple_messages_separately_with_prefetch() {
    setup_dotenv();
    let connection_string = std::env::var("SERVICE_BUS_CONNECTION_STRING").unwrap();
    let topic_name = std::env::var("SERVICE_BUS_TOPIC").unwrap();
    let subscription_name = std::env::var("SERVICE_BUS_SUBSCRIPTION").unwrap();

    let expected = ["test message 1", "test message 2", "test message 3"];
    let messages = vec![
        ServiceBusMessage::new(expected[0]),
        ServiceBusMessage::new(expected[1]),
        ServiceBusMessage::new(expected[2]),
    ];
    let total = messages.len();

    common::create_client_and_send_messages_separately_to_queue_or_topic(
        &connection_string,
        Default::default(),
        &topic_name,
        Default::default(),
        messages.into_iter(),
    )
    .await
    .unwrap();

    let receiver_option = ServiceBusReceiverOptions {
        prefetch_count: total as u32,
        ..Default::default()
    };
    let received = common::create_client_and_receive_messages_from_subscription(
        &connection_string,
        Default::default(),
        &topic_name,
        &subscription_name,
        receiver_option,
        total as u32,
        None,
    )
    .await
    .unwrap();

    assert_eq!(received.len(), total);
    for (i, message) in received.iter().enumerate() {
        let received_message_body = message.body().unwrap();
        assert_eq!(received_message_body, expected[i].as_bytes());
    }
}

#[tokio::test]
#[serial]
async fn send_and_receive_session_messages() {
    setup_dotenv();
    let connection_string = std::env::var("SERVICE_BUS_CONNECTION_STRING").unwrap();
    let topic_name = std::env::var("SERVICE_BUS_SESSION_TOPIC").unwrap();
    let subscription_name = std::env::var("SERVICE_BUS_SESSION_SUBSCRIPTION").unwrap();

    let expected_for_session_id_1 = ["test message 1", "test message 2", "test message 3"];
    let expected_for_session_id_2 = ["test message 4", "test message 5", "test message 6"];
    let session_id_1 = "session1";
    let session_id_2 = "session2";

    let connection_string_clone = connection_string.clone();
    let topic_name_clone = topic_name.clone();
    let subscription_name_clone = subscription_name.clone();
    let handle_1 = tokio::spawn(async move {
        common::create_client_and_receive_sessionful_messages_from_subscription(
            &connection_string_clone,
            Default::default(),
            &topic_name_clone,
            &subscription_name_clone,
            Default::default(),
            session_id_1.to_string(),
            expected_for_session_id_1.len() as u32,
            None,
        )
        .await
    });

    let connection_string_clone = connection_string.clone();
    let topic_name_clone = topic_name.clone();
    let subscription_name_clone = subscription_name.clone();
    let handle_2 = tokio::spawn(async move {
        common::create_client_and_receive_sessionful_messages_from_subscription(
            &connection_string_clone,
            Default::default(),
            &topic_name_clone,
            &subscription_name_clone,
            Default::default(),
            session_id_2.to_string(),
            expected_for_session_id_2.len() as u32,
            None,
        )
        .await
    });

    // Send 2nd session messages first to ensure that the 1st session is not auto-received
    let messages = expected_for_session_id_2.iter().map(|m| {
        let mut message = ServiceBusMessage::new(m.as_bytes());
        message.set_session_id(String::from(session_id_2)).unwrap();
        message
    });
    common::create_client_and_send_messages_separately_to_queue_or_topic(
        &connection_string,
        Default::default(),
        &topic_name,
        Default::default(),
        messages,
    )
    .await
    .unwrap();

    // Send 1st session messages next
    let messages = expected_for_session_id_1.iter().map(|m| {
        let mut message = ServiceBusMessage::new(m.as_bytes());
        message.set_session_id(String::from(session_id_1)).unwrap();
        message
    });
    common::create_client_and_send_messages_separately_to_queue_or_topic(
        &connection_string,
        Default::default(),
        &topic_name,
        Default::default(),
        messages,
    )
    .await
    .unwrap();

    let received_1 = handle_1.await.unwrap().unwrap();
    let received_2 = handle_2.await.unwrap().unwrap();

    assert_eq!(received_1.len(), expected_for_session_id_1.len());
    for (i, message) in received_1.iter().enumerate() {
        let received_message_body = message.body().unwrap();
        assert_eq!(
            received_message_body,
            expected_for_session_id_1[i].as_bytes()
        );
    }

    assert_eq!(received_2.len(), expected_for_session_id_2.len());
    for (i, message) in received_2.iter().enumerate() {
        let received_message_body = message.body().unwrap();
        assert_eq!(
            received_message_body,
            expected_for_session_id_2[i].as_bytes()
        );
    }
}

#[tokio::test]
#[serial]
async fn create_rule_manager() {
    use azure_messaging_servicebus::administration::*;
    use azure_messaging_servicebus::prelude::*;

    setup_dotenv();
    let connection_string = std::env::var("SERVICE_BUS_CONNECTION_STRING").unwrap();
    let topic_name = std::env::var("SERVICE_BUS_RULE_FILTER_TEST_TOPIC").unwrap();
    let subscription_name = std::env::var("SERVICE_BUS_RULE_FILTER_TEST_SUBSCRIPTION").unwrap();

    let mut client = ServiceBusClient::new(connection_string, Default::default())
        .await
        .unwrap();
    let mut rule_manager = client
        .create_rule_manager(topic_name, subscription_name)
        .await
        .unwrap();

    let rules = rule_manager.get_rules().await.unwrap();

    // Remove all existing rules
    for rule in rules {
        let name = rule.name;
        rule_manager.delete_rule(name).await.unwrap();
    }

    // Add a correlation rule filter
    let correlation_filter = CorrelationRuleFilter::builder().subject("subject").build();
    rule_manager
        .create_rule("brand-filter", correlation_filter)
        .await
        .unwrap();

    // Add a SQL rule filter
    let filter = SqlRuleFilter::new("user.color='red'");
    let action = SqlRuleAction::new("SET quantity = quantity / 2;");
    rule_manager
        .create_rule("color-filter", (filter, action))
        .await
        .unwrap();

    // Add a true filter
    rule_manager
        .create_rule("true-filter", TrueRuleFilter::new())
        .await
        .unwrap();

    // Add a false filter
    rule_manager
        .create_rule("false-filter", FalseRuleFilter::new())
        .await
        .unwrap();

    // Get the newly added rules
    let rules = rule_manager.get_rules().await.unwrap();
    assert_eq!(rules.len(), 4);
    let rule_names = rules.iter().map(|r| r.name.as_str()).collect::<Vec<_>>();
    assert!(rule_names.contains(&"brand-filter"));
    assert!(rule_names.contains(&"color-filter"));
    assert!(rule_names.contains(&"true-filter"));
    assert!(rule_names.contains(&"false-filter"));

    rule_manager.dispose().await.unwrap();
    client.dispose().await.unwrap();
}

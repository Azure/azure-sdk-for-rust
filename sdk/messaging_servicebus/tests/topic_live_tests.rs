//! Test Sender and Receiver with a live Service Bus namespace.
//!
//! All tests must be run sequentially as they use the same queue for testing, and every test must
//! ensure that the queue is empty after it is done.
//!
//! To run these tests, you must set the following environment variables or provided using a .env
//! file placed at "azure-sdk-for-rust/sdk/messaging_servicebus/.env"
//!
//! - "SERVICE_BUS_CONNECTION_STRING": The connection string obtained from Azure portal to the
//!   Service Bus namespace
//! - "SERVICE_BUS_NAMESPACE": The namespace of the Service Bus. It should be in the format
//!   "{your-namespace}.servicebus.windows.net"
//! - "SERVICE_BUS_SAS_KEY_NAME": The name of the shared access key used to connect to the Service
//!   Bus
//! - "SERVICE_BUS_SAS_KEY": The value of the shared access key used to connect to the Service Bus
//! - "SERVICE_BUS_QUEUE": The name of a Service Bus queue that does NOT have session enabled
//! - "SERVICE_BUS_SESSION_QUEUE": The name of a Service Bus queue that has session enabled
//! - "SERVICE_BUS_TOPIC": The name of a Service Bus topic whose subscriptions do NOT have session
//!   enabled
//! - "SERVICE_BUS_SUBSCRIPTION": The name of a Service Bus subscription that does NOT have session
//!   enabled
//! - "SERVICE_BUS_SESSION_TOPIC": The name of a Service Bus topic that has session enabled
//!   subscriptions
//! - "SERVICE_BUS_SESSION_SUBSCRIPTION": The name of a Service Bus subscription that has session
//!   enabled
//! - "SERVICE_BUS_RULE_FILTER_TEST_TOPIC": The name of a separate Service Bus topic that is only
//!   used for testing rule filters. This is to avoid interfering with other tests.
//! - "SERVICE_BUS_RULE_FILTER_TEST_SUBSCRIPTION": The name of a separate Service Bus subscription
//!   that is only used for testing rule filters. This is to avoid interfering with other tests.
//!

#![cfg(all(test, feature = "test_e2e"))]

#[macro_use]
mod macros;

cfg_not_wasm32! {
    use azservicebus::{ServiceBusMessage, ServiceBusReceiverOptions};

    mod common;
    use common::setup_dotenv;

    #[tokio::test]
    async fn run_topic_subscription_tests_sequentially() {
        let mut all_result: Result<(), anyhow::Error> = Ok(());

        print!("test send_and_receive_one_message");
        let result = send_and_receive_one_message().await;
        println!(" ... {:?}", result);
        all_result = all_result.and(result);

        print!("test send_and_receive_multiple_messages_separately");
        let result = send_and_receive_multiple_messages_separately().await;
        println!(" ... {:?}", result);
        all_result = all_result.and(result);

        print!("test send_and_receive_multiple_messages_separately_with_prefetch");
        let result = send_and_receive_multiple_messages_separately_with_prefetch().await;
        println!(" ... {:?}", result);
        all_result = all_result.and(result);

        print!("test send_and_receive_session_messages");
        let result = send_and_receive_session_messages().await;
        println!(" ... {:?}", result);
        all_result = all_result.and(result);

        print!("test get_delete_then_create_rules");
        let result = get_delete_then_create_rules().await;
        println!(" ... {:?}", result);
        all_result = all_result.and(result);

        assert!(all_result.is_ok());
    }

    async fn send_and_receive_one_message() -> Result<(), anyhow::Error> {
        setup_dotenv();

        let connection_string = std::env::var("SERVICE_BUS_CONNECTION_STRING")?;
        let topic_name = std::env::var("SERVICE_BUS_TOPIC")?;
        let subscription_name = std::env::var("SERVICE_BUS_SUBSCRIPTION")?;

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
        .await?;

        let received = common::create_client_and_receive_messages_from_subscription(
            &connection_string,
            Default::default(),
            &topic_name,
            &subscription_name,
            Default::default(),
            max_messages,
            None,
        )
        .await?;

        assert_eq!(received.len(), max_messages as usize);
        let received_message_body = received[0].body()?;
        assert_eq!(received_message_body, message_body);

        Ok(())
    }

    async fn send_and_receive_multiple_messages_separately() -> Result<(), anyhow::Error> {
        setup_dotenv();
        let connection_string = std::env::var("SERVICE_BUS_CONNECTION_STRING")?;
        let topic_name = std::env::var("SERVICE_BUS_TOPIC")?;
        let subscription_name = std::env::var("SERVICE_BUS_SUBSCRIPTION")?;

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
        .await?;

        let received = common::create_client_and_receive_messages_from_subscription(
            &connection_string,
            Default::default(),
            &topic_name,
            &subscription_name,
            Default::default(),
            total as u32,
            None,
        )
        .await?;

        assert_eq!(received.len(), total);
        for (i, message) in received.iter().enumerate() {
            let received_message_body = message.body()?;
            assert_eq!(received_message_body, expected[i].as_bytes());
        }

        Ok(())
    }

    async fn send_and_receive_multiple_messages_separately_with_prefetch() -> Result<(), anyhow::Error>
    {
        setup_dotenv();
        let connection_string = std::env::var("SERVICE_BUS_CONNECTION_STRING")?;
        let topic_name = std::env::var("SERVICE_BUS_TOPIC")?;
        let subscription_name = std::env::var("SERVICE_BUS_SUBSCRIPTION")?;

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
        .await?;

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
        .await?;

        assert_eq!(received.len(), total);
        for (i, message) in received.iter().enumerate() {
            let received_message_body = message.body()?;
            assert_eq!(received_message_body, expected[i].as_bytes());
        }
        Ok(())
    }

    async fn send_and_receive_session_messages() -> Result<(), anyhow::Error> {
        setup_dotenv();
        let connection_string = std::env::var("SERVICE_BUS_CONNECTION_STRING")?;
        let topic_name = std::env::var("SERVICE_BUS_SESSION_TOPIC")?;
        let subscription_name = std::env::var("SERVICE_BUS_SESSION_SUBSCRIPTION")?;

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
        .await?;

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
        .await?;

        let received_1 = handle_1.await.unwrap()?;
        let received_2 = handle_2.await.unwrap()?;

        assert_eq!(received_1.len(), expected_for_session_id_1.len());
        for (i, message) in received_1.iter().enumerate() {
            let received_message_body = message.body()?;
            assert_eq!(
                received_message_body,
                expected_for_session_id_1[i].as_bytes()
            );
        }

        assert_eq!(received_2.len(), expected_for_session_id_2.len());
        for (i, message) in received_2.iter().enumerate() {
            let received_message_body = message.body()?;
            assert_eq!(
                received_message_body,
                expected_for_session_id_2[i].as_bytes()
            );
        }

        Ok(())
    }

    async fn get_delete_then_create_rules() -> Result<(), anyhow::Error> {
        use azservicebus::administration::*;
        use azservicebus::prelude::*;

        setup_dotenv();
        let connection_string = std::env::var("SERVICE_BUS_CONNECTION_STRING")?;
        let topic_name = std::env::var("SERVICE_BUS_RULE_FILTER_TEST_TOPIC")?;
        let subscription_name = std::env::var("SERVICE_BUS_RULE_FILTER_TEST_SUBSCRIPTION")?;

        let mut client = ServiceBusClient::new_from_connection_string(connection_string, Default::default()).await?;
        let mut rule_manager = client
            .create_rule_manager(topic_name, subscription_name)
            .await?;

        let rules = rule_manager.get_rules().await?;

        // try remove a non-existent rule
        let result = rule_manager.delete_rule("non-existent").await;
        assert!(result.is_err());

        // Remove all existing rules
        for rule in rules {
            let name = rule.name;
            rule_manager.delete_rule(name).await?;
        }

        // Add a correlation rule filter
        let correlation_filter = CorrelationRuleFilter::builder().subject("subject").build();
        rule_manager
            .create_rule("brand-filter", correlation_filter)
            .await?;

        // Add a SQL rule filter
        let filter = SqlRuleFilter::new("user.color='red'");
        let action = SqlRuleAction::new("SET quantity = quantity / 2;");
        rule_manager
            .create_rule("color-filter", (filter, action))
            .await?;

        // Add a true filter
        rule_manager
            .create_rule("true-filter", TrueRuleFilter::new())
            .await?;

        // Add a false filter
        rule_manager
            .create_rule("false-filter", FalseRuleFilter::new())
            .await?;

        // Get the newly added rules
        let rules = rule_manager.get_rules().await?;
        assert_eq!(rules.len(), 4);
        let rule_names = rules.iter().map(|r| r.name.as_str()).collect::<Vec<_>>();
        assert!(rule_names.contains(&"brand-filter"));
        assert!(rule_names.contains(&"color-filter"));
        assert!(rule_names.contains(&"true-filter"));
        assert!(rule_names.contains(&"false-filter"));

        rule_manager.dispose().await?;
        client.dispose().await?;

        Ok(())
    }
}

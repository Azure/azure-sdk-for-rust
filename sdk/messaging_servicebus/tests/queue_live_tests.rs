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

#[macro_use]
mod macros;

cfg_not_wasm32! {
    use azure_messaging_servicebus::{
        ServiceBusClient, ServiceBusClientOptions,
        SubQueue,
        ServiceBusMessage, ServiceBusReceiverOptions,
    };
    use std::time::Duration as StdDuration;

    use time::OffsetDateTime;

    mod common;
    use common::setup_dotenv;

    /// These tests use the same queue, so they must be run sequentially.
    #[tokio::test]
    async fn run_queue_tests_sequentially() {
        let mut all_result: Result<(), anyhow::Error> = Ok(());

        print!("test send_and_receive_one_message");
        let result = send_and_receive_one_message().await;
        println!(" ... {:?}", result);
        all_result = all_result.and(result);

        print!("test send_one_message_and_try_receive_more_than_one");
        let result = send_one_message_and_try_receive_more_than_one().await;
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

        print!("test send_and_receive_multiple_messages_with_message_batch");
        let result = send_and_receive_multiple_messages_with_message_batch().await;
        println!(" ... {:?}", result);
        all_result = all_result.and(result);

        print!("test send_message_batch_and_try_receive_more_than_sent");
        let result = send_message_batch_and_try_receive_more_than_sent().await;
        println!(" ... {:?}", result);
        all_result = all_result.and(result);

        print!("test send_and_abandon_messages_then_receive_messages");
        let result = send_and_abandon_messages_then_receive_messages().await;
        println!(" ... {:?}", result);
        all_result = all_result.and(result);

        print!("test send_and_deadletter_then_receive_from_deadletter_queue");
        let result = send_and_deadletter_then_receive_from_deadletter_queue().await;
        println!(" ... {:?}", result);
        all_result = all_result.and(result);

        print!("test schedule_and_receive_messages");
        let result = schedule_and_receive_messages().await;
        println!("{:?}", result);
        all_result = all_result.and(result);

        print!("test schedule_and_cancel_scheduled_messages");
        let result = schedule_and_cancel_scheduled_messages().await;
        println!("{:?}", result);
        all_result = all_result.and(result);

        print!("test send_and_peek_messages");
        let result = send_and_peek_messages().await;
        println!("{:?}", result);
        all_result = all_result.and(result);

        print!("test defer_and_receive_deferred_messages");
        let result = defer_and_receive_deferred_messages().await;
        println!("{:?}", result);
        all_result = all_result.and(result);

        print!("test receive_and_renew_lock");
        let result = receive_and_renew_lock().await;
        println!("{:?}", result);
        all_result = all_result.and(result);

        print!("test send_and_receive_on_next_session");
        let result = send_and_receive_on_next_session().await;
        println!(" ... {:?}", result);
        all_result = all_result.and(result);

        print!("test send_and_receive_sessionful_messages");
        let result = send_and_receive_sessionful_messages().await;
        println!(" ... {:?}", result);
        all_result = all_result.and(result);

        print!("test renew_session_lock");
        let result = renew_session_lock_and_set_get_session_state().await;
        println!(" ... {:?}", result);
        all_result = all_result.and(result);

        print!("test set_time_to_live");
        let result = set_time_to_live().await;
        println!(" ... {:?}", result);
        all_result = all_result.and(result);

        assert!(all_result.is_ok())
    }

    async fn send_and_receive_one_message() -> Result<(), anyhow::Error> {
        setup_dotenv();
        let connection_string = std::env::var("SERVICE_BUS_CONNECTION_STRING")?;
        let queue_name = std::env::var("SERVICE_BUS_QUEUE")?;

        let message = ServiceBusMessage::new("test message");
        let messages = std::iter::once(message);
        let total = messages.len();

        common::create_client_and_send_messages_separately_to_queue_or_topic(
            &connection_string,
            Default::default(),
            &queue_name,
            Default::default(),
            messages,
        )
        .await?;

        let received = common::create_client_and_receive_messages_from_queue(
            &connection_string,
            Default::default(),
            &queue_name,
            Default::default(),
            total as u32,
            None,
        )
        .await?;

        assert_eq!(received.len(), total);
        let received_message_body = received[0].body()?;
        assert_eq!(received_message_body, b"test message");
        Ok(())
    }

    async fn send_one_message_and_try_receive_more_than_one() -> Result<(), anyhow::Error> {
        setup_dotenv();
        let connection_string = std::env::var("SERVICE_BUS_CONNECTION_STRING")?;
        let queue_name = std::env::var("SERVICE_BUS_QUEUE")?;

        let message = ServiceBusMessage::new("test message");
        let messages = std::iter::once(message);
        let total = messages.len();

        common::create_client_and_send_messages_separately_to_queue_or_topic(
            &connection_string,
            Default::default(),
            &queue_name,
            Default::default(),
            messages,
        )
        .await?;

        let mut receiver_client_options = ServiceBusClientOptions::default();
        receiver_client_options.retry_options = common::zero_retry_options();

        let received = common::create_client_and_receive_messages_from_queue(
            &connection_string,
            receiver_client_options,
            &queue_name,
            Default::default(),
            total as u32 + 1,
            None,
        )
        .await?;

        // Please note that if the test queue's messsage lock duration is shorter than the max retry
        // delay, the same message may be received more than once.
        assert_eq!(received.len(), total);
        let received_message_body = received[0].body()?;
        assert_eq!(received_message_body, b"test message");
        Ok(())
    }

    async fn send_and_receive_multiple_messages_separately() -> Result<(), anyhow::Error> {
        setup_dotenv();
        let connection_string = std::env::var("SERVICE_BUS_CONNECTION_STRING")?;
        let queue_name = std::env::var("SERVICE_BUS_QUEUE")?;

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
            &queue_name,
            Default::default(),
            messages.into_iter(),
        )
        .await?;

        let received = common::create_client_and_receive_messages_from_queue(
            &connection_string,
            Default::default(),
            &queue_name,
            Default::default(),
            total as u32,
            None,
        )
        .await?;

        assert_eq!(received.len(), total);
        for i in 0..total {
            let received_message_body = received[i].body()?;
            assert_eq!(received_message_body, expected[i].as_bytes());
        }

        Ok(())
    }

    async fn send_and_receive_multiple_messages_separately_with_prefetch() -> Result<(), anyhow::Error>
    {
        setup_dotenv();
        let connection_string = std::env::var("SERVICE_BUS_CONNECTION_STRING")?;
        let queue_name = std::env::var("SERVICE_BUS_QUEUE")?;

        let expected = ["test message 1", "test message 2", "test message 3"];
        let messages = vec![
            ServiceBusMessage::new(expected[0]),
            ServiceBusMessage::new(expected[1]),
            ServiceBusMessage::new(expected[2]),
        ];
        let max_messages = messages.len() as u32;

        common::create_client_and_send_messages_separately_to_queue_or_topic(
            &connection_string,
            Default::default(),
            &queue_name,
            Default::default(),
            messages.into_iter(),
        )
        .await?;

        let mut receiver_options = ServiceBusReceiverOptions::default();
        receiver_options.prefetch_count = max_messages;
        common::create_client_and_receive_messages_from_queue(
            &connection_string,
            Default::default(),
            &queue_name,
            receiver_options,
            max_messages,
            None,
        )
        .await?;

        Ok(())
    }

    async fn send_and_receive_multiple_messages_with_message_batch() -> Result<(), anyhow::Error> {
        setup_dotenv();
        let connection_string = std::env::var("SERVICE_BUS_CONNECTION_STRING")?;
        let queue_name = std::env::var("SERVICE_BUS_QUEUE")?;

        let expected = ["test message 1", "test message 2", "test message 3"];

        let mut client = ServiceBusClient::new(&connection_string, Default::default()).await?;
        let mut sender = client
            .create_sender(&queue_name, Default::default())
            .await?;
        let mut message_batch = sender.create_message_batch(Default::default())?;

        let total = expected.len();
        for message in expected {
            message_batch.try_add_message(message)?;
        }
        sender.send_message_batch(message_batch).await?;

        let received = common::create_client_and_receive_messages_from_queue(
            &connection_string,
            Default::default(),
            &queue_name,
            Default::default(),
            total as u32,
            None,
        )
        .await?;

        assert_eq!(received.len(), total);
        for i in 0..total {
            let received_message_body = received[i].body()?;
            assert_eq!(received_message_body, expected[i].as_bytes());
        }

        Ok(())
    }

    async fn send_message_batch_and_try_receive_more_than_sent() -> Result<(), anyhow::Error> {
        setup_dotenv();
        let connection_string = std::env::var("SERVICE_BUS_CONNECTION_STRING")?;
        let queue_name = std::env::var("SERVICE_BUS_QUEUE")?;

        let expected = ["test message 1", "test message 2", "test message 3"];

        let mut client = ServiceBusClient::new(&connection_string, Default::default()).await?;
        let mut sender = client
            .create_sender(&queue_name, Default::default())
            .await?;
        let mut message_batch = sender.create_message_batch(Default::default())?;

        let total = expected.len();
        for message in expected {
            message_batch.try_add_message(message)?;
        }
        sender.send_message_batch(message_batch).await?;

        let mut receiving_client_options = ServiceBusClientOptions::default();
        receiving_client_options.retry_options = common::zero_retry_options();

        let received = common::create_client_and_receive_messages_from_queue(
            &connection_string,
            receiving_client_options,
            &queue_name,
            Default::default(),
            total as u32 + 1,
            None,
        )
        .await?;

        // Please note that if the test queue's messsage lock duration is shorter than the max retry
        // delay, the same message may be received more than once.
        assert_eq!(received.len(), total);
        for i in 0..total {
            let received_message_body = received[i].body()?;
            assert_eq!(received_message_body, expected[i].as_bytes());
        }
        Ok(())
    }

    async fn send_and_receive_on_next_session() -> Result<(), anyhow::Error> {
        setup_dotenv();
        let connection_string = std::env::var("SERVICE_BUS_CONNECTION_STRING")?;
        let queue_name = std::env::var("SERVICE_BUS_SESSION_QUEUE")?;

        let expected = ["test message 1", "test message 2", "test message 3"];
        let session_id = "test_session";
        let messages = expected.iter().map(|message| {
            let mut message = ServiceBusMessage::new(message.as_bytes());
            message.set_session_id(String::from(session_id)).unwrap();
            message
        });

        common::create_client_and_send_messages_separately_to_queue_or_topic(
            &connection_string,
            Default::default(),
            &queue_name,
            Default::default(),
            messages,
        )
        .await?;

        let received = common::create_client_and_receive_sessionful_messages_from_queue(
            &connection_string,
            Default::default(),
            &queue_name,
            Default::default(),
            None,
            expected.len() as u32,
            None,
        )
        .await?;

        assert_eq!(received.len(), expected.len());
        for i in 0..expected.len() {
            let received_message_body = received[i].body()?;
            assert_eq!(received_message_body, expected[i].as_bytes());
        }

        Ok(())
    }

    async fn send_and_receive_sessionful_messages() -> Result<(), anyhow::Error> {
        setup_dotenv();
        let connection_string = std::env::var("SERVICE_BUS_CONNECTION_STRING")?;
        let queue_name = std::env::var("SERVICE_BUS_SESSION_QUEUE")?;

        let expected_for_session_id_1 = ["test message 1", "test message 2", "test message 3"];
        let expected_for_session_id_2 = ["test message 4", "test message 5", "test message 6"];
        let session_id_1 = "test_session_1";
        let session_id_2 = "test_session_2";

        let connection_string_clone = connection_string.clone();
        let queue_name_clone = queue_name.clone();
        let handle_1 = tokio::spawn(async move {
            common::create_client_and_receive_sessionful_messages_from_queue(
                &connection_string_clone,
                Default::default(),
                &queue_name_clone,
                Default::default(),
                Some(session_id_1.to_string()),
                expected_for_session_id_1.len() as u32,
                None,
            )
            .await
        });

        let connection_string_clone = connection_string.clone();
        let queue_name_clone = queue_name.clone();
        let handle_2 = tokio::spawn(async move {
            common::create_client_and_receive_sessionful_messages_from_queue(
                &connection_string_clone,
                Default::default(),
                &queue_name_clone,
                Default::default(),
                Some(session_id_2.to_string()),
                expected_for_session_id_2.len() as u32,
                None,
            )
            .await
        });

        // Send 2nd session id first
        let messages = expected_for_session_id_2.iter().map(|message| {
            let mut message = ServiceBusMessage::new(message.as_bytes());
            message.set_session_id(String::from(session_id_2)).unwrap();
            message
        });
        common::create_client_and_send_messages_separately_to_queue_or_topic(
            &connection_string,
            Default::default(),
            &queue_name,
            Default::default(),
            messages,
        )
        .await?;

        // Send 1st session id last
        let messages = expected_for_session_id_1.iter().map(|message| {
            let mut message = ServiceBusMessage::new(*message);
            message.set_session_id(String::from(session_id_1)).unwrap(); // length must not exceed max length
            message
        });
        common::create_client_and_send_messages_separately_to_queue_or_topic(
            &connection_string,
            Default::default(),
            &queue_name,
            Default::default(),
            messages,
        )
        .await?;

        let received_from_session_1 = handle_1.await.unwrap()?;
        let received_from_session_2 = handle_2.await.unwrap()?;

        assert_eq!(
            received_from_session_1.len(),
            expected_for_session_id_1.len()
        );
        for i in 0..expected_for_session_id_1.len() {
            let received_message_body = received_from_session_1[i].body()?;
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
            let received_message_body = received_from_session_2[i].body()?;
            assert_eq!(
                received_message_body,
                expected_for_session_id_2[i].as_bytes()
            );
        }

        Ok(())
    }

    async fn renew_session_lock_and_set_get_session_state() -> Result<(), anyhow::Error> {
        setup_dotenv();
        let connection_string = std::env::var("SERVICE_BUS_CONNECTION_STRING")?;
        let queue_name = std::env::var("SERVICE_BUS_SESSION_QUEUE")?;

        let mut client = ServiceBusClient::new(connection_string, Default::default()).await?;
        let mut sender = client
            .create_sender(&queue_name, Default::default())
            .await?;

        let mut session_message = ServiceBusMessage::new("test message");
        session_message.set_session_id(String::from("test_session_1"))?;
        sender.send_message(session_message).await?;

        let mut session_receiver = client
            .accept_next_session_for_queue(queue_name, Default::default())
            .await?;
        let received = session_receiver.receive_message().await?;
        session_receiver.complete_message(&received).await?;
        session_receiver.renew_session_lock().await?;
        session_receiver.set_session_state(vec![1, 2, 3]).await?;
        let state = session_receiver.session_state().await?;
        assert_eq!(state, vec![1, 2, 3]);

        sender.dispose().await?;
        session_receiver.dispose().await?;
        client.dispose().await?;
        Ok(())
    }

    async fn send_and_abandon_messages_then_receive_messages() -> Result<(), anyhow::Error> {
        setup_dotenv();
        let connection_string = std::env::var("SERVICE_BUS_CONNECTION_STRING")?;
        let queue_name = std::env::var("SERVICE_BUS_QUEUE")?;

        let expected = ["test message 1", "test message 2", "test message 3"];
        let messages = expected
            .iter()
            .map(|message| ServiceBusMessage::new(*message));

        common::create_client_and_send_messages_separately_to_queue_or_topic(
            &connection_string,
            Default::default(),
            &queue_name,
            Default::default(),
            messages,
        )
        .await?;

        common::create_client_and_abandon_messages_from_queue(
            &connection_string,
            Default::default(),
            &queue_name,
            Default::default(),
            expected.len() as u32,
            None,
        )
        .await?;

        let received = common::create_client_and_receive_messages_from_queue(
            &connection_string,
            Default::default(),
            &queue_name,
            Default::default(),
            expected.len() as u32,
            None,
        )
        .await?;

        assert_eq!(received.len(), expected.len());
        for i in 0..expected.len() {
            let received_message_body = received[i].body()?;
            assert_eq!(received_message_body, expected[i].as_bytes());
        }

        Ok(())
    }

    async fn send_and_deadletter_then_receive_from_deadletter_queue() -> Result<(), anyhow::Error> {
        setup_dotenv();
        let connection_string = std::env::var("SERVICE_BUS_CONNECTION_STRING")?;
        let queue_name = std::env::var("SERVICE_BUS_QUEUE")?;

        let expected = ["test message 1", "test message 2", "test message 3"];
        let messages = expected
            .iter()
            .map(|message| ServiceBusMessage::new(*message));

        common::create_client_and_send_messages_separately_to_queue_or_topic(
            &connection_string,
            Default::default(),
            &queue_name,
            Default::default(),
            messages,
        )
        .await?;

        common::create_client_and_deadletter_messages_from_queue(
            &connection_string,
            Default::default(),
            &queue_name,
            Default::default(),
            expected.len() as u32,
            None,
        )
        .await?;

        let mut receiver_options = ServiceBusReceiverOptions::default();
        receiver_options.sub_queue = SubQueue::DeadLetter;
        let received = common::create_client_and_receive_messages_from_queue(
            &connection_string,
            Default::default(),
            &queue_name,
            receiver_options,
            expected.len() as u32,
            None,
        )
        .await?;

        assert_eq!(received.len(), expected.len());
        for i in 0..expected.len() {
            let received_message_body = received[i].body()?;
            assert_eq!(received_message_body, expected[i].as_bytes());
        }

        Ok(())
    }

    async fn schedule_and_receive_messages() -> Result<(), anyhow::Error> {
        setup_dotenv();
        let connection_string = std::env::var("SERVICE_BUS_CONNECTION_STRING")?;
        let queue_name = std::env::var("SERVICE_BUS_QUEUE")?;

        let expected = ["test message 1", "test message 2", "test message 3"];
        let messages = expected
            .iter()
            .map(|message| ServiceBusMessage::new(*message));

        let wait_time = StdDuration::from_secs(30);
        let enqueue_time = OffsetDateTime::now_utc() + wait_time;
        let sequence_numbers = common::create_client_and_schedule_messages(
            &connection_string,
            Default::default(),
            &queue_name,
            Default::default(),
            messages,
            enqueue_time,
        )
        .await?;

        assert_eq!(sequence_numbers.len(), expected.len());

        tokio::time::sleep(wait_time).await;

        let received = common::create_client_and_receive_messages_from_queue(
            &connection_string,
            Default::default(),
            &queue_name,
            Default::default(),
            expected.len() as u32,
            None,
        )
        .await?;

        assert_eq!(received.len(), expected.len());
        for i in 0..expected.len() {
            let received_message_body = received[i].body()?;
            assert_eq!(received_message_body, expected[i].as_bytes());
        }

        Ok(())
    }

    async fn schedule_and_cancel_scheduled_messages() -> Result<(), anyhow::Error> {
        setup_dotenv();
        let connection_string = std::env::var("SERVICE_BUS_CONNECTION_STRING")?;
        let queue_name = std::env::var("SERVICE_BUS_QUEUE")?;

        let expected = ["test message 1", "test message 2", "test message 3"];
        let messages = expected
            .iter()
            .map(|message| ServiceBusMessage::new(*message));

        let mut client = ServiceBusClient::new(&connection_string, Default::default()).await?;
        let mut sender = client
            .create_sender(&queue_name, Default::default())
            .await?;

        let wait_time = StdDuration::from_secs(30);
        let enqueue_time = OffsetDateTime::now_utc() + wait_time;
        let sequence_numbers = sender.schedule_messages(messages, enqueue_time).await?;

        for seq in sequence_numbers {
            sender.cancel_scheduled_message(seq).await?;
        }

        tokio::time::sleep(wait_time).await;
        let mut client_options = ServiceBusClientOptions::default();
        client_options.retry_options = common::zero_retry_options();

        let received = common::create_client_and_receive_messages_from_queue(
            &connection_string,
            client_options,
            &queue_name,
            Default::default(),
            expected.len() as u32,
            None,
        )
        .await?;
        assert!(received.is_empty());

        Ok(())
    }

    async fn send_and_peek_messages() -> Result<(), anyhow::Error> {
        setup_dotenv();
        let connection_string = std::env::var("SERVICE_BUS_CONNECTION_STRING")?;
        let queue_name = std::env::var("SERVICE_BUS_QUEUE")?;

        let expected = ["test message 1", "test message 2", "test message 3"];
        let messages = expected
            .iter()
            .map(|message| ServiceBusMessage::new(*message));

        common::create_client_and_send_messages_separately_to_queue_or_topic(
            &connection_string,
            Default::default(),
            &queue_name,
            Default::default(),
            messages,
        )
        .await?;

        let peeked = common::create_client_and_peek_messages(
            &connection_string,
            Default::default(),
            &queue_name,
            Default::default(),
            expected.len() as u32,
        )
        .await?;

        assert_eq!(peeked.len(), expected.len());
        for i in 0..expected.len() {
            let peeked_message_body = peeked[i].body()?;
            assert_eq!(peeked_message_body, expected[i].as_bytes());
        }

        // Removed the peeked messages from the queue
        let _received = common::create_client_and_receive_messages_from_queue(
            &connection_string,
            Default::default(),
            &queue_name,
            Default::default(),
            expected.len() as u32,
            None,
        )
        .await?;

        Ok(())
    }

    async fn defer_and_receive_deferred_messages() -> Result<(), anyhow::Error> {
        setup_dotenv();
        let connection_string = std::env::var("SERVICE_BUS_CONNECTION_STRING")?;
        let queue_name = std::env::var("SERVICE_BUS_QUEUE")?;

        let expected = ["test message 1", "test message 2", "test message 3"];
        let messages = expected
            .iter()
            .map(|message| ServiceBusMessage::new(*message));

        common::create_client_and_send_messages_separately_to_queue_or_topic(
            &connection_string,
            Default::default(),
            &queue_name,
            Default::default(),
            messages,
        )
        .await?;

        let seq_nums = common::create_client_and_defer_messages(
            &connection_string,
            Default::default(),
            &queue_name,
            Default::default(),
            expected.len() as u32,
            None,
        )
        .await?;

        assert_eq!(seq_nums.len(), expected.len());

        let received = common::create_client_and_receive_deferred_messages(
            &connection_string,
            Default::default(),
            &queue_name,
            Default::default(),
            seq_nums,
        )
        .await?;

        assert_eq!(received.len(), expected.len());
        for i in 0..expected.len() {
            let received_message_body = received[i].body()?;
            assert_eq!(received_message_body, expected[i].as_bytes());
        }

        Ok(())
    }

    async fn receive_and_renew_lock() -> Result<(), anyhow::Error> {
        setup_dotenv();
        let connection_string = std::env::var("SERVICE_BUS_CONNECTION_STRING")?;
        let queue_name = std::env::var("SERVICE_BUS_QUEUE")?;

        let message = ["test message 1"];
        let messages = message
            .iter()
            .map(|message| ServiceBusMessage::new(*message));
        common::create_client_and_send_messages_separately_to_queue_or_topic(
            &connection_string,
            Default::default(),
            &queue_name,
            Default::default(),
            messages,
        )
        .await?;

        let mut client = ServiceBusClient::new(&connection_string, Default::default()).await?;
        let mut receiver = client
            .create_receiver_for_queue(queue_name, Default::default())
            .await?;

        let mut message = receiver
            .receive_message_with_max_wait_time(None)
            .await
            .unwrap()
            .expect("Expected a message");
        let old_locked_until = message.locked_until();

        receiver.renew_message_lock(&mut message).await?;
        receiver.complete_message(&message).await?;

        let new_locked_until = message.locked_until();
        match (old_locked_until, new_locked_until) {
            (Some(old), Some(new)) => {
                assert!(new > old);
            }
            _ => panic!("Expected locked_until to be set"),
        }

        Ok(())
    }

    async fn set_time_to_live() -> Result<(), anyhow::Error> {
        use std::time::Duration;

        setup_dotenv();
        let connection_string = std::env::var("SERVICE_BUS_CONNECTION_STRING")?;
        let queue_name = std::env::var("SERVICE_BUS_QUEUE")?;

        let mut client =
            ServiceBusClient::new(connection_string, ServiceBusClientOptions::default()).await?;

        // Create a sender and then send a batch of messages
        let mut sender = client
            .create_sender(&queue_name, Default::default())
            .await?;

        let time_to_live = Duration::from_secs(600);
        let mut message = ServiceBusMessage::new("test message 1");
        message.set_time_to_live(time_to_live)?;
        sender.send_message(message).await?;

        // Create a receiver and then receive the messages
        let mut receiver = client
            .create_receiver_for_queue(queue_name, Default::default())
            .await?;
        // This will wait indefinitely until at least one message is received
        let received = receiver.receive_message().await?;
        receiver.complete_message(&received).await?;
        assert_eq!(received.time_to_live(), Some(time_to_live));

        sender.dispose().await?;
        receiver.dispose().await?;
        client.dispose().await?;
        Ok(())
    }
}

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
        authorization::AzureNamedKeyCredential,
        ServiceBusClient, ServiceBusClientOptions,
        ServiceBusTransportType,
    };

    mod common;
    use common::setup_dotenv;

    #[tokio::test]
    async fn client_can_connect_using_connection_string_over_amqp_tcp() {
        setup_dotenv();

        let connection_string = std::env::var("SERVICE_BUS_CONNECTION_STRING").unwrap();
        let mut option = ServiceBusClientOptions::default();
        option.transport_type = ServiceBusTransportType::AmqpTcp;

        let mut client = ServiceBusClient::new(&connection_string, option)
            .await
            .unwrap();

        // Create a sender for authentication purpose only.
        // Do not create a receiver as it may accidentally receive messages from other tests.
        let queue_name = std::env::var("SERVICE_BUS_QUEUE").unwrap();
        let sender = client
            .create_sender(queue_name, Default::default())
            .await
            .unwrap();
        sender.dispose().await.unwrap();

        client.dispose().await.unwrap();
    }

    #[tokio::test]
    async fn client_can_connect_using_connection_string_over_amqp_websocket() {
        setup_dotenv();

        let connection_string = std::env::var("SERVICE_BUS_CONNECTION_STRING").unwrap();
        let mut option = ServiceBusClientOptions::default();
        option.transport_type = ServiceBusTransportType::AmqpWebSocket;

        let mut client = ServiceBusClient::new(&connection_string, option)
            .await
            .unwrap();

        // Create a sender for authentication purpose only.
        // Do not create a receiver as it may accidentally receive messages from other tests.
        let queue_name = std::env::var("SERVICE_BUS_QUEUE").unwrap();
        let sender = client
            .create_sender(queue_name, Default::default())
            .await
            .unwrap();
        sender.dispose().await.unwrap();

        tokio::time::sleep(std::time::Duration::from_secs(2)).await;
        client.dispose().await.unwrap();
    }

    #[tokio::test]
    async fn client_can_connect_using_named_key_credential() {
        setup_dotenv();
        let namespace = std::env::var("SERVICE_BUS_NAMESPACE").unwrap();
        let key_name = std::env::var("SERVICE_BUS_SAS_KEY_NAME").unwrap();
        let key = std::env::var("SERVICE_BUS_SAS_KEY").unwrap();
        let queue_name = std::env::var("SERVICE_BUS_QUEUE").unwrap();

        let credential = AzureNamedKeyCredential::new(key_name, key);
        let mut client =
            ServiceBusClient::new_with_named_key_credential(namespace, credential, Default::default())
                .await
                .unwrap();

        // Creating sender will perform CBS authentication.
        // Do not create a receiver as it may accidentally receive messages from other tests.
        let sender = client
            .create_sender(queue_name.clone(), Default::default())
            .await
            .unwrap();

        sender.dispose().await.unwrap();
        client.dispose().await.unwrap();
    }
}

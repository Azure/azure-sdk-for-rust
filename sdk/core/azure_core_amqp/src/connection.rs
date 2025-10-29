// Copyright (c) Microsoft Corporation. All Rights reserved
// Licensed under the MIT license.

use crate::{
    error::Result,
    value::{AmqpOrderedMap, AmqpSymbol, AmqpValue},
};
use azure_core::{http::Url, time::Duration};
use std::fmt::Debug;

#[cfg(all(feature = "fe2o3_amqp", not(target_arch = "wasm32")))]
type ConnectionImplementation = super::fe2o3::connection::Fe2o3AmqpConnection;

#[cfg(any(not(feature = "fe2o3_amqp"), target_arch = "wasm32"))]
type ConnectionImplementation = super::noop::NoopAmqpConnection;

/// Options for configuring an AMQP connection.
#[derive(Debug, Default, Clone)]
pub struct AmqpConnectionOptions {
    /// Maximum frame size for the connection in bytes.
    pub max_frame_size: Option<u32>,
    /// Maximum number of channels for the connection.
    pub channel_max: Option<u16>,
    /// Idle timeout for the connection.
    pub idle_timeout: Option<Duration>,
    /// List of outgoing locales for the connection.
    pub outgoing_locales: Option<Vec<String>>,
    /// List of incoming locales for the connection.
    pub incoming_locales: Option<Vec<String>>,
    /// List of offered capabilities for the connection.
    pub offered_capabilities: Option<Vec<AmqpSymbol>>,
    /// List of desired capabilities for the connection.
    pub desired_capabilities: Option<Vec<AmqpSymbol>>,
    /// Properties for the connection.
    pub properties: Option<AmqpOrderedMap<AmqpSymbol, AmqpValue>>,
    /// Buffer size for the connection.
    pub buffer_size: Option<usize>,
    /// Custom endpoint for the connection. Used to connect to a local AMQP proxy server.
    pub custom_endpoint: Option<Url>,
}

impl AmqpConnectionOptions {}

/// Trait defining the asynchronous APIs for AMQP connection operations.
#[cfg_attr(target_arch = "wasm32", async_trait::async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait::async_trait)]
pub trait AmqpConnectionApis {
    /// Asynchronously opens an AMQP connection.
    ///
    /// # Arguments
    /// - `name`: The name of the connection.
    /// - `url`: The URL of the AMQP broker.
    /// - `options`: Optional connection options.
    async fn open(
        &self,
        name: String,
        url: Url,
        options: Option<AmqpConnectionOptions>,
    ) -> Result<()>;

    /// Asynchronously closes the AMQP connection.
    async fn close(&self) -> Result<()>;

    /// Asynchronously closes the AMQP connection with an error condition.
    ///
    /// # Arguments
    /// - `condition`: The error condition as an `AmqpSymbol`.
    /// - `description`: An optional description of the error.
    /// - `info`: Optional additional information as an `AmqpOrderedMap`.
    async fn close_with_error(
        &self,
        condition: AmqpSymbol,
        description: Option<String>,
        info: Option<AmqpOrderedMap<AmqpSymbol, AmqpValue>>,
    ) -> Result<()>;
}

/// Struct representing an AMQP connection.
#[derive(Default)]
pub struct AmqpConnection {
    pub(crate) implementation: ConnectionImplementation,
}

#[cfg_attr(target_arch = "wasm32", async_trait::async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait::async_trait)]
impl AmqpConnectionApis for AmqpConnection {
    async fn open(
        &self,
        name: String,
        url: Url,
        options: Option<AmqpConnectionOptions>,
    ) -> Result<()> {
        self.implementation.open(name, url, options).await
    }

    async fn close(&self) -> Result<()> {
        self.implementation.close().await
    }

    async fn close_with_error(
        &self,
        condition: AmqpSymbol,
        description: Option<String>,
        info: Option<AmqpOrderedMap<AmqpSymbol, AmqpValue>>,
    ) -> Result<()> {
        self.implementation
            .close_with_error(condition, description, info)
            .await
    }
}

impl AmqpConnection {
    /// Creates a new instance of `AmqpConnection`.
    pub fn new() -> Self {
        Self {
            implementation: ConnectionImplementation::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn amqp_connection_options_with_max_frame_size() {
        let connection_options = AmqpConnectionOptions {
            max_frame_size: Some(1024),
            ..Default::default()
        };
        assert_eq!(connection_options.max_frame_size, Some(1024));
    }

    #[test]
    fn test_amqp_connection_options_with_channel_max() {
        let connection_options = AmqpConnectionOptions {
            channel_max: Some(16),
            ..Default::default()
        };

        assert_eq!(connection_options.channel_max, Some(16));
    }

    #[test]
    fn test_amqp_connection_options_with_idle_timeout() {
        let idle_timeout = Duration::minutes(1);
        let connection_options = AmqpConnectionOptions {
            idle_timeout: Some(idle_timeout),
            ..Default::default()
        };

        assert_eq!(connection_options.idle_timeout, Some(idle_timeout));
    }

    #[test]
    fn test_amqp_connection_options_with_outgoing_locales() {
        let outgoing_locales = vec!["en-US".to_string()];
        let connection_options = AmqpConnectionOptions {
            outgoing_locales: Some(outgoing_locales.clone()),
            ..Default::default()
        };

        assert_eq!(connection_options.outgoing_locales, Some(outgoing_locales));
    }

    #[test]
    fn test_amqp_connection_options_with_incoming_locales() {
        let incoming_locales = vec!["en-US".to_string()];
        let connection_options = AmqpConnectionOptions {
            incoming_locales: Some(incoming_locales.clone()),
            ..Default::default()
        };

        assert_eq!(connection_options.incoming_locales, Some(incoming_locales));
    }

    #[test]
    fn test_amqp_connection_options_with_offered_capabilities() {
        let offered_capabilities = vec!["capability".into()];
        let connection_options = AmqpConnectionOptions {
            offered_capabilities: Some(offered_capabilities.clone()),
            ..Default::default()
        };

        assert_eq!(
            connection_options.offered_capabilities,
            Some(offered_capabilities)
        );
    }

    #[test]
    fn test_amqp_connection_options_with_desired_capabilities() {
        let desired_capabilities = vec!["capability".into()];
        let connection_options = AmqpConnectionOptions {
            desired_capabilities: Some(desired_capabilities.clone()),
            ..Default::default()
        };

        assert_eq!(
            connection_options.desired_capabilities,
            Some(desired_capabilities)
        );
    }

    #[test]
    fn test_amqp_connection_options_with_properties() {
        let properties = vec![("key", "value")];
        let connection_options = AmqpConnectionOptions {
            properties: Some(
                properties
                    .iter()
                    .map(|(k, v)| (AmqpSymbol::from(*k), AmqpValue::from(*v)))
                    .collect(),
            ),
            ..Default::default()
        };

        let properties_map: AmqpOrderedMap<AmqpSymbol, AmqpValue> = properties
            .into_iter()
            .map(|(k, v)| (k.into(), v.into()))
            .collect();

        assert_eq!(connection_options.properties, Some(properties_map));
    }

    #[test]
    fn test_amqp_connection_options_with_buffer_size() {
        let buffer_size = 1024;
        let connection_options = AmqpConnectionOptions {
            buffer_size: Some(buffer_size),
            ..Default::default()
        };

        assert_eq!(connection_options.buffer_size, Some(buffer_size));
    }

    #[test]
    fn amqp_connection_options() {
        let connection_options = AmqpConnectionOptions {
            max_frame_size: Some(1024),
            channel_max: Some(16),
            idle_timeout: Some(Duration::minutes(1)),
            outgoing_locales: Some(vec!["en-US".to_string()]),
            incoming_locales: Some(vec!["en-US".to_string()]),
            offered_capabilities: Some(vec!["capability".into()]),
            desired_capabilities: Some(vec!["capability".into()]),
            custom_endpoint: Some(Url::parse("http://localhost:8080").unwrap()),
            properties: Some(
                vec![("key", "value")]
                    .into_iter()
                    .map(|(k, v)| (k.into(), v.into()))
                    .collect(),
            ),
            buffer_size: Some(1024),
        };

        assert_eq!(connection_options.max_frame_size, Some(1024));
        assert_eq!(connection_options.channel_max, Some(16));
        assert_eq!(connection_options.idle_timeout, Some(Duration::minutes(1)));
        assert_eq!(
            connection_options.outgoing_locales,
            Some(vec!["en-US".to_string()])
        );
        assert_eq!(
            connection_options.incoming_locales,
            Some(vec!["en-US".to_string()])
        );
        assert_eq!(
            connection_options.offered_capabilities,
            Some(vec!["capability".into()])
        );
        assert_eq!(
            connection_options.desired_capabilities,
            Some(vec!["capability".into()])
        );
        assert_eq!(
            connection_options.properties,
            Some(
                vec![("key".into(), "value".into())].into_iter().collect() // convert to AmqpOrderedMap
            )
        );
        assert_eq!(
            connection_options.custom_endpoint,
            Some(Url::parse("http://localhost:8080").unwrap())
        );
    }

    // On macOS, there is a periodic issue where loopback TCP connections fail.
    // Disable these tests on macOS.
    #[cfg(not(target_os = "macos"))]
    #[tokio::test]
    async fn amqp_connection_open() {
        let address = std::env::var("TEST_BROKER_ADDRESS");
        if address.is_ok() {
            let connection = AmqpConnection::new();
            let url = Url::parse(&address.unwrap()).unwrap();
            connection
                .open("test".to_string(), url, None)
                .await
                .unwrap();
        } else {
            println!("TEST_BROKER_ADDRESS is not set. Skipping test.");
        }
    }

    #[tokio::test]
    async fn amqp_connection_open_with_error() {
        let address = std::env::var("TEST_BROKER_ADDRESS");
        if address.is_ok() {
            let connection = AmqpConnection::new();
            let url = Url::parse("amqp://localhost:32767").unwrap();
            assert!(connection
                .open("test".to_string(), url, None)
                .await
                .is_err());
        } else {
            println!("TEST_BROKER_ADDRESS is not set. Skipping test.");
        }
    }

    #[cfg(not(target_os = "macos"))]
    #[tokio::test]
    async fn amqp_connection_close() {
        let address = std::env::var("TEST_BROKER_ADDRESS");
        if address.is_ok() {
            let connection = AmqpConnection::new();
            let url = Url::parse(&address.unwrap()).unwrap();
            connection
                .open("test".to_string(), url, None)
                .await
                .unwrap();
            connection.close().await.unwrap();
        } else {
            println!("TEST_BROKER_ADDRESS is not set. Skipping test.");
        }
    }

    #[cfg(not(target_os = "macos"))]
    #[tokio::test]
    async fn amqp_connection_close_with_error() {
        tracing_subscriber::fmt::init();
        let address = std::env::var("TEST_BROKER_ADDRESS");
        if address.is_ok() {
            let connection = AmqpConnection::new();
            let url = Url::parse(&address.unwrap()).unwrap();
            connection
                .open("test".to_string(), url, None)
                .await
                .unwrap();
            let res = connection
                .close_with_error(
                    AmqpSymbol::from("amqp:internal-error"),
                    Some("Internal error.".to_string()),
                    None,
                )
                .await;
            match res {
                Ok(_) => {}
                Err(err) => {
                    println!("Error: {:?}", err);
                    assert!(err.to_string().contains("Internal error."));
                }
            }
        } else {
            println!("TEST_BROKER_ADDRESS is not set. Skipping test.");
        }
    }
}

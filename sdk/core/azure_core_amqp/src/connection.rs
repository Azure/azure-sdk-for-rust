// Copyright (c) Microsoft Corporation. All Rights reserved
// Licensed under the MIT license.

use super::value::{AmqpOrderedMap, AmqpSymbol, AmqpValue};
use azure_core::{error::Result, Url};
use std::fmt::Debug;
use time::Duration;

#[cfg(all(feature = "fe2o3-amqp", not(target_arch = "wasm32")))]
type ConnectionImplementation = super::fe2o3::connection::Fe2o3AmqpConnection;

#[cfg(any(not(feature = "fe2o3-amqp"), target_arch = "wasm32"))]
type ConnectionImplementation = super::noop::NoopAmqpConnection;

#[derive(Debug, Default, Clone)]
pub struct AmqpConnectionOptions {
    pub max_frame_size: Option<u32>,
    pub channel_max: Option<u16>,
    pub idle_timeout: Option<Duration>,
    pub outgoing_locales: Option<Vec<String>>,
    pub incoming_locales: Option<Vec<String>>,
    pub offered_capabilities: Option<Vec<AmqpSymbol>>,
    pub desired_capabilities: Option<Vec<AmqpSymbol>>,
    pub properties: Option<AmqpOrderedMap<AmqpSymbol, AmqpValue>>,
    pub buffer_size: Option<usize>,
}

impl AmqpConnectionOptions {}

pub trait AmqpConnectionApis {
    fn open(
        &self,
        name: String,
        url: Url,
        options: Option<AmqpConnectionOptions>,
    ) -> impl std::future::Future<Output = Result<()>>;
    fn close(&self) -> impl std::future::Future<Output = Result<()>>;
    fn close_with_error(
        &self,
        condition: AmqpSymbol,
        description: Option<String>,
        info: Option<AmqpOrderedMap<AmqpSymbol, AmqpValue>>,
    ) -> impl std::future::Future<Output = Result<()>>;
}

#[derive(Default)]
pub struct AmqpConnection {
    pub(crate) implementation: ConnectionImplementation,
}

impl AmqpConnectionApis for AmqpConnection {
    fn open(
        &self,
        name: String,
        url: Url,
        options: Option<AmqpConnectionOptions>,
    ) -> impl std::future::Future<Output = Result<()>> {
        self.implementation.open(name, url, options)
    }
    fn close(&self) -> impl std::future::Future<Output = Result<()>> {
        self.implementation.close()
    }
    fn close_with_error(
        &self,
        condition: AmqpSymbol,
        description: Option<String>,
        info: Option<AmqpOrderedMap<AmqpSymbol, AmqpValue>>,
    ) -> impl std::future::Future<Output = Result<()>> {
        self.implementation
            .close_with_error(condition, description, info)
    }
}

impl AmqpConnection {
    pub fn new() -> Self {
        Self {
            implementation: ConnectionImplementation::new(),
        }
    }
}

pub(crate) mod error {

    use crate::{error::AmqpErrorKind, AmqpError};

    pub enum AmqpConnectionError {
        ConnectionNotSet,

        ConnectionAlreadySet,

        /// Domain is invalid or not found
        InvalidDomain,

        /// Missing client config for TLS connection
        TlsConnectorNotFound,

        /// Scheme is invalid or not found
        InvalidScheme,

        /// Protocol negotiation failed due to protocol header mismatch
        ProtocolHeaderMismatch(Box<dyn std::error::Error + Send + Sync>),

        /// SASL negotiation failed
        SaslError(Box<dyn std::error::Error + Send + Sync>),

        /// Illegal local connection state
        IllegalState,

        /// Not implemented
        NotImplemented(Option<String>),

        /// Decode error
        DecodeError(String),

        /// Session is not found
        NotFound(Option<String>),

        /// Not allowed
        NotAllowed(Option<String>),

        /// This could occur only when the user attempts to close the connection
        JoinError(Box<dyn std::error::Error + Send + Sync>),

        /// Idle timeout elapsed
        IdleTimeoutElapsed,

        /// Framing error
        FramingError,
    }

    impl From<AmqpConnectionError> for AmqpErrorKind {
        fn from(e: AmqpConnectionError) -> Self {
            AmqpErrorKind::ConnectionError(e)
        }
    }

    impl From<AmqpConnectionError> for azure_core::Error {
        fn from(e: AmqpConnectionError) -> Self {
            AmqpError::from(AmqpErrorKind::from(e)).into()
        }
    }
    impl std::fmt::Display for AmqpConnectionError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::ConnectionNotSet => write!(f, "Connection not set"),
                Self::ConnectionAlreadySet => write!(f, "Connection already set"),
                Self::FramingError => write!(f, "Framing error"),
                Self::IdleTimeoutElapsed => write!(f, "Idle timeout elapsed"),
                Self::NotFound(e) => {
                    if let Some(e) = e {
                        write!(f, "Not found: {}", e)
                    } else {
                        write!(f, "Not found")
                    }
                }
                Self::NotAllowed(e) => {
                    if let Some(e) = e {
                        write!(f, "Not allowed: {}", e)
                    } else {
                        write!(f, "Not allowed")
                    }
                }
                Self::JoinError(e) => {
                    write!(f, "Join error: {}", e)
                }
                Self::InvalidDomain => write!(f, "Invalid domain"),
                Self::TlsConnectorNotFound => {
                    write!(f, "TLS connector is not found")
                }
                Self::InvalidScheme => {
                    write!(
                        f,
                        r#"Invalid scheme. Only "amqp" and "amqps" are supported."#
                    )
                }
                Self::ProtocolHeaderMismatch(e) => {
                    write!(f, "Protocol header mismatch: {:?}", e)
                }
                Self::SaslError(e) => {
                    write!(f, "SASL error code {}", e)
                }
                Self::IllegalState => write!(f, "Illegal local state"),
                Self::NotImplemented(e) => {
                    if let Some(e) = e {
                        write!(f, "Not implemented: {}", e)
                    } else {
                        write!(f, "Not implemented")
                    }
                }
                Self::DecodeError(e) => write!(f, "Decode error: {}", e),
                //                Self::TransportError(e) => write!(f, "Transport error: {}", e),
                //                Self::RemoteClosed => write!(f, "Remote peer closed"),
                //                Self::RemoteClosedWithError(e) => {
                //                    write!(f, "Remote peer closed connection with error: {:?}", e)
                //                }
            }
        }
    }

    impl std::fmt::Debug for AmqpConnectionError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "AmqpConnectionError: {}", self)
        }
    }

    impl std::error::Error for AmqpConnectionError {}
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
        let idle_timeout = time::Duration::seconds(60);
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
            idle_timeout: Some(time::Duration::seconds(60)),
            outgoing_locales: Some(vec!["en-US".to_string()]),
            incoming_locales: Some(vec!["en-US".to_string()]),
            offered_capabilities: Some(vec!["capability".into()]),
            desired_capabilities: Some(vec!["capability".into()]),
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
        assert_eq!(
            connection_options.idle_timeout,
            Some(time::Duration::seconds(60))
        );
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
    }

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

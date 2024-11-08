// Copyright (c) Microsoft Corporation. All Rights reserved
// Licensed under the MIT license.
// cspell: words amqp sasl

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

impl AmqpConnectionOptions {
    pub fn max_frame_size(&self) -> Option<u32> {
        self.max_frame_size
    }
    pub fn channel_max(&self) -> Option<u16> {
        self.channel_max
    }
    pub fn idle_timeout(&self) -> Option<&Duration> {
        self.idle_timeout.as_ref()
    }
    pub fn outgoing_locales(&self) -> Option<&Vec<String>> {
        self.outgoing_locales.as_ref()
    }
    pub fn incoming_locales(&self) -> Option<&Vec<String>> {
        self.incoming_locales.as_ref()
    }
    pub fn offered_capabilities(&self) -> Option<&Vec<AmqpSymbol>> {
        self.offered_capabilities.as_ref()
    }
    pub fn desired_capabilities(&self) -> Option<&Vec<AmqpSymbol>> {
        self.desired_capabilities.as_ref()
    }
    pub fn properties(&self) -> Option<&AmqpOrderedMap<AmqpSymbol, AmqpValue>> {
        self.properties.as_ref()
    }
    pub fn buffer_size(&self) -> Option<usize> {
        self.buffer_size
    }
}

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_amqp_connection_options_with_max_frame_size() {
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
    fn test_amqp_connection_options() {
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
}

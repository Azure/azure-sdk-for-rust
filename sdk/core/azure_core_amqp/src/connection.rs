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
    pub(crate) max_frame_size: Option<u32>,
    pub(crate) channel_max: Option<u16>,
    pub(crate) idle_timeout: Option<Duration>,
    pub(crate) outgoing_locales: Option<Vec<String>>,
    pub(crate) incoming_locales: Option<Vec<String>>,
    pub(crate) offered_capabilities: Option<Vec<AmqpSymbol>>,
    pub(crate) desired_capabilities: Option<Vec<AmqpSymbol>>,
    pub(crate) properties: Option<AmqpOrderedMap<AmqpSymbol, AmqpValue>>,
    pub(crate) buffer_size: Option<usize>,
}

impl AmqpConnectionOptions {
    pub fn builder() -> builders::AmqpConnectionOptionsBuilder {
        builders::AmqpConnectionOptionsBuilder::new()
    }

    pub fn max_frame_size(&self) -> Option<u32> {
        self.max_frame_size
    }
    pub fn channel_max(&self) -> Option<u16> {
        self.channel_max
    }
    pub fn idle_timeout(&self) -> Option<Duration> {
        self.idle_timeout
    }
    pub fn outgoing_locales(&self) -> Option<Vec<String>> {
        self.outgoing_locales.clone()
    }
    pub fn incoming_locales(&self) -> Option<Vec<String>> {
        self.incoming_locales.clone()
    }
    pub fn offered_capabilities(&self) -> Option<Vec<AmqpSymbol>> {
        self.offered_capabilities.clone()
    }
    pub fn desired_capabilities(&self) -> Option<Vec<AmqpSymbol>> {
        self.desired_capabilities.clone()
    }
    pub fn properties(&self) -> Option<AmqpOrderedMap<AmqpSymbol, AmqpValue>> {
        self.properties.clone()
    }
    pub fn buffer_size(&self) -> Option<usize> {
        self.buffer_size
    }
}

pub trait AmqpConnectionApis {
    fn open(
        &self,
        name: impl Into<String>,
        url: Url,
        options: Option<AmqpConnectionOptions>,
    ) -> impl std::future::Future<Output = Result<()>>;
    fn close(&self) -> impl std::future::Future<Output = Result<()>>;
    fn close_with_error(
        &self,
        condition: impl Into<AmqpSymbol>,
        description: Option<String>,
        info: Option<AmqpOrderedMap<AmqpSymbol, AmqpValue>>,
    ) -> impl std::future::Future<Output = Result<()>>;
}

#[derive(Debug, Default)]
pub struct AmqpConnection {
    pub(crate) implementation: ConnectionImplementation,
}

impl AmqpConnectionApis for AmqpConnection {
    fn open(
        &self,
        name: impl Into<String>,
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
        condition: impl Into<AmqpSymbol>,
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

pub mod builders {
    use super::*;
    pub struct AmqpConnectionOptionsBuilder {
        options: AmqpConnectionOptions,
    }

    impl AmqpConnectionOptionsBuilder {
        pub(super) fn new() -> Self {
            Self {
                options: Default::default(),
            }
        }
        pub fn build(&mut self) -> AmqpConnectionOptions {
            self.options.clone()
        }
        pub fn with_max_frame_size(&mut self, max_frame_size: u32) -> &mut Self {
            self.options.max_frame_size = Some(max_frame_size);
            self
        }
        pub fn with_channel_max(&mut self, channel_max: u16) -> &mut Self {
            self.options.channel_max = Some(channel_max);
            self
        }
        pub fn with_idle_timeout(&mut self, idle_timeout: Duration) -> &mut Self {
            self.options.idle_timeout = Some(idle_timeout);
            self
        }
        pub fn with_outgoing_locales(&mut self, outgoing_locales: Vec<String>) -> &mut Self {
            self.options.outgoing_locales = Some(outgoing_locales);
            self
        }
        pub fn with_incoming_locales(&mut self, incoming_locales: Vec<String>) -> &mut Self {
            self.options.incoming_locales = Some(incoming_locales);
            self
        }
        pub fn with_offered_capabilities(
            &mut self,
            offered_capabilities: Vec<AmqpSymbol>,
        ) -> &mut Self {
            self.options.offered_capabilities = Some(offered_capabilities);
            self
        }
        pub fn with_desired_capabilities(
            &mut self,
            desired_capabilities: Vec<AmqpSymbol>,
        ) -> &mut Self {
            self.options.desired_capabilities = Some(desired_capabilities);
            self
        }
        pub fn with_properties<K, V>(
            &mut self,
            properties: impl Into<AmqpOrderedMap<K, V>>,
        ) -> &mut Self
        where
            K: Into<AmqpSymbol> + Debug + Default + PartialEq,
            V: Into<AmqpValue> + Debug + Default,
        {
            let properties_map: AmqpOrderedMap<K, V> = properties.into();
            let properties_map = properties_map
                .into_iter()
                .map(|(k, v)| (k.into(), v.into()))
                .collect();
            self.options.properties = Some(properties_map);
            self
        }
        pub fn with_buffer_size(&mut self, buffer_size: usize) -> &mut Self {
            self.options.buffer_size = Some(buffer_size);
            self
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_amqp_connection_builder_with_max_frame_size() {
        let connection_options = AmqpConnectionOptions::builder()
            .with_max_frame_size(1024)
            .build();

        assert_eq!(connection_options.max_frame_size, Some(1024));
    }

    #[test]
    fn test_amqp_connection_builder_with_channel_max() {
        let connection_options = AmqpConnectionOptions::builder()
            .with_channel_max(16)
            .build();

        assert_eq!(connection_options.channel_max, Some(16));
    }

    #[test]
    fn test_amqp_connection_builder_with_idle_timeout() {
        let idle_timeout = time::Duration::seconds(60);
        let connection_options = AmqpConnectionOptions::builder()
            .with_idle_timeout(idle_timeout)
            .build();

        assert_eq!(connection_options.idle_timeout, Some(idle_timeout));
    }

    #[test]
    fn test_amqp_connection_builder_with_outgoing_locales() {
        let outgoing_locales = vec!["en-US".to_string()];
        let connection_options = AmqpConnectionOptions::builder()
            .with_outgoing_locales(outgoing_locales.clone())
            .build();

        assert_eq!(connection_options.outgoing_locales, Some(outgoing_locales));
    }

    #[test]
    fn test_amqp_connection_builder_with_incoming_locales() {
        let incoming_locales = vec!["en-US".to_string()];
        let connection_options = AmqpConnectionOptions::builder()
            .with_incoming_locales(incoming_locales.clone())
            .build();

        assert_eq!(connection_options.incoming_locales, Some(incoming_locales));
    }

    #[test]
    fn test_amqp_connection_builder_with_offered_capabilities() {
        let offered_capabilities = vec!["capability".into()];
        let connection_options = AmqpConnectionOptions::builder()
            .with_offered_capabilities(offered_capabilities.clone())
            .build();

        assert_eq!(
            connection_options.offered_capabilities,
            Some(offered_capabilities)
        );
    }

    #[test]
    fn test_amqp_connection_builder_with_desired_capabilities() {
        let desired_capabilities = vec!["capability".into()];
        let connection_options = AmqpConnectionOptions::builder()
            .with_desired_capabilities(desired_capabilities.clone())
            .build();

        assert_eq!(
            connection_options.desired_capabilities,
            Some(desired_capabilities)
        );
    }

    #[test]
    fn test_amqp_connection_builder_with_properties() {
        let properties = vec![("key", "value")];
        let connection_options = AmqpConnectionOptions::builder()
            .with_properties(properties.clone())
            .build();

        let properties_map: AmqpOrderedMap<AmqpSymbol, AmqpValue> = properties
            .into_iter()
            .map(|(k, v)| (k.into(), v.into()))
            .collect();

        assert_eq!(connection_options.properties, Some(properties_map));
    }

    #[test]
    fn test_amqp_connection_builder_with_buffer_size() {
        let buffer_size = 1024;
        let connection_options = AmqpConnectionOptions::builder()
            .with_buffer_size(buffer_size)
            .build();

        assert_eq!(connection_options.buffer_size, Some(buffer_size));
    }

    #[test]
    fn test_amqp_connection_builder() {
        let connection_options = AmqpConnectionOptions::builder()
            .with_max_frame_size(1024)
            .with_channel_max(16)
            .with_idle_timeout(time::Duration::seconds(60))
            .with_outgoing_locales(vec!["en-US".to_string()])
            .with_incoming_locales(vec!["en-US".to_string()])
            .with_offered_capabilities(vec!["capability".into()])
            .with_desired_capabilities(vec!["capability".into()])
            .with_properties(vec![("key", "value")])
            .with_buffer_size(1024)
            .build();

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

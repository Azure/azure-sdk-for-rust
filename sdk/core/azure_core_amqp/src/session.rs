// Copyright (c) Microsoft Corporation. All Rights reserved
// Licensed under the MIT license.
// cspell: words amqp sasl

use super::{
    connection::AmqpConnection,
    value::{AmqpOrderedMap, AmqpSymbol, AmqpValue},
};
use azure_core::error::Result;
use std::fmt::Debug;

#[cfg(all(feature = "fe2o3-amqp", not(target_arch = "wasm32")))]
type SessionImplementation = super::fe2o3::session::Fe2o3AmqpSession;

#[cfg(any(not(feature = "fe2o3-amqp"), target_arch = "wasm32"))]
type SessionImplementation = super::noop::NoopAmqpSession;

#[derive(Debug, Default, Clone)]
pub struct AmqpSessionOptions {
    next_outgoing_id: Option<u32>,
    incoming_window: Option<u32>,
    outgoing_window: Option<u32>,
    handle_max: Option<u32>,
    offered_capabilities: Option<Vec<AmqpSymbol>>,
    desired_capabilities: Option<Vec<AmqpSymbol>>,
    properties: Option<AmqpOrderedMap<AmqpSymbol, AmqpValue>>,
    buffer_size: Option<usize>,
}

impl AmqpSessionOptions {
    pub fn builder() -> builders::AmqpSessionOptionsBuilder {
        builders::AmqpSessionOptionsBuilder::new()
    }

    pub fn next_outgoing_id(&self) -> Option<u32> {
        self.next_outgoing_id
    }

    pub fn incoming_window(&self) -> Option<u32> {
        self.incoming_window
    }

    pub fn outgoing_window(&self) -> Option<u32> {
        self.outgoing_window
    }

    pub fn handle_max(&self) -> Option<u32> {
        self.handle_max
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

#[allow(unused_variables)]
pub trait AmqpSessionApis {
    fn begin(
        &self,
        connection: &AmqpConnection,
        options: Option<AmqpSessionOptions>,
    ) -> impl std::future::Future<Output = Result<()>>;
    fn end(&self) -> impl std::future::Future<Output = Result<()>>;
}

#[derive(Debug, Clone, Default)]
pub struct AmqpSession {
    pub(crate) implementation: SessionImplementation,
}

impl AmqpSessionApis for AmqpSession {
    async fn begin(
        &self,
        connection: &AmqpConnection,
        options: Option<AmqpSessionOptions>,
    ) -> Result<()> {
        self.implementation.begin(connection, options).await
    }

    async fn end(&self) -> Result<()> {
        self.implementation.end().await
    }
}

impl AmqpSession {
    pub fn new() -> Self {
        Self {
            implementation: SessionImplementation::new(),
        }
    }
}

pub mod builders {
    use super::*;

    pub struct AmqpSessionOptionsBuilder {
        options: AmqpSessionOptions,
    }

    impl AmqpSessionOptionsBuilder {
        pub(super) fn new() -> Self {
            Self {
                options: Default::default(),
            }
        }
        pub fn build(&self) -> AmqpSessionOptions {
            self.options.clone()
        }
        pub fn with_next_outgoing_id(mut self, next_outgoing_id: u32) -> Self {
            self.options.next_outgoing_id = Some(next_outgoing_id);
            self
        }
        pub fn with_incoming_window(mut self, incoming_window: u32) -> Self {
            self.options.incoming_window = Some(incoming_window);
            self
        }
        pub fn with_outgoing_window(mut self, outgoing_window: u32) -> Self {
            self.options.outgoing_window = Some(outgoing_window);
            self
        }
        pub fn with_handle_max(mut self, handle_max: u32) -> Self {
            self.options.handle_max = Some(handle_max);
            self
        }
        pub fn with_offered_capabilities(mut self, offered_capabilities: Vec<AmqpSymbol>) -> Self {
            self.options.offered_capabilities = Some(offered_capabilities);
            self
        }
        pub fn with_desired_capabilities(mut self, desired_capabilities: Vec<AmqpSymbol>) -> Self {
            self.options.desired_capabilities = Some(desired_capabilities);
            self
        }
        pub fn with_properties<K, V>(mut self, properties: impl Into<AmqpOrderedMap<K, V>>) -> Self
        where
            K: Into<AmqpSymbol> + PartialEq + Default + Debug,
            V: Into<AmqpValue> + Default,
        {
            let properties_map: AmqpOrderedMap<AmqpSymbol, AmqpValue> = properties
                .into()
                .into_iter()
                .map(|(k, v)| (k.into(), v.into()))
                .collect();
            self.options.properties = Some(properties_map);
            self
        }
        pub fn with_buffer_size(mut self, buffer_size: usize) -> Self {
            self.options.buffer_size = Some(buffer_size);
            self
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_amqp_session_options_builder() {
        let session_options = AmqpSessionOptions::builder()
            .with_next_outgoing_id(1)
            .with_incoming_window(1)
            .with_outgoing_window(1)
            .with_handle_max(1)
            .with_offered_capabilities(vec!["capability".into()])
            .with_desired_capabilities(vec!["capability".into()])
            .with_properties(vec![("key", "value")])
            .with_buffer_size(1024)
            .build();

        assert_eq!(session_options.next_outgoing_id, Some(1));
        assert_eq!(session_options.incoming_window, Some(1));
        assert_eq!(session_options.outgoing_window, Some(1));
        assert_eq!(session_options.handle_max, Some(1));
        assert_eq!(
            session_options.offered_capabilities,
            Some(vec!["capability".into()])
        );
        assert_eq!(
            session_options.desired_capabilities,
            Some(vec!["capability".into()])
        );
        assert!(session_options.properties.is_some());
        let properties = session_options.properties.clone().unwrap();
        assert!(properties.contains_key("key"));
        assert_eq!(
            *properties.get("key").unwrap(),
            AmqpValue::String("value".to_string())
        );

        assert_eq!(session_options.buffer_size, Some(1024));
    }
}

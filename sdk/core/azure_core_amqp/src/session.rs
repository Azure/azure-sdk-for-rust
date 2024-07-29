// Copyright (c) Microsoft Corp. All Rights Reserved.
// cspell: words amqp sasl

use super::{
    connection::AmqpConnection,
    value::{AmqpOrderedMap, AmqpSymbol, AmqpValue},
};
use azure_core::error::Result;
use std::fmt::Debug;

#[derive(Debug)]
pub struct AmqpSessionOptions {
    pub(crate) next_outgoing_id: Option<u32>,
    pub(crate) incoming_window: Option<u32>,
    pub(crate) outgoing_window: Option<u32>,
    pub(crate) handle_max: Option<u32>,
    pub(crate) offered_capabilities: Option<Vec<AmqpSymbol>>,
    pub(crate) desired_capabilities: Option<Vec<AmqpSymbol>>,
    pub(crate) properties: Option<AmqpOrderedMap<AmqpSymbol, AmqpValue>>,
    pub(crate) buffer_size: Option<usize>,
}

impl AmqpSessionOptions {
    pub fn builder() -> builders::AmqpSessionOptionsBuilder {
        builders::AmqpSessionOptionsBuilder::new()
    }
}

#[allow(unused_variables)]
pub trait AmqpSessionTrait {
    fn begin(
        &self,
        connection: &AmqpConnection,
        options: Option<AmqpSessionOptions>,
    ) -> impl std::future::Future<Output = Result<()>> {
        async { unimplemented!() }
    }
    fn end(&self) -> impl std::future::Future<Output = Result<()>> {
        async { unimplemented!() }
    }
}

#[derive(Debug, Clone, Default)]
pub(crate) struct AmqpSessionImpl<T>(pub(crate) T);

impl<T> AmqpSessionImpl<T>
where
    T: AmqpSessionTrait + Clone,
{
    pub(crate) fn new(session: T) -> Self {
        Self(session)
    }
}

#[cfg(all(feature = "enable-fe2o3-amqp", not(target_arch = "wasm32")))]
type SessionImplementation = super::fe2o3::session::Fe2o3AmqpSession;

#[cfg(any(not(feature = "enable-fe2o3-amqp"), target_arch = "wasm32"))]
type SessionImplementation = super::noop::NoopAmqpSession;

#[derive(Debug, Clone, Default)]
pub struct AmqpSession(pub(crate) AmqpSessionImpl<SessionImplementation>);

impl AmqpSessionTrait for AmqpSession {
    async fn begin(
        &self,
        connection: &AmqpConnection,
        options: Option<AmqpSessionOptions>,
    ) -> Result<()> {
        self.0 .0.begin(connection, options).await
    }

    async fn end(&self) -> Result<()> {
        self.0 .0.end().await
    }
}

impl AmqpSession {
    pub fn new() -> Self {
        Self(AmqpSessionImpl::new(SessionImplementation::new()))
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
                options: AmqpSessionOptions {
                    next_outgoing_id: None,
                    incoming_window: None,
                    outgoing_window: None,
                    handle_max: None,
                    offered_capabilities: None,
                    desired_capabilities: None,
                    properties: None,
                    buffer_size: None,
                },
            }
        }
        pub fn build(self) -> AmqpSessionOptions {
            self.options
        }
        #[allow(dead_code)]
        pub fn with_next_outgoing_id(mut self, next_outgoing_id: u32) -> Self {
            self.options.next_outgoing_id = Some(next_outgoing_id);
            self
        }
        #[allow(dead_code)]
        pub fn with_incoming_window(mut self, incoming_window: u32) -> Self {
            self.options.incoming_window = Some(incoming_window);
            self
        }
        #[allow(dead_code)]
        pub fn with_outgoing_window(mut self, outgoing_window: u32) -> Self {
            self.options.outgoing_window = Some(outgoing_window);
            self
        }
        #[allow(dead_code)]
        pub fn with_handle_max(mut self, handle_max: u32) -> Self {
            self.options.handle_max = Some(handle_max);
            self
        }
        #[allow(dead_code)]
        pub fn with_offered_capabilities(mut self, offered_capabilities: Vec<AmqpSymbol>) -> Self {
            self.options.offered_capabilities = Some(offered_capabilities);
            self
        }
        #[allow(dead_code)]
        pub fn with_desired_capabilities(mut self, desired_capabilities: Vec<AmqpSymbol>) -> Self {
            self.options.desired_capabilities = Some(desired_capabilities);
            self
        }
        #[allow(dead_code)]
        pub fn with_properties(mut self, properties: Vec<(&str, &str)>) -> Self {
            let properties_map: AmqpOrderedMap<AmqpSymbol, AmqpValue> = properties
                .into_iter()
                .map(|(k, v)| (AmqpSymbol::from(k), AmqpValue::from(v)))
                .collect();
            self.options.properties = Some(properties_map);
            self
        }
        #[allow(dead_code)]
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
        let builder = AmqpSessionOptions::builder()
            .with_next_outgoing_id(1)
            .with_incoming_window(1)
            .with_outgoing_window(1)
            .with_handle_max(1)
            .with_offered_capabilities(vec!["capability".into()])
            .with_desired_capabilities(vec!["capability".into()])
            .with_properties(vec![("key", "value")])
            .with_buffer_size(1024);

        let session_options = builder.build();
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
